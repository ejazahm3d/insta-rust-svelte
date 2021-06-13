#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(serde::Deserialize)]
pub struct SignUpRequest {
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) avatar: Option<String>,
}
