use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use chrono::{DateTime, FixedOffset};

use crate::{
    database::{FeedId, LookupKey, SourceLookup},
    rss_utils::{ChannelHeader, Item},
};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum FeedHeader {
    Rss(ChannelHeader),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum FeedItem {
    Rss(Item),
}

impl FeedItem {
    pub fn display_title(&self) -> Option<&str> {
        match self {
            FeedItem::Rss(item) => item.title.as_deref().map(str::trim),
        }
    }
    pub fn publish_date(&self) -> Option<DateTime<FixedOffset>> {
        match self {
            FeedItem::Rss(item) => item
                .pub_date
                .as_ref()
                .and_then(|v| DateTime::parse_from_rfc2822(v).ok()),
        }
    }
    pub fn publish_date_or_old(&self) -> DateTime<FixedOffset> {
        self.publish_date()
            .unwrap_or_else(|| chrono::DateTime::parse_from_rfc3339("1990-01-01T00:00:00").unwrap())
    }
    pub fn sort<T, F: FnMut(&T) -> &Self>(items: &mut [T], mut f: F) {
        items.sort_by_cached_key(|k| f(k).publish_date_or_old());
    }
    pub fn display_title_without_prefix(&self, prefix: &str) -> Option<&str> {
        self.display_title().map(|t| {
            t.trim()
                .strip_prefix(prefix)
                .unwrap_or(t)
                .trim()
                .strip_prefix("-")
                .unwrap_or(t)
                .trim()
        })
    }
    pub fn content_link(&self) -> Option<&str> {
        match self {
            FeedItem::Rss(item) => item.link.as_deref(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Feed {
    pub name: String,
    pub feed_url: Option<String>,
    pub opml: Option<opml::Outline>,
    #[serde(default)]
    pub feed_headers: Vec<FeedHeader>,
    #[serde(default)]
    pub feeds: Vec<FeedItem>,
    pub parent: Option<FeedId>,
    #[serde(default)]
    pub tags: BTreeSet<String>,
    #[serde(skip)]
    _private: (),
}

impl Feed {
    pub fn new(name: String) -> Self {
        Self {
            name,
            feed_url: None,
            opml: None,
            feed_headers: Vec::new(),
            feeds: Vec::new(),
            parent: None,
            tags: BTreeSet::new(),
            _private: (),
        }
    }
    pub fn display_name(&self) -> &str {
        self.name.trim()
    }
    pub fn key(&self) -> LookupKey<'_> {
        LookupKey {
            name: &self.name,
            feed_url: self.feed_url.as_deref(),
        }
    }
}

#[derive(Default)]
pub struct Storage {
    sources: BTreeMap<FeedId, Feed>,
}

impl Storage {
    fn open(path: &Path) -> std::io::Result<Self> {
        let feed_path = path.join("feeds");
        let mut sources = BTreeMap::new();
        for e in std::fs::read_dir(&feed_path)? {
            let e = e?;
            let feed_file = e.path();
            let id = feed_file
                .file_stem()
                .expect("file does not have a name")
                .to_str()
                .expect("file does not have unicode name")
                .to_owned();
            let file = std::fs::read_to_string(&feed_file)?;
            let feed: Feed =
                serde_json::from_str(&file).expect("file could be read, but not parsed");
            sources.insert(id, feed);
        }
        Ok(Self { sources })
    }
    pub fn open_or_default(storage_path: &Path) -> Self {
        Self::open(&storage_path).unwrap_or_default()
    }
    pub fn save(&self, path: &Path) {
        let feed_path = path.join("feeds");
        std::fs::create_dir_all(&feed_path).unwrap();
        for (feed_id, source) in self.iter() {
            let file_path = feed_path.join(feed_id).with_extension("json");
            super::safe_save_json(source, &file_path);
        }
    }

    pub fn write_to_cache(&self, lookup: &mut SourceLookup) {
        for (feed_id, source) in &self.sources {
            lookup.touch(feed_id, source.key());
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (&FeedId, &Feed)> + '_ {
        self.sources.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&FeedId, &mut Feed)> + '_ {
        self.sources.iter_mut()
    }
    pub fn get_or_insert(&mut self, feed_id: FeedId, feed: &Feed) -> &mut Feed {
        self.sources.entry(feed_id).or_insert_with(|| feed.clone())
    }
    pub fn get_mut(&mut self, feed_id: &FeedId) -> Option<&mut Feed> {
        self.sources.get_mut(feed_id)
    }
}
