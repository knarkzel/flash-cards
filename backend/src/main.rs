// https://actix.rs/docs/getting-started/

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started on 0.0.0.0:8000");
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
