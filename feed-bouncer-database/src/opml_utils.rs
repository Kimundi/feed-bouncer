use std::path::Path;

use opml::OPML;

use crate::{
    database::{Database, FeedId},
    Feed,
};

fn open(path: &Path) -> OPML {
    let opml = std::fs::read_to_string(path).unwrap();
    let opml = opml::OPML::from_str(&opml).unwrap();
    opml
}

impl Database {
    fn add_opml_outline(
        &mut self,
        mut outline: opml::Outline,
        parent: Option<&FeedId>,
        initial_tags: &[String],
    ) {
        let name = outline.title.as_deref().unwrap_or(&outline.text).to_owned();
        let rss = outline.xml_url.as_deref().map(|s| s.to_owned());
        let children: Vec<_> = outline.outlines.drain(..).collect();
        let mut source = Feed::new(name);
        *source.feed_url_mut() = rss;
        *source.opml_mut() = Some(outline);
        source.parent = parent.map(|v| v.to_owned());
        source.extend_tags(initial_tags.iter().map(|s| &s[..]));
        let parent_feed_id = self.insert(source);

        for child in children {
            self.add_opml_outline(child, Some(&parent_feed_id), initial_tags.clone());
        }
    }
    pub fn import_from_opml(&mut self, path: &Path, initial_tags: &[String]) {
        let opml = open(path);
        for outline in opml.body.outlines {
            self.add_opml_outline(outline, None, initial_tags);
        }
    }
}
