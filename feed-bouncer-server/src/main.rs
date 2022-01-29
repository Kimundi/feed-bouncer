#[macro_use]
extern crate rocket;

use std::{path::PathBuf, sync::Arc};

use clap::Parser;
use feed_bouncer_database::Database;
use rocket::tokio::sync::RwLock;
use rocket_dyn_templates::Template;

use crate::common::SyncDatabase;

mod common;
mod handlebars_helper;
mod pages;
mod triggers;

#[derive(Parser)]
struct Opts {
    #[clap(short, long)]
    storage_path: Option<PathBuf>,
}

#[rocket::main]
async fn main() {
    let opts = Opts::parse();

    let mut db = Database::init(opts.storage_path);
    db.import().await;
    let db: SyncDatabase = Arc::new(RwLock::new(db));
    triggers::update::start_periodic_update(&db);

    let cfg = rocket::build()
        .mount(
            "/",
            routes![
                pages::index::index,
                pages::feed::feed,
                pages::feed::feed_add_tag,
                pages::feed::feed_remove_tag,
                pages::feed::feed_add_alias,
                pages::feed::feed_remove_alias,
                pages::feed::feed_set_display,
                pages::feeds::feeds,
                pages::import::import,
                pages::import::import_rss,
                triggers::update::update,
                triggers::mark_read::mark_read,
            ],
        )
        .attach(Template::custom(handlebars_helper::register))
        .manage(db);

    if let Err(e) = cfg.launch().await {
        println!("Whoops! Rocket didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
        drop(e);
    };
}
