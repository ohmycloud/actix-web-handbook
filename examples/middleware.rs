use std::{
    env,
    future::{Future, Ready, ready},
    io::Write,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use actix_web::{
    App, Error, HttpServer,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header::HeaderName,
    middleware::Logger,
    web,
};
use uuid::Uuid;

pub struct TimingMiddleware;

impl<S, B> Transform<S, ServiceRequest> for TimingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TimingMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TimingMiddlewareService { service }))
    }
}

pub struct TimingMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for TimingMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = std::time::Instant::now();
        std::thread::sleep(Duration::from_micros(1000));

        // Process request through next service
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?; // Wait for handler
            let elapsed = start.elapsed();
            println!("Request took {}ms", elapsed.as_millis());
            Ok(res)
        })
    }

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Serving on http://127.0.0.1:8080");

    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}",
                serde_json::json!({
                    "time": chrono::Utc::now().to_rfc3339(),
                    "level": record.level().as_str(),
                    "message": record.args().to_string(),
                    "module": record.module_path().unwrap_or_default()
                })
            )
        })
        .parse_env(env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();

    HttpServer::new(|| {
        App::new()
            .wrap_fn(|mut req, srv| {
                let request_id = Uuid::new_v4().to_string();
                req.headers_mut().insert(
                    HeaderName::from_static("x-request-id"),
                    request_id.parse().unwrap(),
                );
                srv.call(req)
            })
            .wrap(Logger::new("%a %{User-Agent}i %r %s %b %{x-request-id}i")) // adds logging middleware
            .wrap(TimingMiddleware) // custom middleware
            .route("/", web::get().to(|| async { "Hello, Timed" }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
