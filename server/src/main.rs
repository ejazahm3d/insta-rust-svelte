use std::net::TcpListener;

use insta::{configuration::get_configuration, startup::run};
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");

    let listener = TcpListener::bind(address).expect("Failed to bind port");
    run(listener, connection_pool)?.await
}
