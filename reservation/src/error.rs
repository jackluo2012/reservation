use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReservationError {
    #[error("Invalid start or end time for the reservation")]
    InvalidTime,
    #[error("Database error occurred: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("Unknown error occurred")]
    Unknown,
}
