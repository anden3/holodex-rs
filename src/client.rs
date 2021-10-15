use reqwest::header;

#[derive(Debug)]
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

}
