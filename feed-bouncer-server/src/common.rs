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

pub type SyncDatabase = Arc<RwLock<Database>>;

pub enum FilterPattern {
    Has(String),
    HasNot(String),
}
pub struct Filter {
    pattern: Vec<FilterPattern>,
}

impl Filter {
    pub fn new(raw: Option<String>) -> Self {
        let raw = raw.unwrap_or_default();
        let mut pattern = Vec::new();
        for raw in raw.split(',') {
            let raw = raw.trim();
            let (raw, negated) = if let Some(raw) = raw.strip_prefix("!") {
                (raw.trim(), true)
            } else {
                (raw.trim(), false)
            };
            if raw.is_empty() {
                continue;
            }
            if negated {
                pattern.push(FilterPattern::HasNot(raw.to_owned()));
            } else {
                pattern.push(FilterPattern::Has(raw.to_owned()));
            }
        }

        Self { pattern }
    }
    pub fn matches(&self, feed: &Feed) -> bool {
        for pattern in &self.pattern {
            match pattern {
                FilterPattern::Has(tag) => {
                    if !feed.tags.contains(tag) {
                        return false;
                    }
                }
                FilterPattern::HasNot(tag) => {
                    if feed.tags.contains(tag) {
                        return false;
                    }
                }
            }
        }
        true
    }
}
