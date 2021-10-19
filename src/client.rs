use itertools::Itertools;
use reqwest::header;

use crate::{
    errors::Error,
    model::{
        id::{ChannelId, VideoId},
        Channel, ChannelVideoFilter, ChannelVideoType, CommentSearch, Language, PaginatedResult,
        Video, VideoFilter, VideoFull, VideoSearch,
    },
    util::validate_response,
};

#[derive(Debug, Clone)]
/// The client used for interacting with the Holodex API.
pub struct Client {
    http: reqwest::Client,
}

impl Client {
    const ENDPOINT: &'static str = "https://holodex.net/api/v2";
    const USER_AGENT: &'static str =
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    #[must_use = "Unused Holodex client."]
    /// Create a new client with the provided API token.
    ///
    /// # Examples
    /// Create a client that gets the API token from an environment variable:
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::InvalidApiToken`] if `api_token` contains invalid characters.
    ///
    /// Will return [`Error::HttpClientCreationError`] if a TLS backend cannot be initialized, or the resolver cannot load the system configuration.
    pub fn new(api_token: &str) -> Result<Self, Error> {
        let mut headers = header::HeaderMap::new();

        let mut auth_value =
            header::HeaderValue::from_str(api_token).map_err(|_e| Error::InvalidApiToken)?;

        auth_value.set_sensitive(true);
        headers.insert(header::HeaderName::from_static("x-apikey"), auth_value);

        let http = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .user_agent(Self::USER_AGENT)
            .build()
            .map_err(Error::HttpClientCreationError)?;

        Ok(Self { http })
    }

    /// Query videos.
    ///
    /// Pretty much everything you need.
    /// This is the most 'vanilla' variant with almost no preset values,
    /// and [`videos_from_channel`][`Self::videos_from_channel`] and [`live`][`Self::live`] both use the same query structure
    /// but provision default values differently for some of the query params.
    ///
    /// Not as powerful at searching arbitrary text as the Search API (currently not documented/available).
    ///
    /// # Examples
    ///
    /// Retrieve the five closest Japanese streams from independent streamers
    /// scheduled to go live within the next 24 hours, along with their descriptions.
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// use holodex::model::{
    ///     builders::VideoFilterBuilder, ExtraVideoInfo, Language, Organisation,
    ///     SortingCriteria, VideoType
    /// };
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let filter = VideoFilterBuilder::new()
    ///     .organisation(Organisation::Independents)
    ///     .language(&[Language::Japanese])
    ///     .video_type(VideoType::Stream)
    ///     .max_upcoming_hours(24)
    ///     .include(&[ExtraVideoInfo::Description])
    ///     .sort_by(SortingCriteria::StartScheduled)
    ///     .limit(5)
    ///     .build();
    ///
    /// let results = client.videos(&filter).await?;
    ///
    /// for stream in results {
    ///     println!("{}", stream.title);
    /// }
    ///
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub async fn videos(&self, parameters: &VideoFilter) -> Result<PaginatedResult<Video>, Error> {
        self.query_videos("/videos", parameters).await
    }

    /// Query live and upcoming videos.
    ///
    /// This is somewhat similar to calling [`videos`][`Self::videos`].
    ///
    /// However, this endpoint imposes these default values on the query parameters:
    /// You can choose to override them by providing your own values.
    ///
    /// | Parameter  | Default |
    /// |------------|---------|
    /// | Status     | [[`Live`][`crate::model::VideoStatus::Live`], [`Upcoming`][`crate::model::VideoStatus::Upcoming`]] |
    /// | Video type | [`Stream`][`crate::model::VideoType::Stream`]            |
    /// | Sort by    | [`AvailableAt`][`crate::model::SortingCriteria::AvailableAt`]     |
    /// | Order      | [`Ascending`][`crate::model::VideoOrder::Ascending`]     |
    /// | Max upcoming hours | 48 |
    /// | Limit      | 9999    |
    /// | Include    | [[`LiveInfo`][`crate::model::ExtraVideoInfo::LiveInfo`]] |
    ///
    /// # Examples
    ///
    /// Find live or upcoming streams from Hololive talents:
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// use holodex::model::{Organisation, VideoFilter};
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    /// let parameters = VideoFilter {
    ///     org: Some(Organisation::Hololive),
    ///     ..Default::default()
    /// };
    /// let currently_live = client.live(&parameters).await?;
    ///
    /// for video in currently_live.items() {
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
    pub async fn live(&self, parameters: &VideoFilter) -> Result<PaginatedResult<Video>, Error> {
        self.query_videos("/live", parameters).await
    }

    /// Query videos related to channel.
    ///
    /// A simplified endpoint for access channel specific data.
    /// If you want more customization, the same result can be obtained by
    /// calling [`videos`][`Self::videos`].
    ///
    /// # Examples
    ///
    /// Find some English clips of Pekora:
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// use holodex::model::{Language, ChannelVideoType, ChannelVideoFilter};
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let parameters = ChannelVideoFilter {
    ///     lang: vec![Language::English],
    ///     ..Default::default()
    /// };
    /// let pekora_ch_id = "UC1DCedRgGHBdm81E1llLhOQ".into();
    /// let english_clips = client.videos_from_channel(&pekora_ch_id, ChannelVideoType::Clips, &parameters)
    ///     .await?;
    ///
    /// for clip in english_clips.items() {
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
    pub async fn videos_from_channel(
        &self,
        channel_id: &ChannelId,
        video_type: ChannelVideoType,
        parameters: &ChannelVideoFilter,
    ) -> Result<PaginatedResult<Video>, Error> {
        let res = self
            .http
            .get(format!(
                "{}/channels/{}/{}",
                Self::ENDPOINT,
                channel_id,
                video_type
            ))
            .query(&parameters)
            .send()
            .await
            .map_err(|e| Error::ApiRequestFailed {
                endpoint: "/channels/{channel_id}/{type}",
                source: e,
            })?;

        let videos = validate_response(res)
            .await
            .map_err(|e| Error::InvalidResponse {
                endpoint: "/channels/{channel_id}/{type}",
                source: e,
            })?;

        Ok(videos)
    }

    /// Quickly access live/upcoming for a set of channels.
    ///
    /// This method is similar to [`live`](#method.live) and usually replies much faster.
    /// It is more friendly in general. The cost to execute a lookup is significantly cheaper.
    /// It's unfortunately less customizable as a result.
    ///
    /// We recommend using this if you have a fixed set of channel IDs to look up status for.
    ///
    /// # Examples
    ///
    /// Find if Amelia and/or Gura are live:
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let channels = vec!["UCoSrY_IQQVpmIRZ9Xf-y93g".into(), "UCyl1z3jo3XHR1riLFKG5UAg".into()];
    /// let streams = client.live_from_channels(&channels).await?;
    ///
    /// if !streams.is_empty() {
    ///     println!("At least one of the channels is live!");
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
    pub async fn live_from_channels(
        &self,
        channel_ids: &[ChannelId],
    ) -> Result<PaginatedResult<Video>, Error> {
        let res = self
            .http
            .get(format!("{}/users/live", Self::ENDPOINT))
            .query(&[("channels", channel_ids.iter().map(|c| &*c.0).join(","))])
            .send()
            .await
            .map_err(|e| Error::ApiRequestFailed {
                endpoint: "/users/live",
                source: e,
            })?;

        let videos = validate_response(res)
            .await
            .map_err(|e| Error::InvalidResponse {
                endpoint: "/users/live",
                source: e,
            })?;

        Ok(videos)
    }

    /// Get channel information.
    ///
    /// # Examples
    ///
    /// Find out how many subscribers Astel has.
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let channel_id = "UCNVEsYbiZjH5QLmGeSgTSzg".into();
    /// let channel = client.channel(&channel_id).await?;
    ///
    /// if let Some(subs) = &channel.stats.subscriber_count {
    ///     println!("Astel has {} subscribers", subs);
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
    pub async fn channel(&self, channel_id: &ChannelId) -> Result<Channel, Error> {
        let res = self
            .http
            .get(format!("{}/channels/{}", Self::ENDPOINT, channel_id))
            .send()
            .await
            .map_err(|e| Error::ApiRequestFailed {
                endpoint: "/channels/{channel_id}",
                source: e,
            })?;

        let channel = validate_response(res)
            .await
            .map_err(|e| Error::InvalidResponse {
                endpoint: "/channels/{channel_id}",
                source: e,
            })?;

        Ok(channel)
    }

    /// Get a single video's metadata.
    ///
    /// # Examples
    ///
    /// Find songs from Coco's graduation stream :(
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let coco_graduation = "IhiievWaZMI".into();
    /// let metadata = client.video(&coco_graduation).await?;
    ///
    /// for song in &metadata.songs {
    ///     println!("{}", song);
    /// }
    ///
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub async fn video(&self, video_id: &VideoId) -> Result<VideoFull, Error> {
        self.get_video::<()>(video_id, None).await
    }

    /// Get a single video's metadata, along with any indexed comments containing timestamps.
    ///
    /// # Examples
    ///
    /// Find all timestamps for Ollie's birthday stream (in 2021).
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let ollie_birthday = "v6o7LBrQs-I".into();
    /// let metadata = client.video_with_timestamps(&ollie_birthday).await?;
    ///
    /// for comment in &metadata.comments {
    ///     println!("{}", comment);
    /// }
    ///
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub async fn video_with_timestamps(&self, video_id: &VideoId) -> Result<VideoFull, Error> {
        self.get_video(video_id, Some(&[("c", "1")])).await
    }

    /// Get a single video's metadata, along with any recommended videos in languages matching the given filter.
    ///
    /// # Examples
    ///
    /// Get English videos related to Korone's birthday stream (2021).
    /// ```no_run
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// use holodex::model::Language;
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let korone_birthday = "2l3i7MulCgs-I".into();
    /// let metadata = client.video_with_related(&korone_birthday, &[Language::English]).await?;
    ///
    /// for related in &metadata.related {
    ///     println!("{}", related.title);
    /// }
    ///
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub async fn video_with_related(
        &self,
        video_id: &VideoId,
        related_language_filter: &[Language],
    ) -> Result<VideoFull, Error> {
        self.get_video(
            video_id,
            Some(&[(
                "lang",
                related_language_filter
                    .iter()
                    .map(ToString::to_string)
                    .join(","),
            )]),
        )
        .await
    }

    /// Search for videos matching the given search conditions.
    ///
    /// Searching for `topics` and `clips` together is not supported,
    /// because clips do not contain `topics`.
    ///
    /// # Examples
    ///
    /// Find the five latest Okayu/Korone collab streams.
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// use holodex::model::{builders::VideoSearchBuilder, SearchOrder, VideoType};
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let search = VideoSearchBuilder::new()
    ///     .order(SearchOrder::Newest)
    ///     .channels(&["UCvaTdHTWBGv3MKj3KVqJVCw".into(), "UChAnqc_AY5_I3Px5dig3X1Q".into()])
    ///     .types(&[VideoType::Stream])
    ///     .limit(5)
    ///     .build();
    ///
    /// let results = client.search_videos(&search).await?;
    ///
    /// for result in results {
    ///     println!("{}", result.title);
    /// }
    ///
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub async fn search_videos(
        &self,
        search_parameters: &VideoSearch,
    ) -> Result<PaginatedResult<Video>, Error> {
        let res = self
            .http
            .post(format!("{}/search/videoSearch", Self::ENDPOINT))
            .json(search_parameters)
            .send()
            .await
            .map_err(|e| Error::ApiRequestFailed {
                endpoint: "/search/videoSearch",
                source: e,
            })?;

        let videos = validate_response(res)
            .await
            .map_err(|e| Error::InvalidResponse {
                endpoint: "/search/videoSearch",
                source: e,
            })?;

        Ok(videos)
    }

    /// Search for comments matching the given search conditions.
    ///
    /// # Examples
    ///
    /// Find the 50 oldest comments containing the word `peko` on streams from Nijisanji.
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// use holodex::model::{builders::CommentSearchBuilder, Organisation, SearchOrder, VideoType};
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let search = CommentSearchBuilder::new("peko")
    ///     .order(SearchOrder::Oldest)
    ///     .organisations(&[Organisation::Nijisanji])
    ///     .types(&[VideoType::Stream])
    ///     .limit(50)
    ///     .build();
    ///
    /// let videos_with_comments = client.search_comments(&search).await?;
    ///
    /// for comment in videos_with_comments.into_iter().flat_map(|v| v.comments) {
    ///     println!("{}", comment);
    /// }
    ///
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub async fn search_comments(
        &self,
        search_parameters: &CommentSearch,
    ) -> Result<PaginatedResult<VideoFull>, Error> {
        let res = self
            .http
            .post(format!("{}/search/commentSearch", Self::ENDPOINT))
            .json(search_parameters)
            .send()
            .await
            .map_err(|e| Error::ApiRequestFailed {
                endpoint: "/search/commentSearch",
                source: e,
            })?;

        let videos_with_comments =
            validate_response(res)
                .await
                .map_err(|e| Error::InvalidResponse {
                    endpoint: "/search/commentSearch",
                    source: e,
                })?;

        Ok(videos_with_comments)
    }

    async fn get_video<T>(&self, video_id: &VideoId, query: Option<&T>) -> Result<VideoFull, Error>
    where
        T: serde::Serialize + Sync + Send + ?Sized + std::fmt::Debug,
    {
        let mut request = self
            .http
            .get(format!("{}/videos/{}", Self::ENDPOINT, video_id));

        if let Some(query) = query {
            request = request.query(query);
        }

        let res = request.send().await.map_err(|e| Error::ApiRequestFailed {
            endpoint: "/videos/{video_id}",
            source: e,
        })?;

        let video = validate_response(res)
            .await
            .map_err(|e| Error::InvalidResponse {
                endpoint: "/videos/{video_id}",
                source: e,
            })?;

        Ok(video)
    }

    #[fix_hidden_lifetime_bug]
    async fn query_videos(
        &self,
        endpoint: &'static str,
        parameters: &VideoFilter,
    ) -> Result<PaginatedResult<Video>, Error> {
        let res = self
            .http
            .get(format!("{}{}", Self::ENDPOINT, endpoint))
            .query(&parameters)
            .send()
            .await
            .map_err(|e| Error::ApiRequestFailed {
                endpoint,
                source: e,
            })?;

        let videos = validate_response(res)
            .await
            .map_err(|e| Error::InvalidResponse {
                endpoint,
                source: e,
            })?;

        Ok(videos)
    }
}
