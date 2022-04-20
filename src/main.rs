#[macro_use]
extern crate actix_web;
use actix_web::{App, HttpServer, web};

// modules
mod handler;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .service()
                    // add service handler here .service(module::function)
            )
    })
    .keep_alive(15)
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
