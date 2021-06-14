use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub(crate) id: Uuid,
    pub(crate) email: String,
    pub(crate) username: String,
}

#[derive(serde::Deserialize)]
pub struct SignUpRequest {
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) avatar: Option<String>,
}

#[derive(serde::Serialize)]
pub struct SignUpResponse {
    pub(crate) id: Uuid,
    pub(crate) email: String,
    pub(crate) username: String,
}

#[derive(serde::Serialize)]
pub struct CurrentUser {
    pub(crate) id: String,
}

#[derive(serde::Serialize)]
pub struct CurrentUserResponse {
    pub(crate) user: Option<CurrentUser>,
}
