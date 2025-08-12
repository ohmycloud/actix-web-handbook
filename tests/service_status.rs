use actix_web::{App, get, test, web};
use serde_json::json;

#[get("/status")]
async fn serivce_status() -> web::Json<serde_json::Value> {
    web::Json(json!({"status": "operational"}))
}

#[actix_web::test]
async fn test_status_handler() {
    let app = test::init_service(App::new().service(serivce_status)).await;
    let req = test::TestRequest::get().uri("/status").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "operational");
}
