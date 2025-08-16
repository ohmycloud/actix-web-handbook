#![feature(str_as_str)]

use actix_web::{App, HttpResponse, HttpServer, post, web};
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
    #[validate(must_match(other = "password"))]
    password_confirmation: String,
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

async fn validate_data<T: Validate>(data: &T) -> Result<(), actix_web::Error> {
    if let Err(errors) = data.validate() {
        let mut error_details = Vec::new();
        for (field, errors) in errors.field_errors() {
            let messages: Vec<&str> = errors
                .iter()
                .map(|e| {
                    let m = e.message.as_ref().unwrap();
                    m.as_str()
                })
                .collect();
            error_details.push(format!("{}: {}", field, messages.join(", ")));
        }
        let error_message = format!("Validation errors: {}", error_details.join("; "));
        return Err(actix_web::error::ErrorBadRequest(error_message));
    }
    Ok(())
}

#[post("/register")]
async fn register(user: Json<NewUser>) -> Result<HttpResponse, actix_web::Error> {
    let user = user.into_inner();
    validate_data(&user).await?;

    // Save valid user data
    Ok(HttpResponse::Created().body(format!("User: {:?} created", user)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    println!("Listening on: http://{}", addr);
    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Condition::new(
                true, // Enable in production
                actix_web::middleware::DefaultHeaders::new()
                    .add(("Content-Type", "application/json")),
            ))
            .app_data(web::JsonConfig::default().limit(4096)) // 4kb payload limit
            .service(register)
    })
    .bind(addr)?
    .run()
    .await
}
