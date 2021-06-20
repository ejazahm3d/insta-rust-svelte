use actix_cors::Cors;
use actix_session::CookieSession;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::configure_routes;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
                    .supports_credentials(),
            )
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .configure(configure_routes)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
