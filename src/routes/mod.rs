pub mod auth;
pub mod health_check;
pub mod posts;

use actix_web::web::{self, get, post};
pub use health_check::*;

use self::{
    auth::{current_user, login, logout, sign_up},
    posts::{create_post, list_all_posts, post_comments, post_details},
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health_check", get().to(health_check));

    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("/signup", post().to(sign_up))
                    .route("/login", post().to(login))
                    .route("/logout", post().to(logout))
                    .route("/current", get().to(current_user)),
            )
            .service(
                web::scope("/posts")
                    .route("", get().to(list_all_posts))
                    .route("", post().to(create_post))
                    .route("/{post_id}", get().to(post_details))
                    .route("/{post_id}/comments", get().to(post_comments)),
            ),
    );
}
