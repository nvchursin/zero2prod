use zero2prod::{
    configuration::get_configuration,
    startup::{Application, get_connection_pool},
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
    let pool = get_connection_pool(&configuration.database);

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    let application = Application::build(configuration).await?;

    application.run_until_stopped().await?;

    Ok(())
}
