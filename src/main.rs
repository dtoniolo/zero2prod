use std::net::TcpListener;
use sqlx::PgPool;
use env_logger::Env;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration()
        .expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
    ).await.expect("Failed to connect to Postgres.");
    let listener = TcpListener::bind(address)
        .expect("Failed to bind to random port.");
    run(listener, connection_pool)?.await
}
