use actix_web::{App, HttpResponse, HttpServer, ResponseError, Result, web};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct LoginParams {
    username: String,
    password: String,
}

// #[get("/login")]
async fn login(params: web::Query<LoginParams>) -> Result<HttpResponse, AppError> {
    if params.username == "admin" && params.password == "password" {
        Ok(HttpResponse::Ok().body("Logged in"))
    } else {
        Err(AppError::InvalidCredentials)
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
            AppError::InvalidCredentials => {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/login", web::get().to(login)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[actix_web::test]
async fn test_login_failure() {
    use actix_web::http::StatusCode;
    use actix_web::test;

    let app = test::init_service(App::new().route("/login", web::get().to(login))).await;
    let req = test::TestRequest::get()
        .uri("/login?username=wrong&password=wrong")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}
