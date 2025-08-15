use actix_web::{App, Error, HttpResponse, HttpServer, post};
use actix_web_validator::Json;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
struct NewUser {
    #[validate(length(min = 4, max = 20))]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
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
