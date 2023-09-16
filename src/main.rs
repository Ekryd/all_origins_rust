mod app_test;
mod get_page;
mod page_types;
mod process_request;
mod server;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting all_origins_rust {VERSION}");
    let (http_server, https_server) = server::start().await;
    tokio::join!(http_server, https_server);
    Ok(())
}
