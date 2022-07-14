use actix_web::{get, web, App, HttpServer, Responder};
use rustweb::call_url;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/call").route(web::post().to(call_url)))
    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await
}