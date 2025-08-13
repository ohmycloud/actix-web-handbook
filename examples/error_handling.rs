use actix_web::error::ErrorUnauthorized;

use actix_web::{App, HttpResponse, HttpServer, ResponseError, Result, get};
use derive_more::Display;

#[get("/login")]
async fn login(username: String, password: String) -> Result<HttpResponse> {
    if username == "admin" && password == "password" {
        Ok(HttpResponse::Ok().body("Logged in"))
    } else {
        Err(ErrorUnauthorized("Invalid credentials"))
    }
}

#[derive(Debug, Display)]
enum AppError {
    #[display("Database unavailable")]
    DbError,
    #[display("User {} not found", _0)]
    NotFound(u32),
    #[display("Invalid credentials")]
    InvalidCredentials,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DbError => HttpResponse::ServiceUnavailable().finish(),
            AppError::NotFound(id) => {
                HttpResponse::NotFound().body(format!("User {} not found", id))
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(login))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[actix_web::test]
async fn test_login_failure() {
    use actix_web::http::StatusCode;
    use actix_web::test;

    let app = test::init_service(App::new().service(login)).await;
    let req = test::TestRequest::get()
        .uri("/login?username=wrong&passoword=wrong")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}
