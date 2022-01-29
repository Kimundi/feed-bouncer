use rocket::State;
use rocket_dyn_templates::Template;

use crate::common::{Filter, ItemBuilder, ItemsGroups, Nav, SyncDatabase};

#[derive(serde::Serialize)]
struct Index<'a> {
    items: ItemsGroups<'a>,
    nav: Nav<'a>,
}

#[get("/?<filter>")]
pub async fn index(db: &State<SyncDatabase>, filter: Option<String>) -> Template {
    let filter = Filter::new(filter);
    let mut items = ItemBuilder::new(true);

    let db = db.read().await;

    {
        let mut feeds = db.get_items_ordered_by_time();
        feeds.reverse();
        feeds.dedup_by(|a, b| a.2.item.content_link() == b.2.item.content_link());
        // let feeds = &feeds[0..(feeds.len().min(10))];
        for (feed_id, feed, item) in &feeds[..] {
            if !filter.matches(feed) {
                continue;
            }
            items.push_sorted(&item.item, &feed_id, feed);
        }
    }
    let items = items.into_groups();

    Template::render(
        "pages/index",
        &Index {
            items,
            nav: Nav::new(&db, &filter),
        },
    )
}
