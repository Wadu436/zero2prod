use std::io::stdout;
use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;
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

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;
    let connection_pool =
        PgPool::connect(configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to database");
    run(listener, connection_pool)?.await
}
