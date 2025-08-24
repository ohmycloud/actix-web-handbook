use actix_web::{
    App, Error, HttpResponse, HttpServer,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::StatusCode,
    middleware::{ErrorHandlerResponse, ErrorHandlers, Next, from_fn},
    web,
};
use log::error;
use std::time::Instant;

async fn timing_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let start = Instant::now();
    let res = next.call(req).await?;
    let duration = start.elapsed();

    println!("Request took: {:?}", duration);
    Ok(res)
}

fn handle_internal_error<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, Error> {
    error!("Internal server error occurred");

    // split service response into request and response components
    let (req, res) = res.into_parts();
    // set body of response to modified body
    let res = res.set_body("Internal server error");
    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}

fn handle_bad_request<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, Error> {
    error!("Bad request error occurred");

    // split service response into request and response components
    let (req, res) = res.into_parts();
    // set body of response to modified body
    let res = res.set_body("Bad request");
    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}

// 404 错误处理器
fn handle_not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, Error> {
    error!("404 Not Found");

    // split service response into request and response components
    let (req, _) = res.into_parts();
    // set body of response to modified body
    let res = HttpResponse::NotFound().json(serde_json::json!({
        "error": "Not found",
        "message": "The requested resource was not found"
    }));
    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Serving on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(from_fn(timing_middleware)) // timing middleware
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::INTERNAL_SERVER_ERROR, handle_internal_error)
                    .handler(StatusCode::BAD_REQUEST, handle_bad_request)
                    .handler(StatusCode::NOT_FOUND, handle_not_found),
            )
            .route("/", web::get().to(|| async { "Hello, Timed" }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
