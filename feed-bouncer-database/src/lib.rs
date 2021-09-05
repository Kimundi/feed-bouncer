mod database;
mod feeds;
mod import;
mod opml_utils;

pub use database::storage::Feed;
pub use database::storage::FeedHeader;
pub use database::storage::FeedItem;
pub use database::Database;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("reqwest error {0}")]
    Reqwest(reqwest::Error),
}
