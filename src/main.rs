use newsletter::configuration::get_configuration;
use newsletter::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    
    println!("Sever is listening on port {}", configuration.application_port);
    run(listener)?.await
}
