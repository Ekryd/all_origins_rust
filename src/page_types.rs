use reqwest::{header, Error, Response};
use serde::Serialize;

/// Return data from service
#[derive(Serialize)]
pub struct PageContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_code: Option<u16>,
    pub response_time: u32,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl PageContent {
    pub fn info(resp: Response) -> Self {
        PageContent {
            url: resp.url().to_string(),
            content_type: resp
                .headers()
                .get(header::CONTENT_TYPE)
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string()),
            content_length: resp
                .headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse().ok()),
            http_code: Some(resp.status().as_u16()),
            response_time: 0,
            contents: None,
            error: if resp.status().is_success() {
                None
            } else {
                Some(resp.status().to_string())
            },
        }
    }

    pub fn error(err: Error, url: String) -> PageContent {
        PageContent {
            url,
            content_type: None,
            content_length: None,
            http_code: err.status().map(|status| status.as_u16()),
            response_time: 0,
            contents: None,
            error: Some(err.to_string()),
        }
    }

    pub async fn data(resp: Response) -> PageContent {
        let content_type = resp
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        let http_code = Some(resp.status().as_u16());
        let url = resp.url().to_string();
        let error = if resp.status().is_success() {
            None
        } else {
            Some(resp.status().to_string())
        };
        let contents = resp.text().await.ok().filter(|s| !s.is_empty());
        let content_length = contents.as_ref().map(|c| c.len() as u64);

        PageContent {
            content_length,
            content_type,
            http_code,
            response_time: 0,
            url,
            contents,
            error,
        }
    }
}
