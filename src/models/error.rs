use std::fmt::Display;

/// Serializable structs for Soniox API errors.
use serde::Deserialize;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum SonioxApiError {
    #[error("Invalid request: {0}")]
    InvalidRequest(RawSonioxApiError), // 400

    #[error("Authentication error: {0}")]
    AuthenticationError(RawSonioxApiError), // 401

    #[error("File not found: {0}")]
    FileNotFound(RawSonioxApiError), // 404

    #[error("Invalid transcription state: {0}")]
    InvalidTranscriptionState(RawSonioxApiError), // 409

    #[error("Internal server error: {0}")]
    InternalServerError(RawSonioxApiError), // 500
}

/// Deserializable struct for error response from Soniox API.
/// The same structure is used for all error types.
#[derive(Debug, Deserialize)]
pub struct RawSonioxApiError {
    pub status_code: StatusCode,
    pub error_type: String,
    pub message: String,
    pub validation_errors: Vec<ValidationError>,
    pub request_id: Uuid,
}

impl Display for RawSonioxApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

// This doesn't seem useful because we have HTTP status code already.
// But consider using `http-serde`.
pub type StatusCode = u16;

#[derive(Debug, Deserialize)]
pub struct ValidationError {
    pub error_type: String,
    pub location: String,
    pub message: String,
}
