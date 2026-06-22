use std::{net::TcpListener, time::Duration};

use sqlx::postgres::PgPoolOptions;
use zero2prod::{
    configurations::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(
        String::from("zero2prod"),
        String::from("info"),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(2))
        .connect_lazy(&configuration.database.connection_string())
        .expect("Failed to connect to db");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(&address)
        .unwrap_or_else(|_| panic!("Failed to bind to address {}", address));

    run(listener, connection)?.await
}
