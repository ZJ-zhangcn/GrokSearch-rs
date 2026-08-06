use async_trait::async_trait;
use reqwest::header::USER_AGENT;
use reqwest::Client;
use url::Url;

use crate::error::Result;
use crate::sources::{get_text, SourceCaps, SourceExtractor, SourceType};

const UA: &str = "grok-search-rs/0.2 (https://github.com/Episkey-G/GrokSearch-rs)";

/// Reads public raw GitHub files directly. Tavily's extract endpoint commonly
/// returns an empty body for `raw.githubusercontent.com`, while the raw asset is
/// already plain text and needs no generic content extraction.
pub struct GithubRawFileExtractor;

fn is_raw_github_file(url: &Url) -> bool {
    url.scheme() == "https"
        && url.host_str() == Some("raw.githubusercontent.com")
        && url
            .path_segments()
            .map(|segments| segments.filter(|segment| !segment.is_empty()).count() >= 4)
            .unwrap_or(false)
}

#[async_trait]
impl SourceExtractor for GithubRawFileExtractor {
    fn matches(&self, url: &Url) -> bool {
        is_raw_github_file(url)
    }

    fn kind(&self) -> SourceType {
        SourceType::Generic
    }

    async fn fetch_render(&self, client: &Client, url: &Url, _caps: &SourceCaps) -> Result<String> {
        get_text(client, url.as_str(), &[(USER_AGENT, UA)], "GitHub Raw").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_only_https_raw_github_files() {
        assert!(is_raw_github_file(
            &Url::parse("https://raw.githubusercontent.com/owner/repo/main/rules/file.yaml")
                .unwrap()
        ));
        assert!(!is_raw_github_file(
            &Url::parse("http://raw.githubusercontent.com/owner/repo/main/file.yaml").unwrap()
        ));
        assert!(!is_raw_github_file(
            &Url::parse("https://github.com/owner/repo/raw/main/file.yaml").unwrap()
        ));
    }
}
