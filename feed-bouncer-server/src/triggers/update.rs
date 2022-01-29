use rocket::{response::Redirect, State};

use crate::{common::SyncDatabase, triggers::GetHeaders};
use std::time::Duration;

use super::redirect_back;

#[get("/update")]
pub async fn update(db: &State<SyncDatabase>, referer: GetHeaders) -> Redirect {
    start_update(db);
    redirect_back(referer)
}

pub fn start_update(db: &SyncDatabase) {
    let db: SyncDatabase = db.clone();

    rocket::tokio::spawn(async move {
        // get tasks during a temporary read lock
        let tasks = {
            let db = db.read().await;
            db.update_feeds_task()
        };

        // Run the task updates while the lock is not held
        let results = tasks.run().await;

        // commit the updates
        let mut db = db.write().await;
        db.commit_from(results).await;
        db.save();
    });
}

pub fn start_periodic_update(db: &SyncDatabase) {
    let db: SyncDatabase = db.clone();
    rocket::tokio::spawn(async move {
        loop {
            start_update(&db);
            rocket::tokio::time::sleep(Duration::from_secs(60 * 60)).await;
        }
    });
}
