use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::Hash,
};

use ::feed_rs::model::Feed as FeedRs;
use ::rss::Channel;

use crate::{
    database::{
        storage::{Feed, FeedHeader, FeedItem},
        Database, FeedId,
    },
    feeds::rss::ChannelHeader,
};

pub mod feed_rs;
pub mod rss;

trait FeedConvert {
    type Output;
    fn convert(self) -> Self::Output;
}

impl<K, V> FeedConvert for HashMap<K, V>
where
    V: FeedConvert,
    K: Hash + Ord,
{
    type Output = BTreeMap<K, V::Output>;
    fn convert(self) -> Self::Output {
        self.into_iter().map(|(k, v)| (k, v.convert())).collect()
    }
}

impl<T> FeedConvert for Vec<T>
where
    T: FeedConvert,
{
    type Output = Vec<T::Output>;
    fn convert(self) -> Self::Output {
        self.into_iter().map(|v| v.convert()).collect()
    }
}

impl FeedConvert for String {
    type Output = String;

    fn convert(self) -> Self::Output {
        self
    }
}

impl<T> FeedConvert for Option<T>
where
    T: FeedConvert,
{
    type Output = Option<T::Output>;
    fn convert(self) -> Self::Output {
        self.map(|v| v.convert())
    }
}

pub struct UpdateFeedsTask {
    feeds: Vec<(FeedId, String, HashSet<ItemKey>, String)>,
    seq_no: u64,
}
impl UpdateFeedsTask {
    pub async fn run(self) -> UpdateFeedsTaskResult {
        let mut results = HashMap::new();

        'outer: for (feed_id, rss_feed, existing_feeds, name) in self.feeds {
            let mut retries = 0;
            let channel = loop {
                retries += 1;
                match download(&rss_feed).await {
                    Ok(res) => break res,
                    _ => {
                        if retries > 5 {
                            eprintln!("WARN: could not download {}", rss_feed);
                            continue 'outer;
                        }
                        // tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                }
            };
            let channel = match channel {
                Some(channel) => channel,
                None => continue,
            };

            let (header, mut current_feed_items) = channel.split_header();
            FeedItem::sort(&mut current_feed_items, |v| v);

            let (feed_headers, feeds): &mut (Vec<FeedHeader>, Vec<FeedItem>) =
                results.entry(feed_id).or_default();
            feed_headers.push(header);

            let mut header = true;
            for item in current_feed_items {
                let key = item_key(&item);
                if !existing_feeds.contains(&key) {
                    if header {
                        println!("New entries for [{}]", name);
                        header = false;
                    }
                    println!("  [{}]", item.display_title().unwrap_or(""));
                    feeds.push(item);
                }
            }
        }

        UpdateFeedsTaskResult {
            results,
            seq_no: self.seq_no,
        }
    }
}

pub struct UpdateFeedsTaskResult {
    results: HashMap<FeedId, (Vec<FeedHeader>, Vec<FeedItem>)>,
    seq_no: u64,
}

impl Database {
    pub fn update_feeds_task(&self) -> UpdateFeedsTask {
        let mut feeds = Vec::new();

        for (feed_id, source) in self.storage.iter() {
            /*
            println!(
                "Prepare to query RSS feed of [{}]...",
                &source.display_name()
            );
            */
            let rss_feed = match &source.feed_url {
                Some(rss) => rss,
                None => continue,
            };

            let mut existing = HashSet::new();
            for item in &source.feeds {
                let key = item_key(&item);
                existing.insert(key);
            }

            feeds.push((
                feed_id.clone(),
                rss_feed.clone(),
                existing,
                source.display_name().to_string(),
            ));
        }

        println!("Prepared query tasks");
        UpdateFeedsTask {
            feeds,
            seq_no: self.get_update_seq_no(),
        }
    }

    pub async fn commit_from(&mut self, results: UpdateFeedsTaskResult) {
        if results.seq_no != self.get_update_seq_no() {
            println!("Detected an update race condition, discarding",);
            return;
        }

        println!("Committing new items, seq_no={}...", results.seq_no);
        for (feed_id, (feed_headers, feed_items)) in results.results {
            if let Some(feed) = self.get_mut(&feed_id) {
                // println!("Commit feed of [{}]...", &feed.display_name());
                for feed_header in feed_headers {
                    if !feed.feed_headers.contains(&feed_header) {
                        feed.feed_headers.push(feed_header);
                    }
                }
                feed.feeds.extend(feed_items);
                FeedItem::sort(&mut feed.feeds, |v| v);
            }
        }
        self.last_feed_update = Some(chrono::Utc::now());
        self.set_update_seq_no(results.seq_no + 1);
        println!("  Done, seq_no={}", self.get_update_seq_no());
    }

    /*
    pub async fn update_feeds(&mut self) {
        let tasks = self.update_feeds_task();
        let results = tasks.run().await;
        self.commit_from(results).await;
    }
    */

    pub async fn import_from_rss(&mut self, url: &str, initial_tags: &[String]) {
        if let Some(feeds) = self.lookup.check_rss(url) {
            for feed in feeds {
                let source = self.storage.get_mut(feed).unwrap();
                source.tags.extend(initial_tags.iter().cloned());
            }
            return;
        }
        if let Some(channel) = download(url).await.unwrap() {
            let mut source = Feed::new(channel.title().to_owned());
            source.feed_url = Some(url.to_owned());
            source.tags.extend(initial_tags.iter().cloned());
            self.insert(source);
        }
    }
}

type ItemKey = (Option<String>, Option<String>);
fn item_key(item: &FeedItem) -> ItemKey {
    match item {
        FeedItem::Rss(item) => (item.title.clone(), item.pub_date.clone()),
        FeedItem::FeedRs(item) => (
            item.title.as_ref().map(|text| text.content.clone()),
            item.published.as_ref().map(|v| v.to_rfc2822()),
        ),
    }
}

pub enum FeedDownload {
    Rss(Channel),
    Feed(FeedRs),
}

impl FeedDownload {
    fn title(&self) -> &str {
        match self {
            FeedDownload::Rss(x) => &x.title,
            FeedDownload::Feed(x) => x
                .title
                .as_ref()
                .map(|v| v.content.as_str())
                .unwrap_or_default(),
        }
    }
    fn split_header(self) -> (FeedHeader, Vec<FeedItem>) {
        match self {
            FeedDownload::Rss(feed) => {
                let (header, items) = ChannelHeader::split(feed);
                (
                    FeedHeader::Rss(header),
                    items.into_iter().map(FeedItem::Rss).collect(),
                )
            }
            FeedDownload::Feed(feed) => {
                let (header, items) = self::feed_rs::FeedHeader::split(feed);
                (
                    FeedHeader::FeedRs(header),
                    items.into_iter().map(FeedItem::FeedRs).collect(),
                )
            }
        }
    }
}

async fn download(url: &str) -> reqwest::Result<Option<FeedDownload>> {
    let res = reqwest::get(url).await?;
    let body = res.bytes().await?;
    if let Ok(chan) = Channel::read_from(&body[..]) {
        return Ok(Some(FeedDownload::Rss(chan)));
    }

    if let Ok(alt) = ::feed_rs::parser::parse_with_uri(&body[..], Some(url)) {
        return Ok(Some(FeedDownload::Feed(alt)));
    }

    Ok(None)
}
