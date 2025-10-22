use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateTemporaryApiKeyRequest {
    usage_type: String,
    expires_in_seconds: u32,
    client_reference_id: String,
}
