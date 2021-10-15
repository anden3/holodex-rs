//! Types for various errors that can occur when interacting with the API.

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
/// Errors that can occur when interacting with the Holodex API.
pub enum Error {
    #[error("API token contains invalid characters.")]
    /// The API token provided to the client is invalid.
    InvalidApiToken,
    #[error("Error creating HTTP client: {0:?}")]
    /// An error occurred while creating the HTTP client.
    HttpClientCreationError(#[source] reqwest::Error),
}

}
