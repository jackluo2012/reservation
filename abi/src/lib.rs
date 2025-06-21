mod pb;

use chrono::{DateTime, Utc};
pub use pb::*;
use prost_types::Timestamp;

pub type ReservationId = String;
pub type UserId = String;
pub type ResourceId = String;
// 日期转换
pub fn convert_to_utc_time(ts: Timestamp) -> DateTime<Utc> {
    DateTime::<Utc>::from_timestamp(ts.seconds, ts.nanos as u32).unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use chrono::TimeZone;
    #[test]
    fn test_convert_to_utc_time() {
        let ts = Timestamp {
            seconds: 1633072800, // 2021-10-01 07:20:00 UTC
            nanos: 0,
        };
        let dt = convert_to_utc_time(ts);
        println!("Converted DateTime: {}", dt);
        assert_eq!(dt, Utc.with_ymd_and_hms(2021, 10, 1, 7, 20, 0).unwrap());
    }
}
