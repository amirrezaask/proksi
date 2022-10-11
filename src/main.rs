use std::net::SocketAddr;
use hyper::http::HeaderValue;
use hyper::{Body, Request, Response, Server, Client};
use hyper::service::{make_service_fn, service_fn};
use anyhow::Result;

const UPSTREAM: &'static str = "http://localhost:8080";  
const PROKSI_HEADER: &'static str = "PROKSI_ORIGINAL_HOST";

async fn bridge(mut req: Request<Body>) -> Result<Response<Body>> {
    // Copy host -> PROKSI_ORIGINAL_HOST
    // Set host to UPSTREAM server.
    println!("bridge...");
    let original_host = req.headers().get("HOST").unwrap().clone();
    req.headers_mut().insert(PROKSI_HEADER, original_host);
    req.headers_mut().insert("HOST", HeaderValue::from_str(UPSTREAM).unwrap());
    dbg!(&req);
    Ok(Response::new("Hello, World".into()))
}

async fn upstream(mut req: Request<Body>) -> Result<Response<Body>> {
    // Copy PROKSI_ORIGINAL_HOST -> HOST
    // remove PROKSI_ORIGINAL_HOST
    println!("upstream...");
    dbg!(req.uri());
    Ok(Response::new("Hello, World".into()))
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
