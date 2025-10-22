use thiserror::Error;

use crate::models::error::SonioxApiError;

#[derive(Debug, Error)]
pub enum SonioxError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("API error: {0}")]
    ApiError(#[from] SonioxApiError),

    #[error("Missing API key")]
    MissingApiKey,

    #[error("Invalid endpoint URL: {0}")]
    InvalidEndpointUrl(#[from] url::ParseError),

    #[error("Unknown status code: {0}")]
    UnknownStatusCode(http::StatusCode),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
