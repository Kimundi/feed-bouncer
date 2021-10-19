mod database;
mod feeds;
mod import;
mod opml_utils;

use std::path::Path;

pub use database::storage::Feed;
pub use database::storage::FeedHeader;
pub use database::storage::FeedItem;
pub use database::Database;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("reqwest error {0}")]
    Reqwest(reqwest::Error),
}

fn safe_save_json(data: &impl serde::Serialize, path: &Path, what: &str, allow_shrink: bool) {
    let storage = serde_json::to_string_pretty(data).unwrap();
    let new_path = path.with_extension("new.json");
    std::fs::write(&new_path, storage).unwrap();
    let new_size = std::fs::metadata(&new_path).unwrap().len();
    let old_size = std::fs::metadata(path).map(|v| v.len()).unwrap_or(0);
    if allow_shrink || (new_size >= old_size) {
        std::fs::rename(new_path, path).unwrap();
    } else {
        eprintln!(
            "WARN: suspicious file size change when saving {}, aborting the attempt",
            what
        )
    }
}
