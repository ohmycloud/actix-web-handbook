use actix_web::{App, HttpResponse, HttpServer, get, route, web, web::Path};

#[get("/users/{user_id}")]
async fn get_user(path: Path<u32>) -> HttpResponse {
    let user_id = path.into_inner();
    HttpResponse::Ok().body(format!("User ID: {}", user_id))
}

#[get("/posts/{year}/{month}/{day}")]
async fn get_post(path: Path<(u32, u32, u32)>) -> HttpResponse {
    let (year, month, day) = path.into_inner();
    HttpResponse::Ok().body(format!("Post from {}/{}/{}", year, month, day))
}

#[route("/item/{item_id}", method = "POST", method = "PUT")]
async fn update_item(path: Path<u32>) -> HttpResponse {
    let item_id = path.into_inner();
    HttpResponse::Ok().body(format!("Updated item ID: {}", item_id))
}

// Resource-based grouping organizes related endpoints
async fn create_user() -> HttpResponse {
    HttpResponse::Created().body("User created")
}

async fn list_users() -> HttpResponse {
    HttpResponse::Ok().body(format!("List of users"))
}

fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::post().to(create_user))
            .route(web::get().to(list_users)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    println!("Listening on: http://{}", addr);
    HttpServer::new(|| {
        App::new()
            .service(get_user)
            .service(get_post)
            .service(update_item)
            .configure(config_routes)
    })
    .bind(addr)?
    .run()
    .await
}
