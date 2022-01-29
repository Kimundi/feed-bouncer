use rocket::{response::Redirect, State};

use crate::{common::SyncDatabase, triggers::GetHeaders};

#[get("/mark_read/<feed_id>/<item_id>?<filter>")]
pub async fn mark_read(
    db: &State<SyncDatabase>,
    headers: GetHeaders,
    feed_id: &str,
    item_id: usize,
    filter: Option<String>,
) -> Redirect {
    super::redirect_back(headers)
}
