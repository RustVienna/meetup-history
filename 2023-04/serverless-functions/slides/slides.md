---
layout: cover
background: ferris.svg
---

# Serverless functions
## Welcome @ Rust meetup

---
layout: two-cols
---

# About me

<v-clicks>

Name: Patrick Lamprecht 

Nickname: Lampe

Hobbies:
- Via Ferrata fan

- Loves cats

<img src="cat.png" class="h-60 rounded shadow" />

</v-clicks>

::right::

<v-clicks>

- Likes low level languages like Rust

- Pizza lover / Hobby pizza maker

<img src="pizza.png" class="h-50 rounded shadow" />

- Working at

<img src="george.png" class="h-30 rounded shadow" />

- Age at *George*: ~4 years

</v-clicks>

---

# What is serverless?

<v-clicks>

- Not `No servers`

- > Serverless computing is a method of providing backend services on an as-used basis. Servers are still used, but a company that gets backend services from a serverless vendor is charged based on usage, not a fixed amount of bandwidth or number of servers. - Cloudflare

- Summary: Pay what you use and don't care about servers

</v-clicks>

---

# Free stuff

<v-clicks>

- Services that are freely available
- Good for hobby projects to experiment
- (some) can also be used in production later

</v-clicks>

---
layout: cover
background: demo-gods.png
---

---
layout: fact
---

# Live demos

---

# Knative

https://knative.dev/docs/install/yaml-install/serving/install-serving-with-yaml/

```shell
docker build -t dev.local/knative-serving .
```

```shell
k apply -f service.yaml
```

```shell
k get ksvc
```

---

# AWS Lambda

```shell
brew tap cargo-lambda/cargo-lambda
brew install cargo-lambda
```

```shell
cargo lambda new aws-lambda
# Is this function an HTTP function? Yes
# Which service is this function receiving events from? AWS Lambda function URLs
```

```shell
cargo add reqwest --no-default-features --features rustls-tls
```

```shell
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
```

```shell
cargo lambda watch
```

---

# Fermyon Spin/Cloud

https://cloud.fermyon.com/

```shell
spin new http-rust
```

```shell
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
```

`spin.toml`
```toml
allowed_http_hosts = [ "https://www.rust-lang.org" ]
```

```shell
spin watch
```

```shell
spin login
```

```shell
spin deploy
```

---

# Cloudflare Worker

https://github.com/cloudflare/workers-sdk

```shell
brew install cloudflare-wrangler2
```

Check that you have version 2 of `wrangler`

```shell
wrangler generate cloudflare-worker cloudflare/workers-sdk/templates/experimental/worker-rust
```

Templates can be found here: https://github.com/cloudflare/workers-sdk/tree/main/templates

```shell
cargo add reqwest
```

```rust
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
```

```shell
wrangler dev
```

```shell
wrangler login
```

```shell
wrangler publish
```

```shell
http https://worker-rust.lampe.workers.dev
```

```shell
oha https://worker-rust.lampe.workers.dev -n 2000
```

---
background: iceberg.png
---

# Just the tip of the ice berg

<v-clicks>

- Vercel
- Netlify
- Google Cloud Functions
- and many more...
- For more inspiration see http://freestuff.dev

</v-clicks>

---
layout: center
---

# Summary

---

# Knative
## Benefits

<v-clicks>

- Runs on any Kubernetes
- Only one small kubernetes yaml
- Just needs a docker image (can run anything)
- Shuts down pods when not used

</v-clicks>


## Drawbacks

<v-clicks>

- Needs warmup (>1 second)
- Long startup with function calling another
- Most complex setup compared to others
- Dockerfile

</v-clicks>

---

# AWS Lambda
## Benefits

<v-clicks>

- ARM instances -> cost reduction
- Fast startup (about half a second, with latency)
- Easy deployment
- Function URLS (no api gateway needed anymore)
- No Dockerfile
- Whole AWS ecosystem (monitoring, logging,...)

</v-clicks>

## Drawbacks

<v-clicks>

- IAM problems (can't use admin token for cargo-lambda, still valid?)

</v-clicks>

---

# Fermyon spin/cloud
## Benefits

<v-clicks>

- Interesting technology (WASI)
- Fast startup
- Easy setup and deployment
- No Dockerfile

</v-clicks>

## Drawbacks

<v-clicks>

- WASI is relatively young
- Many crates miss WASI support
- WASI has no standard for sockets till now (wasi-sockets)
- One thread (wasi-threads)

</v-clicks>

---

# Cloudflare Worker
## Benefits

<v-clicks>

- Uses WASM
- WASM support on crates.io is good (eg reqwest)
- Offer key/value store and pub/sub
- Very easy setup with prompt to login in browser
- URL per function

</v-clicks>

## Drawbacks

<v-clicks>

- Sending mails is problematic (openssl has no wasm/wasi support)

</v-clicks>

---
layout: fact
---

# Serverless != no server

You just don't care about the servers anymore ;)

---
layout: center
---

## Thank you very much for listening to me
## when I talk about my hobbies!

---
layout: end
---