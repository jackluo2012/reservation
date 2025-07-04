pub mod error;
mod pager;
mod pb;
pub mod types;
mod utils;
pub use error::Error;
pub use pb::*;
pub use utils::*;

pub type ReservationId = i64;
pub type UserId = String;
pub type ResourceId = String;

/// validate the data structure, raise error if invalid
pub trait Validator {
    fn validate(&self) -> Result<(), Error>;
}
/// validate the data structure, raise error if invalid
pub trait Normalizer: Validator {
    /// caller should call normalize to make sure the data structure is ready to use
    fn normalize(&mut self) -> Result<(), Error> {
        self.validate()?;
        self.do_normalize();
        Ok(())
    }

    /// user shall implement do_normalize() to normalize the data structure
    fn do_normalize(&mut self);
}

pub trait ToSql {
    fn to_sql(&self) -> String;
}

impl Validator for ReservationId {
    fn validate(&self) -> Result<(), Error> {
        if *self <= 0 {
            Err(Error::InvalidReservationId(*self))
        } else {
            Ok(())
        }
    }
}

/// database equivalent of the "reservation_status" enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "rsvp.reservation_status", rename_all = "lowercase")]
pub enum RsvpStatus {
    Unknown,
    Pending,
    Confirmed,
    Blocked,
}
