use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await
        .map_err(|_| "failed to get")?
        .text()
        .await
        .map_err(|_| "failed to convert to text")?;
    Response::ok(body)
}
