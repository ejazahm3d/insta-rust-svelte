use axum::response::IntoResponse;
use axum_sessions::extractors::WritableSession;

pub async fn logout(mut session: WritableSession) -> impl IntoResponse {
    session.destroy();
    axum::http::StatusCode::OK
}
