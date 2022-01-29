use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    path::PathBuf,
};

use chrono::{DateTime, Utc};

use crate::database::{
    storage::Storage,
    storage_feed::Feed,
    storage_feed_item::{FeedItem, FeedItemMeta},
    user_data::UserDataStorage,
};

pub mod storage;
pub mod storage_feed;
pub mod storage_feed_header;
pub mod storage_feed_item;
pub mod user_data;

pub type FeedId = String;

#[derive(Default)]
pub struct SourceLookup {
    rss_lookup: HashMap<String, HashSet<FeedId>>,
    title_lookup: HashMap<String, HashSet<FeedId>>,
}

pub struct LookupKey<'a> {
    name: &'a str,
    feed_url: Option<&'a str>,
}

impl SourceLookup {
    fn touch(&mut self, feed_id: &FeedId, key: LookupKey<'_>) {
        self.title_lookup
            .entry(key.name.to_owned())
            .or_default()
            .insert(feed_id.clone());

        if let Some(rss) = key.feed_url {
            self.rss_lookup
                .entry(rss.to_owned())
                .or_default()
                .insert(feed_id.clone());
        }
    }
    fn check(&self, key: LookupKey<'_>) -> Option<FeedId> {
        let title_matches = self.title_lookup.get(key.name).cloned().unwrap_or_default();
        let rss_matches = if let Some(rss) = key.feed_url {
            self.rss_lookup.get(rss).cloned().unwrap_or_default()
        } else {
            HashSet::new()
        };
        if rss_matches.len() == 1 {
            return rss_matches.into_iter().next();
        }
        if title_matches.len() == 1 {
            return rss_matches.into_iter().next();
        }
        if rss_matches.len() > 1 || title_matches.len() > 1 {
            eprintln!("WARN: Multiple matches for {}", key.name);
        }
        None
    }
    pub fn check_rss(&self, url: &str) -> Option<&HashSet<FeedId>> {
        self.rss_lookup.get(url)
    }
}

pub struct Database {
    pub(crate) storage: Storage,
    pub(crate) user_data_storage: UserDataStorage,
    pub(crate) storage_path: PathBuf,
    pub(crate) lookup: SourceLookup,
    pub(crate) last_feed_update: Option<DateTime<Utc>>,
    pub(crate) update_seq_no: u64,
}

impl Database {
    pub fn init(storage_path: Option<PathBuf>) -> Self {
        let storage_path: PathBuf = storage_path.unwrap_or_else(|| "./storage".into());
        let storage = Storage::open_or_default(&storage_path);
        let user_data_storage = UserDataStorage::open_or_default(&storage_path);

        let mut ret = Self {
            storage,
            user_data_storage,
            storage_path,
            lookup: SourceLookup::default(),
            last_feed_update: None,
            update_seq_no: 0,
        };
        ret.recreate_cache();
        ret
    }

    fn recreate_cache(&mut self) {
        self.storage.write_to_cache(&mut self.lookup);
    }

    pub fn save(&mut self) {
        self.storage.save(&self.storage_path);
        self.user_data_storage.save(&self.storage_path);
    }

    pub fn save_shrunk(&mut self) {
        self.storage.save_shrunk(&self.storage_path);
        self.user_data_storage.save(&self.storage_path);
    }

    pub fn insert(&mut self, mut item: Feed) -> FeedId {
        let feed_id = match self.lookup.check(item.key()) {
            Some(feed_id) => feed_id,
            None => {
                use sha2::Digest;
                let mut hash = sha2::Sha256::new();
                hash.update(item.name());
                if let Some(rss) = item.feed_url() {
                    hash.update(rss);
                }
                let hash = hash.finalize();
                let hash = format!("{:x}", hash);
                hash
            }
        };
        let ret = feed_id.clone();

        self.lookup.touch(&feed_id, item.key());
        let existing_entry = self.storage.get_or_insert(feed_id, &item);

        update_or_warn(existing_entry.feed_url_mut(), item.feed_url_mut().take());
        warn_if_not_equal(existing_entry.name(), item.name());
        update_or_warn(existing_entry.opml_mut(), item.opml_mut().take());
        existing_entry.extend_tags(item.tags());

        ret
    }

    pub fn get_items_ordered_by_time(&self) -> Vec<(&FeedId, &Feed, &FeedItemMeta)> {
        let mut items = Vec::new();
        for (feed_id, feed) in self.storage.iter() {
            for item in feed.feeds() {
                items.push((feed_id, feed, item));
            }
        }

        FeedItem::sort(&mut items, |v| &v.2.item);

        items
    }

    pub fn get_feeds(&self) -> Vec<(&FeedId, &Feed)> {
        self.storage.iter().collect()
    }

    pub fn last_feed_update(&self) -> &Option<DateTime<Utc>> {
        &self.last_feed_update
    }

    pub fn get(&self, feed_id: &FeedId) -> Option<&Feed> {
        self.storage.get(feed_id)
    }

    pub fn get_mut(&mut self, feed_id: &FeedId) -> Option<&mut Feed> {
        self.storage.get_mut(feed_id)
    }

    pub fn get_update_seq_no(&self) -> u64 {
        self.update_seq_no
    }

    pub fn set_update_seq_no(&mut self, v: u64) {
        assert!(self.update_seq_no < v);
        self.update_seq_no = v;
    }
}

fn warn_if_not_equal<T: PartialEq + Debug + ?Sized>(dst: &T, value: &T) {
    if dst != value {
        eprintln!("WARN: Mismatching value {:?} != {:?}", dst, value);
    }
}

fn update_or_warn<T: PartialEq + Debug>(dst: &mut Option<T>, value: Option<T>) {
    let value = match value {
        Some(value) => value,
        None => return,
    };
    match dst {
        Some(dst) => {
            warn_if_not_equal(dst, &value);
        }
        None => *dst = Some(value),
    }
}
