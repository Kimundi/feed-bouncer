use std::sync::Arc;

use feed_bouncer_database::{Database, Feed};
use rocket::tokio::sync::RwLock;

#[derive(serde::Serialize)]
pub struct Item<'a> {
    pub feed_name: &'a str,
    pub feed_id: &'a str,
    pub item_name: &'a str,
    pub content_link: Option<&'a str>,
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
    Has(String),
    HasNot(String),
}
pub struct Filter {
    pattern: Vec<FilterPattern>,
    raw: String,
    exact: bool,
}

impl Filter {
    pub fn new(raw: Option<String>) -> Self {
        let raw = raw.unwrap_or_default();
        let mut pattern = Vec::new();
        let mut exact = false;
        for raw in raw.split(',') {
            let raw = raw.trim();
            let (raw, negated) = if let Some(raw) = raw.strip_prefix("!") {
                (raw.trim(), true)
            } else {
                (raw.trim(), false)
            };
            if raw == "=" {
                exact = true;
                continue;
            }
            if raw.is_empty()
                || raw
                    .chars()
                    .any(|c| !"abcdefghijklmnopqrstuvwxyz_".contains(c))
            {
                continue;
            }
            if negated {
                pattern.push(FilterPattern::HasNot(raw.to_owned()));
            } else {
                pattern.push(FilterPattern::Has(raw.to_owned()));
            }
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
                    if !feed.tags.contains(tag) {
                        return false;
                    }
                    matches += 1;
                }
                FilterPattern::HasNot(tag) => {
                    if feed.tags.contains(tag) {
                        return false;
                    }
                }
            }
        }

        (!self.exact) || (matches == feed.tags.len())
    }
    pub fn raw(&self) -> &str {
        &self.raw
    }
    pub fn raw_opt(&self) -> Option<&str> {
        (!self.raw.is_empty()).then(|| &self.raw[..])
    }
}
