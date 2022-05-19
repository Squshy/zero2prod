use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we cannot read config
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!("localhost:{}", configuration.application_port);
    let listener = TcpListener::bind(&address)?;
    run(listener)?.await
}
