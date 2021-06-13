pub mod auth;
pub mod health_check;
pub mod posts;

use actix_web::web::{self, get, post};
pub use health_check::*;

use self::{
    auth::{login, logout, sign_up},
    posts::list,
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health_check", get().to(health_check));

    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("/signup", post().to(sign_up))
                    .route("/login", post().to(login))
                    .route("/logout", post().to(logout)),
            )
            .service(web::scope("/posts").route("", get().to(list))),
    );
}
