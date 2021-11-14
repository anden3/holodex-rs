//! Various types wrapping different IDs used in the API.
#![allow(clippy::module_name_repetitions)]

use std::{convert::TryFrom, fmt::Display, ops::Deref, str::FromStr};

use regex::Regex;
use serde::{self, Deserialize, Serialize};

use crate::{
    errors::Error,
    model::{
        Channel, ChannelVideoFilter, ChannelVideoType, Language, PaginatedResult, Video, VideoFull,
    },
    Client,
};

#[cfg(feature = "streams")]
use futures_core::Stream;

#[cfg(not(feature = "sso"))]
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
/// The ID of a video.
pub struct VideoId(pub(crate) String);

#[cfg(feature = "sso")]
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
/// The ID of a video.
pub struct VideoId(pub(crate) smartstring::alias::String);

impl VideoId {
    /// Get all the metadata associated with this channel.
    ///
    /// # Examples
    ///
    /// Get all songs sung in the Lazu Light karaoke (2021-10-05).
    /// ```rust
    /// use holodex::model::{id::VideoId, Language};
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let video_id: VideoId = "https://www.youtube.com/watch?v=V2SBDtZ4khY".parse()?;
    /// let video = video_id.metadata(&client)?;
    ///
    /// for song in video.songs {
    ///     println!("{}", song);
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn metadata(&self, client: &Client) -> Result<VideoFull, Error> {
        client.video(self)
    }

    /// Get all indexed comments containing timestamps for this video.
    ///
    /// # Examples
    ///
    /// Print all timestamped comments from Elira's birthday stream (2021).
    /// ```rust
    /// use holodex::model::id::VideoId;
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let video: VideoId = "https://www.youtube.com/watch?v=tDXvkK_MLl0".parse()?;
    /// let timestamps = video.timestamps(&client)?;
    ///
    /// for timestamp in timestamps {
    ///     println!("{}", timestamp);
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn timestamps(&self, client: &Client) -> Result<impl Iterator<Item = String> + '_, Error> {
        let metadata = client.video_with_timestamps(self)?;

        Ok(metadata.comments.into_iter().map(|c| c.message))
    }

    /// Get all videos related to this video that are in the given languages.
    ///
    /// # Examples
    ///
    /// Get Japanese clips related to Calli's birthday stream (2021).
    /// ```rust
    /// use holodex::model::{id::VideoId, Language};
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let video: VideoId = "https://www.youtube.com/watch?v=NiziRRHFZGA".parse()?;
    /// let clips = video.related(&client, &[Language::Japanese])?;
    ///
    /// for clip in clips {
    ///     println!("{}", clip.title);
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn related(
        &self,
        client: &Client,
        languages: &[Language],
    ) -> Result<impl Iterator<Item = Video> + '_, Error> {
        let metadata = client.video_with_related(self, languages)?;

        Ok(metadata.related.into_iter())
    }
}

impl Display for VideoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for VideoId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<String> for VideoId {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl FromStr for VideoId {
    type Err = Error;

    #[allow(clippy::unwrap_in_result)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[allow(clippy::expect_used)]
        let regex =
            Regex::new(r"[0-9A-Za-z_-]{10}[048AEIMQUYcgkosw]").expect("Video ID regex broke.");

        Ok(Self(
            regex
                .find(s)
                .ok_or_else(|| Error::InvalidVideoId(s.to_owned()))?
                .as_str()
                .into(),
        ))
    }
}

#[cfg(not(feature = "sso"))]
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
/// The ID of a channel.
pub struct ChannelId(pub(crate) String);

#[cfg(feature = "sso")]
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
/// The ID of a channel.
pub struct ChannelId(pub(crate) smartstring::alias::String);

impl ChannelId {
    /// Get all the metadata associated with this channel.
    ///
    /// # Examples
    ///
    /// Show the top topics associated with Aruran's channel.
    /// ```rust
    /// use holodex::model::id::ChannelId;
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let channel_id: ChannelId = "UCKeAhJvy8zgXWbh9duVjIaQ".parse()?;
    /// let channel = channel_id.metadata(&client)?;
    ///
    /// for topic in channel.top_topics {
    ///     println!("{}", topic);
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn metadata(&self, client: &Client) -> Result<Channel, Error> {
        client.channel(self)
    }

    /// Get videos that this channel has uploaded.
    ///
    /// # Examples
    ///
    /// Print some videos uploaded by Kiara.
    /// ```rust
    /// use holodex::model::id::ChannelId;
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let channel_id: ChannelId = "UCHsx4Hqa-1ORjQTh9TYDhww".parse()?;
    /// let videos = channel_id.videos(&client)?;
    ///
    /// for video in videos {
    ///     println!("{}", video.title);
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn videos(&self, client: &Client) -> Result<PaginatedResult<Video>, Error> {
        client.videos_from_channel(
            self,
            ChannelVideoType::Videos,
            &ChannelVideoFilter {
                paginated: false,
                ..ChannelVideoFilter::default()
            },
        )
    }

    #[cfg(feature = "streams")]
    /// Returns a stream of all videos that this channel has uploaded.
    ///
    /// /// Print the latest 200 videos uploaded by Kiara.
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// use holodex::model::id::ChannelId;
    /// use futures::{self, pin_mut, StreamExt, TryStreamExt};
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let channel_id: ChannelId = "UCHsx4Hqa-1ORjQTh9TYDhww".parse()?;
    ///
    /// let stream = channel_id.video_stream(&client).take(200);
    /// pin_mut!(stream);
    ///
    /// while let Some(video) = stream.try_next().await? {
    ///     println!("{}", video.title);
    /// }
    /// # Ok(())
    /// # })
    /// # }
    pub fn video_stream(self, client: &Client) -> impl Stream<Item = Result<Video, Error>> + '_ {
        Self::stream_channel_video_type(client, self, ChannelVideoType::Videos)
    }

    /// Get clips related to this channel.
    ///
    /// # Examples
    ///
    /// Show some clips related to Uto.
    /// ```rust
    /// use holodex::model::id::ChannelId;
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let channel_id: ChannelId = "UCdYR5Oyz8Q4g0ZmB4PkTD7g".parse()?;
    /// let clips = channel_id.clips(&client)?;
    ///
    /// for clip in clips {
    ///     println!("{}", clip.title);
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn clips(&self, client: &Client) -> Result<PaginatedResult<Video>, Error> {
        client.videos_from_channel(
            self,
            ChannelVideoType::Clips,
            &ChannelVideoFilter {
                paginated: false,
                ..ChannelVideoFilter::default()
            },
        )
    }

    #[cfg(feature = "streams")]
    /// Returns a stream of all videos that this channel has uploaded.
    ///
    /// /// Print the latest 200 clips made about Kiara.
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// use holodex::model::id::ChannelId;
    /// use futures::{self, pin_mut, StreamExt, TryStreamExt};
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let channel_id: ChannelId = "UCHsx4Hqa-1ORjQTh9TYDhww".parse()?;
    ///
    /// let stream = channel_id.clip_stream(&client).take(200);
    /// pin_mut!(stream);
    ///
    /// while let Some(clip) = stream.try_next().await? {
    ///     println!("{}", clip.title);
    /// }
    /// # Ok(())
    /// # })
    /// # }
    pub fn clip_stream(self, client: &Client) -> impl Stream<Item = Result<Video, Error>> + '_ {
        Self::stream_channel_video_type(client, self, ChannelVideoType::Clips)
    }

    /// Get collabs from other videos that mention this channel.
    ///
    /// # Examples
    ///
    /// Show some collabs with Korone.
    /// ```rust
    /// use holodex::model::id::ChannelId;
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let channel_id: ChannelId = "UChAnqc_AY5_I3Px5dig3X1Q".parse()?;
    /// let collabs = channel_id.collabs(&client)?;
    ///
    /// for collab in collabs {
    ///     println!("{}", collab.title);
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn collabs(&self, client: &Client) -> Result<PaginatedResult<Video>, Error> {
        client.videos_from_channel(
            self,
            ChannelVideoType::Clips,
            &ChannelVideoFilter {
                paginated: false,
                ..ChannelVideoFilter::default()
            },
        )
    }

    #[cfg(feature = "streams")]
    /// Returns a stream of all collabs from other videos that have mentioned this channel.
    ///
    /// /// Print the latest 50 collabs with Subaru.
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// use holodex::model::id::ChannelId;
    /// use futures::{self, pin_mut, StreamExt, TryStreamExt};
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let channel_id: ChannelId = "UCvzGlP9oQwU--Y0r9id_jnA".parse()?;
    ///
    /// let stream = channel_id.collab_stream(&client).take(50);
    /// pin_mut!(stream);
    ///
    /// while let Some(collab) = stream.try_next().await? {
    ///     println!("{}", collab.title);
    /// }
    /// # Ok(())
    /// # })
    /// # }
    pub fn collab_stream(self, client: &Client) -> impl Stream<Item = Result<Video, Error>> + '_ {
        Self::stream_channel_video_type(client, self, ChannelVideoType::Collabs)
    }

    #[cfg(feature = "streams")]
    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    fn stream_channel_video_type(
        client: &Client,
        channel_id: ChannelId,
        video_type: ChannelVideoType,
    ) -> impl Stream<Item = Result<Video, Error>> + '_ {
        let (mut async_sender, async_receiver) = async_stream::yielder::pair();

        async_stream::AsyncStream::new(async_receiver, async move {
            const CHUNK_SIZE: u32 = 50;

            let mut filter = ChannelVideoFilter {
                paginated: true,
                limit: CHUNK_SIZE,
                ..ChannelVideoFilter::default()
            };
            let mut counter = 0_u32;

            while let PaginatedResult::Page { total, items } =
                match client.videos_from_channel(&channel_id, video_type, &filter) {
                    Ok(v) => v,
                    Err(e) => {
                        async_sender.send(Err(e)).await;
                        return;
                    }
                }
            {
                counter += items.len() as u32;
                let total: u32 = total.into();

                for video in items {
                    async_sender.send(Ok(video)).await;
                }

                if counter >= total {
                    break;
                }

                filter.offset += CHUNK_SIZE as i32;
            }
        })
    }
}

impl Display for ChannelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for ChannelId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<String> for ChannelId {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl FromStr for ChannelId {
    type Err = Error;

    #[allow(clippy::unwrap_in_result)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[allow(clippy::expect_used)]
        let regex = Regex::new(r"UC[0-9a-zA-Z_-]{21}[AQgw]").expect("Channel ID regex broke.");

        Ok(Self(
            regex
                .find(s)
                .ok_or_else(|| Error::InvalidChannelId(s.to_owned()))?
                .as_str()
                .into(),
        ))
    }
}
