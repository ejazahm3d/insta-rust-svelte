use std::net::TcpListener;

use insta::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind port");
    run(address)?.await
}
