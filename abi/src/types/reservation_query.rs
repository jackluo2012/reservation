use prost_types::Timestamp;

use crate::{Error, ReservationQuery, ReservationStatus, ToSql, Validator, convert_to_utc_time};

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

impl ToSql for ReservationQuery {
    fn to_sql(&self) -> String {
        let status = self.get_status();

        let timespan = format!(
            "tstzrange('{}', '{}')",
            get_time_string(self.start.as_ref(), true),
            get_time_string(self.end.as_ref(), false)
        );

        let condition = match (self.user_id.is_empty(), self.resource_id.is_empty()) {
            (true, true) => "TRUE".into(),
            (true, false) => format!("resource_id = '{}'", self.resource_id),
            (false, true) => format!("user_id = '{}'", self.user_id),
            (false, false) => format!(
                "user_id = '{}' AND resource_id = '{}'",
                self.user_id, self.resource_id
            ),
        };

        let direction = if self.desc { "DESC" } else { "ASC" };

        format!(
            "SELECT * FROM rsvp.reservations WHERE {timespan} @> timespan AND status = '{status}'::rsvp.reservation_status AND {condition} ORDER BY lower(timespan) {direction}"
        )
    }
}
fn get_time_string(ts: Option<&Timestamp>, start: bool) -> String {
    match ts {
        Some(ts) => convert_to_utc_time(*ts).to_rfc3339(),
        None => (if start { "-infinity" } else { "infinity" }).into(),
    }
}
