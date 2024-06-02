//! axum server for cente
/// apis: v1/task_create, v1/main/modules, v1/main/alarm, v1/history/conter, v1/history/list
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

#[allow(unused)]
fn app() -> Router {
    Router::new()
        .nest(
            "/v1",
            Router::new()
                .nest(
                    "/main",
                    Router::new()
                        .route("/modules", get(api_root_handler))
                        .route("/alarm", get(api_root_handler)),
                )
                .nest(
                    "/history",
                    Router::new()
                        .route("/counter", get(api_root_handler))
                        .route("/list", get(api_root_handler)),
                )
                .route("/task_create", get(api_root_handler)),
        )
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}
#[allow(unused)]
async fn api_server_start() {
    let addr = "0.0.0.0:9305";
    let app = app();
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[allow(unused)]
async fn api_root_handler() -> String {
    "Hello, World!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_root_handler() {
        let response = api_root_handler().await;
        assert_eq!(response, "Hello, World!".to_string());
    }
}
