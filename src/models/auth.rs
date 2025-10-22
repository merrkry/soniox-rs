use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{client::SonioxClient, error::SonioxError};

impl SonioxClient {
    /// Wrapper for `POST /v1/auth/temporary-api-key`.
    ///
    /// # Errors
    ///
    /// Returns `SonioxError` if the request fails or the response cannot be parsed.
    pub async fn create_temporary_api_key(
        &self,
        request: CreateTemporaryApiKeyRequest,
    ) -> Result<CreateTemporaryApiKeyResponse, SonioxError> {
        let req_builder = self
            .request_builder(http::Method::POST, "/v1/auth/temporary-api-key")?
            .json(&request);
        self.request_with_auth(req_builder).await
    }
}

#[derive(Debug, Serialize, Builder)]
#[builder(pattern = "mutable", build_fn(validate = "Self::validate"))]
pub struct CreateTemporaryApiKeyRequest {
    #[builder(setter(into))]
    usage_type: String,
    expires_in_seconds: u32,
    #[builder(setter(strip_option, into), default)]
    client_reference_id: Option<String>,
}

impl CreateTemporaryApiKeyRequest {
    pub fn builder() -> CreateTemporaryApiKeyRequestBuilder {
        CreateTemporaryApiKeyRequestBuilder::default()
    }
}

const MAX_EXPIRES_IN_SECONDS: u32 = 3600;

impl CreateTemporaryApiKeyRequestBuilder {
    fn validate(&self) -> Result<(), String> {
        match self.usage_type.as_deref() {
            Some("transcribe_websocket") => {}
            _ => return Err(format!("Invalid usage_type: {:?}", self.usage_type)),
        }

        match self.expires_in_seconds {
            Some(seconds) if (1..=MAX_EXPIRES_IN_SECONDS).contains(&seconds) => {}
            _ => {
                return Err(format!(
                    "expires_in_seconds must be between 1 and {}, got: {:?}",
                    MAX_EXPIRES_IN_SECONDS, self.expires_in_seconds
                ));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTemporaryApiKeyResponse {
    pub api_key: String,
    pub expires_at: DateTime<Utc>,
}
