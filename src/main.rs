use anyhow::Result;
use rmcp::{
    ServerHandler, ServiceExt,
    model::{ServerCapabilities, ServerInfo},
    schemars, tool,
    transport::stdio,
};
use libphext;

const USER_AGENT: &str = "sqm/1.0";

#[derive(Debug, serde::Deserialize)]
pub struct PhextResponse {
    pub scroll: String
}

#[derive(Debug, Clone)]
pub struct sqclient {
    client: reqwest::Client,
}

async fn make_request<T>(&self, url: &str) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    let response = self
        .client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    match response.status() {
        reqwest::StatusCode::OK => response
            .json::<T>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e)),
        status => Err(format!("Request failed with status: {}", status)),
    }
}

fn main() {
    println!("sqm v{}", env!("CARGO_PKG_VERSION"));
}
