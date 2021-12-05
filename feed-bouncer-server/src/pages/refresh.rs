use std::time::Duration;

use rocket::{response::Redirect, State};

use crate::SyncDatabase;

#[get("/refresh")]
pub async fn refresh(db: &State<SyncDatabase>) -> Redirect {
    start_refresh(db);
    Redirect::to(uri!(crate::index::index(None::<String>)))
}

pub fn start_refresh(db: &SyncDatabase) {
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

pub fn start_periodic_refresh(db: &SyncDatabase) {
    let db: SyncDatabase = db.clone();
    rocket::tokio::spawn(async move {
        loop {
            start_refresh(&db);
            rocket::tokio::time::sleep(Duration::from_secs(60 * 60)).await;
        }
    });
}
