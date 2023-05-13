use anyhow::Result;
use spin_sdk::outbound_http::send_request;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[http_component]
fn handle_fermyon_spin(req: Request) -> Result<Response> {
    let request = http::Request::builder()
        .uri("https://www.rust-lang.org")
        .body(None)?;

    let response = send_request(request)?;
    Ok(http::Response::builder()
        .status(200)
        .body(response.into_body())?)
}
