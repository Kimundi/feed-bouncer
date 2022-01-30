use feed_bouncer_database::FeedId;
use rocket::{response::Redirect, State};

use crate::{common::SyncDatabase, triggers::GetHeaders};

#[get("/mark_read/<feed_id>/<item_id>")]
pub async fn mark_read(
    db: &State<SyncDatabase>,
    headers: GetHeaders,
    feed_id: FeedId,
    item_id: usize,
) -> Redirect {
    let mut db = db.write().await;

    let opt = db
        .get(&feed_id)
        .map(|feed| {
            let mut vec = Vec::new();
            for item in feed.items() {
                vec.push((item.id(), item.publish_date_or_old()));
            }
            vec
        })
        .and_then(|index| {
            let date = index.iter().find(|v| v.0 == item_id)?.1;
            Some((date, index))
        });

    if let Some((date, index)) = opt {
        for (id, id_date) in index {
            if id_date <= date {
                db.mark_read(&feed_id, id);
            }
        }
        db.save_user_data();
    }

    super::redirect_back(headers)
}
