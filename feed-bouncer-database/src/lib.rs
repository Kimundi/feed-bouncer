mod database;
mod import;
mod opml_utils;
mod rss_utils;

pub use database::Database;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("reqwest error {0}")]
    Reqwest(reqwest::Error),
}
