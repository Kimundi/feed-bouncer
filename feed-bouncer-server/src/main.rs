#[macro_use]
extern crate rocket;

use std::{path::PathBuf, sync::Arc};

use clap::{AppSettings, Clap};
use feed_bouncer_database::Database;
use rocket::tokio::sync::RwLock;
use rocket_dyn_templates::Template;

mod feed;
mod index;
mod refresh;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long)]
    storage_path: Option<PathBuf>,
}

type SyncDatabase = Arc<RwLock<Database>>;

#[rocket::main]
async fn main() {
    let opts = Opts::parse();

    let mut db = Database::init(opts.storage_path);
    db.import().await;
    let db: SyncDatabase = Arc::new(RwLock::new(db));
    refresh::start_periodic_refresh(&db);

    let cfg = rocket::build()
        .mount(
            "/",
            routes![
                index::index,
                refresh::refresh,
                feed::feed,
                feed::feed_add_tag
            ],
        )
        .attach(Template::fairing())
        .manage(db);

    if let Err(e) = cfg.launch().await {
        println!("Whoops! Rocket didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
        drop(e);
    };
}
