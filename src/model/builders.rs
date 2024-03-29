//! Builders for ergonomically creating various large structs.

use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{self, Serialize};

use crate::errors::Error;

use super::{
    id::{ChannelId, VideoId},
    ChannelFilter, ChannelSortingCriteria, ChannelType, CommentSearch, ExtraVideoInfo, Language,
    Order, Organisation, SearchOrder, VideoFilter, VideoSearch, VideoSearchCondition,
    VideoSortingCriteria, VideoStatus, VideoType,
};

#[derive(Serialize, Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Builder for creating a [`VideoFilter`].
pub struct VideoFilterBuilder {
    filter: VideoFilter,
}

impl VideoFilterBuilder {
    #[inline]
    #[must_use]
    /// Create a new `VideoFilterBuilder` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    #[must_use]
    /// Request extra information to be included with each video.
    pub fn include(mut self, include: &[ExtraVideoInfo]) -> Self {
        self.filter.include = include.to_vec();
        self
    }

    #[inline]
    #[must_use]
    /// Enable pagination.
    pub const fn paginated(mut self, paginated: bool) -> Self {
        self.filter.paginated = paginated;
        self
    }

    #[inline]
    #[must_use]
    /// Limit how many videos are returned. This will turn on pagination.
    pub const fn limit(mut self, limit: u32) -> Self {
        self.filter.limit = limit;
        self.filter.paginated = true;
        self
    }

    #[inline]
    #[must_use]
    /// Offset the results by the given amount. This will turn on pagination.
    pub const fn offset(mut self, offset: i32) -> Self {
        self.filter.offset = offset;
        self.filter.paginated = true;
        self
    }

    #[inline]
    #[must_use]
    /// Sort videos by the given criteria.
    pub const fn sort_by(mut self, sort_by: VideoSortingCriteria) -> Self {
        self.filter.sort_by = sort_by;
        self
    }

    #[inline]
    #[must_use]
    /// Sort videos in the given order.
    pub const fn order(mut self, order: Order) -> Self {
        self.filter.order = order;
        self
    }

    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    /// Only return videos from the given channel.
    pub fn channel_id(mut self, channel_id: ChannelId) -> Self {
        self.filter.channel_id = Some(channel_id);
        self
    }

    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    /// Only return videos with any of the given IDs.
    pub fn id(mut self, ids: &[VideoId]) -> Self {
        self.filter.id = ids.to_vec();
        self
    }

    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    /// Only return videos from a channel part of the given organisation.
    pub fn organisation(mut self, org: Organisation) -> Self {
        self.filter.org = Some(org);
        self
    }

    #[inline]
    #[must_use]
    /// Only return videos in any of the given languages.
    pub fn language(mut self, lang: &[Language]) -> Self {
        self.filter.lang = lang.to_vec();
        self
    }

    #[inline]
    #[must_use]
    /// Only return videos scheduled to go live within the given amount of hours.
    pub const fn max_upcoming_hours(mut self, hours: u32) -> Self {
        self.filter.max_upcoming_hours = hours;
        self
    }

    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    /// Only return videos mentioning the given channel.
    pub fn mentioned_channel_id(mut self, channel_id: ChannelId) -> Self {
        self.filter.mentioned_channel_id = Some(channel_id);
        self
    }

    #[inline]
    #[must_use]
    /// Only return videos related to the given topic.
    pub fn topic(mut self, topic: &str) -> Self {
        self.filter.topic = Some(topic.to_owned());
        self
    }

    #[inline]
    #[must_use]
    /// Only return videos of the given type.
    pub const fn video_type(mut self, video_type: VideoType) -> Self {
        self.filter.video_type = video_type;
        self
    }

    #[inline]
    #[must_use]
    /// Only return videos with any of the given statuses.
    pub fn status(mut self, status: &[VideoStatus]) -> Self {
        self.filter.status = status.to_vec();
        self
    }

    #[inline]
    #[must_use]
    /// Only return videos made available after the given time.
    pub const fn after(mut self, after: DateTime<Utc>) -> Self {
        self.filter.from = Some(after);
        self
    }

    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    /// Consume the builder, returning the constructed filter.
    pub fn build(self) -> VideoFilter {
        self.filter
    }
}

impl Display for VideoFilterBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", stringify!(VideoFilterBuilder), self.filter)
    }
}

impl From<VideoFilterBuilder> for VideoFilter {
    fn from(builder: VideoFilterBuilder) -> Self {
        builder.filter
    }
}

#[derive(Serialize, Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Builder for creating a [`ChannelFilter`].
pub struct ChannelFilterBuilder {
    filter: ChannelFilter,
}

impl ChannelFilterBuilder {
    #[inline]
    #[must_use]
    /// Create a new `ChannelFilterBuilder` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    #[must_use]
    /// Sort channels by the given criteria.
    pub const fn sort_by(mut self, sort_by: ChannelSortingCriteria) -> Self {
        self.filter.sort_by = sort_by;
        self
    }

    #[inline]
    #[must_use]
    /// Sort channels in the given order.
    pub const fn order(mut self, order: Order) -> Self {
        self.filter.order = order;
        self
    }

    #[inline]
    #[must_use]
    /// Only return channels that uses any of the given languages as their main language.
    pub fn language(mut self, lang: &[Language]) -> Self {
        self.filter.languages = lang.to_vec();
        self
    }

    #[inline]
    #[must_use]
    /// Only return channels of the given type.
    pub const fn channel_type(mut self, channel_type: ChannelType) -> Self {
        self.filter.channel_type = Some(channel_type);
        self
    }

    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    /// Only return channels part of the given organisation.
    pub fn organisation(mut self, organisation: Organisation) -> Self {
        self.filter.organisation = Some(organisation);
        self
    }

    #[inline]
    #[must_use]
    /// Limit the number of returned channels to the given value.
    ///
    /// Value must be between `0` and `50`, inclusive.
    pub const fn limit(mut self, limit: u32) -> Self {
        self.filter.limit = limit;
        self
    }

    #[inline]
    #[must_use]
    /// Offset the returned values by the given amount of places.
    pub const fn offset(mut self, offset: i32) -> Self {
        self.filter.offset = offset;
        self
    }

    /// Consume the builder, returning the constructed filter.
    ///
    /// # Errors
    /// Will return [`Error::FilterCreationError`] if the filter was constructed with invalid arguments.
    pub fn build(self) -> Result<ChannelFilter, Error> {
        match &self.filter.limit {
            0..=50 => (),
            _ => {
                return Err(Error::FilterCreationError(format!(
                "Could not instantiate {} with a limit of {}. Valid range is 0 to 50, inclusive.",
                stringify!(ChannelFilter),
                self.filter.limit
            )))
            }
        }

        Ok(self.filter)
    }
}

#[derive(Serialize, Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Builder for creating a [`VideoSearch`].
pub struct VideoSearchBuilder {
    search: VideoSearch,
}

impl VideoSearchBuilder {
    #[inline]
    #[must_use]
    /// Create a new `VideoSearchBuilder` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    #[must_use]
    /// Only return videos that meet the given conditions.
    pub fn conditions(mut self, conditions: &[VideoSearchCondition]) -> Self {
        self.search.conditions = conditions.to_vec();
        self
    }

    #[inline]
    #[must_use]
    /// Enable pagination.
    pub const fn paginated(mut self, paginated: bool) -> Self {
        self.search.paginated = paginated;
        self
    }

    #[inline]
    #[must_use]
    /// Limit how many videos are returned. This will turn on pagination.
    pub const fn limit(mut self, limit: u32) -> Self {
        self.search.limit = limit;
        self.search.paginated = true;
        self
    }

    #[inline]
    #[must_use]
    /// Offset the results by the given amount. This will turn on pagination.
    pub const fn offset(mut self, offset: i32) -> Self {
        self.search.offset = offset;
        self.search.paginated = true;
        self
    }

    #[inline]
    #[must_use]
    /// In what order the videos should be returned.
    pub const fn order(mut self, order: SearchOrder) -> Self {
        self.search.sort_order = order;
        self
    }

    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    /// Only return videos that involve all of the given channels.
    ///
    /// If two or more channel IDs are specified, only collabs with all of them will be returned,
    /// or if one channel is a clipper, it will only show clips of the other channels made by this clipper.
    pub fn channels(mut self, channels: &[ChannelId]) -> Self {
        self.search.channels = channels.to_vec();
        self
    }

    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    /// Only return videos from channels in the given organisation, or are clips from a channel in the organisation.
    pub fn organisations(mut self, organisations: &[Organisation]) -> Self {
        self.search.organisations = organisations.to_vec();
        self
    }

    #[inline]
    #[must_use]
    /// Only return videos in any of the given languages.
    pub fn languages(mut self, languages: &[Language]) -> Self {
        self.search.languages = languages.to_vec();
        self
    }

    #[inline]
    #[must_use]
    /// Only return videos that are related to any of the given topics.
    pub fn topics(mut self, topics: &[String]) -> Self {
        self.search.topics = topics.to_vec();
        self
    }

    #[inline]
    #[must_use]
    /// Only return videos that are any of the given types.
    pub fn types(mut self, types: &[VideoType]) -> Self {
        self.search.types = types.to_vec();
        self
    }

    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    /// Consume the builder, returning the constructed search.
    pub fn build(self) -> VideoSearch {
        self.search
    }
}

impl From<VideoSearchBuilder> for VideoSearch {
    fn from(builder: VideoSearchBuilder) -> Self {
        builder.search
    }
}

#[derive(Serialize, Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Builder for creating a [`CommentSearch`].
pub struct CommentSearchBuilder {
    search: CommentSearch,
}

impl CommentSearchBuilder {
    #[inline]
    #[must_use]
    /// Create a new `CommentSearchBuilder` with default values and the given substring to search for.
    pub fn new(search: &str) -> Self {
        Self {
            search: CommentSearch {
                search: search.to_owned(),
                ..CommentSearch::default()
            },
        }
    }

    #[inline]
    #[must_use]
    /// Enable pagination.
    pub const fn paginated(mut self, paginated: bool) -> Self {
        self.search.paginated = paginated;
        self
    }

    #[inline]
    #[must_use]
    /// Limit how many comments on videos are returned. This will turn on pagination.
    pub const fn limit(mut self, limit: u32) -> Self {
        self.search.limit = limit;
        self.search.paginated = true;
        self
    }

    #[inline]
    #[must_use]
    /// Offset the results by the given amount. This will turn on pagination.
    pub const fn offset(mut self, offset: i32) -> Self {
        self.search.offset = offset;
        self.search.paginated = true;
        self
    }

    #[inline]
    #[must_use]
    /// In what order the comments should be returned.
    pub const fn order(mut self, order: SearchOrder) -> Self {
        self.search.sort_order = order;
        self
    }

    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    /// Only return comments on videos that involve all of the given channels.
    ///
    /// If two or more channel IDs are specified,
    /// only comments on collabs with all of them will be returned,
    /// or if one channel is a clipper,
    /// it will only return comments on clips of the other channels made by this clipper.
    pub fn channels(mut self, channels: &[ChannelId]) -> Self {
        self.search.channels = channels.to_vec();
        self
    }

    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    /// Only return comments on videos from channels in the given organisation,
    /// or that are clips from a channel in the organisation.
    pub fn organisations(mut self, organisations: &[Organisation]) -> Self {
        self.search.organisations = organisations.to_vec();
        self
    }

    #[inline]
    #[must_use]
    /// Filter away any comments on clips that are not in any of the given languages.
    ///
    /// Comment on streams will always be included no matter their language.
    pub fn languages(mut self, languages: &[Language]) -> Self {
        self.search.languages = languages.to_vec();
        self
    }

    #[inline]
    #[must_use]
    /// Only return comments on videos that are related to any of the given topics.
    pub fn topics(mut self, topics: &[String]) -> Self {
        self.search.topics = topics.to_vec();
        self
    }

    #[inline]
    #[must_use]
    /// Only return comments on videos that are any of the given types.
    pub fn types(mut self, types: &[VideoType]) -> Self {
        self.search.types = types.to_vec();
        self
    }

    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    /// Consume the builder, returning the constructed search.
    pub fn build(self) -> CommentSearch {
        self.search
    }
}

impl From<CommentSearchBuilder> for CommentSearch {
    fn from(builder: CommentSearchBuilder) -> Self {
        builder.search
    }
}
