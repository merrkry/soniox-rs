use soniox_rs::{client::SonioxClient, models::auth::CreateTemporaryApiKeyRequest};

#[tokio::main]
async fn main() {
    let client = SonioxClient::from_env().unwrap();

    let req = CreateTemporaryApiKeyRequest::builder()
        .usage_type("transcribe_websocket")
        .expires_in_seconds(1)
        // .client_reference_id("example-client-ref")
        .build()
        .unwrap();

    let resp = client.create_temporary_api_key(req).await.unwrap();

    println!("Temporary API Key: {}", resp.api_key);
}
