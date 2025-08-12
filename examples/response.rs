use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, get, http::header, web::Bytes,
};
use serde::Serialize;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type(header::ContentType::json())
        .json(User {
            id: 1,
            name: "Larry Wall".to_string(),
        })
}

#[get("/redirect")]
async fn redirect() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "/new-page"))
        .append_header(("Cache-Control", "max-age=3600"))
        .append_header(("X-Custom-Header", "value"))
        .body("Content with headers")
    // .finish()
}

#[get("/download")]
async fn large_file() -> HttpResponse {
    let file = File::open("large_video.mp4").await.unwrap();
    let stream = ReaderStream::new(file);
    HttpResponse::Ok()
        .content_type("video/mp4")
        .streaming(stream)
}

#[get("/fast-response")]
async fn fast_response() -> HttpResponse {
    let data = Bytes::from_static(b"Fast response");
    HttpResponse::Ok()
        .content_type(header::ContentType::plaintext())
        .body(data)
}

#[get("/inspect")]
async fn inspect_request(req: HttpRequest) -> HttpResponse {
    // Access request components
    let method = req.method();
    let path = req.path();
    let headers = req.headers();

    // Log request information
    println!("Method: {}", method);
    println!("Path: {}", path);
    println!("Headers: {:?}", headers);

    // Return empty response
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(inspect_request)
            .service(hello)
            .service(redirect)
            .service(large_file)
            .service(fast_response)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
