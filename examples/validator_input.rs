use actix_web::{App, Error, HttpResponse, HttpServer, post};
use actix_web_validator::Json;
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
struct NewUser {
    #[validate(length(min = 4, max = 20))]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8), custom(function = "validate_password_strength"))]
    password: String,
}

fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    let has_upper = password.chars().any(char::is_uppercase);
    let has_lower = password.chars().any(char::is_lowercase);
    let has_digit = password.chars().any(char::is_numeric);

    if !(has_upper && has_lower && has_digit) {
        return Err(ValidationError::new("password is too weak!"));
    }
    Ok(())
}

#[post("/register")]
async fn register(user: Json<NewUser>) -> Result<HttpResponse, Error> {
    // Save valid user data
    Ok(HttpResponse::Created().body(format!("User: {:?} created", user.into_inner())))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    println!("Listening on: http://{}", addr);
    HttpServer::new(|| App::new().service(register))
        .bind(addr)?
        .run()
        .await
}
