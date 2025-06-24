use abi::reservation_service_server::ReservationServiceServer;
use anyhow::{Error, Ok, Result};
use reservation_service::RsvpService;
use tonic::transport::Server;
#[tokio::main]
async fn main() -> Result<(), Error> {
    let svc = RsvpService::from_config().await?;
    let svc = ReservationServiceServer::new(svc);
    let addr = format!("{}:{}", "127.0.0.1", "50051").parse()?;
    println!("listening on {}", addr);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
