use std::io::stdout;
use std::net::TcpListener;
use std::time::Duration;

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::*;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Set up the `tracing` crate output
    let subscriber = get_subscriber("zero2prod".to_owned(), "info".to_owned(), stdout);
    init_subscriber(subscriber);

    // Load configuration and start the server
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(&address)?;
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(2))
        .connect_lazy(configuration.database.connection_string().expose_secret())
        .expect("Failed to create Postgres connection pool");

    tracing::info!("Running server on `{}`", address);
    run(listener, connection_pool)?.await
}
