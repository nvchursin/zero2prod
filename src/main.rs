use std::fmt::{Debug, Display};
use tokio::task::JoinError;
use zero2prod::{
    configuration::get_configuration,
    issue_delivery_worker::run_worker_until_stopped,
    startup::{Application, get_connection_pool},
    telemetry::{get_subscriber, init_subscriber},
};

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(err)) => {
            tracing::error!(
                error.cause_chain = ?err,
                error.message = %err,
                "{} failed",
                task_name
            )
        }
        Err(err) => {
            tracing::error!(
                error.cause_chain = ?err,
                error.message = %err,
                "{} task failed to complete",
                task_name
            )
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

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

    let application = Application::build(configuration.clone()).await?;
    let application_task = tokio::spawn(application.run_until_stopped());
    let worker_task = tokio::spawn(run_worker_until_stopped(configuration));

    tokio::select! {
        outcome = application_task => report_exit("api", outcome),
        outcome = worker_task => report_exit("background worker", outcome),
    };

    Ok(())
}
