pub mod auth;
pub mod comments;
pub mod health_check;
pub mod posts;
pub mod profiles;

use actix_files as fs;
use actix_web::web::{self, delete, get, patch, post};
pub use health_check::*;

use self::{
    auth::{current_user, login, logout, sign_up},
    comments::{
        create_comment, delete_comment, like_or_dislike_comment, likes_by_comment, post_comments,
        update_comment,
    },
    posts::{
        create_post, delete_post, like_or_dislike_post, likes_by_post, list_all_posts,
        post_details, upload_post_photo,
    },
    profiles::me,
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health_check", get().to(health_check));

    cfg.service(fs::Files::new("/tmp", "./tmp/").show_files_listing());

    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("/signup", post().to(sign_up))
                    .route("/login", post().to(login))
                    .route("/logout", post().to(logout))
                    .route("/current", get().to(current_user)),
            )
            .service(web::scope("/profiles").route("/me", get().to(me)))
            .service(
                web::scope("/posts")
                    .route("", get().to(list_all_posts))
                    .route("", post().to(create_post))
                    .route("/upload", post().to(upload_post_photo))
                    .route("/{post_id}", get().to(post_details))
                    .route("/{post_id}", delete().to(delete_post))
                    .route("/{post_id}/likes", get().to(likes_by_post))
                    .route("/{post_id}/likes", post().to(like_or_dislike_post))
                    .route("/{post_id}/comments", get().to(post_comments))
                    .route("/{post_id}/comments", post().to(create_comment))
                    .route(
                        "/{post_id}/comments/{comment_id}",
                        delete().to(delete_comment),
                    )
                    .route(
                        "/{post_id}/comments/{comment_id}",
                        patch().to(update_comment),
                    )
                    .route(
                        "/{post_id}/comments/{comment_id}/likes",
                        get().to(likes_by_comment),
                    )
                    .route(
                        "/{post_id}/comments/{comment_id}/likes",
                        post().to(like_or_dislike_comment),
                    ),
            ),
    );
}
