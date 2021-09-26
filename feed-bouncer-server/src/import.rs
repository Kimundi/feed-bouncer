use rocket::form::Form;
use rocket::{response::Redirect, State};
use rocket_dyn_templates::Template;

use crate::common::SyncDatabase;

#[derive(serde::Serialize)]
struct Context {}

#[get("/import")]
pub async fn import(_db: &State<SyncDatabase>) -> Option<Template> {
    Some(Template::render("import", &Context {}))
}

#[derive(FromForm)]
pub struct NewRss<'r> {
    rss_url: &'r str,
}

#[post("/import/rss", data = "<new_rss>")]
pub async fn import_rss(db: &State<SyncDatabase>, new_rss: Form<NewRss<'_>>) -> Option<Redirect> {
    let mut db = db.write().await;

    // TODO: Do not await here blockingly
    // TODO: initial tags
    if let Ok(feed_ids) = db.import_from_rss(new_rss.rss_url, &[]).await {
        if let Some(feed_id) = feed_ids.iter().next() {
            return Some(Redirect::to(uri!(crate::feed::feed(feed_id))));
        }
    }

    Some(Redirect::to(uri!(crate::index::index(None::<String>))))
}
