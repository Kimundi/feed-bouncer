use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use crate::FeedId;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct FeedUserData {
    read_ids: BTreeSet<usize>,
}

#[derive(Default)]
pub struct UserDataStorage {
    storage: BTreeMap<FeedId, FeedUserData>,
}

impl UserDataStorage {
    fn open_user_data(path: &Path) -> std::io::Result<BTreeMap<FeedId, FeedUserData>> {
        let user_data_path = path.join("user_data.json");
        let user_data = std::fs::read_to_string(&user_data_path)
            .map(|v| serde_json::from_str(&v).expect("could not parse user_data"))?;

        Ok(user_data)
    }

    pub fn open_or_default(storage_path: &Path) -> Self {
        Self {
            storage: Self::open_user_data(storage_path).unwrap_or_default(),
        }
    }
    pub fn save(&self, path: &Path) {
        let user_data_path = path.join("user_data.json");
        crate::safe_save_json(&self.storage, &user_data_path, "user_data", true);
    }
    pub fn mark_read(&mut self, feed_id: &FeedId, item_id: usize) {
        self.storage
            .entry(feed_id.clone())
            .or_default()
            .read_ids
            .insert(item_id);
    }
}
