use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::config::Config;

/// Device code response from Trakt.
#[derive(Debug, Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_url: String,
    expires_in: u64,
    interval: u64,
}

/// Token response from Trakt.
#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
    scope: Option<String>,
}

/// Error response from Trakt token endpoint.
#[derive(Debug, Deserialize)]
struct TokenErrorResponse {
    error: String,
    error_description: Option<String>,
}

/// Perform the OAuth2 Device Authorization flow for Trakt.
///
/// The function reads `client_id` and `client_secret` from the provided `Config`.
/// It then requests a device code, prints the verification URL and user code,
/// and polls the token endpoint until the user authorizes the app. The obtained
/// access and refresh tokens are stored back into the configuration file.
pub async fn device_auth(config: &mut Config) -> Result<()> {
    // Ensure client credentials are present.
    if config.client_id.is_empty() || config.client_secret.is_empty() {
        anyhow::bail!("client_id and client_secret must be set in the config file");
    }

    let client = Client::new();

    // Step 1: Request device code
    let resp = client
        .post("https://api.trakt.tv/oauth/device/code")
        .header("Content-Type", "application/json")
        .header("trakt-api-key", &config.client_id)
        .header("trakt-api-version", "2")
        .header("User-Agent", "trakt-cli/1.0")
        .json(&serde_json::json!({
            "client_id": config.client_id
        }))
        .send()
        .await?;
    
    // Debug: print status and text if not success
    if !resp.status().is_success() {
        let text = resp.text().await?;
        anyhow::bail!("Device code request failed: {}", text);
    }
    
    let device_code_resp: DeviceCodeResponse = resp.json().await?;

    println!(
        "\nVisit {} and enter the code: {}",
        device_code_resp.verification_url, device_code_resp.user_code
    );
    println!("The code expires in {} seconds.\n", device_code_resp.expires_in);

    // Step 2: Poll for token
    let interval_secs = device_code_resp.interval;
    loop {
        sleep(Duration::from_secs(interval_secs)).await;

        let resp = client
            .post("https://api.trakt.tv/oauth/device/token")
            .header("Content-Type", "application/json")
            .header("trakt-api-key", &config.client_id)
            .header("trakt-api-version", "2")
            .header("User-Agent", "trakt-cli/1.0")
            .json(&serde_json::json!({
                "code": device_code_resp.device_code,
                "client_id": config.client_id,
                "client_secret": config.client_secret
            }))
            .send()
            .await?;

        // Check status and handle response
        let status = resp.status();
        if status.is_success() {
            let token: TokenResponse = resp.json().await?;
            config.access_token = Some(token.access_token);
            config.refresh_token = token.refresh_token;
            config.save()?;
            println!("Authentication successful!");
            return Ok(());
        }
        
        // Handle error case - try to get error message
        let error_text = resp.text().await?;
        
        // Try to parse as JSON error
        if let Ok(error) = serde_json::from_str::<TokenErrorResponse>(&error_text) {
            match error.error.as_str() {
                "authorization_pending" => {
                    // Still waiting for user to authorize - continue polling
                    continue;
                }
                "slow_down" => {
                    // Increase interval by 5 seconds as per spec.
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
                _ => {
                    anyhow::bail!(
                        "OAuth error: {} - {:?}",
                        error.error,
                        error.error_description
                    );
                }
            }
        }
        
        // Check for HTTP 400 which might mean authorization_pending
        if status.as_u16() == 400 {
            // 400 with empty body often means "authorization_pending" - continue polling
            continue;
        }
        
        // Couldn't parse error as JSON
        anyhow::bail!("OAuth request failed (status {}): {}", status, error_text)
    }
}
