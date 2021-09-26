use rocket::State;
use rocket_dyn_templates::Template;

use crate::common::{Filter, Nav, SyncDatabase};

#[derive(serde::Serialize)]
pub struct Feed<'a> {
    pub feed_name: &'a str,
    pub feed_id: &'a str,
    pub tags: String,
}

#[derive(serde::Serialize)]
struct Feeds<'a> {
    feeds: Vec<Feed<'a>>,
    nav: Nav<'a>,
}

#[get("/feeds?<filter>")]
pub async fn feeds(db: &State<SyncDatabase>, filter: Option<String>) -> Template {
    let filter = Filter::new(filter);
    let mut feeds = Vec::new();

    let db = db.read().await;

    let mut feeds_src = db.get_feeds();
    feeds_src.sort_by_key(|(_, feed)| feed.display_name());

    for (feed_id, feed) in feeds_src {
        if !filter.matches(feed) {
            continue;
        }
        feeds.push(Feed {
            feed_name: feed.display_name(),
            feed_id: &feed_id,
            tags: feed
                .tags
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", "),
        });
    }

    Template::render(
        "feeds",
        &Feeds {
            feeds,
            nav: Nav::new(&db, &filter),
        },
    )
}
