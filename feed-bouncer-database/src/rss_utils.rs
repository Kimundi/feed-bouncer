use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::Hash,
};

use rss::{
    extension::{atom, dublincore, itunes, syndication},
    Category, Channel, Cloud, Image, TextInput,
};

use crate::database::{
    storage::{Feed, FeedHeader, FeedItem},
    Database,
};

pub type ExtensionMap = BTreeMap<String, BTreeMap<String, Vec<Extension>>>;

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Extension {
    pub name: String,
    pub value: Option<String>,
    pub attrs: BTreeMap<String, String>,
    pub children: BTreeMap<String, Vec<Extension>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct ChannelHeader {
    /// The name of the channel.
    pub title: String,
    /// The URL for the website corresponding to the channel.
    pub link: String,
    /// A description of the channel.
    pub description: String,
    /// The language of the channel.
    pub language: Option<String>,
    /// The copyright notice for the channel.
    pub copyright: Option<String>,
    /// The email address for the managing editor.
    pub managing_editor: Option<String>,
    /// The email address for the webmaster.
    pub webmaster: Option<String>,
    /// The publication date for the content of the channel as an RFC822 timestamp.
    pub pub_date: Option<String>,
    /// The date that the contents of the channel last changed as an RFC822 timestamp.
    pub last_build_date: Option<String>,
    /// The categories the channel belongs to.
    pub categories: Vec<Category>,
    /// A string indicating the program used to generate the channel.
    pub generator: Option<String>,
    /// A URL that points to the documentation for the RSS format.
    pub docs: Option<String>,
    /// The cloud to register with to be notified of updates to the channel.
    pub cloud: Option<Cloud>,
    /// The PICS rating for the channel.
    pub rating: Option<String>,
    /// The number of minutes the channel can be cached before refreshing.
    pub ttl: Option<String>,
    /// An image that can be displayed with the channel.
    pub image: Option<Image>,
    /// A text input box that can be displayed with the channel.
    pub text_input: Option<TextInput>,
    /// A hint to tell the aggregator which hours it can skip.
    pub skip_hours: Vec<String>,
    /// A hint to tell the aggregator which days it can skip.
    pub skip_days: Vec<String>,
    /// The extensions for the channel.
    pub extensions: ExtensionMap,
    /// The Atom extension for the channel.
    pub atom_ext: Option<atom::AtomExtension>,
    /// The iTunes extension for the channel.
    pub itunes_ext: Option<itunes::ITunesChannelExtension>,
    /// The Dublin Core extension for the channel.
    pub dublin_core_ext: Option<dublincore::DublinCoreExtension>,
    /// The Syndication extension for the channel.
    pub syndication_ext: Option<syndication::SyndicationExtension>,
    /// The namespaces present in the RSS tag.
    pub namespaces: BTreeMap<String, String>,
}

impl ChannelHeader {
    pub fn split(channel: Channel) -> (Self, Vec<Item>) {
        let Channel {
            title,
            link,
            description,
            language,
            copyright,
            managing_editor,
            webmaster,
            pub_date,
            last_build_date,
            categories,
            generator,
            docs,
            cloud,
            rating,
            ttl,
            image,
            text_input,
            skip_hours,
            skip_days,
            items,
            extensions,
            atom_ext,
            itunes_ext,
            dublin_core_ext,
            syndication_ext,
            namespaces,
        } = channel;

        (
            Self {
                title,
                link,
                description,
                language,
                copyright,
                managing_editor,
                webmaster,
                pub_date,
                last_build_date,
                categories,
                generator,
                docs,
                cloud,
                rating,
                ttl,
                image,
                text_input,
                skip_hours,
                skip_days,

                extensions: extensions.hash_2_tree(),
                atom_ext,
                itunes_ext,
                dublin_core_ext,
                syndication_ext,
                namespaces: namespaces.into_iter().collect(),
            },
            items.hash_2_tree(),
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Item {
    /// The title of the item.
    pub title: Option<String>,
    /// The URL of the item.
    pub link: Option<String>,
    /// The item synopsis.
    pub description: Option<String>,
    /// The email address of author of the item.
    pub author: Option<String>,
    /// The categories the item belongs to.
    pub categories: Vec<Category>,
    /// The URL for the comments page of the item.
    pub comments: Option<String>,
    /// The description of a media object that is attached to the item.
    pub enclosure: Option<rss::Enclosure>,
    /// A unique identifier for the item.
    pub guid: Option<rss::Guid>,
    /// The date the item was published as an RFC 2822 timestamp.
    pub pub_date: Option<String>,
    /// The RSS channel the item came from.
    pub source: Option<rss::Source>,
    /// The HTML contents of the item.
    pub content: Option<String>,
    /// The extensions for the item.
    pub extensions: ExtensionMap,
    /// The Atom extension for the channel.
    pub atom_ext: Option<atom::AtomExtension>,
    /// The iTunes extension for the item.
    pub itunes_ext: Option<itunes::ITunesItemExtension>,
    /// The Dublin Core extension for the item.
    pub dublin_core_ext: Option<dublincore::DublinCoreExtension>,
}

trait Hash2Tree {
    type Output;
    fn hash_2_tree(self) -> Self::Output;
}

impl<K, V> Hash2Tree for HashMap<K, V>
where
    V: Hash2Tree,
    K: Hash + Ord,
{
    type Output = BTreeMap<K, V::Output>;
    fn hash_2_tree(self) -> Self::Output {
        self.into_iter()
            .map(|(k, v)| (k, v.hash_2_tree()))
            .collect()
    }
}

impl<T> Hash2Tree for Vec<T>
where
    T: Hash2Tree,
{
    type Output = Vec<T::Output>;
    fn hash_2_tree(self) -> Self::Output {
        self.into_iter().map(|v| v.hash_2_tree()).collect()
    }
}

impl Hash2Tree for rss::extension::Extension {
    type Output = Extension;

    fn hash_2_tree(self) -> Self::Output {
        let rss::extension::Extension {
            name,
            value,
            attrs,
            children,
        } = self;

        Extension {
            name,
            value,
            attrs: attrs.hash_2_tree(),
            children: children.hash_2_tree(),
        }
    }
}

impl Hash2Tree for String {
    type Output = String;

    fn hash_2_tree(self) -> Self::Output {
        self
    }
}

impl Hash2Tree for rss::Item {
    type Output = Item;

    fn hash_2_tree(self) -> Self::Output {
        let rss::Item {
            title,
            link,
            description,
            author,
            categories,
            comments,
            enclosure,
            guid,
            pub_date,
            source,
            content,
            extensions,
            atom_ext,
            itunes_ext,
            dublin_core_ext,
        } = self;

        Item {
            title,
            link,
            description,
            author,
            categories,
            comments,
            enclosure,
            guid,
            pub_date,
            source,
            content,
            extensions: extensions.hash_2_tree(),
            atom_ext,
            itunes_ext,
            dublin_core_ext,
        }
    }
}

impl Database {
    pub async fn update_feeds(&mut self) {
        'outer: for (_feed_id, source) in self.storage.iter_mut() {
            println!("Query RSS feed of [{}]...", &source.display_name());
            let rss_feed = match &source.feed_url {
                Some(rss) => rss,
                None => continue,
            };

            let mut retries = 0;
            let channel = loop {
                retries += 1;
                match download(rss_feed).await {
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

            let (header, current_feed_items) = ChannelHeader::split(channel);
            let mut current_feed_items: Vec<_> =
                current_feed_items.into_iter().map(FeedItem::Rss).collect();
            FeedItem::sort(&mut current_feed_items, |v| v);
            let header = FeedHeader::Rss(header);

            if !source.feed_headers.contains(&header) {
                source.feed_headers.push(header);
            }

            let mut existing = HashSet::new();
            for item in &source.feeds {
                let key = item_key(&item);
                existing.insert(key);
            }

            for item in current_feed_items {
                let key = item_key(&item);
                if !existing.contains(&key) {
                    println!("  Add [{}]", item.display_title().unwrap_or(""));
                    source.feeds.push(item);
                }
            }

            FeedItem::sort(&mut source.feeds, |v| v);
        }

        self.last_feed_update = Some(chrono::Utc::now());
        println!("Done");
    }

    pub async fn import_from_rss(&mut self, url: &str, initial_tags: &[String]) {
        if let Some(feeds) = self.lookup.check_rss(url) {
            for feed in feeds {
                let source = self.storage.get_mut(feed).unwrap();
                source.tags.extend(initial_tags.iter().cloned());
            }
            return;
        }
        if let Some(channel) = download(url).await.unwrap() {
            let mut source = Feed::new(channel.title.clone());
            source.feed_url = Some(url.to_owned());
            source.tags.extend(initial_tags.iter().cloned());
            self.insert(source);
        }
    }
}

fn item_key(item: &FeedItem) -> (Option<String>, Option<String>) {
    match item {
        FeedItem::Rss(item) => (item.title.clone(), item.pub_date.clone()),
    }
}

async fn download(url: &str) -> reqwest::Result<Option<Channel>> {
    let res = reqwest::get(url).await?;
    let body = res.bytes().await?;
    let chan = Channel::read_from(&body[..]);
    if chan.is_err() {
        //let s = String::from_utf8_lossy(&body[..1000]);
        //eprintln!("{}", s);
        //eprintln!("invalid rss at {}", url);
        let _alt = feed_rs::parser::parse_with_uri(&body[..], Some(url));
    }
    Ok(chan.ok())
}
