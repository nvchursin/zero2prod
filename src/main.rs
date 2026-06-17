use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{configurations::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to db");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(&address)
        .unwrap_or_else(|_| panic!("Failed to bind to address {}", address));

    run(listener, connection)?.await
}
