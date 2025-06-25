use chrono::{DateTime, Utc};
use prost_types::Timestamp;
// 日期转换
pub fn convert_to_utc_time(ts: Timestamp) -> DateTime<Utc> {
    DateTime::<Utc>::from_timestamp(ts.seconds, ts.nanos as u32).unwrap()
}
// 将UTC时间转换为Timestamp
pub fn convert_to_timestamp(dt: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    }
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
        println!("Converted DateTime: {dt}");
        assert_eq!(dt, Utc.with_ymd_and_hms(2021, 10, 1, 7, 20, 0).unwrap());
    }
    #[test]
    fn test_convert_to_timestamp() {
        let dt = Utc.with_ymd_and_hms(2021, 10, 1, 7, 20, 0).unwrap();
        let ts = convert_to_timestamp(dt);
        println!("Converted Timestamp: {}.{}", ts.seconds, ts.nanos);
        assert_eq!(ts.seconds, 1633072800);
        assert_eq!(ts.nanos, 0);
        let dt = "2021-10-01T07:20:00Z".parse::<DateTime<Utc>>().unwrap();
        let ts = convert_to_timestamp(dt);
        println!("Converted Timestamp: {}.{}", ts.seconds, ts.nanos);
        assert_eq!(ts.seconds, 1633072800);
        assert_eq!(ts.nanos, 0);
    }
}
