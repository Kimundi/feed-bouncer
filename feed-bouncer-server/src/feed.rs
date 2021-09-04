use std::collections::BTreeSet;

use feed_bouncer_database::{Feed, FeedItem};
use rocket::form::Form;
use rocket::{response::Redirect, State};
use rocket_dyn_templates::Template;

use crate::SyncDatabase;
#[derive(serde::Serialize)]
struct Item<'a> {
    feed_name: &'a str,
    feed_id: &'a str,
    item_name: &'a str,
    content_link: Option<&'a str>,
}

#[derive(serde::Serialize)]
struct Context<'a> {
    title: &'a str,
    tags: Vec<&'a str>,
    known_tags: Vec<&'a str>,
    items: Vec<Item<'a>>,
    feed_id: &'a str,
}

#[get("/feed/<feed_id>")]
pub async fn feed(db: &State<SyncDatabase>, feed_id: String) -> Option<Template> {
    let db = db.read().await;
    let feed = db.get(&feed_id)?;
    let Feed {
        name: _,
        parent: _,
        feed_url,
        opml,
        feed_headers,
        feeds,
        tags,
        ..
    } = feed;

    let tags = tags.iter().map(|s| s.as_str()).collect();

    let mut items = Vec::new();
    {
        let mut feeds: Vec<&FeedItem> = feeds.iter().collect();
        FeedItem::sort(&mut feeds, |x| x);
        feeds.reverse();
        for item in feeds {
            let content_link = item.content_link();
            items.push(Item {
                feed_name: feed.display_name(),
                feed_id: &feed_id,
                item_name: item.display_title().unwrap_or("???"),
                content_link,
            });
        }
    }

    let known_tags: Vec<_> = db
        .get_feeds()
        .into_iter()
        .map(|feed| feed.1.tags.iter())
        .flatten()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|s| s.as_str())
        .collect();

    Some(Template::render(
        "feed",
        &Context {
            items,
            tags,
            known_tags,
            title: feed.display_name(),
            feed_id: &feed_id,
        },
    ))
}

#[derive(FromForm)]
pub struct NewTag<'r> {
    name: &'r str,
}

#[post("/feed/<feed_id>/tag/add", data = "<new_tag>")]
pub async fn feed_add_tag(
    db: &State<SyncDatabase>,
    feed_id: String,
    new_tag: Form<NewTag<'_>>,
) -> Option<Redirect> {
    let mut db = db.write().await;
    let feed = db.get_mut(&feed_id)?;
    let is_new = feed.tags.insert(new_tag.name.to_owned());
    if is_new {
        db.save();
    }

    Some(Redirect::to(uri!(feed(feed_id))))
}
