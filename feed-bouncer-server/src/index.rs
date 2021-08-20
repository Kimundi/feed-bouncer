use rocket::State;
use rocket_dyn_templates::Template;

use crate::SyncDatabase;

#[derive(serde::Serialize)]
struct Item<'a> {
    feed_name: &'a str,
    item_name: &'a str,
    content_link: Option<&'a str>,
}

#[derive(serde::Serialize)]
struct Index<'a> {
    items: Vec<Item<'a>>,
    last_update: Option<String>,
}

#[get("/")]
pub async fn index(db: &State<SyncDatabase>) -> Template {
    let mut items = Vec::new();

    let db = db.read().await;

    {
        let mut feeds = db.get_items_ordered_by_time();
        feeds.reverse();
        // let feeds = &feeds[0..(feeds.len().min(10))];
        for (feed, item) in &feeds[..] {
            let content_link = item.content_link();
            items.push(Item {
                feed_name: feed.display_name(),
                item_name: item
                    .display_title_without_prefix(&feed.display_name())
                    .unwrap_or("???"),
                content_link,
            });
        }
    }

    Template::render(
        "index",
        &Index {
            items,
            last_update: db.last_feed_update().map(|v| v.to_rfc3339()),
        },
    )
}
