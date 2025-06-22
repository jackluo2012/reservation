use crate::{
    Error, Reservation, ReservationStatus, convert_to_utc_time, utils::convert_to_timestamp,
};
use chrono::{DateTime, Utc};
use std::ops::Range;

impl Reservation {
    pub fn new_pending(
        uid: impl Into<String>,
        rid: impl Into<String>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            id: 0,
            user_id: uid.into(),
            resource_id: rid.into(),
            start: Some(convert_to_timestamp(start)),
            end: Some(convert_to_timestamp(end)),
            note: note.into(),
            status: ReservationStatus::Pending as i32,
        }
    }
    pub fn validate(&self) -> Result<(), Error> {
        if self.user_id.is_empty() {
            return Err(Error::InvalidUserId(self.user_id.clone()));
        }
        if self.resource_id.is_empty() {
            return Err(Error::InvalidResourceId(self.resource_id.clone()));
        }
        if self.start.is_none() || self.end.is_none() {
            return Err(Error::InvalidTime);
        }
        let start = convert_to_utc_time(self.start.unwrap());
        let end = convert_to_utc_time(self.end.unwrap());
        if start >= end {
            return Err(Error::InvalidTime);
        }

        Ok(())
    }

    pub fn get_timespan(&self) -> Range<DateTime<Utc>> {
        let start = convert_to_utc_time(self.start.unwrap());
        let end = convert_to_utc_time(self.end.unwrap());
        Range { start, end }
    }
}
