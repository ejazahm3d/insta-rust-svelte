use axum::response::IntoResponse;

pub async fn health_check() -> impl IntoResponse {
    axum::http::StatusCode::OK
}
