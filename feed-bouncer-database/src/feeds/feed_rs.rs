use std::time::Duration;

use super::FeedConvert;
use chrono::{DateTime, Utc};
use feed_rs::model::Feed;

/// Type of a feed (RSS, Atom etc)
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum FeedType {
    Atom,
    JSON,
    RSS0,
    RSS1,
    RSS2,
}
impl FeedConvert for feed_rs::model::FeedType {
    type Output = FeedType;

    fn convert(self) -> Self::Output {
        match self {
            Self::Atom => FeedType::Atom,
            Self::JSON => FeedType::JSON,
            Self::RSS0 => FeedType::RSS0,
            Self::RSS1 => FeedType::RSS1,
            Self::RSS2 => FeedType::RSS2,
        }
    }
}

/// Textual content, or link to the content, for a given entry.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Text {
    // TODO
    // pub content_type: Mime,
    pub src: Option<String>,
    pub content: String,
}
impl FeedConvert for feed_rs::model::Text {
    type Output = Text;

    fn convert(self) -> Self::Output {
        let feed_rs::model::Text {
            content_type: _,
            src,
            content,
        } = self;
        Text { src, content }
    }
}

/// Represents an author, contributor etc.
///
/// [Atom spec]: http://www.atomenabled.org/developers/syndication/#person
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Person {
    /// Atom: human-readable name for the person.
    /// JSON Feed: human-readable name for the person.
    pub name: String,
    /// Atom: home page for the person.
    /// JSON Feed: link to media (Twitter etc) for the person
    pub uri: Option<String>,
    /// Atom: An email address for the person.
    pub email: Option<String>,
}
impl FeedConvert for feed_rs::model::Person {
    type Output = Person;

    fn convert(self) -> Self::Output {
        let feed_rs::model::Person { name, uri, email } = self;
        Person { name, uri, email }
    }
}

/// Represents a link to an associated resource for the feed or entry.
///
/// [Atom spec]: http://www.atomenabled.org/developers/syndication/#link
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Link {
    /// Link to additional content
    /// * Atom: The URI of the referenced resource (typically a Web page).
    /// * RSS 2: The URL to the HTML website corresponding to the channel or item.
    /// * JSON Feed: the URI to the attachment, feed etc
    pub href: String,
    /// A single link relationship type.
    pub rel: Option<String>,
    /// Indicates the media type of the resource.
    pub media_type: Option<String>,
    /// Indicates the language of the referenced resource.
    pub href_lang: Option<String>,
    /// Human readable information about the link, typically for display purposes.
    pub title: Option<String>,
    /// The length of the resource, in bytes.
    pub length: Option<u64>,
}
impl FeedConvert for feed_rs::model::Link {
    type Output = Link;

    fn convert(self) -> Self::Output {
        let feed_rs::model::Link {
            href,
            rel,
            media_type,
            href_lang,
            title,
            length,
        } = self;
        Link {
            href,
            rel,
            media_type,
            href_lang,
            title,
            length,
        }
    }
}

/// Represents the category of a feed or entry
///
/// [Atom spec]: http://www.atomenabled.org/developers/syndication/#category
/// [RSS 2 spec]: https://validator.w3.org/feed/docs/rss2.html#ltcategorygtSubelementOfLtitemgt
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Category {
    /// The category as a human readable string
    /// * Atom (required): Identifies the category.
    /// * RSS 2: The value of the element is a forward-slash-separated string that identifies a hierarchic location in the indicated taxonomy. Processors may establish conventions for the interpretation of categories.
    /// * JSON Feed: the value of the tag
    pub term: String,
    /// Atom (optional): Identifies the categorization scheme via a URI.
    pub scheme: Option<String>,
    /// Atom (optional): Provides a human-readable label for display.
    pub label: Option<String>,
}
impl FeedConvert for feed_rs::model::Category {
    type Output = Category;

    fn convert(self) -> Self::Output {
        let feed_rs::model::Category {
            term,
            scheme,
            label,
        } = self;
        Category {
            term,
            scheme,
            label,
        }
    }
}

/// Information on the tools used to generate the feed
///
/// Atom: Identifies the software used to generate the feed, for debugging and other purposes.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Generator {
    /// Atom: Additional data
    /// RSS 2: A string indicating the program used to generate the channel.
    pub content: String,
    /// Atom: Link to the tool
    pub uri: Option<String>,
    /// Atom: Tool version
    pub version: Option<String>,
}
impl FeedConvert for feed_rs::model::Generator {
    type Output = Generator;

    fn convert(self) -> Self::Output {
        let feed_rs::model::Generator {
            content,
            uri,
            version,
        } = self;
        Generator {
            content,
            uri,
            version,
        }
    }
}

/// Represents a a link to an image.
///
/// [Atom spec]:  http://www.atomenabled.org/developers/syndication/#optionalFeedElements
/// [RSS 2 spec]: https://validator.w3.org/feed/docs/rss2.html#ltimagegtSubelementOfLtchannelgt
/// [RSS 1 spec]: https://validator.w3.org/feed/docs/rss1.html#s5.4
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Image {
    /// Link to the image
    /// * Atom: The URL to an image or logo
    /// * RSS 1 + 2: the URL of a GIF, JPEG or PNG image that represents the channel.
    pub uri: String,
    /// RSS 1 + 2: describes the image, it's used in the ALT attribute of the HTML <img> tag when the channel is rendered in HTML.
    pub title: Option<String>,
    /// RSS 1 + 2: the URL of the site, when the channel is rendered, the image is a link to the site.
    pub link: Option<Link>,

    /// RSS 2 (optional): width of the image
    pub width: Option<u32>,
    /// RSS 2 (optional): height of the image
    pub height: Option<u32>,
    /// RSS 2 (optional): contains text that is included in the TITLE attribute of the link formed around the image in the HTML rendering.
    pub description: Option<String>,
}
impl FeedConvert for feed_rs::model::Image {
    type Output = Image;

    fn convert(self) -> Self::Output {
        let feed_rs::model::Image {
            uri,
            title,
            link,
            width,
            height,
            description,
        } = self;
        Image {
            uri,
            title,
            link: link.convert(),
            width,
            height,
            description,
        }
    }
}

/// Rating of the feed, item or media within the content
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct MediaRating {
    // The scheme (defaults to "simple" per the spec)
    pub urn: String,
    // The rating text
    pub value: String,
}
impl FeedConvert for feed_rs::model::MediaRating {
    type Output = MediaRating;

    fn convert(self) -> Self::Output {
        let feed_rs::model::MediaRating { urn, value } = self;
        MediaRating { urn, value }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct FeedHeader {
    /// Type of this feed (e.g. RSS2, Atom etc)
    pub feed_type: FeedType,
    /// A unique identifier for this feed
    /// * Atom (required): Identifies the feed using a universally unique and permanent URI.
    /// * RSS doesn't require an ID so it is initialised to the hash of the first link or a UUID if not found
    pub id: String,
    /// The title of the feed
    /// * Atom (required): Contains a human readable title for the feed. Often the same as the title of the associated website. This value should not be blank.
    /// * RSS 1 + 2 (required) "title": The name of the channel. It's how people refer to your service.
    /// * JSON Feed: is the name of the feed
    pub title: Option<Text>,
    /// The time at which the feed was last modified. If not provided in the source, or invalid, it is `None`.
    /// * Atom (required): Indicates the last time the feed was modified in a significant way.
    /// * RSS 2 (optional) "lastBuildDate": The last time the content of the channel changed.
    pub updated: Option<DateTime<Utc>>,

    /// Atom (recommended): Collection of authors defined at the feed level.
    /// JSON Feed: specifies the feed author.
    pub authors: Vec<Person>,
    /// Description of the feed
    /// * Atom (optional): Contains a human-readable description or subtitle for the feed (from <subtitle>).
    /// * RSS 1 + 2 (required): Phrase or sentence describing the channel.
    /// * JSON Feed: description of the feed
    pub description: Option<Text>,
    /// Links to related pages
    /// * Atom (recommended): Identifies a related Web page.
    /// * RSS 1 + 2 (required): The URL to the HTML website corresponding to the channel.
    /// * JSON Feed: the homepage and feed URLs
    pub links: Vec<Link>,

    /// Structured classification of the feed
    /// * Atom (optional): Specifies a category that the feed belongs to. A feed may have multiple category elements.
    /// * RSS 2 (optional) "category": Specify one or more categories that the channel belongs to.
    pub categories: Vec<Category>,
    /// People who have contributed to the feed
    /// * Atom (optional): Names one contributor to the feed. A feed may have multiple contributor elements.
    /// * RSS 2 (optional) "managingEditor": Email address for person responsible for editorial content.
    /// * RSS 2 (optional) "webMaster": Email address for person responsible for technical issues relating to channel.
    pub contributors: Vec<Person>,
    /// Information on the software used to build the feed
    /// * Atom (optional): Identifies the software used to generate the feed, for debugging and other purposes.
    /// * RSS 2 (optional): A string indicating the program used to generate the channel.
    pub generator: Option<Generator>,
    /// A small icon
    /// * Atom (optional): Identifies a small image which provides iconic visual identification for the feed.
    /// * JSON Feed: is the URL of an image for the feed suitable to be used in a source list.
    pub icon: Option<Image>,
    /// RSS 2 (optional): The language the channel is written in.
    pub language: Option<String>,
    /// An image used to visually identify the feed
    /// * Atom (optional): Identifies a larger image which provides visual identification for the feed.
    /// * RSS 1 + 2 (optional) "image": Specifies a GIF, JPEG or PNG image that can be displayed with the channel.
    /// * JSON Feed: is the URL of an image for the feed suitable to be used in a timeline
    pub logo: Option<Image>,
    /// RSS 2 (optional): The publication date for the content in the channel.
    pub published: Option<DateTime<Utc>>,
    /// Rating for the content
    /// * Populated from the media or itunes namespaces
    pub rating: Option<MediaRating>,
    /// Rights restricting content within the feed
    /// * Atom (optional): Conveys information about rights, e.g. copyrights, held in and over the feed.
    /// * RSS 2 (optional) "copyright": Copyright notice for content in the channel.
    pub rights: Option<Text>,
    /// RSS 2 (optional): It's a number of minutes that indicates how long a channel can be cached before refreshing from the source.
    pub ttl: Option<u32>,
}

impl FeedHeader {
    pub fn split(feed: Feed) -> (FeedHeader, Vec<Entry>) {
        let Feed {
            feed_type,
            id,
            title,
            updated,
            authors,
            description,
            links,
            categories,
            contributors,
            generator,
            icon,
            language,
            logo,
            published,
            rating,
            rights,
            ttl,
            entries,
        } = feed;

        (
            FeedHeader {
                feed_type: feed_type.convert(),
                id,
                title: title.convert(),
                updated,
                authors: authors.convert(),
                description: description.convert(),
                links: links.convert(),
                categories: categories.convert(),
                contributors: contributors.convert(),
                generator: generator.convert(),
                icon: icon.convert(),
                language,
                logo: logo.convert(),
                published,
                rating: rating.convert(),
                rights: rights.convert(),
                ttl,
            },
            entries.convert(),
        )
    }
}

/// Content, or link to the content, for a given entry.
///
/// [Atom spec]: http://www.atomenabled.org/developers/syndication/#contentElement
/// [RSS 2.0]: https://validator.w3.org/feed/docs/rss2.html#ltenclosuregtSubelementOfLtitemgt
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Content {
    /// Atom
    /// * If the type attribute ends in +xml or /xml, then an xml document of this type is contained inline.
    /// * If the type attribute starts with text, then an escaped document of this type is contained inline.
    /// * Otherwise a base64 encoded document of the indicated media type is contained inline.
    pub body: Option<String>,

    // TODO
    // Type of content
    // * Atom: The type attribute is either text, html, xhtml, in which case the content element is defined identically to other text constructs.
    // * RSS 2: Type says what its type is, a standard MIME type
    // pub content_type: Mime,
    /// RSS 2.0: Length of the content in bytes
    pub length: Option<u64>,
    /// Source of the content
    /// * Atom: If the src attribute is present, it represents the URI of where the content can be found. The type attribute, if present, is the media type of the content.
    /// * RSS 2.0: where the enclosure is located
    pub src: Option<Link>,
}
impl FeedConvert for feed_rs::model::Content {
    type Output = Content;

    fn convert(self) -> Self::Output {
        let feed_rs::model::Content {
            body,
            content_type: _,
            length,
            src,
        } = self;
        Content {
            body,
            length,
            src: src.convert(),
        }
    }
}

/// Represents a "media:content" item from the RSS Media spec
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct MediaContent {
    // TODO
    /// The direct URL
    pub url: Option<String>,
    // TODO
    // Standard MIME type
    //pub content_type: Option<Mime>,
    /// Height and width
    pub height: Option<u32>,
    pub width: Option<u32>,
    /// Duration the media plays
    pub duration: Option<Duration>,
    /// Size of media in bytes
    pub size: Option<u64>,
    /// Rating
    pub rating: Option<MediaRating>,
}
impl FeedConvert for feed_rs::model::MediaContent {
    type Output = MediaContent;

    fn convert(self) -> Self::Output {
        let feed_rs::model::MediaContent {
            url,
            content_type: _,
            height,
            width,
            duration,
            size,
            rating,
        } = self;
        MediaContent {
            url: url.map(|url| url.into()),
            height,
            width,
            duration,
            size,
            rating: rating.convert(),
        }
    }
}

/// Represents a "media:thumbnail" item from the RSS Media spec
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct MediaThumbnail {
    /// The thumbnail image
    pub image: Image,
    /// The time this thumbnail represents
    pub time: Option<Duration>,
}
impl FeedConvert for feed_rs::model::MediaThumbnail {
    type Output = MediaThumbnail;

    fn convert(self) -> Self::Output {
        let feed_rs::model::MediaThumbnail { image, time } = self;
        MediaThumbnail {
            image: image.convert(),
            time,
        }
    }
}

/// Represents a "media:text" item from the RSS Media spec
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct MediaText {
    /// The text
    pub text: Text,
    /// The start time offset that the text starts being relevant to the media object.
    pub start_time: Option<Duration>,
    /// The end time that the text is relevant. If this attribute is not provided, and a start time is used, it is expected that the end time is either the end of the clip or the start of the next <media:text> element.
    pub end_time: Option<Duration>,
}
impl FeedConvert for feed_rs::model::MediaText {
    type Output = MediaText;

    fn convert(self) -> Self::Output {
        let feed_rs::model::MediaText {
            text,
            start_time,
            end_time,
        } = self;
        MediaText {
            text: text.convert(),
            start_time,
            end_time,
        }
    }
}

/// Represents a "media:community" item from the RSS Media spec
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct MediaCommunity {
    /// Star rating
    pub stars_avg: Option<f64>,
    pub stars_count: Option<u64>,
    pub stars_min: Option<u64>,
    pub stars_max: Option<u64>,

    /// Statistics on engagement
    pub stats_views: Option<u64>,
    pub stats_favorites: Option<u64>,
}
impl FeedConvert for feed_rs::model::MediaCommunity {
    type Output = MediaCommunity;

    fn convert(self) -> Self::Output {
        let feed_rs::model::MediaCommunity {
            stars_avg,
            stars_count,
            stars_min,
            stars_max,
            stats_views,
            stats_favorites,
        } = self;
        MediaCommunity {
            stars_avg,
            stars_count,
            stars_min,
            stars_max,
            stats_views,
            stats_favorites,
        }
    }
}

/// Represents a "media:credit" item from the RSS Media spec
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct MediaCredit {
    /// The entity being credited
    pub entity: String,
}
impl FeedConvert for feed_rs::model::MediaCredit {
    type Output = MediaCredit;

    fn convert(self) -> Self::Output {
        let feed_rs::model::MediaCredit { entity } = self;
        MediaCredit { entity }
    }
}

/// The top-level representation of a media object
/// i.e. combines "media:*" elements from the RSS Media spec such as those under a media:group
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct MediaObject {
    /// Title of the object (from the media:title element)
    pub title: Option<Text>,
    /// Collection of the media content elements
    pub content: Vec<MediaContent>,
    /// Duration of the object
    pub duration: Option<Duration>,
    /// Representative images for the object (from media:thumbnail elements)
    pub thumbnails: Vec<MediaThumbnail>,
    /// A text transcript, closed captioning or lyrics of the media content.
    pub texts: Vec<MediaText>,
    /// Short description of the media object (from the media:description element)
    pub description: Option<Text>,
    /// Community info (from the media:community element)
    pub community: Option<MediaCommunity>,
    /// Credits
    pub credits: Vec<MediaCredit>,
}
impl FeedConvert for feed_rs::model::MediaObject {
    type Output = MediaObject;

    fn convert(self) -> Self::Output {
        let feed_rs::model::MediaObject {
            title,
            content,
            duration,
            thumbnails,
            texts,
            description,
            community,
            credits,
        } = self;
        MediaObject {
            title: title.convert(),
            content: content.convert(),
            duration,
            thumbnails: thumbnails.convert(),
            texts: texts.convert(),
            description: description.convert(),
            community: community.convert(),
            credits: credits.convert(),
        }
    }
}

/// An item within a feed
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Entry {
    /// A unique identifier for this item with a feed. If not supplied it is initialised to a hash of the first link or a UUID if not available.
    /// * Atom (required): Identifies the entry using a universally unique and permanent URI.
    /// * RSS 2 (optional) "guid": A string that uniquely identifies the item.
    /// * RSS 1: does not specify a unique ID as a separate item, but does suggest the URI should be "the same as the link" so we use a hash of the link if found
    /// * JSON Feed: is unique for that item for that feed over time.
    pub id: String,
    /// Title of this item within the feed
    /// * Atom, RSS 1(required): Contains a human readable title for the entry.
    /// * RSS 2 (optional): The title of the item.
    /// * JSON Feed: The title of the item.
    pub title: Option<Text>,
    /// Time at which this item was last modified. If not provided in the source, or invalid, it is `None`.
    /// * Atom (required): Indicates the last time the entry was modified in a significant way.
    /// * RSS doesn't specify this field.
    /// * JSON Feed: the last modification date of this item
    pub updated: Option<DateTime<Utc>>,

    /// Authors of this item
    /// * Atom (recommended): Collection of authors defined at the entry level.
    /// * RSS 2 (optional): Email address of the author of the item.
    /// * JSON Feed: the author of the item
    pub authors: Vec<Person>,
    /// The content of the item
    /// * Atom (recommended): Contains or links to the complete content of the entry.
    /// * RSS 2 (optional) "content:encoded": The HTML form of the content
    /// * JSON Feed: the html content of the item, or the text content if no html is specified
    pub content: Option<Content>,
    /// Links associated with this item
    /// * Atom (recommended): Identifies a related Web page.
    /// * RSS 2 (optional): The URL of the item.
    /// * RSS 1 (required): The item's URL.
    /// * JSON Feed: the url and external URL for the item is the first items, then each subsequent attachment
    pub links: Vec<Link>,
    /// A short summary of the item
    /// * Atom (recommended): Conveys a short summary, abstract, or excerpt of the entry.
    /// * RSS 1+2 (optional): The item synopsis.
    /// * JSON Feed: the summary for the item, or the text content if no summary is provided and both text and html content are specified
    pub summary: Option<Text>,

    /// Structured classification of the item
    /// * Atom (optional): Specifies a category that the entry belongs to. A feed may have multiple category elements.
    /// * RSS 2 (optional): Includes the item in one or more categories.
    /// * JSON Feed: the supplied item tags
    pub categories: Vec<Category>,
    /// Atom (optional): Names one contributor to the entry. A feed may have multiple contributor elements.
    pub contributors: Vec<Person>,
    /// Time at which this item was first published
    /// * Atom (optional): Contains the time of the initial creation or first availability of the entry.
    /// * RSS 2 (optional) "pubDate": Indicates when the item was published.
    /// * JSON Feed: the date at which the item was published
    pub published: Option<DateTime<Utc>>,
    /// Atom (optional): If an entry is copied from one feed into another feed, then this contains the source feed metadata.
    pub source: Option<String>,
    /// Atom (optional): Conveys information about rights, e.g. copyrights, held in and over the feed.
    pub rights: Option<Text>,

    /// Extension for MediaRSS - https://www.rssboard.org/media-rss
    /// A MediaObject will be created in two cases:
    /// 1) each "media:group" element encountered in the feed
    /// 2) a default for any other "media:*" elements found at the item level
    /// See the Atom tests for youtube and newscred for examples
    pub media: Vec<MediaObject>,
}
impl FeedConvert for feed_rs::model::Entry {
    type Output = Entry;

    fn convert(self) -> Self::Output {
        let feed_rs::model::Entry {
            id,
            title,
            updated,
            authors,
            content,
            links,
            summary,
            categories,
            contributors,
            published,
            source,
            rights,
            media,
        } = self;
        Entry {
            id,
            title: title.convert(),
            updated,
            authors: authors.convert(),
            content: content.convert(),
            links: links.convert(),
            summary: summary.convert(),
            categories: categories.convert(),
            contributors: contributors.convert(),
            published,
            source,
            rights: rights.convert(),
            media: media.convert(),
        }
    }
}
