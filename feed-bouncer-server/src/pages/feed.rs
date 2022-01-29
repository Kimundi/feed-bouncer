use std::collections::BTreeSet;

use feed_bouncer_database::{FeedItem, FeedItemMeta};
use rocket::form::Form;
use rocket::{response::Redirect, State};
use rocket_dyn_templates::Template;

use crate::common::{ItemBuilder, ItemsGroups, SyncDatabase, Tag};

#[derive(serde::Serialize)]
struct Context<'a> {
    title: &'a str,
    original_title: &'a str,
    title_aliases: Vec<&'a str>,
    tags: Vec<&'a str>,
    known_tags: Vec<&'a str>,
    items: ItemsGroups<'a>,
    feed_id: &'a str,
    feed_url: Option<&'a str>,
}

#[get("/feed/<feed_id>")]
pub async fn feed(db: &State<SyncDatabase>, feed_id: String) -> Option<Template> {
    let db = db.read().await;
    let feed = db.get(&feed_id)?;

    let tags: Vec<_> = feed.tags().collect();

    let mut items = ItemBuilder::new(false);
    {
        let mut feeds: Vec<&FeedItemMeta> = feed.feeds().iter().collect();
        FeedItem::sort(&mut feeds, |x| &x.item);
        feeds.reverse();
        feeds.dedup_by(|a, b| a.item.content_link() == b.item.content_link());
        for item in feeds {
            items.push_sorted(&item.item, &feed_id, feed);
        }
    }
    let items = items.into_groups();

    let known_tags: Vec<_> = db
        .get_feeds()
        .into_iter()
        .map(|feed| feed.1.tags())
        .flatten()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter(|tag| !tags.contains(&tag))
        .collect();

    let title_aliases: Vec<_> = feed.title_aliases.iter().map(|s| &s[..]).collect();

    Some(Template::render(
        "pages/feed",
        &Context {
            items,
            tags,
            known_tags,
            title: feed.display_name(),
            original_title: feed.original_display_name(),
            feed_id: &feed_id,
            feed_url: feed.feed_url(),
            title_aliases,
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
            is_new |= feed.extend_tags([name.as_str()]);
        }
    }

    if is_new {
        db.save();
    }

    Some(Redirect::to(uri!(feed(feed_id))))
}

#[get("/feed/<feed_id>/tag/remove/<tag>")]
pub async fn feed_remove_tag(
    db: &State<SyncDatabase>,
    feed_id: String,
    tag: &str,
) -> Option<Redirect> {
    let mut db = db.write().await;
    let feed = db.get_mut(&feed_id)?;

    if feed.remove_tag(tag) {
        db.save_shrunk();
    }

    Some(Redirect::to(uri!(feed(feed_id))))
}

#[derive(FromForm)]
pub struct NewTitle<'r> {
    name: &'r str,
}

#[post("/feed/<feed_id>/alias/add", data = "<new_title>")]
pub async fn feed_add_alias(
    db: &State<SyncDatabase>,
    feed_id: String,
    new_title: Form<NewTitle<'_>>,
) -> Option<Redirect> {
    let mut db = db.write().await;
    let feed = db.get_mut(&feed_id)?;
    let is_new = feed.title_aliases.insert(new_title.name.to_owned());

    if is_new {
        db.save();
    }

    Some(Redirect::to(uri!(feed(feed_id))))
}

#[get("/feed/<feed_id>/alias/remove/<title>")]
pub async fn feed_remove_alias(
    db: &State<SyncDatabase>,
    feed_id: String,
    title: &str,
) -> Option<Redirect> {
    let mut db = db.write().await;
    let feed = db.get_mut(&feed_id)?;

    if feed.title_aliases.remove(title) {
        db.save_shrunk();
    }

    Some(Redirect::to(uri!(feed(feed_id))))
}

#[get("/feed/<feed_id>/display/set/<title>")]
pub async fn feed_set_display(
    db: &State<SyncDatabase>,
    feed_id: String,
    title: &str,
) -> Option<Redirect> {
    let mut db = db.write().await;
    let feed = db.get_mut(&feed_id)?;

    feed.set_display_name(title.to_owned());
    db.save_shrunk();

    Some(Redirect::to(uri!(feed(feed_id))))
}
