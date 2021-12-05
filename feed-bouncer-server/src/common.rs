use std::sync::Arc;

use feed_bouncer_database::{Database, Feed};
use rocket::tokio::sync::RwLock;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ItemBase<S> {
    pub feed_name: S,
    pub feed_id: S,
    pub item_name: S,
    pub content_link: Option<S>,
    pub show_feed: bool,
}

pub type Item<'a> = ItemBase<&'a str>;
pub type ItemOwned = ItemBase<String>;

#[derive(serde::Serialize)]
pub struct ItemsGroup<'a> {
    items: Vec<Item<'a>>,
}

#[derive(serde::Serialize)]
pub struct ItemsGroups<'a> {
    item_groups: Vec<ItemsGroup<'a>>,
}

impl<'a> ItemsGroups<'a> {
    pub fn new(items: Vec<Item<'a>>) -> Self {
        Self {
            item_groups: vec![ItemsGroup { items }],
        }
    }
}

#[derive(serde::Serialize)]
pub struct Nav<'a> {
    last_update: Option<String>,
    filter: &'a str,
    home_link: String,
    feeds_link: String,
}

impl<'a> Nav<'a> {
    pub fn new(db: &Database, filter: &'a Filter) -> Self {
        Self {
            last_update: db.last_feed_update().map(|v| v.to_rfc3339()),
            filter: filter.raw(),
            home_link: uri!(crate::index::index(filter.raw_opt())).to_string(),
            feeds_link: uri!(crate::feeds::feeds(filter.raw_opt())).to_string(),
        }
    }
}

pub type SyncDatabase = Arc<RwLock<Database>>;

pub enum FilterPattern {
    Has(Tag),
    HasNot(Tag),
}
pub struct Filter {
    pattern: Vec<FilterPattern>,
    raw: String,
    exact: bool,
}

pub const VALID_TAG_CHARS: &str = "abcdefghijklmnopqrstuvwxyz_";

#[derive(Debug, Clone)]
pub struct Tag(String);
impl Tag {
    pub fn new(raw: &str) -> Option<Self> {
        let raw = raw.trim();
        if raw.is_empty() || raw.chars().any(|c| !VALID_TAG_CHARS.contains(c)) {
            return None;
        }
        Some(Self(raw.to_owned()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Filter {
    pub fn new(raw: Option<String>) -> Self {
        let raw = raw.unwrap_or_default();
        let mut pattern = Vec::new();
        let mut exact = false;
        for raw in raw.split(',') {
            let raw = raw.trim();
            if raw == "=" {
                exact = true;
                continue;
            }
            let (raw, pat) = raw
                .strip_prefix("!")
                .map(|raw| (raw, FilterPattern::HasNot as fn(_) -> _))
                .unwrap_or((raw, FilterPattern::Has));
            let tag = match Tag::new(raw) {
                Some(tag) => tag,
                None => continue,
            };
            pattern.push(pat(tag));
        }

        Self {
            pattern,
            raw,
            exact,
        }
    }
    pub fn matches(&self, feed: &Feed) -> bool {
        let mut matches: usize = 0;
        for pattern in &self.pattern {
            match pattern {
                FilterPattern::Has(tag) => {
                    if !feed.contains_tag(tag.as_str()) {
                        return false;
                    }
                    matches += 1;
                }
                FilterPattern::HasNot(tag) => {
                    if feed.contains_tag(tag.as_str()) {
                        return false;
                    }
                }
            }
        }

        (!self.exact) || (matches == feed.tags().count())
    }
    pub fn raw(&self) -> &str {
        &self.raw
    }
    pub fn raw_opt(&self) -> Option<&str> {
        (!self.raw.is_empty()).then(|| &self.raw[..])
    }
}
