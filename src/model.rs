//! Structs modelling the data types used by the API.
#![allow(clippy::use_self)]

pub mod builders;
pub mod id;

use std::{fmt::Display, ops::Deref, string::ToString};

use chrono::{DateTime, Duration, Utc};
use itertools::Itertools;
use serde::{self, Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use serde_with::{serde_as, CommaSeparator, DisplayFromStr, DurationSeconds, StringWithSeparator};
use strum::Display as EnumDisplay;

use crate::util::is_default;

use self::id::{ChannelId, VideoId};

#[serde_as]
#[derive(Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Filtering criteria for the various video endpoints.
pub struct VideoFilter {
    /// Only return videos from that channel.
    pub channel_id: Option<ChannelId>,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, VideoId>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Only return videos with any of these IDs.
    pub id: Vec<VideoId>,
    /// Only return videos from a specific organization.
    pub org: Option<Organisation>,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, ExtraVideoInfo>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Extra information to include with each video.
    pub include: Vec<ExtraVideoInfo>,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, Language>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// If only videos of a specific [`Language`] should be returned.
    pub lang: Vec<Language>,
    /// Max amount of hours in the future to return videos from. Videos scheduled further in the future will not be returned.
    pub max_upcoming_hours: u32,
    /// If only videos mentioning a specific channel should be returned.
    pub mentioned_channel_id: Option<ChannelId>,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, VideoStatus>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Which statuses the videos should have.
    pub status: Vec<VideoStatus>,
    /// A topic that the videos should be related to.
    pub topic: Option<String>,
    #[serde(rename = "type")]
    /// The type of the videos.
    pub video_type: VideoType,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(skip_serializing_if = "is_default")]
    /// If the results should be paginated.
    /// If so, the length of the results will limited to `limit`, with an offset of `offset`.
    pub paginated: bool,
    /// If `paginated` is true, only this many videos will be returned.
    pub limit: u32,
    /// If `paginated` is true, the results will be offset by this many videos.
    pub offset: i32,

    #[serde(rename = "sort")]
    /// By what criteria the videos should be sorted.
    pub sort_by: VideoSortingCriteria,
    /// In what order the videos should be sorted, ascending or descending.
    pub order: Order,
}

impl VideoFilter {
    #[must_use]
    /// Create a new `VideoFilter` with default values.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for VideoFilter {
    fn default() -> Self {
        Self {
            channel_id: None,
            id: Vec::new(),
            include: vec![ExtraVideoInfo::LiveInfo],
            lang: vec![Language::All],
            limit: 9999,
            max_upcoming_hours: 48,
            mentioned_channel_id: None,
            offset: 0,
            order: Order::Descending,
            org: Some(Organisation::Hololive),
            paginated: true,
            sort_by: VideoSortingCriteria::AvailableAt,
            status: Vec::new(),
            topic: None,
            video_type: VideoType::Stream,
        }
    }
}

impl Display for VideoFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ channel_id: {}, id: {}, org: {}, include: {}, lang: {}, max_upcoming_hours: {}, mentioned_channel_id: {}, paginated: {}, limit: {}, offset: {}, sort_by: {}, order: {}, status: {}, topic: {}, video_type: {} }}",
            stringify!(VideoFilter),
            self.channel_id.as_ref().map_or("None", |id| &*id.0),
            self.id.iter().map(ToString::to_string).join(", "),
            self.org.as_ref().map_or("None".to_owned(), ToString::to_string),
            self.include.iter().map(ToString::to_string).join(", "),
            self.lang.iter().map(ToString::to_string).join(", "),
            self.max_upcoming_hours,
            self.mentioned_channel_id.as_ref().map_or("None", |id| &*id.0),
            self.paginated,
            self.limit,
            self.offset,
            self.sort_by,
            self.order,
            self.status.iter().map(ToString::to_string).join(", "),
            self.topic.as_ref().map_or("None".to_owned(), ToString::to_string),
            self.video_type,
        )
    }
}

#[serde_as]
#[derive(Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Filtering criteria for videos related to a channel.
pub struct ChannelVideoFilter {
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, ExtraVideoInfo>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Extra information to include with each video.
    pub include: Vec<ExtraVideoInfo>,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, Language>")]
    #[serde(rename = "lang")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// If only videos of a specific [`Language`] should be returned.
    pub languages: Vec<Language>,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(skip_serializing_if = "is_default")]
    /// If the results should be paginated.
    /// If so, the length of the results will limited to `limit`, with an offset of `offset`.
    pub paginated: bool,
    /// If `paginated` is true, only this many videos will be returned.
    pub limit: u32,
    /// If `paginated` is true, the results will be offset by this many videos.
    pub offset: i32,
}

impl ChannelVideoFilter {
    #[must_use]
    /// Create a new `ChannelVideoFilter` with default values.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ChannelVideoFilter {
    fn default() -> Self {
        Self {
            include: vec![ExtraVideoInfo::LiveInfo],
            languages: vec![Language::All],
            limit: 100,
            offset: 0,
            paginated: true,
        }
    }
}

impl Display for ChannelVideoFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ include: {}, lang: {}, paginated: {}, limit: {}, offset: {} }}",
            stringify!(ChannelVideoFilter),
            self.include.iter().map(ToString::to_string).join(", "),
            self.languages.iter().map(ToString::to_string).join(", "),
            self.paginated,
            self.limit,
            self.offset
        )
    }
}

#[serde_as]
#[derive(Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Filtering criteria for channels.
pub struct ChannelFilter {
    #[serde(rename = "lang")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Only show channels that uses any of the given languages as their main language.
    pub languages: Vec<Language>,
    /// In what order the channels should be sorted, ascending or descending.
    pub order: Order,
    #[serde(rename = "sort")]
    /// By what criteria the channels should be sorted.
    pub sort_by: ChannelSortingCriteria,

    #[serde(rename = "org")]
    /// Only return channels from a specific organization.
    pub organisation: Option<Organisation>,
    #[serde(rename = "type")]
    /// Only show channels of the given type.
    pub channel_type: Option<ChannelType>,

    /// Limit the number of returned channels to the given value.
    ///
    /// Value must be between `0` and `50`, inclusive.
    pub limit: u32,
    /// Offset the returned values by the given amount of places.
    pub offset: i32,
}

impl Default for ChannelFilter {
    fn default() -> Self {
        Self {
            languages: Vec::new(),
            order: Order::Ascending,
            sort_by: ChannelSortingCriteria::Organisation,
            organisation: None,
            channel_type: None,
            limit: 25,
            offset: 0,
        }
    }
}

#[serde_as]
#[derive(Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Filtering criteria for video searches.
pub struct VideoSearch {
    #[serde(rename = "sort")]
    /// In what order the videos should be returned.
    pub sort_order: SearchOrder,

    #[serde(rename = "lang")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Filter away any clips that are not in any of the given languages.
    ///
    /// Streams will always be included no matter their language.
    pub languages: Vec<Language>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Only return videos that are any of the given types.
    pub types: Vec<VideoType>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Only return videos that meet the given conditions.
    pub conditions: Vec<VideoSearchCondition>,
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Only return videos that are related to any of the given topics.
    pub topics: Vec<String>,
    #[serde(rename = "vch")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Only return videos that involve all of the given channels.
    ///
    /// If two or more channel IDs are specified, only collabs with all of them will be returned,
    /// or if one channel is a clipper, it will only show clips of the other channels made by this clipper.
    pub channels: Vec<ChannelId>,
    #[serde(rename = "org")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Only return videos from channels in the given organisation,
    /// or are clips from a channel in the organisation.
    pub organisations: Vec<Organisation>,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(skip_serializing_if = "is_default")]
    /// If the results should be paginated.
    /// If so, the length of the results will limited to `limit`, with an offset of `offset`.
    pub paginated: bool,
    /// If `paginated` is true, only this many videos will be returned.
    pub limit: u32,
    /// If `paginated` is true, the results will be offset by this many videos.
    pub offset: i32,
}

impl Default for VideoSearch {
    fn default() -> Self {
        Self {
            sort_order: SearchOrder::Newest,
            languages: Vec::default(),
            types: Vec::default(),
            conditions: Vec::default(),
            topics: Vec::default(),
            channels: Vec::default(),
            organisations: Vec::default(),
            paginated: true,
            limit: 30,
            offset: 0,
        }
    }
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all(serialize = "snake_case"))]
/// A condition that a video must meet to be eligible.
pub enum VideoSearchCondition {
    /// The video must include this string in its title or description.
    Text(String),
}

#[serde_as]
#[derive(Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Filtering criteria for comment searches.
pub struct CommentSearch {
    /// Only return comments that include the given substring.
    pub search: String,
    #[serde(rename = "sort")]
    /// In what order the comments should be returned.
    pub sort_order: SearchOrder,

    #[serde(rename = "lang")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Filter away any comments on clips that are not in any of the given languages.
    ///
    /// Comment on streams will always be included no matter their language.
    pub languages: Vec<Language>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Only return comments on videos that are any of the given types.
    pub types: Vec<VideoType>,
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Only return comments on videos that are related to any of the given topics.
    pub topics: Vec<String>,
    #[serde(rename = "vch")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Only return comments on videos that involve all of the given channels.
    ///
    /// If two or more channel IDs are specified,
    /// only comments on collabs with all of them will be returned,
    /// or if one channel is a clipper,
    /// it will only return comments on clips of the other channels made by this clipper.
    pub channels: Vec<ChannelId>,
    #[serde(rename = "org")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Only return comments on videos from channels in the given organisation,
    /// or that are clips from a channel in the organisation.
    pub organisations: Vec<Organisation>,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(skip_serializing_if = "is_default")]
    /// If the results should be paginated.
    /// If so, the length of the results will limited to `limit`, with an offset of `offset`.
    pub paginated: bool,
    /// If `paginated` is true, only this many comments will be returned.
    pub limit: u32,
    /// If `paginated` is true, the results will be offset by this many comments.
    pub offset: i32,
}

impl Default for CommentSearch {
    fn default() -> Self {
        Self {
            search: String::default(),
            sort_order: SearchOrder::Newest,
            languages: Vec::default(),
            types: Vec::default(),
            topics: Vec::default(),
            channels: Vec::default(),
            organisations: Vec::default(),
            paginated: true,
            limit: 30,
            offset: 0,
        }
    }
}

#[derive(Serialize, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all(serialize = "snake_case"))]
/// The order in which search results should be returned.
pub enum SearchOrder {
    /// Return the oldest videos first.
    Oldest,
    /// Return the newest videos first.
    Newest,
}

#[non_exhaustive]
#[derive(Serialize, Debug, EnumDisplay, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all(serialize = "snake_case"))]
#[strum(serialize_all = "snake_case")]
/// What extra info to include in the response.
pub enum ExtraVideoInfo {
    /// Any clips created from the video.
    Clips,
    /// Any videos referencing the video in their description.
    Refers,
    /// Any videos listed as sources for the video.
    Sources,
    /// Any videos that refer to the video and go live or are uploaded around the same time.
    Simulcasts,
    /// Any channels mentioned in the description of the video.
    Mentions,
    /// The description of the video.
    Description,
    /// The [`VideoLiveInfo`] of the video, if it is a stream.
    LiveInfo,
    /// The statistics of the channel that uploaded the video.
    ChannelStats,
    /// Any songs that were played in the video.
    Songs,
}

#[non_exhaustive]
#[derive(
    Serialize_enum_str, Deserialize_enum_str, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
/// What language to filter videos by.
pub enum Language {
    #[serde(rename = "all")]
    /// Include all languages.
    All,
    #[serde(rename = "en")]
    /// Only English videos.
    English,
    #[serde(rename = "es")]
    /// Only Spanish videos.
    Spanish,
    #[serde(rename = "id")]
    /// Only Indonesian videos.
    Indonesian,
    #[serde(rename = "ja")]
    /// Only Japanese videos.
    Japanese,
    #[serde(rename = "ko")]
    /// Only Korean videos.
    Korean,
    #[serde(rename = "ru")]
    /// Only Russian videos.
    Russian,
    #[serde(rename = "zh")]
    /// Only Chinese videos.
    Chinese,

    /// Other language, please open a pull request to add support for it!
    #[serde(other)]
    Other(String),
}

#[derive(
    Serialize, Deserialize, EnumDisplay, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
/// What order items should be returned in, ascending or descending.
pub enum Order {
    #[serde(rename = "asc")]
    /// Sort videos in ascending order.
    Ascending,
    #[serde(rename = "desc")]
    /// Sort videos in descending order.
    Descending,
}

#[non_exhaustive]
#[allow(clippy::use_self)]
#[derive(
    Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "PascalCase")]
/// Which organization the VTuber(s) are a part of.
pub enum Organisation {
    /// VTubers from [Hololive Production](https://en.hololive.tv/)
    Hololive,
    /// VTubers from [Nijisanji](https://www.nijisanji.jp/en/)
    Nijisanji,
    /// VTubers not part of any organization.
    Independents,
    #[serde(other)]
    /// Organization not covered by other variants, please submit a pull request to add them!
    Other(String),
}

#[non_exhaustive]
#[derive(
    Serialize, Deserialize, EnumDisplay, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "snake_case")]
/// Different criteria for sorting videos.
pub enum VideoSortingCriteria {
    /// Sort by [`Video::id`].
    Id,
    /// Sort alphabetically by [`Video::title`].
    Title,
    /// Sort by the [`VideoType`] of the video.
    Type,
    #[serde(rename = "topic_id")]
    /// Sort by [`Video::topics`].
    Topics,
    /// Sort by when the video was first published.
    PublishedAt,
    /// Sort by the first `Some` value of [`live_info.end_actual`][`VideoLiveInfo::end_actual`],
    /// [`live_info.start_actual`][`VideoLiveInfo::start_actual`],
    /// [`live_info.start_scheduled`][VideoLiveInfo::start_scheduled`], or
    /// [`published_at`][`Video::published_at`].
    AvailableAt,
    /// Sort by video length.
    Duration,
    /// Sort by the [`VideoStatus`] of the video.
    Status,
    /// Sort by when the video is scheduled to start, if it is a stream or premiere.
    StartScheduled,
    /// Sort by when the video started, if it is a stream or premiere.
    StartActual,
    /// Sort by when the video ended, if it is a stream or premiere.
    EndActual,
    /// Sort by amount of viewers, if the video is a stream or premiere.
    LiveViewers,
    /// Sort alphabetically by video description.
    Description,
    #[serde(rename = "songcount")]
    /// Sort by amount of songs in the video.
    SongCount,
    /// Sort alphabetically by the uploader's channel ID.
    ChannelId,
}

#[non_exhaustive]
#[derive(
    Serialize, Deserialize, EnumDisplay, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "snake_case")]
/// Different criteria for sorting channels.
pub enum ChannelSortingCriteria {
    /// Sort by [`Channel::id`].
    Id,
    /// Sort alphabetically by the channel's name..
    Name,
    /// Sort alphabetically by the channel's English name.
    EnglishName,
    /// Sort by the [`ChannelType`] of the channel.
    Type,
    #[serde(rename = "org")]
    /// Sort by the [`Organisation`] the channel belongs to.
    Organisation,
    #[serde(rename = "suborg")]
    /// Sort by the sub-organisation the channel belongs to.
    SubOrganisation,
    /// Sort by the URL of the channel's profile picture.
    Photo,
    /// Sort by the URL of the channel's banner image.
    Banner,
    /// Sort by the channel's Twitter handle.
    Twitter,
    /// Sort by the number of videos the channel has uploaded.
    VideoCount,
    /// Sort by the number of subscribers the channel has.
    SubscriberCount,
    /// Sort by the number of views the channel has.
    ViewCount,
    /// Sort by the number of clips made that involves the channel.
    ClipCount,
    #[serde(rename = "lang")]
    /// Sort by the primary [`Language`] of the channel.
    Language,
    /// Sort by when the channel was first published.
    PublishedAt,
    /// Sort by if the channel is marked as [`Channel::inactive`] or not.
    Inactive,
    /// Sort alphabetically by channel description.
    Description,
}

#[non_exhaustive]
#[derive(
    Serialize, Deserialize, EnumDisplay, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "snake_case")]
/// The type of the video.
pub enum VideoType {
    /// The video is a livestream.
    Stream,
    /// The video is a clip.
    Clip,
}

#[non_exhaustive]
#[derive(
    Serialize, Deserialize, Debug, EnumDisplay, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "snake_case")]
/// The type of videos to fetch from a channel.
pub enum ChannelVideoType {
    /// Clip videos of a `VTuber` channel.
    Clips,
    /// Uploaded videos from this channel.
    Videos,
    /// Videos uploaded by other channels that mention this channel.
    Collabs,
}

#[non_exhaustive]
#[allow(dead_code)]
#[derive(
    Serialize, Deserialize, Debug, Copy, Clone, EnumDisplay, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "lowercase")]
/// The status of the [`Video`].
pub enum VideoStatus {
    /// The video hasn't been properly indexed yet.
    New,
    /// The video is scheduled to be available at a later time.
    Upcoming,
    /// The video is a stream that is currently live or a video that is premiering.
    Live,
    /// The video is a stream that has ended or a video that has premiered.
    Past,
    /// The video used to exist, but is no longer available.
    Missing,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
/// Workaround for Holodex API returning [`PaginatedResult::total`] as either `String` or `u32`.
pub enum PaginatedTotal {
    /// The total returned as an `u32`.
    U32(u32),
    /// The total returned as a `String`, parsed into an `u32`.
    String(#[serde_as(as = "DisplayFromStr")] u32),
}

#[allow(clippy::from_over_into)]
impl Into<u32> for PaginatedTotal {
    #[inline]
    fn into(self) -> u32 {
        match self {
            PaginatedTotal::U32(n) | PaginatedTotal::String(n) => n,
        }
    }
}

#[serde_as]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
/// A paginated result.
pub enum PaginatedResult<T> {
    /// All items that matched the criteria.
    Items(Vec<T>),
    /// A paginated result.
    Page {
        /// How many items in total matched the criteria.
        total: PaginatedTotal,
        #[serde(default = "Default::default")]
        /// `limit` items, offset by `offset`.
        items: Vec<T>,
    },
}

impl<T> PaginatedResult<T> {
    #[must_use]
    #[inline]
    /// Get the items from the response.
    pub fn items(&self) -> &[T] {
        match self {
            PaginatedResult::Items(items) | PaginatedResult::Page { items, .. } => items,
        }
    }

    #[must_use]
    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    /// Convert response into a [`Vec<T>`].
    pub fn into_items(self) -> Vec<T> {
        match self {
            PaginatedResult::Items(items) | PaginatedResult::Page { items, .. } => items,
        }
    }
}

impl<T> Deref for PaginatedResult<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.items()
    }
}

impl<T> IntoIterator for PaginatedResult<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        match self {
            PaginatedResult::Items(items) | PaginatedResult::Page { items, .. } => {
                items.into_iter()
            }
        }
    }
}

#[allow(clippy::from_over_into)]
impl<T> Into<Vec<T>> for PaginatedResult<T> {
    #[inline]
    fn into(self) -> Vec<T> {
        self.into_items()
    }
}

#[serde_as]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A video, that can be either a stream, premiere, or clip.
pub struct Video {
    /// The ID of the video.
    pub id: VideoId,
    /// The title of the video.
    pub title: String,
    #[serde(rename = "type")]
    /// The type of the video.
    pub video_type: VideoType,
    #[serde(default)]
    #[serde(rename = "topic_id")]
    /// The topic(s) the video is about.
    /// Videos of type `clip` cannot not have topic.
    pub topics: Option<String>,
    #[serde(default)]
    /// The date the video was first published.
    pub published_at: Option<DateTime<Utc>>,
    /// Takes on the first `Some` value of [`live_info.end_actual`][`VideoLiveInfo::end_actual`],
    /// [`live_info.start_actual`][`VideoLiveInfo::start_actual`],
    /// [`live_info.start_scheduled`][VideoLiveInfo::start_scheduled`], or
    /// [`published_at`](#structfield.published_at).
    pub available_at: DateTime<Utc>,
    #[serde_as(as = "DurationSeconds<i64>")]
    /// The length of the video in seconds.
    pub duration: Duration,
    /// The status of the video.
    pub status: VideoStatus,
    #[serde(flatten)]
    /// Live stream information regarding the video, if it is a stream.
    ///
    /// Included when [`VideoFilter::include`] includes [`ExtraVideoInfo::LiveInfo`].
    pub live_info: VideoLiveInfo,
    #[serde(default)]
    /// The description of the video.
    ///
    /// Included when [`VideoFilter::include`] includes [`ExtraVideoInfo::Description`].
    pub description: Option<String>,
    #[serde(rename = "songcount")]
    #[serde(default)]
    /// How many songs have been sung in the video, if any.
    pub song_count: Option<u32>,
    #[serde(alias = "channel_id")]
    /// The channel the video was uploaded by.
    pub channel: VideoChannel,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Smaller version of [`Channel`] with less metadata.
pub struct ChannelMin {
    /// The ID of the channel.
    pub id: ChannelId,
    /// The name of the channel.
    pub name: String,
    #[serde(default)]
    /// The English name of the channel, if any.
    pub english_name: Option<String>,
    #[serde(rename = "type")]
    /// The type of the channel.
    pub channel_type: Option<ChannelType>,
    /// The URL of the channel's profile picture.
    pub photo: String,
    #[serde(default)]
    /// The organization the channel belongs to, if any.
    pub org: Option<Organisation>,

    #[serde(flatten)]
    /// Channel statistics.
    pub stats: ChannelStats,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A channel that uploads videos and/or streams.
pub struct Channel {
    /// The ID of the channel.
    pub id: ChannelId,
    /// The name of the channel.
    pub name: String,
    #[serde(default)]
    /// If the channel has been marked as inactive.
    pub inactive: bool,
    #[serde(rename = "type")]
    /// The type of the channel.
    pub channel_type: ChannelType,

    #[serde(default)]
    /// The description of the channel.
    pub description: Option<String>,
    #[serde(default)]
    /// The primary language of the channel, if any.
    pub lang: Option<Language>,
    #[serde(default)]
    /// The English name of the channel, if any.
    pub english_name: Option<String>,
    #[serde(default)]
    /// The organization the channel belongs to, if any.
    pub org: Option<Organisation>,
    #[serde(default)]
    /// The sub-organization the channel belongs to, if any.
    pub suborg: Option<String>,
    #[serde(default)]
    /// The URL of the channel's profile picture, if any.
    pub photo: Option<String>,
    #[serde(default)]
    /// The URL of the channel's banner picture, if any.
    pub banner: Option<String>,
    #[serde(default)]
    /// The Twitter handle of the channel, if any.
    pub twitter: Option<String>,

    #[serde(flatten)]
    /// Channel statistics.
    pub stats: ChannelStats,

    #[serde(default)]
    /// The top topics associated with the channel.
    pub top_topics: Vec<String>,

    /// The date the channel was created.
    pub published_at: Option<DateTime<Utc>>,
    /// The date this channel metadata was last indexed.
    pub crawled_at: Option<DateTime<Utc>>,
    /// The date the comments posted on videos uploaded by this channel were last indexed.
    pub comments_crawled_at: Option<DateTime<Utc>>,
}

#[serde_as]
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Various statistics about a channel.
pub struct ChannelStats {
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// The amount of videos the channel has uploaded.
    pub video_count: Option<u32>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// The amount of subscribers the channel has.
    pub subscriber_count: Option<u32>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// The amount of views the channel has in total.
    pub view_count: Option<u32>,
    #[serde(default)]
    /// The amount of clips that have been made from videos uploaded by this channel.
    pub clip_count: Option<u32>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
/// A channel reference.
pub enum VideoChannel {
    /// A channel ID.
    Id(ChannelId),
    /// An object containing some channel metadata.
    Min(ChannelMin),
}

impl VideoChannel {
    #[inline]
    #[must_use]
    /// Returns the channel ID.
    pub const fn id(&self) -> &ChannelId {
        match self {
            Self::Id(id) => id,
            Self::Min(d) => &d.id,
        }
    }
}

#[non_exhaustive]
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "lowercase")]
/// Different types of channels.
pub enum ChannelType {
    /// A VTuber that provides content, such as streams or videos.
    VTuber,
    /// A channel that takes content from a `VTuber` and edits it to make it more accessible.
    Subber,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A struct containing information about a video and any possible extra metadata that was requested.
pub struct VideoFull {
    #[serde(flatten)]
    /// A video.
    pub video: Video,

    #[serde(default)]
    /// Any clips that were made from this video.
    pub clips: Vec<Video>,
    #[serde(default)]
    /// Any sources this video was based on.
    pub sources: Vec<Video>,
    #[serde(default)]
    /// Any videos that were mentioned in this video's description.
    pub refers: Vec<Video>,
    #[serde(default)]
    /// Any videos that refer to this video and go live or are uploaded around the same time.
    pub simulcasts: Vec<Video>,
    #[serde(default)]
    /// Any channels that were mentioned in this video's description.
    pub mentions: Vec<ChannelMin>,

    #[serde(default)]
    #[serde(rename = "songcount")]
    /// How many songs were sung in this video.
    pub song_count: Option<u32>,
    #[serde(default)]
    /// Songs that were sung in this video.
    pub songs: Vec<Song>,

    #[serde(default)]
    /// Comments posted on this video.
    pub comments: Vec<Comment>,

    #[serde(default)]
    #[serde(alias = "recommendations")]
    /// Related videos.
    pub related: Vec<Video>,
}

#[derive(
    Deserialize, Serialize, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(default)]
/// The livestream metadata of a video.
pub struct VideoLiveInfo {
    /// When the stream is scheduled to start.
    pub start_scheduled: Option<DateTime<Utc>>,
    /// When the stream actually started.
    pub start_actual: Option<DateTime<Utc>>,
    /// When the stream ended.
    pub end_actual: Option<DateTime<Utc>>,
    /// The amount of viewers the stream has, if applicable.
    pub live_viewers: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A comment that was left on a video.
pub struct Comment {
    /// The ID of the comment.
    pub comment_key: String,
    #[serde(default)]
    /// The ID of the video the comment was left on.
    pub video_id: Option<VideoId>,
    /// The message contents of the comment.
    pub message: String,
}

impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A song that was played in a video.
pub struct Song {
    /// The name of the song.
    pub name: String,
    #[serde(rename = "original_artist")]
    /// The artist of the song.
    pub artist: String,
    #[serde(rename = "art")]
    /// URL to song artwork, if available.
    pub artwork: Option<String>,
    #[serde(rename = "itunesid")]
    /// The ID of the song on iTunes, if available.
    pub itunes_id: Option<u64>,

    #[serde_as(as = "DurationSeconds<i64>")]
    /// When in the video the song started being played.
    pub start: Duration,
    #[serde_as(as = "DurationSeconds<i64>")]
    /// When in the video the song finished being played.
    pub end: Duration,
}

impl Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} by {}", self.name, self.artist)
    }
}
