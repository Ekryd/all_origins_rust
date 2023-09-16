use crate::get_page::GetPage;
use tokio::time::Instant;
use warp::http::{header, HeaderValue, Method};
use warp::hyper::Body;
use warp::reply::{json, Json, Response};

pub async fn process_request_info(url: String) -> Json {
    let now = Instant::now();
    println!("info {url}");
    let page = GetPage::new(url);
    let mut content = page.get_page_info().await;
    content.response_time = now.elapsed().as_millis() as u32;
    json(&content)
}

pub async fn process_request_raw(url: String, method: Method) -> Result<Response, Json> {
    let now = Instant::now();
    println!("raw {} {url}", method.as_str());
    let page = GetPage::new(url);
    let mut content = page.get_page(method).await;

    if let Some(ref _error) = content.error {
        content.response_time = now.elapsed().as_millis() as u32;
        return Err(json(&content));
    }

    let mut response = Response::new(Body::from(content.contents.unwrap_or("".to_string())));
    let headers = response.headers_mut();
    if let Some(content_length) = content.content_length {
        headers.insert(header::CONTENT_LENGTH, HeaderValue::from(content_length));
    }
    if let Some(content_type) = content.content_type {
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_str(content_type.as_str()).unwrap(),
        );
    }
    Ok(response)
}

pub async fn process_request_get(url: String, method: Method) -> Json {
    let now = Instant::now();
    println!("get {} {url}", method.as_str());
    let page = GetPage::new(url);
    let mut content = page.get_page(method).await;
    content.response_time = now.elapsed().as_millis() as u32;
    json(&content)
}
