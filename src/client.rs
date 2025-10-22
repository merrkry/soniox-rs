use http::StatusCode;
use serde::de::DeserializeOwned;
use url::Url;

use crate::{error::SonioxError, models::error::SonioxApiError};

const DEFAULT_SONIOX_API_ENDPOINT: &str = "https://api.soniox.com";
const DEFAULT_SONIOX_WEBSOCKET_ENDPOINT: &str = "wss://stt-rt.soniox.com/transcribe-websocket";

pub struct SonioxClient {
    reqwest_client: reqwest::Client,
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
            reqwest_client: reqwest::Client::new(),
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
        let ws_endpoint = DEFAULT_SONIOX_WEBSOCKET_ENDPOINT.parse()?;
        Ok(Self {
            reqwest_client: reqwest::Client::new(),
            api_key,
            rest_endpoint,
            ws_endpoint,
        })
    }

    pub(crate) fn request_builder(
        &self,
        method: http::Method,
        path: &str,
    ) -> Result<reqwest::RequestBuilder, SonioxError> {
        let url = self.rest_endpoint.join(path)?;
        Ok(self.reqwest_client.request(method, url))
    }

    pub(crate) async fn request_with_auth<T>(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<T, SonioxError>
    where
        T: DeserializeOwned,
    {
        let req = req.bearer_auth(&self.api_key);

        let resp = req.send().await?;

        if resp.status().is_success() {
            let data = resp.json::<T>().await?;
            Ok(data)
        } else {
            let api_error = match resp.status() {
                StatusCode::BAD_REQUEST => SonioxApiError::InvalidRequest(resp.json().await?),
                StatusCode::UNAUTHORIZED => SonioxApiError::AuthenticationError(resp.json().await?),
                StatusCode::NOT_FOUND => SonioxApiError::FileNotFound(resp.json().await?),
                StatusCode::CONFLICT => {
                    SonioxApiError::InvalidTranscriptionState(resp.json().await?)
                }
                StatusCode::INTERNAL_SERVER_ERROR => {
                    SonioxApiError::InternalServerError(resp.json().await?)
                }
                _ => {
                    return Err(SonioxError::UnknownStatusCode(resp.status()));
                }
            };

            Err(SonioxError::ApiError(api_error))
        }
    }
}
