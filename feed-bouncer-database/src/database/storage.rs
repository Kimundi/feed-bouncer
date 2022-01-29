use std::{collections::BTreeMap, path::Path};

use crate::database::{storage_feed::Feed, FeedId, SourceLookup};

#[derive(Default)]
pub struct Storage {
    sources: BTreeMap<FeedId, Feed>,
}

impl Storage {
    fn open_feeds(path: &Path) -> std::io::Result<BTreeMap<FeedId, Feed>> {
        let feed_path = path.join("feeds");
        let mut sources = BTreeMap::new();
        for e in std::fs::read_dir(&feed_path)? {
            let e = e?;
            let feed_file = e.path();
            let id = feed_file
                .file_stem()
                .expect("file does not have a name")
                .to_str()
                .expect("file does not have unicode name")
                .to_owned();
            let file = std::fs::read_to_string(&feed_file)?;
            // TODO: Remove at a later point
            let file = file.replace(r#""period": "HOURLY""#, r#""period": "Hourly""#);
            let mut feed: Feed = serde_json::from_str(&file).expect(&format!(
                "file {:?} could be read, but not parsed",
                feed_file
            ));

            feed.migrate_data();

            sources.insert(id, feed);
        }

        Ok(sources)
    }

    pub fn open_or_default(storage_path: &Path) -> Self {
        Self {
            sources: Self::open_feeds(storage_path).unwrap_or_default(),
        }
    }
    fn save_internal(&self, path: &Path, allow_shrink: bool) {
        let feed_path = path.join("feeds");
        std::fs::create_dir_all(&feed_path).unwrap();
        for (feed_id, source) in self.iter() {
            let file_path = feed_path.join(feed_id).with_extension("json");
            crate::safe_save_json(source, &file_path, "database", allow_shrink);
        }
    }
    pub fn save(&self, path: &Path) {
        self.save_internal(path, false)
    }
    pub fn save_shrunk(&self, path: &Path) {
        self.save_internal(path, true)
    }
    pub fn write_to_cache(&self, lookup: &mut SourceLookup) {
        for (feed_id, source) in &self.sources {
            lookup.touch(feed_id, source.key());
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (&FeedId, &Feed)> + '_ {
        self.sources.iter()
    }
    /*
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&FeedId, &mut Feed)> + '_ {
        self.sources.iter_mut()
    }
    */
    pub fn get_or_insert(&mut self, feed_id: FeedId, feed: &Feed) -> &mut Feed {
        self.sources.entry(feed_id).or_insert_with(|| feed.clone())
    }
    pub fn get(&self, feed_id: &FeedId) -> Option<&Feed> {
        self.sources.get(feed_id)
    }
    pub fn get_mut(&mut self, feed_id: &FeedId) -> Option<&mut Feed> {
        self.sources.get_mut(feed_id)
    }
}
