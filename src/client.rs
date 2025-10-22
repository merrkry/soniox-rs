use url::Url;

use crate::error::SonioxError;

const DEFAULT_SONIOX_API_ENDPOINT: &str = "https://api.soniox.com";
const DEFAULT_SONIX_WEBSOCKET_ENDPOINT: &str = "wss://stt-rt.soniox.com/transcribe-websocket";

pub struct SonioxClient {
    api_key: String,
    rest_endpoint: Url,
    ws_endpoint: Url,
}

impl SonioxClient {
    /// Creates a new `SonioxClient` with the given API key and endpoints.
    pub fn new<T, U>(api_key: T, rest_endpoint: U, ws_endpoint: U) -> Self
    where
        T: Into<String>,
        U: Into<Url>,
    {
        Self {
            api_key: api_key.into(),
            rest_endpoint: rest_endpoint.into(),
            ws_endpoint: ws_endpoint.into(),
        }
    }

    /// Creates a `SonioxClient` using the API key from the environment variable `SONIOX_API_KEY`.
    ///
    /// # Errors
    ///
    /// Returns `SonioxError::MissingApiKey` if the environment variable is not set.
    pub fn from_env() -> Result<Self, SonioxError> {
        let api_key = std::env::var("SONIOX_API_KEY").map_err(|_| SonioxError::MissingApiKey)?;
        let rest_endpoint = DEFAULT_SONIOX_API_ENDPOINT.parse()?;
        let ws_endpoint = DEFAULT_SONIX_WEBSOCKET_ENDPOINT.parse()?;
        Ok(Self {
            api_key,
            rest_endpoint,
            ws_endpoint,
        })
    }
}
