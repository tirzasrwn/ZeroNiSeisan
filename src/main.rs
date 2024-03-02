use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2seisan::configuration::get_configuration;
use zero2seisan::startup::run;
use zero2seisan::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2seisan".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect(&configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind port 8000.");
    run(listener, connection_pool)?.await
}
