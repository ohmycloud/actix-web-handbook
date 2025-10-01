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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route(
            "/secure",
            web::route().guard(ApiKeyGuard).to(secure_endpoint),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
