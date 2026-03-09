use anyhow::{anyhow, Result};
use reqwest::{Client, Method, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::time::Duration;
use tokio::time::sleep;

use crate::config::Config;

/// Wrapper around `reqwest::Client` that automatically adds Trakt‑specific headers
/// and handles simple rate‑limit retries.
pub struct TraktClient {
    pub inner: Client,
    pub base_url: String,
    config: Config,
}

impl TraktClient {
    /// Create a new client using supplied configuration.
    /// This function builds a `reqwest::Client` with required default headers.
    pub async fn new(config: Config) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        // User‑Agent can be customized; we use a static identifier.
        headers.insert(
            reqwest::header::USER_AGENT,
            "trakt-cli/0.1.0".parse().unwrap(),
        );
        headers.insert(
            "trakt-api-key",
            config.client_id.clone().parse().unwrap(),
        );
        headers.insert(
            "trakt-api-version",
            "2".parse().unwrap(),
        );
        if let Some(token) = &config.access_token {
            let bearer = format!("Bearer {}", token);
            headers.insert(
                reqwest::header::AUTHORIZATION,
                bearer.parse().unwrap(),
            );
        }

        let client = Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            inner: client,
            base_url: "https://api.trakt.tv".to_string(),
            config,
        })
    }

    /// Perform a GET request.
    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<T> {
        self.request(Method::GET, endpoint, query, None).await
    }

    /// Perform a POST request with a JSON body.
    pub async fn post<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: Option<&serde_json::Value>,
    ) -> Result<T> {
        self.request(Method::POST, endpoint, None, body).await
    }
    
    /// Perform a PUT request with a JSON body.
    pub async fn put<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: Option<&serde_json::Value>,
    ) -> Result<T> {
        self.request(Method::PUT, endpoint, None, body).await
    }
    
    /// Perform a DELETE request.
    pub async fn delete<T: DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T> {
        self.request(Method::DELETE, endpoint, None, None).await
    }
    
    /// Core request implementation handling retries for rate limiting.
    async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        endpoint: &str,
        query: Option<&[(&str, &str)]>,
        body: Option<&serde_json::Value>,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut attempts = 0;
        loop {
            attempts += 1;
            let mut req = self.inner.request(method.clone(), &url);
            if let Some(q) = query {
                req = req.query(q);
            }
            if let Some(b) = body {
                req = req.json(b);
            }
            let resp = req.send().await?;
            match resp.status() {
                StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => {
                    return Self::parse_response(resp).await;
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    // Respect Retry‑After header if present, otherwise back‑off.
                    let wait = resp
                        .headers()
                        .get("Retry-After")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| s.parse::<u64>().ok())
                        .map(Duration::from_secs)
                        .unwrap_or_else(|| Duration::from_secs(5));
                    if attempts >= 3 {
                        return Err(anyhow!("Rate limit exceeded after {} attempts", attempts));
                    }
                    sleep(wait).await;
                    continue;
                }
                other => {
                    let txt = resp.text().await.unwrap_or_default();
                    return Err(anyhow!("HTTP {} error: {}", other, txt));
                }
            }
        }
    }

    async fn parse_response<T: DeserializeOwned>(resp: Response) -> Result<T> {
        let status = resp.status();
        let bytes = resp.bytes().await?;
        if !status.is_success() {
            return Err(anyhow!("Unexpected status {}: {}", status, String::from_utf8_lossy(&bytes)));
        }
        let parsed = serde_json::from_slice(&bytes)?;
        Ok(parsed)
    }
}
