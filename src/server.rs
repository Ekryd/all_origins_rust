use std::future::Future;

use reqwest::Method;
use serde::{Deserialize, Serialize};
use warp::http::{header, HeaderMap, HeaderValue};
use warp::hyper::{Body, StatusCode};
use warp::reply::Response;
use warp::{http, Filter, Rejection, Reply};

use crate::process_request::{process_request_get, process_request_info, process_request_raw};

/// The query params accepted by the service
#[derive(Deserialize, Serialize)]
pub struct QueryParams {
    pub url: Option<String>,
    pub charset: Option<String>,
}

/// The path for info
fn info_filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    warp::path("info")
        .and(warp::query::<QueryParams>())
        .and(warp::header::headers_cloned())
        .then(info_handler)
}

async fn info_handler(q: QueryParams, headers: HeaderMap) -> Response {
    if let Some(bad_request_response) = check_empty_url(&q) {
        return bad_request_response;
    }
    let (url, charset) = (q.url.unwrap(), q.charset);

    let mut content = process_request_info(url).await.into_response();
    add_headers(headers, charset, &mut content);

    content
}

/// The path for get (not same as method GET)
fn get_filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    warp::path("get")
        .and(warp::query::<QueryParams>())
        .and(warp::method())
        .and(warp::header::headers_cloned())
        .then(get_handler)
}

async fn get_handler(q: QueryParams, m: Method, headers: HeaderMap) -> Response {
    if let Some(bad_request_response) = check_empty_url(&q) {
        return bad_request_response;
    }
    let (url, charset) = (q.url.unwrap(), q.charset);

    let mut content = process_request_get(url, m).await.into_response();
    add_headers(headers, charset, &mut content);

    content
}

/// The path for raw
fn raw_filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    warp::path("raw")
        .and(warp::query::<QueryParams>())
        .and(warp::method())
        .and(warp::header::headers_cloned())
        .then(raw_handler)
}

async fn raw_handler(q: QueryParams, m: Method, headers: HeaderMap) -> Response {
    if let Some(bad_request_response) = check_empty_url(&q) {
        return bad_request_response;
    }
    let (url, charset) = (q.url.unwrap(), q.charset);

    let response = process_request_raw(url, m).await;
    match response {
        Ok(mut content) => {
            add_headers(headers, charset, &mut content);
            content
        }
        Err(json) => json.into_response(),
    }
}

fn check_empty_url(query_params: &QueryParams) -> Option<Response> {
    match query_params.url {
        None => Some(
            http::response::Builder::new()
                .status(StatusCode::BAD_REQUEST)
                .header(header::CONTENT_TYPE, "text/plain")
                .body(Body::from("No 'url' query parameter"))
                .into_response(),
        ),
        Some(_) => None,
    }
}

fn add_headers(headers: HeaderMap, charset: Option<String>, content: &mut Response) {
    let response_headers = content.headers_mut();
    if let Some(cache_control) = headers.get(header::CACHE_CONTROL) {
        response_headers.insert(header::CACHE_CONTROL, cache_control.clone());
    }
    match headers.get(header::ORIGIN) {
        Some(origin) => {
            response_headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin.clone());
        }
        None => {
            response_headers.insert(
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                HeaderValue::from_static("*"),
            );
        }
    }
    response_headers.insert(
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("true"),
    );
    response_headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("Origin, X-Requested-With, Content-Type, Accept, Cache-Control"),
    );
    response_headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("OPTIONS, GET, POST, PATCH, PUT, DELETE"),
    );
    response_headers.insert(header::VIA, HeaderValue::from_static("all_origins_rust"));
    if let Some(charset) = charset {
        let content_type = response_headers.get(header::CONTENT_TYPE);
        if let Some(content_type) = content_type {
            response_headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_str(
                    format!("{}; charset={}", content_type.to_str().unwrap(), charset).as_str(),
                )
                .unwrap(),
            );
        }
    }
}

pub fn all_filters() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    info_filter().or(get_filter()).or(raw_filter())
}

/// Start the service
pub async fn start() -> (
    impl Future<Output = ()> + Sized,
    impl Future<Output = ()> + Sized,
) {
    const PORT_HTTP: u16 = 38724;
    const PORT_HTTPS: u16 = 38725;

    println!("Listening (http) on {PORT_HTTP}");
    let (_http_addr, http_server) = warp::serve(all_filters()).bind_with_graceful_shutdown(
        ([0, 0, 0, 0], PORT_HTTP),
        async move {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen to shutdown signal");
        },
    );

    println!("Listening (https) on {PORT_HTTPS}");
    let (_https_addr, https_server) = warp::serve(all_filters())
        .tls()
        .cert_path("ssl/cert.pem")
        .key_path("ssl/privkey.pem")
        .bind_with_graceful_shutdown(([0, 0, 0, 0], PORT_HTTPS), async move {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen to shutdown signal");
        });

    (http_server, https_server)
}
