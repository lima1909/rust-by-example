use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}\n", info.1, info.0)
}

// video on youtube: https://www.youtube.com/channel/UCSkHbGjrjJmuAbDPhIQ5T0A/videos

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    println!("starting server on: {}", addr);

    HttpServer::new(|| App::new().service(index))
        .bind(addr)?
        .run()
        .await
}
