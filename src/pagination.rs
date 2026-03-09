use anyhow::Result;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::client::TraktClient;

/// Pagination parameters for Trakt API requests.
#[derive(Debug, Clone, Default)]
pub struct Pagination {
    /// Page number (1-indexed).
    pub page: Option<u32>,
    /// Number of items per page (default: 10, max: 100).
    pub limit: Option<u32>,
}

impl Pagination {
    /// Create a new Pagination with default settings (page 1, limit 10).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the page number.
    pub fn with_page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set the items per page.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Convert to query parameters for the API request.
    pub fn to_query(&self) -> Vec<(&str, String)> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page", page.to_string()));
        }
        if let Some(limit) = self.limit {
            query.push(("limit", limit.to_string()));
        }
        query
    }
}

/// Paginated response metadata from Trakt API headers.
#[derive(Debug, Clone, Default)]
pub struct PaginatedMeta {
    /// Current page number.
    pub page: u32,
    /// Items per page.
    pub limit: u32,
    /// Total number of pages.
    pub page_count: u32,
    /// Total number of items.
    pub item_count: u32,
}

impl PaginatedMeta {
    /// Parse pagination metadata from response headers.
    pub fn from_response(resp: &Response) -> Option<Self> {
        let page = resp
            .headers()
            .get("X-Pagination-Page")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())?;
        let limit = resp
            .headers()
            .get("X-Pagination-Limit")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())?;
        let page_count = resp
            .headers()
            .get("X-Pagination-Page-Count")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())?;
        let item_count = resp
            .headers()
            .get("X-Pagination-Item-Count")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())?;

        Some(Self {
            page,
            limit,
            page_count,
            item_count,
        })
    }
}

/// Fetch a single page of results.
pub async fn fetch_page<T: DeserializeOwned>(
    client: &TraktClient,
    endpoint: &str,
    pagination: &Pagination,
) -> Result<(T, PaginatedMeta)> {
    let query = pagination.to_query();
    let query_ref: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();

    let resp = client
        .inner
        .request(reqwest::Method::GET, format!("{}{}", client.base_url, endpoint))
        .query(&query_ref)
        .send()
        .await?;

    let meta = PaginatedMeta::from_response(&resp)
        .ok_or_else(|| anyhow::anyhow!("Missing pagination headers"))?;

    let data: T = serde_json::from_str(&resp.text().await?)?;
    Ok((data, meta))
}

/// Fetch all pages for a given endpoint.
/// This function will loop through all pages until there are no more.
pub async fn fetch_all(
    client: &TraktClient,
    endpoint: &str,
    max_pages: Option<u32>,
) -> Result<Vec<Value>> {
    let mut all_items: Vec<Value> = Vec::new();
    let mut current_page = 1;

    loop {
        let pagination = Pagination::new()
            .with_page(current_page)
            .with_limit(100); // Maximum allowed by Trakt

        let query: Vec<(&str, String)> = pagination.to_query();
        let query_ref: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let resp = client
            .inner
            .request(
                reqwest::Method::GET,
                format!("{}{}", client.base_url, endpoint),
            )
            .query(&query_ref)
            .send()
            .await?;

        let meta = match PaginatedMeta::from_response(&resp) {
            Some(m) => m,
            None => break, // No pagination headers means single page response
        };

        let text = resp.text().await?;
        let items: Value = serde_json::from_str(&text)?;

        // Append items to the collection
        if let Some(arr) = items.as_array() {
            all_items.extend(arr.clone());
        } else {
            // If it's not an array, just store it as-is on the first iteration
            if current_page == 1 {
                return Ok(vec![items]);
            }
        }

        // Check if we've reached the last page
        if meta.page >= meta.page_count {
            break;
        }

        // Check if we've hit the max pages limit
        if let Some(max) = max_pages {
            if current_page >= max {
                break;
            }
        }

        current_page += 1;
    }

    Ok(all_items)
}
