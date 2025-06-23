use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid start or end time for the reservation")]
    InvalidTime,
    #[error("Database error occurred: {0}")]
    DbError(#[from] sqlx::Error),
    #[error("Invalid user id: {0}")]
    InvalidUserId(String),
    #[error("Invalid sesouce id: {0}")]
    InvalidResourceId(String),
    #[error("Invalid reservation id: {0}")]
    InvalidReservationId(i64),
    #[error("Unknown error occurred")]
    Unknown,
}

// impl From<sqlx::Error> for Error {
//     fn from(e: sqlx::Error) -> Self {
//         match e {
//             sqlx::Error::Database(e) => match e.code() {
//                 Some(code) => Error::InvalidTime,
//                 _ => Error::Unknown,
//             },
//             _ => Error::DbError(e),
//         }
//     }
// }
