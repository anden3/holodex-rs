use crate::{
    errors::Error,
    model::{
        id::{ChannelId, VideoId},
        Channel, ChannelFilter, ChannelVideoFilter, ChannelVideoType, CommentSearch, Language,
        PaginatedResult, Video, VideoFilter, VideoFull, VideoSearch,
    },
    util::validate_response,
};

#[cfg(feature = "streams")]
use futures_core::Stream;

#[derive(Debug, Clone)]
/// The client used for interacting with the Holodex API.
pub struct Client {
    http: ureq::Agent,
    token: String,
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
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::InvalidApiToken`] if `api_token` contains invalid characters.
    ///
    /// Will return [`Error::HttpClientCreationError`] if a TLS backend cannot be initialized, or the resolver cannot load the system configuration.
    pub fn new(api_token: &str) -> Result<Self, Error> {
        let http = ureq::builder().user_agent(Self::USER_AGENT).build();

        Ok(Self {
            http,
            token: api_token.to_owned(),
        })
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
    /// use holodex::model::{
    ///     builders::VideoFilterBuilder, ExtraVideoInfo, Language, Organisation,
    ///     VideoSortingCriteria, VideoType
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
    ///     .sort_by(VideoSortingCriteria::StartScheduled)
    ///     .limit(5)
    ///     .build();
    ///
    /// let results = client.videos(&filter)?;
    ///
    /// for stream in results {
    ///     println!("{}", stream.title);
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn videos(&self, parameters: &VideoFilter) -> Result<PaginatedResult<Video>, Error> {
        Self::query_videos(&self.http, &self.token, "/videos", parameters)
    }

    #[cfg(feature = "streams")]
    /// Returns a stream of all videos matching the `filter`.
    ///
    /// # Examples
    ///
    /// Get all streams that are currently live.
    /// ```rust
    /// # fn main() -> Result<(), holodex::errors::Error> {
    /// # tokio_test::block_on(async {
    /// use holodex::model::{builders::VideoFilterBuilder, VideoStatus, VideoType};
    /// use futures::{self, pin_mut, StreamExt, TryStreamExt};
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let filter = VideoFilterBuilder::new()
    ///     .video_type(VideoType::Stream)
    ///     .status(&[VideoStatus::Live])
    ///     .build();
    ///
    /// let stream = client.video_stream(&filter);
    /// pin_mut!(stream);
    ///
    /// while let Some(video) = stream.try_next().await? {
    ///     println!("{}", video.title);
    /// }
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub fn video_stream<'a>(
        &'a self,
        parameters: &'a VideoFilter,
    ) -> impl Stream<Item = Result<Video, Error>> + 'a {
        Self::stream_endpoint(&self.http, &self.token, "/videos", parameters)
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
    /// | Sort by    | [`AvailableAt`][`crate::model::VideoSortingCriteria::AvailableAt`]     |
    /// | Order      | [`Ascending`][`crate::model::Order::Ascending`]     |
    /// | Max upcoming hours | 48 |
    /// | Limit      | 9999    |
    /// | Include    | [[`LiveInfo`][`crate::model::ExtraVideoInfo::LiveInfo`]] |
    ///
    /// # Examples
    ///
    /// Find live or upcoming streams from Hololive talents:
    /// ```rust
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
    /// let currently_live = client.live(&parameters)?;
    ///
    /// for video in currently_live.items() {
    ///     println!("{}", video.title);
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn live(&self, parameters: &VideoFilter) -> Result<PaginatedResult<Video>, Error> {
        Self::query_videos(&self.http, &self.token, "/live", parameters)
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
    /// use holodex::model::{Language, ChannelVideoType, ChannelVideoFilter};
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let parameters = ChannelVideoFilter {
    ///     languages: vec![Language::English],
    ///     ..Default::default()
    /// };
    /// let pekora_ch_id = "UC1DCedRgGHBdm81E1llLhOQ".parse()?;
    /// let english_clips = client.videos_from_channel(&pekora_ch_id, ChannelVideoType::Clips, &parameters)?;
    ///
    /// for clip in english_clips.items() {
    ///     println!("{}", clip.title);
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn videos_from_channel(
        &self,
        channel_id: &ChannelId,
        video_type: ChannelVideoType,
        parameters: &ChannelVideoFilter,
    ) -> Result<PaginatedResult<Video>, Error> {
        let query_string = serde_urlencoded::to_string(parameters)
            .map_err(|e| Error::FilterCreationError(e.to_string()))?;
        let query_pairs: Vec<(&str, String)> = serde_urlencoded::from_str(&query_string)
            .map_err(|e| Error::FilterCreationError(e.to_string()))?;

        let mut request = self
            .http
            .get(&format!(
                "{}/channels/{}/{}",
                Self::ENDPOINT,
                channel_id,
                video_type
            ))
            .set("x-apikey", &self.token);

        for (key, value) in query_pairs {
            request = request.query(key, &value);
        }
        let res = request.call().map_err(|e| Error::ApiRequestFailed {
            endpoint: "/channels/{channel_id}/{type}",
            source: e,
        })?;

        let videos = validate_response(res).map_err(|e| Error::InvalidResponse {
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
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let channels = vec!["UCoSrY_IQQVpmIRZ9Xf-y93g".parse()?, "UCyl1z3jo3XHR1riLFKG5UAg".parse()?];
    /// let streams = client.live_from_channels(&channels)?;
    ///
    /// if !streams.is_empty() {
    ///     println!("At least one of the channels is live!");
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn live_from_channels(
        &self,
        channel_ids: &[ChannelId],
    ) -> Result<PaginatedResult<Video>, Error> {
        let res = self
            .http
            .get(&format!("{}/users/live", Self::ENDPOINT))
            .set("x-apikey", &self.token)
            .query(
                "channels",
                &channel_ids
                    .iter()
                    .map(|c| &*c.0)
                    .collect::<Vec<&str>>()
                    .join(","),
            )
            .call()
            .map_err(|e| Error::ApiRequestFailed {
                endpoint: "/users/live",
                source: e,
            })?;

        let videos = validate_response(res).map_err(|e| Error::InvalidResponse {
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
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let channel_id = "UCNVEsYbiZjH5QLmGeSgTSzg".parse()?;
    /// let channel = client.channel(&channel_id)?;
    ///
    /// if let Some(subs) = &channel.stats.subscriber_count {
    ///     println!("Astel has {} subscribers", subs);
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn channel(&self, channel_id: &ChannelId) -> Result<Channel, Error> {
        let res = self
            .http
            .get(&format!("{}/channels/{}", Self::ENDPOINT, channel_id))
            .set("x-apikey", &self.token)
            .call()
            .map_err(|e| Error::ApiRequestFailed {
                endpoint: "/channels/{channel_id}",
                source: e,
            })?;

        let channel = validate_response(res).map_err(|e| Error::InvalidResponse {
            endpoint: "/channels/{channel_id}",
            source: e,
        })?;

        Ok(channel)
    }

    /// Get all channels matching the given filter.
    ///
    /// # Examples
    ///
    /// Print the top 10 vtuber channels by number of subscribers.
    /// ```rust
    /// use holodex::model::{
    ///     builders::ChannelFilterBuilder, ChannelFilter, ChannelSortingCriteria,
    ///     Order, Organisation
    /// };
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let filter = ChannelFilterBuilder::new()
    ///     .sort_by(ChannelSortingCriteria::SubscriberCount)
    ///     .order(Order::Descending)
    ///     .limit(10)
    ///     .build()?;
    ///
    /// let channels = client.channels(&filter)?;
    ///
    /// for channel in channels {
    ///     println!(
    ///         "{} has {} subscribers!",
    ///         channel.name, channel.stats.subscriber_count.unwrap_or_default()
    ///     );
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn channels(&self, filter: &ChannelFilter) -> Result<Vec<Channel>, Error> {
        let query_string = serde_urlencoded::to_string(filter)
            .map_err(|e| Error::FilterCreationError(e.to_string()))?;
        let query_pairs: Vec<(&str, String)> = serde_urlencoded::from_str(&query_string)
            .map_err(|e| Error::FilterCreationError(e.to_string()))?;

        let mut request = self
            .http
            .get(&format!("{}/channels", Self::ENDPOINT))
            .set("x-apikey", &self.token);

        for (key, value) in query_pairs {
            request = request.query(key, &value);
        }

        let res = request.call().map_err(|e| Error::ApiRequestFailed {
            endpoint: "/channels",
            source: e,
        })?;

        let channels = validate_response(res).map_err(|e| Error::InvalidResponse {
            endpoint: "/channels",
            source: e,
        })?;

        Ok(channels)
    }

    /// Get a single video's metadata.
    ///
    /// # Examples
    ///
    /// Find songs from Coco's graduation stream :(
    /// ```rust
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let coco_graduation = "IhiievWaZMI".parse()?;
    /// let metadata = client.video(&coco_graduation)?;
    ///
    /// for song in &metadata.songs {
    ///     println!("{}", song);
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn video(&self, video_id: &VideoId) -> Result<VideoFull, Error> {
        self.get_video::<()>(video_id, None)
    }

    /// Get a single video's metadata, along with any indexed comments containing timestamps.
    ///
    /// # Examples
    ///
    /// Find all timestamps for Ollie's birthday stream (in 2021).
    /// ```rust
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let ollie_birthday = "v6o7LBrQs-I".parse()?;
    /// let metadata = client.video_with_timestamps(&ollie_birthday)?;
    ///
    /// for comment in &metadata.comments {
    ///     println!("{}", comment);
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn video_with_timestamps(&self, video_id: &VideoId) -> Result<VideoFull, Error> {
        self.get_video(video_id, Some(&[("c", "1")]))
    }

    /// Get a single video's metadata, along with any recommended videos in languages matching the given filter.
    ///
    /// # Examples
    ///
    /// Get English videos related to Korone's birthday stream (2021).
    /// ```rust
    /// use holodex::model::Language;
    ///
    /// # if std::env::var_os("HOLODEX_API_TOKEN").is_none() {
    /// #   std::env::set_var("HOLODEX_API_TOKEN", "my-api-token");
    /// # }
    /// let token = std::env::var("HOLODEX_API_TOKEN").unwrap();
    /// let client = holodex::Client::new(&token)?;
    ///
    /// let korone_birthday = "2l3i7MulCgs-I".parse()?;
    /// let metadata = client.video_with_related(&korone_birthday, &[Language::English])?;
    ///
    /// for related in &metadata.related {
    ///     println!("{}", related.title);
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn video_with_related(
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
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        )
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
    ///     .channels(&["UCvaTdHTWBGv3MKj3KVqJVCw".parse()?, "UChAnqc_AY5_I3Px5dig3X1Q".parse()?])
    ///     .types(&[VideoType::Stream])
    ///     .limit(5)
    ///     .build();
    ///
    /// let results = client.search_videos(&search)?;
    ///
    /// for result in results {
    ///     println!("{}", result.title);
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn search_videos(
        &self,
        search_parameters: &VideoSearch,
    ) -> Result<PaginatedResult<Video>, Error> {
        let res = self
            .http
            .post(&format!("{}/search/videoSearch", Self::ENDPOINT))
            .set("x-apikey", &self.token)
            .send_json(
                ureq::serde_to_value(search_parameters)
                    .map_err(|e| Error::FilterCreationError(e.to_string()))?,
            )
            .map_err(|e| Error::ApiRequestFailed {
                endpoint: "/search/videoSearch",
                source: e,
            })?;

        let videos = validate_response(res).map_err(|e| Error::InvalidResponse {
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
    /// let videos_with_comments = client.search_comments(&search)?;
    ///
    /// for comment in videos_with_comments.into_iter().flat_map(|v| v.comments) {
    ///     println!("{}", comment);
    /// }
    /// # Ok::<(), holodex::errors::Error>(())
    /// ```
    ///
    /// # Errors
    /// Will return [`Error::ApiRequestFailed`] if sending the API request fails.
    ///
    /// Will return [`Error::InvalidResponse`] if the API returned a faulty response or server error.
    pub fn search_comments(
        &self,
        search_parameters: &CommentSearch,
    ) -> Result<PaginatedResult<VideoFull>, Error> {
        let res = self
            .http
            .post(&format!("{}/search/commentSearch", Self::ENDPOINT))
            .set("x-apikey", &self.token)
            .send_json(
                ureq::serde_to_value(search_parameters)
                    .map_err(|e| Error::FilterCreationError(e.to_string()))?,
            )
            .map_err(|e| Error::ApiRequestFailed {
                endpoint: "/search/commentSearch",
                source: e,
            })?;

        let videos_with_comments = validate_response(res).map_err(|e| Error::InvalidResponse {
            endpoint: "/search/commentSearch",
            source: e,
        })?;

        Ok(videos_with_comments)
    }

    fn get_video<T>(&self, video_id: &VideoId, query: Option<&T>) -> Result<VideoFull, Error>
    where
        T: serde::Serialize + Sync + Send + ?Sized + std::fmt::Debug,
    {
        let query_string = serde_urlencoded::to_string(query)
            .map_err(|e| Error::FilterCreationError(e.to_string()))?;
        let query_pairs: Vec<(&str, String)> = serde_urlencoded::from_str(&query_string)
            .map_err(|e| Error::FilterCreationError(e.to_string()))?;

        let mut request = self
            .http
            .get(&format!("{}/videos/{}", Self::ENDPOINT, video_id))
            .set("x-apikey", &self.token);

        for (key, value) in query_pairs {
            request = request.query(key, &value);
        }

        let res = request.call().map_err(|e| Error::ApiRequestFailed {
            endpoint: "/videos/{video_id}",
            source: e,
        })?;

        let video = validate_response(res).map_err(|e| Error::InvalidResponse {
            endpoint: "/videos/{video_id}",
            source: e,
        })?;

        Ok(video)
    }

    fn query_videos(
        http: &ureq::Agent,
        token: &str,
        endpoint: &'static str,
        parameters: &VideoFilter,
    ) -> Result<PaginatedResult<Video>, Error> {
        let query_string = serde_urlencoded::to_string(parameters)
            .map_err(|e| Error::FilterCreationError(e.to_string()))?;
        let query_pairs: Vec<(&str, String)> = serde_urlencoded::from_str(&query_string)
            .map_err(|e| Error::FilterCreationError(e.to_string()))?;

        let mut request = http
            .get(&format!("{}{}", Self::ENDPOINT, endpoint))
            .set("x-apikey", token);

        for (key, value) in query_pairs {
            request = request.query(key, &value);
        }

        let res = request.call().map_err(|e| Error::ApiRequestFailed {
            endpoint,
            source: e,
        })?;

        let videos = validate_response(res).map_err(|e| Error::InvalidResponse {
            endpoint,
            source: e,
        })?;

        Ok(videos)
    }

    #[cfg(feature = "streams")]
    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    fn stream_endpoint<'a>(
        http: &'a ureq::Agent,
        token: &'a str,
        endpoint: &'static str,
        parameters: &'a VideoFilter,
    ) -> impl Stream<Item = Result<Video, Error>> + 'a {
        {
            let (mut async_sender, async_receiver) = async_stream::yielder::pair();

            async_stream::AsyncStream::new(async_receiver, async move {
                const CHUNK_SIZE: u32 = 50;
                let mut filter = VideoFilter {
                    paginated: true,
                    limit: CHUNK_SIZE,
                    offset: 0,
                    ..parameters.clone()
                };
                let mut counter = 0_u32;

                while let PaginatedResult::Page { total, items } =
                    match Self::query_videos(http, token, endpoint, &filter) {
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
}
