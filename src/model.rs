use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use serde_with::{serde_as, CommaSeparator, DisplayFromStr, StringWithSeparator};
use strum::Display;

use crate::util::is_default;

#[serde_as]
#[derive(Serialize, Debug, Clone)]
/// Filtering criteria for the various video endpoints.
pub struct VideoQueryParameters {
    /// Only return videos from that channel.
    pub channel_id: Option<String>,
    /// Only return the video with that specific ID.
    pub id: Option<String>,
    /// Only return videos from a specific organization.
    pub org: Option<Organisation>,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, ExtraVideoInfo>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Extra information to include with each video.
    pub include: Vec<ExtraVideoInfo>,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, VideoLanguage>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// If only videos of a specific [`VideoLanguage`] should be returned.
    pub lang: Vec<VideoLanguage>,
    /// Max amount of hours in the future to return videos from. Videos scheduled further in the future will not be returned.
    pub max_upcoming_hours: u32,
    /// If only videos mentioning a specific channel should be returned.
    pub mentioned_channel_id: Option<String>,

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
    pub sort_by: SortBy,
    /// In what order the videos should be sorted, ascending or descending.
    pub order: VideoOrder,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, VideoStatus>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Which statuses the videos should have.
    pub status: Vec<VideoStatus>,
    /// The topics that videos should be related to.
    pub topic: Option<String>,
    #[serde(rename = "type")]
    /// The type of the videos.
    pub video_type: VideoType,
}

impl Default for VideoQueryParameters {
    fn default() -> Self {
        Self {
            channel_id: None,
            id: None,
            include: vec![ExtraVideoInfo::LiveInfo],
            lang: vec![VideoLanguage::All],
            limit: 9999,
            max_upcoming_hours: 672,
            mentioned_channel_id: None,
            offset: 0,
            order: VideoOrder::Descending,
            org: Some(Organisation::Hololive),
            paginated: true,
            sort_by: SortBy::AvailableAt,
            status: Vec::new(),
            topic: None,
            video_type: VideoType::Stream,
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Debug, Display, Copy, Clone)]
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
#[derive(Serialize, Debug, Display, Copy, Clone)]
#[serde(rename_all(serialize = "lowercase"))]
#[strum(serialize_all = "lowercase")]
/// What language to filter videos by.
pub enum VideoLanguage {
    /// All languages.
    All,
    /// Only English videos.
    EN,
    /// Only Japanese videos.
    JP,
}

#[derive(Serialize, Debug, Copy, Clone)]
/// What order that videos should be in, ascending or descending.
/// For specifying what order they should be sorted in, see [`SortBy`].
pub enum VideoOrder {
    #[serde(rename = "asc")]
    /// Sort videos in ascending order.
    Ascending,
    #[serde(rename = "desc")]
    /// Sort videos in descending order.
    Descending,
}

#[non_exhaustive]
#[allow(clippy::use_self)]
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
/// Which organization the VTuber(s) are a part of.
pub enum Organisation {
    /// VTubers from [`Hololive Production`]
    ///
    /// [`Hololive Production`]: https://en.hololive.tv/
    Hololive,
    /// VTubers from [`Nijisanji`]
    ///
    /// [`Nijisanji`]: https://www.nijisanji.jp/en/
    Nijisanji,
    /// VTubers not part of any organization.
    Independents,
    #[serde(other)]
    /// Organization not covered by other variants, please submit a pull request to add them!
    Other(String),
}

#[non_exhaustive]
#[derive(Serialize, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
/// Different criteria for sorting videos.
pub enum SortBy {
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
    /// Sort by when the video was made available.
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
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
/// The type of the video.
pub enum VideoType {
    /// The video is a livestream.
    Stream,
    /// The video is a clip.
    Clip,
}

#[non_exhaustive]
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Display, PartialEq, Eq)]
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
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
/// The result of calling an endpoint returning videos.
pub enum VideoResponse {
    /// All the videos that fit the [`VideoQueryParameters`] criteria.
    Videos(Vec<Video>),
    /// A paginated result.
    Page {
        #[serde_as(as = "DisplayFromStr")]
        /// How many videos in total matched the [`VideoQueryParameters`] criteria.
        total: i32,
        #[serde(default)]
        /// [`VideoQueryParameters::limit`] videos, offset by [`VideoQueryParameters::offset`].
        items: Vec<Video>,
    },
}

#[derive(Deserialize, Debug, Clone)]
/// A video, that can be either a stream, premiere, or clip.
pub struct Video {
    /// The ID of the video.
    pub id: String,
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
    /// Takes on the first Some value of [`live_info.end_actual`](VideoLiveInfo#structfield.end_actual),
    /// [`live_info.start_actual`](VideoLiveInfo#structfield.start_actual),
    /// [`live_info.start_scheduled`](VideoLiveInfo#structfield.start_scheduled), or
    /// [`published_at`](#structfield.published_at).
    pub available_at: DateTime<Utc>,
    /// The length of the video in seconds.
    pub duration: u32,
    /// The status of the video.
    pub status: VideoStatus,
    #[serde(flatten)]
    /// Live stream information regarding the video, if it is a stream.
    ///
    /// Included when [`VideoQueryParameters::include`] includes [`ExtraVideoInfo::LiveInfo`].
    pub live_info: VideoLiveInfo,
    #[serde(default)]
    /// The description of the video.
    ///
    /// Included when [`VideoQueryParameters::include`] includes [`ExtraVideoInfo::Description`].
    pub description: Option<String>,
    #[serde(default)]
    /// Map of live translators and the amount of messages they have sent for the video.
    pub live_tl_count: Option<HashMap<String, u32>>,
    #[serde(rename = "songcount")]
    #[serde(default)]
    /// How many songs have been sung in the video, if any.
    pub song_count: Option<u32>,
    #[serde(alias = "channel_id")]
    /// The channel the video was uploaded by.
    pub channel: VideoChannel,
}

#[derive(Deserialize, Debug, Clone)]
/// Smaller version of [`Channel`] with less metadata.
pub struct ChannelMin {
    /// The ID of the channel.
    pub id: String,
    /// The name of the channel.
    pub name: String,
    #[serde(default)]
    /// The English name of the channel, if any.
    pub english_name: Option<String>,
    #[serde(rename = "type")]
    /// The type of the channel.
    pub channel_type: ChannelType,
    /// The URL of the channel's profile picture.
    pub photo: String,
    #[serde(default)]
    /// The organization the channel belongs to, if any.
    pub org: Option<Organisation>,
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
/// A channel that uploads videos and/or streams.
pub struct Channel {
    /// The ID of the channel.
    pub id: String,
    /// The name of the channel.
    pub name: String,
    /// The description of the channel.
    pub description: String,
    /// If the channel has been marked as inactive.
    pub inactive: bool,

    #[serde(rename = "type")]
    /// The type of the channel.
    pub channel_type: ChannelType,

    #[serde(default)]
    /// The primary language of the channel, if any.
    pub lang: Option<String>,
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
    /// The URL of the channel's twitter profile, if any.
    pub twitter: Option<String>,

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
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// The amount of clips that have been made from videos uploaded by this channel.
    pub clip_count: Option<u32>,

    /// The date the channel was created.
    pub published_at: DateTime<Utc>,
    /// The date this channel metadata was last indexed.
    pub crawled_at: Option<DateTime<Utc>>,
    /// The date the comments posted on videos uploaded by this channel were last indexed.
    pub comments_crawled_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
/// A channel reference.
pub enum VideoChannel {
    /// A channel ID.
    Id(String),
    /// An object containing some channel metadata.
    Data(ChannelMin),
}

impl VideoChannel {
    #[inline]
    #[must_use]
    /// Returns the channel ID.
    pub fn id(&self) -> &str {
        match self {
            Self::Id(id) => id,
            Self::Data(d) => &d.id,
        }
    }
}

#[non_exhaustive]
#[allow(dead_code)]
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
/// Different types of channels.
pub enum ChannelType {
    /// A VTuber that provides content, such as streams or videos.
    VTuber,
    /// A channel that takes content from a `VTuber` and edits it to make it more accessible.
    Subber,
}

#[derive(Deserialize, Debug, Clone)]
/// A struct containing information about both a video and the channel it was uploaded by.
pub struct VideoWithChannel {
    #[serde(flatten)]
    /// A video.
    pub video: Video,
    #[serde(flatten)]
    /// The channel the video was uploaded by.
    pub channel: ChannelMin,
}

#[derive(Deserialize, Debug, Clone)]
/// A struct containing information about a video and any possible extra metadata that was requested.
pub struct VideoFull {
    #[serde(flatten)]
    /// A video.
    pub video: Video,

    #[serde(default)]
    /// Any clips that were made from this video.
    pub clips: Vec<VideoWithChannel>,
    #[serde(default)]
    /// Any sources this video was based on.
    pub sources: Vec<VideoWithChannel>,
    #[serde(default)]
    /// Any videos that were mentioned in this video's description.
    pub refers: Vec<VideoWithChannel>,
    #[serde(default)]
    /// Any videos that refer to this video and go live or are uploaded around the same time.
    pub simulcasts: Vec<VideoWithChannel>,
    #[serde(default)]
    /// Any channels that were mentioned in this video's description.
    pub mentions: Vec<ChannelMin>,
    #[serde(default)]
    #[serde(rename = "songs")]
    /// How many songs were sung in this video.
    pub song_count: Option<u32>,
}

#[derive(Deserialize, Debug, Clone, Copy)]
/// The livestream metadata of a video.
pub struct VideoLiveInfo {
    #[serde(default)]
    /// When the stream is scheduled to start.
    pub start_scheduled: Option<DateTime<Utc>>,
    #[serde(default)]
    /// When the stream actually started.
    pub start_actual: Option<DateTime<Utc>>,
    #[serde(default)]
    /// When the stream ended.
    pub end_actual: Option<DateTime<Utc>>,
    #[serde(default)]
    /// The amount of viewers the stream has, if applicable.
    pub live_viewers: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
/// A comment that was left on a video.
pub struct Comment {
    /// The ID of the comment.
    pub comment_key: String,
    /// The ID of the video the comment was left on.
    pub video_id: String,
    /// The message contents of the comment.
    pub message: String,
}
