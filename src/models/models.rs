use serde::Deserialize;

use crate::client::SonioxClient;

impl SonioxClient {
    /// Wrapper for `GET /v1/models`.
    ///
    /// # Errors
    ///
    /// Returns `SonioxError` if the request fails or the response cannot be parsed.
    pub async fn get_models(&self) -> Result<GetModelsResponse, crate::error::SonioxError> {
        let req_builder = self.request_builder(http::Method::GET, "/v1/models")?;
        self.request_with_auth(req_builder).await
    }
}

// TODO: Represent `all_languages` in Rust type system.

#[derive(Debug, Deserialize)]
pub struct GetModelsResponse {
    pub models: Vec<ModelInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub aliased_model_id: Option<String>,
    pub name: String,
    pub context_version: u32,
    pub transcription_mode: TranscriptionMode,
    pub languages: Vec<SupportedLanguage>,
    pub translation_targets: Vec<TranslationTarget>,
    pub two_way_translation_pairs: Vec<String>,
    pub one_way_translation: Option<String>,
    pub two_way_translation: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum TranscriptionMode {
    #[serde(rename = "real_time")]
    Realtime,
    #[serde(rename = "async")]
    Async,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct SupportedLanguage {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TranslationTarget {
    pub target_language: String,
    pub source_languages: Vec<String>,
    pub exclude_source_languages: Vec<String>,
}
