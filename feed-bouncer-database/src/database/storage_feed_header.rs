#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum FeedHeader {
    Rss(crate::feeds::rss::ChannelHeader),
    FeedRs(crate::feeds::feed_rs::FeedHeader),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub struct FeedHeaderMeta {
    id: usize,
    pub header: FeedHeader,
}
impl FeedHeaderMeta {
    pub fn new(id: usize, header: FeedHeader) -> Self {
        Self { id, header }
    }
    pub fn id(&self) -> usize {
        self.id
    }
}
