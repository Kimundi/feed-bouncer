use std::borrow::Cow;

use chrono::{DateTime, FixedOffset};

use crate::database::storage_feed::Feed;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum FeedItem {
    Rss(crate::feeds::rss::Item),
    FeedRs(crate::feeds::feed_rs::Entry),
}

impl FeedItem {
    fn publish_date(&self) -> Option<DateTime<FixedOffset>> {
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
    fn publish_date_or_old(&self) -> DateTime<FixedOffset> {
        self.publish_date().unwrap_or_else(old_date)
    }
    pub fn sort<T, F: FnMut(&T) -> &Self>(items: &mut [T], mut f: F) {
        items.sort_by_cached_key(|k| f(k).publish_date_or_old());
    }
    pub(crate) fn display_title(&self) -> Option<&str> {
        match self {
            FeedItem::Rss(item) => item.title.as_deref().map(str::trim),
            FeedItem::FeedRs(entry) => entry.title.as_ref().map(|v| v.content.trim()),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub struct FeedItemMeta {
    id: usize,
    pub item: FeedItem,
}

impl FeedItemMeta {
    pub fn new(id: usize, item: FeedItem) -> Self {
        Self { id, item }
    }
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn publish_date_or_old(&self) -> DateTime<FixedOffset> {
        self.item.publish_date_or_old()
    }

    pub fn display_title(&self) -> Option<&str> {
        self.item.display_title()
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
            let mut prefixes: Vec<_> = feed.titles().map(|e| e.trim()).collect();
            prefixes.sort_by_key(|e| e.len());
            prefixes.reverse();
            for a in prefixes {
                t = Self::strip_prefix(t, a);
            }
            t
        })
    }
    pub fn content_link(&self) -> Option<&str> {
        match &self.item {
            FeedItem::Rss(item) => item.link.as_deref(),
            FeedItem::FeedRs(entry) => entry.links.first().map(|link| &link.href[..]),
        }
    }
}

fn old_date() -> DateTime<FixedOffset> {
    chrono::DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap()
}

#[test]
fn test_old_date() {
    old_date();
}
