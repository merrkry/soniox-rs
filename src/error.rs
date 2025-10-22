use thiserror::Error;

use crate::models::error::SonixApiError;

#[derive(Debug, Error)]
pub enum SonioxError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("API error: {0}")]
    ApiError(#[from] SonixApiError),
    #[error("Missing API key")]
    MissingApiKey,
    #[error("Invalid endpoint URL: {0}")]
    InvalidEndpointUrl(#[from] url::ParseError),
}
