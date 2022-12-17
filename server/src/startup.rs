use axum::{
    routing::{delete, get, get_service, patch, post, IntoMakeService},
    Router, Server,
};
use axum_sessions::{async_session::MemoryStore, SessionLayer};
use hyper::server::conn::AddrIncoming;
use rand::Rng;
use sqlx::PgPool;
use std::{io, net::TcpListener};
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::routes::{auth, comments, followers_leaders, posts, profiles};

pub type App = Server<AddrIncoming, IntoMakeService<Router>>;

pub fn run(listener: TcpListener, pool: PgPool) -> hyper::Result<App> {
    let store = MemoryStore::new();
    let mut rng = rand::thread_rng();
    let secret: [u8; 128] = rng.gen();
    let session_layer = SessionLayer::new(store, &secret).with_secure(false);

    let app = Router::new()
        .route("/health_check", get(crate::routes::health_check))
        .nest_service(
            "/tmp",
            get_service(ServeDir::new("tmp")).handle_error(|error: io::Error| async move {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )
        .nest(
            "/api",
            Router::new()
                .nest(
                    "/auth",
                    Router::new()
                        .route("/signup", post(auth::sign_up))
                        .route("/login", post(auth::login))
                        .route("/logout", post(auth::logout))
                        .route("/current", get(auth::current_user)),
                )
                .nest(
                    "/posts",
                    Router::new()
                        .route("/", get(posts::list_all_posts).post(posts::create_post))
                        .route("/upload", post(posts::upload_post_photo))
                        .nest(
                            "/:post_id",
                            Router::new()
                                .route("/", get(posts::post_details).delete(posts::delete_post))
                                .route("/hasLiked", get(posts::has_liked))
                                .route("/likes", get(posts::likes_by_post))
                                .route("/likes", post(posts::like_or_dislike_post))
                                .route("/comments", get(comments::post_comments))
                                .route("/comments", post(comments::create_comment))
                                .route("/comments/:comment_id", delete(comments::delete_comment))
                                .route("/comments/:comment_id", patch(comments::update_comment))
                                .route(
                                    "/comments/:comment_id/likes",
                                    get(comments::likes_by_comment),
                                )
                                .route("/comments/:comment_id/hasLiked", get(comments::has_liked))
                                .route(
                                    "/comments/:comment_id/likes",
                                    post(comments::like_or_dislike_comment),
                                ),
                        ),
                )
                .nest(
                    "/followers",
                    Router::new()
                        .route("/:leader_id", get(followers_leaders::followers))
                        .route("/:leader_id/count", get(followers_leaders::followers_count)),
                )
                .nest(
                    "/leaders",
                    Router::new()
                        .route(
                            "/:id",
                            get(followers_leaders::leaders)
                                .post(followers_leaders::follow_or_unfollow),
                        )
                        .route("/:id/count", get(followers_leaders::leaders_count))
                        .route("/:id/isFollowing", get(followers_leaders::is_following)),
                )
                .nest(
                    "/profiles",
                    Router::new()
                        .route("/:user_id", get(profiles::profile_details))
                        .route("/:user_id/posts", get(profiles::profile_posts)),
                ),
        )
        .with_state(pool)
        .layer(session_layer)
        .layer(
            CorsLayer::default()
                .allow_credentials(true)
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::PATCH,
                    axum::http::Method::DELETE,
                ])
                .allow_origin(["http://localhost:5173".parse().unwrap()])
                .allow_headers([axum::http::header::ACCEPT, axum::http::header::CONTENT_TYPE]),
        );

    Ok(Server::from_tcp(listener)?.serve(app.into_make_service()))
}
