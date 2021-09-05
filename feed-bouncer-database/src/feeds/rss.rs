use std::collections::BTreeMap;

use rss::{
    extension::{atom, dublincore, itunes, syndication},
    Category, Channel, Cloud, Image, TextInput,
};

use crate::feeds::FeedConvert;

pub type ExtensionMap = BTreeMap<String, BTreeMap<String, Vec<Extension>>>;

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Extension {
    pub name: String,
    pub value: Option<String>,
    pub attrs: BTreeMap<String, String>,
    pub children: BTreeMap<String, Vec<Extension>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct ChannelHeader {
    /// The name of the channel.
    pub title: String,
    /// The URL for the website corresponding to the channel.
    pub link: String,
    /// A description of the channel.
    pub description: String,
    /// The language of the channel.
    pub language: Option<String>,
    /// The copyright notice for the channel.
    pub copyright: Option<String>,
    /// The email address for the managing editor.
    pub managing_editor: Option<String>,
    /// The email address for the webmaster.
    pub webmaster: Option<String>,
    /// The publication date for the content of the channel as an RFC822 timestamp.
    pub pub_date: Option<String>,
    /// The date that the contents of the channel last changed as an RFC822 timestamp.
    pub last_build_date: Option<String>,
    /// The categories the channel belongs to.
    pub categories: Vec<Category>,
    /// A string indicating the program used to generate the channel.
    pub generator: Option<String>,
    /// A URL that points to the documentation for the RSS format.
    pub docs: Option<String>,
    /// The cloud to register with to be notified of updates to the channel.
    pub cloud: Option<Cloud>,
    /// The PICS rating for the channel.
    pub rating: Option<String>,
    /// The number of minutes the channel can be cached before refreshing.
    pub ttl: Option<String>,
    /// An image that can be displayed with the channel.
    pub image: Option<Image>,
    /// A text input box that can be displayed with the channel.
    pub text_input: Option<TextInput>,
    /// A hint to tell the aggregator which hours it can skip.
    pub skip_hours: Vec<String>,
    /// A hint to tell the aggregator which days it can skip.
    pub skip_days: Vec<String>,
    /// The extensions for the channel.
    pub extensions: ExtensionMap,
    /// The Atom extension for the channel.
    pub atom_ext: Option<atom::AtomExtension>,
    /// The iTunes extension for the channel.
    pub itunes_ext: Option<itunes::ITunesChannelExtension>,
    /// The Dublin Core extension for the channel.
    pub dublin_core_ext: Option<dublincore::DublinCoreExtension>,
    /// The Syndication extension for the channel.
    pub syndication_ext: Option<syndication::SyndicationExtension>,
    /// The namespaces present in the RSS tag.
    pub namespaces: BTreeMap<String, String>,
}

impl ChannelHeader {
    pub fn split(channel: Channel) -> (Self, Vec<Item>) {
        let Channel {
            title,
            link,
            description,
            language,
            copyright,
            managing_editor,
            webmaster,
            pub_date,
            last_build_date,
            categories,
            generator,
            docs,
            cloud,
            rating,
            ttl,
            image,
            text_input,
            skip_hours,
            skip_days,
            items,
            extensions,
            atom_ext,
            itunes_ext,
            dublin_core_ext,
            syndication_ext,
            namespaces,
        } = channel;

        (
            Self {
                title,
                link,
                description,
                language,
                copyright,
                managing_editor,
                webmaster,
                pub_date,
                last_build_date,
                categories,
                generator,
                docs,
                cloud,
                rating,
                ttl,
                image,
                text_input,
                skip_hours,
                skip_days,

                extensions: extensions.convert(),
                atom_ext,
                itunes_ext,
                dublin_core_ext,
                syndication_ext,
                namespaces: namespaces.into_iter().collect(),
            },
            items.convert(),
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Item {
    /// The title of the item.
    pub title: Option<String>,
    /// The URL of the item.
    pub link: Option<String>,
    /// The item synopsis.
    pub description: Option<String>,
    /// The email address of author of the item.
    pub author: Option<String>,
    /// The categories the item belongs to.
    pub categories: Vec<Category>,
    /// The URL for the comments page of the item.
    pub comments: Option<String>,
    /// The description of a media object that is attached to the item.
    pub enclosure: Option<rss::Enclosure>,
    /// A unique identifier for the item.
    pub guid: Option<rss::Guid>,
    /// The date the item was published as an RFC 2822 timestamp.
    pub pub_date: Option<String>,
    /// The RSS channel the item came from.
    pub source: Option<rss::Source>,
    /// The HTML contents of the item.
    pub content: Option<String>,
    /// The extensions for the item.
    pub extensions: ExtensionMap,
    /// The Atom extension for the channel.
    pub atom_ext: Option<atom::AtomExtension>,
    /// The iTunes extension for the item.
    pub itunes_ext: Option<itunes::ITunesItemExtension>,
    /// The Dublin Core extension for the item.
    pub dublin_core_ext: Option<dublincore::DublinCoreExtension>,
}

impl FeedConvert for rss::extension::Extension {
    type Output = Extension;

    fn convert(self) -> Self::Output {
        let rss::extension::Extension {
            name,
            value,
            attrs,
            children,
        } = self;

        Extension {
            name,
            value,
            attrs: attrs.convert(),
            children: children.convert(),
        }
    }
}

impl FeedConvert for rss::Item {
    type Output = Item;

    fn convert(self) -> Self::Output {
        let rss::Item {
            title,
            link,
            description,
            author,
            categories,
            comments,
            enclosure,
            guid,
            pub_date,
            source,
            content,
            extensions,
            atom_ext,
            itunes_ext,
            dublin_core_ext,
        } = self;

        Item {
            title,
            link,
            description,
            author,
            categories,
            comments,
            enclosure,
            guid,
            pub_date,
            source,
            content,
            extensions: extensions.convert(),
            atom_ext,
            itunes_ext,
            dublin_core_ext,
        }
    }
}
