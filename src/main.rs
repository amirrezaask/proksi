use std::convert::Infallible;
use std::net::SocketAddr;
use std::ops::Deref;
use hyper::http::HeaderValue;
use hyper::{Body, Request, Response, Server, Client};
use hyper::service::{make_service_fn, service_fn};
use serde::{Deserialize, Serialize};
use anyhow::Result;

// const UPSTREAM: &'static str = "http://localhost:8080";  
async fn bridge(mut req: Request<Body>) -> Result<Response<Body>> {
    println!("bridge...");
    let client = Client::new();
    let actual_host = req.headers().get("HOST").unwrap().clone();
    req.headers_mut().append("PROKSI-REAL-HOST", actual_host.clone());
    dbg!(req.uri());
    req.headers_mut().insert("HOST", HeaderValue::from_str("localhost:8080").unwrap());
    let mut resp = client.request(req).await?;
    Ok(Response::new("Hello, World".into()))
}

async fn upstream(mut req: Request<Body>) -> Result<Response<Body>> {
    println!("upstream...");
    dbg!(req.uri());
    return Ok(Response::new("Hello, World".into()))
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

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, anyhow::Error>(service_fn(handler))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
