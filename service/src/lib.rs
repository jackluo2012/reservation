mod service;

use std::pin::Pin;

use abi::Reservation;
use reservation::ReservationManager;
use tonic::{Status, codegen::tokio_stream::Stream};
pub struct RsvpService {
    manager: ReservationManager,
}

impl RsvpService {
    pub async fn from_config() -> Result<Self, anyhow::Error> {
        Ok(Self {
            manager: ReservationManager::from_env().await?,
        })
    }
}

type ReservationStream = Pin<Box<dyn Stream<Item = Result<Reservation, Status>> + Send + 'static>>;
