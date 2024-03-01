use sqlx::PgPool;
use std::net::TcpListener;
use zero2seisan::configuration::get_configuration;
use zero2seisan::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind port 8000.");
    run(listener, connection)?.await
}
