use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await
        .map_err(|_| "failed to get")?
        .text()
        .await
        .map_err(|_| "failed to convert to text")?;
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(body.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}