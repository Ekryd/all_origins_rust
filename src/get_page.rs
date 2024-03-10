use reqwest::{header, Method};

use crate::page_types::PageContent;
use crate::VERSION;

/// Get external web page given a URL
pub struct GetPage {
    url: String,
}

impl GetPage {
    pub(crate) fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn get_page_info(&self) -> PageContent {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        let response = client
            .request(Method::HEAD, self.url.clone())
            .header(
                header::USER_AGENT,
                format!("Mozilla/5.0 (compatible; all_origins_rust/{VERSION}"),
            )
            .send()
            .await;
        match response {
            Ok(response) => PageContent::info(response),
            Err(err) => PageContent::error(err, self.url.to_string()),
        }
    }

    pub async fn get_page(&self, method: Method) -> PageContent {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        let response = client
            .request(method, self.url.clone())
            .header(
                header::USER_AGENT,
                format!("Mozilla/5.0 (compatible; all_origins_rust/{VERSION}"),
            )
            .send()
            .await;
        match response {
            Ok(response) => PageContent::data(response).await,
            Err(err) => PageContent::error(err, self.url.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn get_method_should_return_data() {
        let server = setup().await;
        let page_content = GetPage::new(format!("{}/example", server.uri()))
            .get_page(Method::GET)
            .await;

        assert_eq!(page_content.url, server.uri() + "/example");
        assert_eq!(page_content.content_type.unwrap(), "text/plain");
        assert_eq!(page_content.http_code.unwrap(), 200);
        assert_eq!(page_content.content_length.unwrap(), 10);
        assert_eq!(page_content.contents.unwrap(), "Hello, Get");
    }

    #[tokio::test]
    async fn delete_method_should_return_data() {
        let server = setup().await;
        let page_content = GetPage::new(format!("{}/example", server.uri()))
            .get_page(Method::DELETE)
            .await;

        assert_eq!(page_content.url, server.uri() + "/example");
        assert_eq!(page_content.content_type.unwrap(), "text/plain");
        assert_eq!(page_content.http_code.unwrap(), 200);
        assert_eq!(page_content.content_length.unwrap(), 13);
        assert_eq!(page_content.contents.unwrap(), "Hello, Delete");
    }

    #[tokio::test]
    async fn put_method_should_return_data() {
        let server = setup().await;
        let page_content = GetPage::new(format!("{}/example", server.uri()))
            .get_page(Method::PUT)
            .await;

        assert_eq!(page_content.url, server.uri() + "/example");
        assert_eq!(page_content.content_type.unwrap(), "text/plain");
        assert_eq!(page_content.http_code.unwrap(), 200);
        assert_eq!(page_content.content_length.unwrap(), 10);
        assert_eq!(page_content.contents.unwrap(), "Hello, Put");
    }

    #[tokio::test]
    async fn post_method_should_return_data() {
        let server = setup().await;
        let page_content = GetPage::new(format!("{}/example", server.uri()))
            .get_page(Method::POST)
            .await;

        assert_eq!(page_content.url, server.uri() + "/example");
        assert_eq!(page_content.content_type.unwrap(), "text/plain");
        assert_eq!(page_content.http_code.unwrap(), 201);
        assert_eq!(page_content.content_length.unwrap(), 11);
        assert_eq!(page_content.contents.unwrap(), "Hello, Post");
    }

    #[tokio::test]
    async fn options_method_should_return_info() {
        let server = setup().await;
        let page_content = GetPage::new(format!("{}/example", server.uri()))
            .get_page(Method::OPTIONS)
            .await;

        assert_eq!(page_content.url, server.uri() + "/example");
        assert_eq!(page_content.content_type.unwrap(), "text/plain");
        assert_eq!(page_content.http_code.unwrap(), 200);
        assert_eq!(page_content.content_length.unwrap(), 14);
        assert_eq!(page_content.contents.unwrap(), "Hello, Options");
    }

    #[tokio::test]
    async fn head_method_should_return_info() {
        let server = setup().await;
        let page_content = GetPage::new(format!("{}/example", server.uri()))
            .get_page(Method::HEAD)
            .await;

        assert_eq!(page_content.url, server.uri() + "/example");
        assert_eq!(page_content.content_type.unwrap(), "text/plain");
        assert_eq!(page_content.http_code.unwrap(), 200);
        assert_eq!(page_content.content_length.is_none(), true);
        assert_eq!(page_content.contents.is_none(), true);
    }

    #[tokio::test]
    async fn get_page_info_should_always_return_head() {
        let server = setup().await;
        let page_content = GetPage::new(format!("{}/example", server.uri()))
            .get_page_info()
            .await;

        assert_eq!(page_content.url, server.uri() + "/example");
        assert_eq!(page_content.content_type.unwrap(), "text/plain");
        assert_eq!(page_content.http_code.unwrap(), 200);
        assert_eq!(page_content.content_length.unwrap(), 11);
        assert_eq!(page_content.contents.is_none(), true);
    }

    async fn setup() -> MockServer {
        let server = MockServer::start().await;

        // Define a mock for your API endpoint
        Mock::given(method("GET"))
            .and(path("/example"))
            .respond_with(
                ResponseTemplate::new(200)
                    .append_header(header::CONTENT_TYPE.as_str(), "text/plain")
                    .set_body_string("Hello, Get"),
            )
            .mount(&server)
            .await;

        Mock::given(method("DELETE"))
            .and(path("/example"))
            .respond_with(
                ResponseTemplate::new(200)
                    .append_header(header::CONTENT_TYPE.as_str(), "text/plain")
                    .set_body_string("Hello, Delete"),
            )
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path("/example"))
            .respond_with(
                ResponseTemplate::new(201)
                    .append_header(header::CONTENT_TYPE.as_str(), "text/plain")
                    .set_body_string("Hello, Post"),
            )
            .mount(&server)
            .await;

        Mock::given(method("PUT"))
            .and(path("/example"))
            .respond_with(
                ResponseTemplate::new(200)
                    .append_header(header::CONTENT_TYPE.as_str(), "text/plain")
                    .set_body_string("Hello, Put"),
            )
            .mount(&server)
            .await;

        Mock::given(method("HEAD"))
            .and(path("/example"))
            .respond_with(
                ResponseTemplate::new(200)
                    .append_header(header::CONTENT_TYPE.as_str(), "text/plain")
                    .append_header(header::CONTENT_LENGTH.as_str(), "11")
                    .set_body_string("Hello, Head"),
            )
            .mount(&server)
            .await;

        Mock::given(method("OPTIONS"))
            .and(path("/example"))
            .respond_with(
                ResponseTemplate::new(200)
                    .append_header(header::CONTENT_TYPE.as_str(), "text/plain")
                    .set_body_string("Hello, Options"),
            )
            .mount(&server)
            .await;

        server
    }
}
