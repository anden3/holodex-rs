//! Various types wrapping different IDs used in the API.
#![allow(clippy::module_name_repetitions)]

use std::{fmt::Display, ops::Deref, str::FromStr};

use regex::Regex;
use serde::{self, Deserialize, Serialize};

use crate::{
    errors::Error,
    model::{
        Channel, ChannelVideoFilter, ChannelVideoType, Language, PaginatedResult, Video, VideoFull,
    },
    Client,
};

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
/// The ID of a video.
pub struct VideoId(pub(crate) String);

impl VideoId {
    /// Get all the metadata associated with this channel.
    ///
    /// # Examples
    ///
    /// Get all songs sung in the Lazu Light karaoke (2021-10-05).
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// use holodex::model::{id::VideoId, Language};
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let video_id: VideoId = "https://www.youtube.com/watch?v=V2SBDtZ4khY".parse()?;
    /// let video = video_id.metadata(&client).await?;
    ///
    /// for song in video.songs {
    ///     println!("{}", song);
    /// }
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub async fn metadata(&self, client: &Client) -> Result<VideoFull, Error> {
        client.video(self).await
    }

    /// Get all indexed comments containing timestamps for this video.
    ///
    /// # Examples
    ///
    /// Print all timestamped comments from Elira's birthday stream (2021).
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// use holodex::model::id::VideoId;
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let video: VideoId = "https://www.youtube.com/watch?v=tDXvkK_MLl0".parse()?;
    /// let timestamps = video.timestamps(&client).await?;
    ///
    /// for timestamp in timestamps {
    ///     println!("{}", timestamp);
    /// }
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub async fn timestamps(
        &self,
        client: &Client,
    ) -> Result<impl Iterator<Item = String> + '_, Error> {
        let metadata = client.video_with_timestamps(self).await?;

        Ok(metadata.comments.into_iter().map(|c| c.message))
    }

    /// Get all videos related to this video that are in the given languages.
    ///
    /// # Examples
    ///
    /// Get Japanese clips related to Calli's birthday stream (2021).
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// use holodex::model::{id::VideoId, Language};
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let video: VideoId = "https://www.youtube.com/watch?v=NiziRRHFZGA".parse()?;
    /// let clips = video.related(&client, &[Language::Japanese]).await?;
    ///
    /// for clip in clips {
    ///     println!("{}", clip.title);
    /// }
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub async fn related(
        &self,
        client: &Client,
        languages: &[Language],
    ) -> Result<impl Iterator<Item = Video> + '_, Error> {
        let metadata = client.video_with_related(self, languages).await?;

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

impl From<&str> for VideoId {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for VideoId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl FromStr for VideoId {
    type Err = Error;

    #[allow(clippy::unwrap_in_result)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[allow(clippy::expect_used)]
        let regex =
            Regex::new(r"[0-9A-Za-z_-]{10}[048AEIMQUYcgkosw]").expect("Video ID regex broke.");

        Ok(regex
            .find(s)
            .ok_or_else(|| Error::InvalidVideoId(s.to_owned()))?
            .as_str()
            .into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
/// The ID of a channel.
pub struct ChannelId(pub(crate) String);

impl ChannelId {
    /// Get all the metadata associated with this channel.
    ///
    /// # Examples
    ///
    /// Show the top topics associated with Aruran's channel.
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// use holodex::model::id::ChannelId;
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let channel_id: ChannelId = "UCKeAhJvy8zgXWbh9duVjIaQ".parse()?;
    /// let channel = channel_id.metadata(&client).await?;
    ///
    /// for topic in channel.top_topics {
    ///     println!("{}", topic);
    /// }
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub async fn metadata(&self, client: &Client) -> Result<Channel, Error> {
        client.channel(self).await
    }

    /// Get videos that this channel has uploaded.
    ///
    /// # Examples
    ///
    /// Print some videos uploaded by Kiara.
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// use holodex::model::id::ChannelId;
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let channel_id: ChannelId = "UCHsx4Hqa-1ORjQTh9TYDhww".parse()?;
    /// let videos = channel_id.videos(&client).await?;
    ///
    /// for video in videos {
    ///     println!("{}", video.title);
    /// }
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub async fn videos(&self, client: &Client) -> Result<PaginatedResult<Video>, Error> {
        client
            .videos_from_channel(
                self,
                ChannelVideoType::Videos,
                &ChannelVideoFilter {
                    paginated: false,
                    ..ChannelVideoFilter::default()
                },
            )
            .await
    }

    /// Get clips related to this channel.
    ///
    /// # Examples
    ///
    /// Show some clips related to Uto.
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// use holodex::model::id::ChannelId;
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let channel_id: ChannelId = "UCdYR5Oyz8Q4g0ZmB4PkTD7g".parse()?;
    /// let clips = channel_id.clips(&client).await?;
    ///
    /// for clip in clips {
    ///     println!("{}", clip.title);
    /// }
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub async fn clips(&self, client: &Client) -> Result<PaginatedResult<Video>, Error> {
        client
            .videos_from_channel(
                self,
                ChannelVideoType::Clips,
                &ChannelVideoFilter {
                    paginated: false,
                    ..ChannelVideoFilter::default()
                },
            )
            .await
    }

    /// Get collabs from other videos that mention this channel.
    ///
    /// # Examples
    ///
    /// Show some collabs with Korone.
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// use holodex::model::id::ChannelId;
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let channel_id: ChannelId = "UChAnqc_AY5_I3Px5dig3X1Q".parse()?;
    /// let collabs = channel_id.collabs(&client).await?;
    ///
    /// for collab in collabs {
    ///     println!("{}", collab.title);
    /// }
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub async fn collabs(&self, client: &Client) -> Result<PaginatedResult<Video>, Error> {
        client
            .videos_from_channel(
                self,
                ChannelVideoType::Clips,
                &ChannelVideoFilter {
                    paginated: false,
                    ..ChannelVideoFilter::default()
                },
            )
            .await
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

impl From<&str> for ChannelId {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for ChannelId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl FromStr for ChannelId {
    type Err = Error;

    #[allow(clippy::unwrap_in_result)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[allow(clippy::expect_used)]
        let regex = Regex::new(r"UC[0-9a-zA-Z_-]{21}[AQgw]").expect("Channel ID regex broke.");

        Ok(regex
            .find(s)
            .ok_or_else(|| Error::InvalidChannelId(s.to_owned()))?
            .as_str()
            .into())
    }
}
