use std::{
    future::{Future, Ready, ready},
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use actix_web::{
    App, Error, HttpServer,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    middleware::Logger,
    web,
};

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
    unsafe {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // adds logging middleware
            .wrap(TimingMiddleware) // custom middleware
            .route("/", web::get().to(|| async { "Hello, Timed" }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
