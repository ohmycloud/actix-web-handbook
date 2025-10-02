use actix_web::{App, HttpResponse, HttpServer, guard::Guard, web};

struct ApiKeyGuard;

impl Guard for ApiKeyGuard {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        ctx.head()
            .headers()
            .get("X-API-Key")
            .and_then(|key| key.to_str().ok())
            .map(|key| key == "secret-key")
            .unwrap_or(false)
    }
}

async fn secure_endpoint() -> HttpResponse {
    HttpResponse::Ok().body("Authorized")
}

async fn get_users() -> HttpResponse {
    HttpResponse::Ok().body("User list")
}

async fn create_user() -> HttpResponse {
    HttpResponse::Ok().body("User created")
}

async fn admin() -> HttpResponse {
    HttpResponse::Ok().body("Admin endpoint")
}

async fn dashboard() -> HttpResponse {
    HttpResponse::Ok().body("Dashboard endpoint")
}

async fn reports() -> HttpResponse {
    HttpResponse::Ok().body("Reports endpoint")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            // Group routes under /users
            .service(
                web::scope("/users")
                    .route("", web::get().to(get_users)) // Matches /users
                    .route("", web::post().to(create_user)), // Matches /users
            )
            // Admin routes guarded by ApiKeyGuard
            .service(
                web::scope("/admin")
                    .guard(ApiKeyGuard)
                    .route("/dashboard", web::get().to(dashboard))
                    .route("/reports", web::get().to(reports)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
