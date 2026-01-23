//! GitHub API client implementation

use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

use crate::services::{
    FetchResult, GitHubClient, RateLimitInfo, UpdateInfo, UrlClient, UrlFetchResult,
};
use crate::utils::error::{Error, Result};

/// GitHub API client
pub struct GitHubClientImpl {
    client: Client,
    base_url: String,
    token: Option<String>,
}

impl GitHubClientImpl {
    /// Create a new GitHub client
    pub fn new(token: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.github.com".to_string(),
            token,
        }
    }

    /// Create with custom base URL (for testing)
    pub fn with_base_url(base_url: impl Into<String>, token: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
            token,
        }
    }

    /// Build a request with common headers
    fn build_request(&self, url: &str) -> reqwest::RequestBuilder {
        let mut req = self.client.get(url);
        req = req.header("User-Agent", "claude-skill-manager");
        req = req.header("Accept", "application/vnd.github.v3+json");

        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        req
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GitHubFileResponse {
    sha: String,
    content: String,
    encoding: String,
}

#[derive(Debug, Deserialize)]
struct GitHubCommitResponse {
    sha: String,
}

#[derive(Debug, Deserialize)]
struct GitHubCompareResponse {
    ahead_by: usize,
    commits: Vec<GitHubCommitInfo>,
}

#[derive(Debug, Deserialize)]
struct GitHubCommitInfo {
    commit: GitHubCommitDetail,
}

#[derive(Debug, Deserialize)]
struct GitHubCommitDetail {
    message: String,
}

#[derive(Debug, Deserialize)]
struct GitHubRateLimitResponse {
    rate: GitHubRateLimit,
}

#[derive(Debug, Deserialize)]
struct GitHubRateLimit {
    limit: u32,
    remaining: u32,
    reset: u64,
}

#[async_trait]
impl GitHubClient for GitHubClientImpl {
    async fn fetch_content(
        &self,
        owner: &str,
        repo: &str,
        path: Option<&str>,
        ref_spec: Option<&str>,
    ) -> Result<FetchResult> {
        let file_path = path.unwrap_or("CLAUDE.md");
        let ref_param = ref_spec.unwrap_or("HEAD");

        // Fetch file content
        let url = format!(
            "{}/repos/{}/{}/contents/{}?ref={}",
            self.base_url, owner, repo, file_path, ref_param
        );

        let response = self.build_request(&url).send().await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(Error::RepoNotFound {
                owner: owner.to_string(),
                repo: repo.to_string(),
            });
        }

        if response.status() == reqwest::StatusCode::FORBIDDEN {
            // Check if rate limited
            if response
                .headers()
                .get("x-ratelimit-remaining")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u32>().ok())
                == Some(0)
            {
                return Err(Error::RateLimited);
            }
        }

        if !response.status().is_success() {
            return Err(Error::github(format!(
                "GitHub API error: {}",
                response.status()
            )));
        }

        let file_info: GitHubFileResponse = response.json().await?;

        // Decode base64 content
        let content = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            file_info.content.replace('\n', ""),
        )
        .map_err(|e| Error::github(format!("Failed to decode content: {}", e)))?;

        let content_str = String::from_utf8(content)
            .map_err(|e| Error::github(format!("Invalid UTF-8 content: {}", e)))?;

        // Get current commit SHA
        let commit_sha = self.get_commit_sha(owner, repo, ref_param).await?;

        Ok(FetchResult {
            content: content_str,
            sha: file_info.sha,
            commit_sha,
        })
    }

    async fn check_updates(
        &self,
        owner: &str,
        repo: &str,
        current_sha: &str,
        ref_spec: Option<&str>,
    ) -> Result<Option<UpdateInfo>> {
        let ref_param = ref_spec.unwrap_or("HEAD");

        // Get latest commit SHA
        let latest_sha = self.get_commit_sha(owner, repo, ref_param).await?;

        if latest_sha == current_sha {
            return Ok(None);
        }

        // Compare commits
        let url = format!(
            "{}/repos/{}/{}/compare/{}...{}",
            self.base_url, owner, repo, current_sha, latest_sha
        );

        let response = self.build_request(&url).send().await?;

        if !response.status().is_success() {
            // If compare fails, just return basic info
            return Ok(Some(UpdateInfo {
                current_sha: current_sha.to_string(),
                latest_sha,
                commits_behind: 1,
                commit_messages: vec!["Update available".to_string()],
            }));
        }

        let comparison: GitHubCompareResponse = response.json().await?;

        Ok(Some(UpdateInfo {
            current_sha: current_sha.to_string(),
            latest_sha,
            commits_behind: comparison.ahead_by,
            commit_messages: comparison
                .commits
                .iter()
                .map(|c| c.commit.message.lines().next().unwrap_or("").to_string())
                .collect(),
        }))
    }

    async fn rate_limit(&self) -> Result<RateLimitInfo> {
        let url = format!("{}/rate_limit", self.base_url);

        let response = self.build_request(&url).send().await?;

        if !response.status().is_success() {
            return Err(Error::github("Failed to fetch rate limit info"));
        }

        let info: GitHubRateLimitResponse = response.json().await?;

        Ok(RateLimitInfo {
            limit: info.rate.limit,
            remaining: info.rate.remaining,
            reset: info.rate.reset,
        })
    }
}

impl GitHubClientImpl {
    async fn get_commit_sha(&self, owner: &str, repo: &str, ref_spec: &str) -> Result<String> {
        let url = format!(
            "{}/repos/{}/{}/commits/{}",
            self.base_url, owner, repo, ref_spec
        );

        let response = self.build_request(&url).send().await?;

        if !response.status().is_success() {
            return Err(Error::github(format!(
                "Failed to get commit: {}",
                response.status()
            )));
        }

        let commit: GitHubCommitResponse = response.json().await?;
        Ok(commit.sha)
    }
}

/// Simple URL client for fetching content from URLs
pub struct SimpleUrlClient {
    client: Client,
}

impl SimpleUrlClient {
    /// Create a new URL client
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl Default for SimpleUrlClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UrlClient for SimpleUrlClient {
    async fn fetch(&self, url: &str) -> Result<UrlFetchResult> {
        let response = self
            .client
            .get(url)
            .header("User-Agent", "claude-skill-manager")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::FetchFailed(format!(
                "HTTP {}: {}",
                response.status(),
                url
            )));
        }

        let etag = response
            .headers()
            .get("etag")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let content = response.text().await?;

        Ok(UrlFetchResult { content, etag })
    }

    async fn check_modified(&self, url: &str, etag: Option<&str>) -> Result<bool> {
        let mut req = self
            .client
            .head(url)
            .header("User-Agent", "claude-skill-manager");

        if let Some(etag) = etag {
            req = req.header("If-None-Match", etag);
        }

        let response = req.send().await?;

        // 304 Not Modified means content hasn't changed
        Ok(response.status() != reqwest::StatusCode::NOT_MODIFIED)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Integration tests would use wiremock for mocking
    // Unit tests for URL client
    #[test]
    fn test_simple_url_client_creation() {
        let client = SimpleUrlClient::new();
        // Just verify it can be created
        let _ = client;
    }
}
