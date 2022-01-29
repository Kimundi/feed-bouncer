use std::collections::BTreeSet;

use crate::database::{
    storage_feed_header::{FeedHeader, FeedHeaderMeta},
    storage_feed_item::{FeedItem, FeedItemMeta},
    FeedId, LookupKey,
};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Feed {
    pub name: String,
    pub feed_url: Option<String>,
    pub opml: Option<opml::Outline>,

    #[serde(default)]
    feed_headers: Vec<FeedHeader>,
    #[serde(default)]
    feed_headers_v2: Vec<FeedHeaderMeta>,
    #[serde(default)]
    feed_headers_counter: usize,

    #[serde(default)]
    feeds: Vec<FeedItem>,
    #[serde(default)]
    feeds_v2: Vec<FeedItemMeta>,
    #[serde(default)]
    feeds_counter: usize,

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
            feed_headers_v2: Vec::new(),
            feed_headers_counter: 0,

            feeds: Vec::new(),
            feeds_v2: Vec::new(),
            feeds_counter: 0,

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
    pub fn migrate_data(&mut self) {
        for header in self.feed_headers.drain(..) {
            self.feed_headers_v2
                .push(FeedHeaderMeta::new(self.feed_headers_counter, header));
            self.feed_headers_counter += 1;
        }

        for item in self.feeds.drain(..) {
            self.feeds_v2
                .push(FeedItemMeta::new(self.feeds_counter, item));
            self.feeds_counter += 1;
        }
    }

    pub fn feed_headers(&self) -> &[FeedHeaderMeta] {
        &self.feed_headers_v2
    }
    pub fn contains_feed_header(&self, h: &FeedHeader) -> bool {
        self.feed_headers_v2.iter().any(|v| v.header == *h)
    }
    pub fn push_feed_header(&mut self, header: FeedHeader) {
        self.feed_headers_v2
            .push(FeedHeaderMeta::new(self.feed_headers_counter, header));
        self.feed_headers_counter += 1;
    }

    pub fn feeds(&self) -> &[FeedItemMeta] {
        &self.feeds_v2
    }
    pub fn feeds_mut(&mut self) -> &mut [FeedItemMeta] {
        &mut self.feeds_v2
    }
    pub fn push_feed(&mut self, item: FeedItem) {
        self.feeds_v2
            .push(FeedItemMeta::new(self.feeds_counter, item));
        self.feeds_counter += 1;
    }
}
