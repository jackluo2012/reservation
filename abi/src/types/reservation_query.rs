use crate::{Error, ReservationQuery, ReservationStatus, Validator};

impl ReservationQuery {
    pub fn get_status(&self) -> ReservationStatus {
        ReservationStatus::try_from(self.status).unwrap()
    }
}

impl Validator for ReservationQuery {
    fn validate(&self) -> Result<(), Error> {
        // ReservationStatus::try_from(self.status);

        if let (Some(start), Some(end)) = (self.start.as_ref(), self.end.as_ref())
            && start.seconds >= end.seconds
        {
            return Err(Error::InvalidTime);
        }

        Ok(())
    }
}
