/// Integration test
#[cfg(test)]
mod tests {
    use crate::server::all_filters;
    use reqwest::header;
    use serde_json::Value;
    use std::time::Duration;
    use warp::test::request;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_basic_get_request() {
        let server = setup().await;
        let example_uri = server.uri();

        let response = request()
            .path(format!("/get?url={example_uri}/test.html").as_str())
            .reply(&all_filters())
            .await;

        assert_eq!(response.status(), 200);
        assert_eq!(response.headers()[header::CONTENT_TYPE], "application/json");

        let body = String::from_utf8(response.body().to_vec()).unwrap();
        let response_body: Value =
            serde_json::from_str(body.as_str()).expect("Failed to parse JSON response");

        // Assert specific properties of the JSON response
        assert_eq!(response_body["content_length"].as_i64(), Some(15));
        assert_eq!(response_body["content_type"].as_str(), Some("text/example"));
        assert_eq!(response_body["contents"].as_str(), Some("Hi, allOrigins!"));
        assert_eq!(response_body["http_code"].as_i64(), Some(200));
        assert!(response_body["response_time"].as_i64().unwrap() > 0);
        assert_eq!(
            response_body["url"].as_str(),
            Some(format!("{example_uri}/test.html").as_str())
        );
        assert!(response_body["error"].is_null());
    }

    #[tokio::test]
    async fn test_post_to_get_endpoint() {
        let server = setup().await;
        let example_uri = server.uri();

        let response = request()
            .method("POST")
            .path(format!("/get?url={example_uri}/test.html").as_str()) // Adjust the path as needed
            .reply(&all_filters())
            .await;

        assert_eq!(response.status(), 200);

        let body = String::from_utf8(response.body().to_vec()).unwrap();
        let response_body: Value =
            serde_json::from_str(body.as_str()).expect("Failed to parse JSON response");

        // Assert specific properties of the JSON response
        assert_eq!(response_body["content_length"].as_i64(), Some(28));
        assert_eq!(response_body["content_type"].as_str(), Some("text/plain"));
        assert_eq!(
            response_body["contents"].as_str(),
            Some("Hi, allOrigins! It's a POST!")
        );
        assert_eq!(response_body["http_code"].as_i64(), Some(201));
        assert!(response_body["response_time"].as_i64().unwrap() > 0);
        assert_eq!(
            response_body["url"].as_str(),
            Some(format!("{example_uri}/test.html").as_str())
        );
        assert!(response_body["error"].is_null());
    }

    #[tokio::test]
    async fn test_get_request_to_not_found_url() {
        let server = setup().await;
        let example_uri = server.uri();

        let response = request()
            .path(format!("/get?url={example_uri}/not-found.html").as_str()) // Adjust the path as needed
            .reply(&all_filters())
            .await;

        assert_eq!(response.status(), 200);

        let body = String::from_utf8(response.body().to_vec()).unwrap();
        let response_body: Value =
            serde_json::from_str(body.as_str()).expect("Failed to parse JSON response");

        assert_eq!(response_body["content_length"].as_i64(), None);
        assert!(response_body["content_type"].is_null());
        assert_eq!(response_body["contents"].as_str(), None);
        assert_eq!(response_body["http_code"].as_i64(), Some(404));
        assert!(response_body["response_time"].as_i64().unwrap() > 0);
        assert_eq!(
            response_body["url"].as_str(),
            Some(format!("{example_uri}/not-found.html").as_str())
        );
        assert_eq!(response_body["error"].as_str(), Some("404 Not Found"));
    }

    #[tokio::test]
    async fn test_raw_request() {
        let server = setup().await;
        let example_uri = server.uri();

        let response = request()
            .path(format!("/raw?url={example_uri}/test.html").as_str()) // Adjust the path as needed
            .reply(&all_filters())
            .await;

        assert_eq!(response.status(), 200);

        let body = String::from_utf8(response.body().to_vec()).unwrap();

        assert_eq!(response.headers()[header::CONTENT_LENGTH], "15");
        assert_eq!(response.headers()[header::CONTENT_TYPE], "text/example");
        assert_eq!(body, "Hi, allOrigins!");
    }

    #[tokio::test]
    async fn raw_request_to_error_should_return_json_structure() {
        let server = setup().await;
        let example_uri = server.uri();

        let response = request()
            .path(format!("/raw?url={example_uri}/not-found.html").as_str()) // Adjust the path as needed
            .reply(&all_filters())
            .await;

        assert_eq!(response.status(), 200);
        assert_eq!(response.headers()[header::CONTENT_TYPE], "application/json");

        let body = String::from_utf8(response.body().to_vec()).unwrap();
        let response_body: Value =
            serde_json::from_str(body.as_str()).expect("Failed to parse JSON response");

        // Assert specific properties of the JSON response
        assert_eq!(response_body["content_length"].as_i64(), None);
        assert!(response_body["content_type"].is_null());
        assert_eq!(response_body["contents"].as_str(), None);
        assert_eq!(response_body["http_code"].as_i64(), Some(404));
        assert!(response_body["response_time"].as_i64().unwrap() > 0);
        assert_eq!(
            response_body["url"].as_str(),
            Some(format!("{example_uri}/not-found.html").as_str())
        );
        assert_eq!(response_body["error"].as_str(), Some("404 Not Found"));
    }

    #[tokio::test]
    async fn test_info_request() {
        let server = setup().await;
        let example_uri = server.uri();

        let response = request()
            .path(format!("/info?url={example_uri}/test.html").as_str()) // Adjust the path as needed
            .reply(&all_filters())
            .await;

        assert_eq!(response.status(), 200);

        let body = String::from_utf8(response.body().to_vec()).unwrap();
        let response_body: Value =
            serde_json::from_str(body.as_str()).expect("Failed to parse JSON response");

        assert!(response_body["content_length"].is_null());
        assert_eq!(response_body["content_type"].as_str(), Some("text/html"));
        assert!(response_body["contents"].is_null());
        assert_eq!(response_body["http_code"].as_i64(), Some(204));
        assert!(response_body["response_time"].as_i64().unwrap() > 0);
        assert_eq!(
            response_body["url"].as_str(),
            Some(format!("{example_uri}/test.html").as_str())
        );
    }

    #[tokio::test]
    async fn test_options_request() {
        let server = setup().await;
        let example_uri = server.uri();

        let random_origin = format!("https://{}/random", rand::random::<f64>());

        let response = request()
            .method("OPTIONS")
            .path(format!("/get?url={example_uri}/test.html").as_str())
            .header("Origin", &random_origin)
            .reply(&all_filters())
            .await;

        assert_eq!(response.status(), 200);

        assert_eq!(
            response.headers()["access-control-allow-origin"],
            &random_origin
        );
        assert_eq!(
            response.headers()["access-control-allow-methods"],
            "OPTIONS, GET, POST, PATCH, PUT, DELETE"
        );

        let body = String::from_utf8(response.body().to_vec()).unwrap();
        let response_body: Value =
            serde_json::from_str(body.as_str()).expect("Failed to parse JSON response");

        assert_eq!(response_body["error"].as_str(), Some("404 Not Found"));
    }

    #[tokio::test]
    async fn test_ignore_other_requests() {
        let response = request()
            .path("/favicon.ico") // Adjust the path as needed
            .reply(&all_filters())
            .await;

        assert_eq!(response.status(), 404);

        let body = String::from_utf8(response.body().to_vec()).unwrap();
        assert_eq!(body.len(), 0);
    }

    #[tokio::test]
    async fn test_support_cache_header() {
        let server = setup().await;
        let example_uri = server.uri();

        let response = request()
            .path(format!("/get?url={example_uri}/test.html").as_str()) // Adjust the path as needed
            .header("Cache-Control", "public, max-age=300")
            .reply(&all_filters())
            .await;

        assert_eq!(response.status(), 200);

        let body = String::from_utf8(response.body().to_vec()).unwrap();

        let response_body: Value =
            serde_json::from_str(body.as_str()).expect("Failed to parse JSON response");

        assert_eq!(response_body["content_length"].as_i64(), Some(15));
        assert_eq!(response_body["content_type"].as_str(), Some("text/example"));
        assert_eq!(response_body["contents"].as_str(), Some("Hi, allOrigins!"));
        assert_eq!(response_body["http_code"].as_i64(), Some(200));
        assert!(response_body["response_time"].as_i64().unwrap() > 0);
        assert_eq!(
            response_body["url"].as_str(),
            Some(format!("{example_uri}/test.html").as_str())
        );
        assert!(response_body["error"].is_null());

        assert_eq!(response.headers()["cache-control"], "public, max-age=300");
    }

    #[tokio::test]
    async fn supplying_charset_should_add_it_to_content_type() {
        let server = setup().await;
        let example_uri = server.uri();

        let response = request()
            .path(format!("/get?url={example_uri}/test.html&charset=UTF-8").as_str())
            .reply(&all_filters())
            .await;

        assert_eq!(response.status(), 200);
        assert_eq!(
            response.headers()[header::CONTENT_TYPE],
            "application/json; charset=UTF-8"
        );
    }

    #[tokio::test]
    async fn not_supplying_url_is_an_error() {
        let _server = setup().await;

        let response = request()
            .path(format!("/get").as_str())
            .reply(&all_filters())
            .await;

        assert_eq!(response.status(), 400);
        assert_eq!(response.headers()[header::CONTENT_TYPE], "text/plain");
        let body = String::from_utf8(response.body().to_vec()).unwrap();
        assert_eq!(body, "No 'url' query parameter");
    }

    async fn setup() -> MockServer {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/test.html"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw("Hi, allOrigins!", "text/example")
                    .set_delay(Duration::from_millis(5)),
            )
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/not-found.html"))
            .respond_with(ResponseTemplate::new(404).set_delay(Duration::from_millis(5)))
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path("/test.html"))
            .respond_with(
                ResponseTemplate::new(201)
                    .set_body_string("Hi, allOrigins! It's a POST!")
                    .set_delay(Duration::from_millis(5)),
            )
            .mount(&server)
            .await;

        Mock::given(method("HEAD"))
            .and(path("/test.html"))
            .respond_with(
                ResponseTemplate::new(204)
                    .set_delay(Duration::from_millis(5))
                    .insert_header(header::CONTENT_TYPE, "text/html")
                    .insert_header(header::CONTENT_LENGTH, "invalid"),
            )
            .mount(&server)
            .await;

        server
    }
}
