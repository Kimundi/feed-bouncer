use std::collections::BTreeSet;

use crate::database::{
    storage_feed_header::{FeedHeader, FeedHeaderMeta},
    storage_feed_item::{FeedItem, FeedItemMeta},
    FeedId, LookupKey,
};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Feed {
    name: String,
    feed_url: Option<String>,
    opml: Option<opml::Outline>,

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

    parent: Option<FeedId>,
    #[serde(default)]
    tags: BTreeSet<String>,
    #[serde(default)]
    title_aliases: BTreeSet<String>,
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

    pub fn items(&self) -> &[FeedItemMeta] {
        &self.feeds_v2
    }
    pub fn items_mut(&mut self) -> &mut [FeedItemMeta] {
        &mut self.feeds_v2
    }
    pub fn push_item(&mut self, item: FeedItem) {
        self.feeds_v2
            .push(FeedItemMeta::new(self.feeds_counter, item));
        self.feeds_counter += 1;
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn feed_url(&self) -> Option<&str> {
        self.feed_url.as_deref()
    }
    pub fn feed_url_mut(&mut self) -> &mut Option<String> {
        &mut self.feed_url
    }
    pub fn opml(&self) -> Option<&opml::Outline> {
        self.opml.as_ref()
    }
    pub fn opml_mut(&mut self) -> &mut Option<opml::Outline> {
        &mut self.opml
    }
    pub fn set_parent(&mut self, parent: Option<String>) {
        self.parent = parent;
    }
    pub fn title_aliases(&self) -> &BTreeSet<String> {
        &self.title_aliases
    }
    pub fn title_alias_insert(&mut self, name: &str) -> bool {
        self.title_aliases.insert(name.trim().to_owned())
    }
    pub fn title_alias_remove(&mut self, name: &str) -> bool {
        let mut keys = Vec::new();
        for title_alias in &self.title_aliases {
            if title_alias == name.trim() {
                keys.push(title_alias.to_owned());
            }
        }
        let mut was_deleted = false;
        for key in keys {
            was_deleted |= self.title_aliases.remove(&key);
        }
        if !was_deleted {
            println!("did not delete {:?}, not found", name);
            for candidate in &self.title_aliases {
                println!("title: {:?}", candidate);
            }
        }
        was_deleted
    }
}
