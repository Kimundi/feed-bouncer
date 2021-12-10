use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use chrono::{DateTime, FixedOffset};

use crate::database::{FeedId, LookupKey, SourceLookup};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum FeedHeader {
    Rss(crate::feeds::rss::ChannelHeader),
    FeedRs(crate::feeds::feed_rs::FeedHeader),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum FeedItem {
    Rss(crate::feeds::rss::Item),
    FeedRs(crate::feeds::feed_rs::Entry),
}

impl FeedItem {
    pub fn display_title(&self) -> Option<&str> {
        match self {
            FeedItem::Rss(item) => item.title.as_deref().map(str::trim),
            FeedItem::FeedRs(entry) => entry.title.as_ref().map(|v| v.content.trim()),
        }
    }
    pub fn publish_date(&self) -> Option<DateTime<FixedOffset>> {
        match self {
            FeedItem::Rss(item) => item.pub_date.as_ref().and_then(|v| {
                // NB: Some feeds have a non-rfc2822 conform date format where
                // they write out the day fully. We fix this here.
                let mut v = Cow::Borrowed(v);
                for day in [
                    "Monday",
                    "Tuesday",
                    "Wednesday",
                    "Thursday",
                    "Friday",
                    "Saturday",
                    "Sunday",
                ] {
                    if v.contains(day) {
                        v = Cow::Owned(v.replace(day, &day[..3]));
                    }
                }
                let res = DateTime::parse_from_rfc2822(&v);
                if res.is_err() {
                    println!("Could not parse date {}", v);
                }
                res.ok()
            }),
            FeedItem::FeedRs(entry) => entry.published.as_ref().map(|v| {
                // TODO: This is ugly
                DateTime::parse_from_rfc2822(&v.to_rfc2822()).unwrap()
            }),
        }
    }
    pub fn publish_date_or_old(&self) -> DateTime<FixedOffset> {
        self.publish_date().unwrap_or_else(old_date)
    }
    pub fn sort<T, F: FnMut(&T) -> &Self>(items: &mut [T], mut f: F) {
        items.sort_by_cached_key(|k| f(k).publish_date_or_old());
    }
    fn strip_prefix<'a>(mut t: &'a str, prefix: &str) -> &'a str {
        t = t.trim();
        t = t.strip_prefix(prefix).unwrap_or(t).trim();
        t = t.strip_prefix("-").unwrap_or(t).trim();
        t = t.strip_prefix(":").unwrap_or(t).trim();
        t
    }
    pub fn display_title_without_prefixes(&self, feed: &Feed) -> Option<&str> {
        self.display_title().map(|mut t| {
            for a in feed.titles() {
                t = Self::strip_prefix(t, a);
            }
            t
        })
    }
    pub fn content_link(&self) -> Option<&str> {
        match self {
            FeedItem::Rss(item) => item.link.as_deref(),
            FeedItem::FeedRs(entry) => entry.links.first().map(|link| &link.href[..]),
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
    tags: BTreeSet<String>,
    #[serde(default)]
    pub title_aliases: BTreeSet<String>,
    #[serde(default)]
    display_name: Option<String>,
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
            title_aliases: BTreeSet::new(),
            display_name: None,
            _private: (),
        }
    }
    pub fn display_name(&self) -> &str {
        self.display_name.as_deref().unwrap_or(&self.name).trim()
    }
    pub fn original_display_name(&self) -> &str {
        self.name.trim()
    }
    pub fn key(&self) -> LookupKey<'_> {
        LookupKey {
            name: &self.name,
            feed_url: self.feed_url.as_deref(),
        }
    }
    pub fn tags(&self) -> impl Iterator<Item = &str> {
        self.tags.iter().map(|s| &s[..])
    }
    pub fn extend_tags<'a>(&mut self, iter: impl IntoIterator<Item = &'a str>) -> bool {
        let mut ret = false;
        for s in iter {
            ret |= self.tags.insert(s.to_owned());
        }
        ret
    }
    pub fn contains_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        self.tags.remove(tag)
    }
    pub fn titles(&self) -> impl Iterator<Item = &str> {
        std::slice::from_ref(&self.name)
            .iter()
            .chain(self.title_aliases.iter())
            .map(|s| &s[..])
    }
    pub fn set_display_name(&mut self, name: String) {
        self.display_name = Some(name);
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
            crate::safe_save_json(source, &file_path, "database", false);
        }
    }
    pub fn save_shrunk(&self, path: &Path) {
        let feed_path = path.join("feeds");
        std::fs::create_dir_all(&feed_path).unwrap();
        for (feed_id, source) in self.iter() {
            let file_path = feed_path.join(feed_id).with_extension("json");
            crate::safe_save_json(source, &file_path, "database", true);
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
    /*
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&FeedId, &mut Feed)> + '_ {
        self.sources.iter_mut()
    }
    */
    pub fn get_or_insert(&mut self, feed_id: FeedId, feed: &Feed) -> &mut Feed {
        self.sources.entry(feed_id).or_insert_with(|| feed.clone())
    }
    pub fn get(&self, feed_id: &FeedId) -> Option<&Feed> {
        self.sources.get(feed_id)
    }
    pub fn get_mut(&mut self, feed_id: &FeedId) -> Option<&mut Feed> {
        self.sources.get_mut(feed_id)
    }
}

fn old_date() -> DateTime<FixedOffset> {
    chrono::DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap()
}

#[test]
fn test_old_date() {
    old_date();
}
