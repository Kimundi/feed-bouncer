use crate::{database::Database, safe_save_json};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum ImportEntry {
    Rss {
        url: String,
        #[serde(default)]
        ignore: bool,
        #[serde(default)]
        tags: Vec<String>,
    },
    Opml {
        path: String,
        #[serde(default)]
        ignore: bool,
        #[serde(default)]
        tags: Vec<String>,
    },
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Import {
    sources: Vec<ImportEntry>,
}

impl Database {
    pub async fn import(&mut self) {
        let import_file = self.storage_path.join("import.json");

        if let Ok(v) = std::fs::read(&import_file) {
            eprintln!("Import from import file");
            match serde_json::from_slice(&v) {
                Ok(import) => {
                    let mut import: Import = import;
                    for source in &mut import.sources {
                        match source {
                            ImportEntry::Rss { url, ignore, tags } => {
                                if !*ignore {
                                    eprintln!("   add {}", url);
                                    self.import_from_rss(&url, &tags).await.ok();
                                    *ignore = true;
                                } else {
                                    eprintln!("  skip {}", url);
                                }
                            }
                            ImportEntry::Opml { path, ignore, tags } => {
                                if !*ignore {
                                    eprintln!("   add {}", path);
                                    let path = self.storage_path.join(path);
                                    self.import_from_opml(path.as_ref(), &tags);
                                    *ignore = true;
                                } else {
                                    eprintln!("  skip {}", path);
                                }
                            }
                        }
                    }
                    safe_save_json(&import, &import_file, "import file", true);
                }
                Err(e) => {
                    eprintln!("Error when importing: {}", e);
                }
            }
            eprintln!();
        }
    }
}
