//! axum server for cente
/// apis: v1/task_create, v1/main/modules, v1/main/alarm, v1/history/conter, v1/history/list
use axum::{
    http::HeaderValue,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct RResponse {
    status: RResponseStatus,
    msg: String,
    data: String, // json string
}

#[derive(Debug, Serialize, Deserialize)]
enum RResponseStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    #[allow(unused)]
    Error,
    #[serde(rename = "fail")]
    #[allow(unused)]
    Fail,
}

impl IntoResponse for RResponse {
    fn into_response(self) -> Response {
        let json_data = serde_json::to_string(&self).unwrap();
        let mut res = Response::new(json_data.into());
        res.headers_mut()
            .insert("content-type", HeaderValue::from_static("application/json"));
        res
    }
}

#[allow(unused)]
fn response_with_smd(status: RResponseStatus, msg: String, data: String) -> Response {
    RResponse { status, msg, data }.into_response()
}

#[allow(unused)]
fn response_with_sm(status: RResponseStatus, msg: String) -> Response {
    response_with_smd(status, msg, "".to_string())
}

#[allow(unused)]
fn response_with_s(status: RResponseStatus) -> Response {
    response_with_sm(status, "".to_string())
}

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
        .route("/", get(api_root_handler))
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
async fn api_root_handler() -> Response {
    response_with_s(RResponseStatus::Success)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_root_handler() {
        let res = api_root_handler().await;
        assert_eq!(res.status(), 200);
    }
}
