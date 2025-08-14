use actix_files::{Files, NamedFile};
use actix_web::{App, HttpServer, get};

#[get("/favicon.ico")]
async fn favicon() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("assets/images/favicon.ico")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                Files::new("/static", "assets")
                    .prefer_utf8(true)
                    .use_etag(true)
                    .use_last_modified(true)
                    .disable_content_disposition()
                    .show_files_listing(),
            )
            .service(favicon)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
