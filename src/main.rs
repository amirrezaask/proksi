use actix_web::{route, App, HttpRequest, HttpResponse, HttpServer, Responder};

// fn create_request(_req: HttpRequest, client: reqwest::Client) -> reqwest::Request {
//     client.request(_req.method, )
// }
#[route(
    "/",
    method = "GET",
    method = "POST",
    method = "PUT",
    method = "PATCH",
    method = "DELETE",
    method = "OPTIONS",
    method = "HEAD",
    method = "CONNECT",
    method = "TRACE"
)]

async fn proksi_bridge(_req: HttpRequest) -> impl Responder {
    dbg!(_req);
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(proksi_bridge))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
