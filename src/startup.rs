use actix_session::CookieSession;
use actix_web::{
    dev::Server,
    web::{self, post},
    App, HttpServer,
};
use sqlx::PgPool;
use std::net::TcpListener;
use web::get;

use crate::routes::{get_all_users, health_check, login, sign_up};

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .route("/health_check", get().to(health_check))
            .route("/api/users", get().to(get_all_users))
            .route("/api/auth/signup", post().to(sign_up))
            .route("/api/auth/login", post().to(login))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
