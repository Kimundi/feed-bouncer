use std::collections::BTreeSet;

use feed_bouncer_database::{Feed, FeedItem};
use rocket::form::Form;
use rocket::{response::Redirect, State};
use rocket_dyn_templates::Template;

use crate::common::{Item, SyncDatabase, Tag};

#[derive(serde::Serialize)]
struct Context<'a> {
    title: &'a str,
    tags: Vec<&'a str>,
    known_tags: Vec<&'a str>,
    items: Vec<Item<'a>>,
    feed_id: &'a str,
    feed_url: Option<&'a str>,
}

#[get("/feed/<feed_id>")]
pub async fn feed(db: &State<SyncDatabase>, feed_id: String) -> Option<Template> {
    let db = db.read().await;
    let feed = db.get(&feed_id)?;
    let Feed {
        name: _,
        parent: _,
        opml: _,
        feed_headers: _, // ignore for now, but could have useful extra metadata
        feed_url,
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
            feed_url: feed_url.as_deref(),
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
    let mut is_new = false;

    for name in new_tag.name.split(',') {
        if let Some(name) = Tag::new(name) {
            is_new |= feed.tags.insert(name.as_str().to_owned());
        }
    }

    if is_new {
        db.save();
    }

    Some(Redirect::to(uri!(feed(feed_id))))
}
