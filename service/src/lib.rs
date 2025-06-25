mod service;

use abi::{Reservation, reservation_service_server::ReservationServiceServer};
use anyhow::{Error, Ok, Result};
use reservation::ReservationManager;
use std::pin::Pin;
use tonic::{Status, codegen::tokio_stream::Stream, transport::Server};
pub struct RsvpService {
    manager: ReservationManager,
}

impl RsvpService {
    pub async fn from_config() -> Result<Self, Error> {
        Ok(Self {
            manager: ReservationManager::from_env().await?,
        })
    }
}

type ReservationStream = Pin<Box<dyn Stream<Item = Result<Reservation, Status>> + Send + 'static>>;

pub async fn start_server(url: &str) -> Result<(), Error> {
    let svc = RsvpService::from_config().await?;
    let svc = ReservationServiceServer::new(svc);
    let addr = url.parse()?;
    println!("listening on {addr}");
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
