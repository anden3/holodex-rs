//! Types for various errors that can occur when interacting with the API.

use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    #[allow(missing_docs)]
    /// Errors that can occur when interacting with the Holodex API.
    pub enum Error {
        /// The API token provided to the client is invalid.
        InvalidApiToken {
            display("API token contains invalid characters.")
        }
        /// An error occurred while creating the HTTP client.
        HttpClientCreationError(err: ureq::Error) {
            display("Error creating HTTP client: {:?}", err)
            source(err)
        }
        /// An error occurred while sending a request to the API.
        ApiRequestFailed { source: ureq::Error, endpoint: &'static str } {
            display("Error sending request to {}: {:?}", endpoint, source)
            source(source)
        }
        /// The API returned a faulty response or server error.
        InvalidResponse { source: ValidationError, endpoint: &'static str } {
            display("Invalid response received from {}: {:?}", endpoint, source)
            source(source)
        }
        /// An invalid video ID was passed to the API.
        InvalidVideoId(id: String) {
            display("The provided video ID was not valid: {}", id)
        }
        /// An invalid channel ID was passed to the API.
        InvalidChannelId(id: String) {
            display("The provided channel ID was not valid: {}", id)
        }
        /// A filter could not be constructed due to invalid arguments.
        FilterCreationError(err: String) {
            display("The filter could not be constructed due to invalid arguments: {}", err)
        }
    }
}

quick_error! {
    #[derive(Debug)]
    /// Errors that can occur when validating a response from the Holodex API.
    pub enum ValidationError {
        /// The API returned a server error.
        ServerError(err: ServerError) {
            display("Server error: {}", err)
            from()
        }
        /// The response from the API could not be parsed.
        ParseError(err: ParseError) {
            display("Parse error: {}", err)
            from()
        }
    }
}

quick_error! {
    #[derive(Debug)]
    /// Errors that occur when the API returns an error code.
    pub enum ServerError {
        /// The API returned an error code.
        ErrorCode(code: u16) {
            display("Server returned an error code: {}", code)
            from()
        }
        /// The API returned an error code with a message.
        ErrorCodeWithValue(code: u16, message: String) {
            display("Server returned an error message: [{}] {}", code, message)
        }
        /// The API returned an error with a message that could not be parsed.
        ErrorCodeWithValueParseError(code: u16, source: ParseError) {
            display("Server returned code {} with a message that could not be parsed: {:?}", code, source)
            from(source)
        }
    }
}

quick_error! {
    #[derive(Debug)]
    /// Errors that occur when parsing a response from the API.
    pub enum ParseError {
        /// The response from the API could not be converted into bytes.
        ResponseDecodeError(err: std::io::Error) {
            display("Could not decode response: {}", err)
            source(err)
        }
        /// The response from the API lacked a header.
        MissingHeader(header: &'static str) {
            display("Response lacked header: {}", header)
        }
        /// The response from the API could not be parsed as JSON.
        ResponseJsonParseError(err: serde_json::Error, response: String) {
            display("Failed to parse response as JSON: {}\nResponse: {}", err, response)
            source(err)
        }
        /// The response from the API could not be parsed.
        ResponseParseError(err: serde_json::Error, response: serde_json::Value) {
            display("Failed to parse response: {}\nResponse: {}", err, response)
            source(err)
        }
        /// The response from the API could not be parsed as JSON or UTF-8.
        ResponseUtf8Error(err: std::str::Utf8Error) {
            display("Response was neither valid JSON nor valid UTF-8.")
            source(err)
            from()
        }
    }
}
