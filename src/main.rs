use anyhow::Result;
use hyper::http::HeaderValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, Uri};
use std::net::SocketAddr;
use std::str::FromStr;

const UPSTREAM: &'static str = "http://localhost:8080/upstream";
const PROKSI_HOST_HEADER: &'static str = "PROKSI_ORIGINAL_HOST";
const PROKSI_URI_HEADER: &'static str = "PROKSI_ORIGINAL_URI";

async fn bridge(mut req: Request<Body>) -> Result<Response<Body>> {
    println!("bridge...");
    let original_host = req.headers().get("HOST").unwrap().clone();
    req.headers_mut().insert(PROKSI_HOST_HEADER, original_host);
    let original_uri = req.uri().clone();
    req.headers_mut()
        .insert("HOST", HeaderValue::from_str(UPSTREAM).unwrap());
    req.headers_mut().insert(PROKSI_URI_HEADER, HeaderValue::from_str(&original_uri.to_string()).unwrap());
    *req.uri_mut() = Uri::from_str(UPSTREAM).unwrap();
    dbg!(&req);
    let client = Client::new();
    let resp = client.request(req).await?;
    Ok(resp)
}

async fn upstream(mut req: Request<Body>) -> Result<Response<Body>> {
    // Copy PROKSI_HOST_HEADER -> HOST
    // Set URI from PROKSI_URI_HEADER
    // remove PROKSI_HOST_HEADER and PROKSI_URI_HEADER headers
    let original_host = req.headers().get(PROKSI_HOST_HEADER).unwrap().clone();
    let original_uri = req.headers().get(PROKSI_URI_HEADER).unwrap().clone();
    req.headers_mut().insert("HOST", original_host);
    *req.uri_mut() = Uri::from_str(original_uri.to_str().unwrap()).unwrap();
    req.headers_mut()
        .remove(PROKSI_HOST_HEADER);
    req.headers_mut()
        .remove(PROKSI_URI_HEADER);
    println!("upstream...");
    dbg!(&req);
    let client = Client::new();
    let resp = client.request(req).await?;
    Ok(resp)
}
async fn handler(req: Request<Body>) -> Result<Response<Body>> {
    if req.uri().path().contains("upstream") {
        upstream(req).await
    } else {
        bridge(req).await
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, anyhow::Error>(service_fn(handler)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
