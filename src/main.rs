use actix_web::{HttpResponse, get, web};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(index)
    });

    server.bind("127.0.0.1:8000")?.run().await
}