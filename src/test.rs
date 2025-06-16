#[path = "main.rs"]
mod main;
#[path = "handlers.rs"]
mod handlers;

use actix_web::{test, web, App};

#[actix_rt::test]
async fn test_index() {
    let mut app = App::new().service(main::index);
    let mut app = test::init_service(app).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_health_check() {
    let mut app =
        App::new().route("/health", web::get().to(handlers::health_check));
    let mut app = test::init_service(app).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, r#"{"status":"ok"}"#);
}

