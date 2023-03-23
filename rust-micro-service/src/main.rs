use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    std::thread::sleep(std::time::Duration::from_millis(500)); // Simulate some processing time
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind("0.0.0.0:80")?
        .run()
        .await
}
