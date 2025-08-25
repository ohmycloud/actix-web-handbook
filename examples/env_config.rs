use std::sync::Arc;

use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    web::{self},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
struct AppConfig {
    #[serde(default = "default_port")]
    app_port: u16,
    db_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    api_key: Option<String>,
}

fn default_port() -> u16 {
    3000 // Fallback value
}

impl AppConfig {
    fn from_env() -> Result<Self, envy::Error> {
        envy::from_env()
    }
}

async fn app_config_check(config: web::Data<Arc<AppConfig>>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!(config.as_ref().as_ref()))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let config = Arc::new(AppConfig::from_env().expect("Failed to load configuration"));
    let port = config.app_port;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/", web::get().to(app_config_check))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
