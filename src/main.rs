use actix_web::{HttpResponse, get, web};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let bind_addr = format!("{}:{}", host, port);

    let server = actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(index)
    });

    server.bind(bind_addr)?.run().await
}
