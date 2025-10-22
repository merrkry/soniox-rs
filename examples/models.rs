use soniox_rs::client::SonioxClient;

#[tokio::main]
async fn main() {
    let client = SonioxClient::from_env().unwrap();

    let resp = client.get_models().await.unwrap();

    println!("Available Models:");
    for model in resp.models {
        println!("Model ID: {}, Name: {}", model.id, model.name);
    }
}
