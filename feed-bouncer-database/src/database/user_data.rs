use std::{collections::BTreeMap, path::Path};

use crate::FeedId;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct FeedUserData {}

#[derive(Default)]
pub struct UserDataStorage {
    user_data: BTreeMap<FeedId, FeedUserData>,
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
            user_data: Self::open_user_data(storage_path).unwrap_or_default(),
        }
    }
    fn save_internal(&self, path: &Path, allow_shrink: bool) {
        let user_data_path = path.join("user_data.json");
        crate::safe_save_json(&self.user_data, &user_data_path, "user_data", allow_shrink);
    }
    pub fn save(&self, path: &Path) {
        self.save_internal(path, true)
    }
}
