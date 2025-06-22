use crate::{ReservationManager, Rsvp};
use abi::{self, ReservationId, error::Error as ReservationError};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use sqlx::postgres::types::PgRange;

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(
        &self,
        mut rsvp: abi::Reservation,
    ) -> Result<abi::Reservation, ReservationError> {
        // 实现预订逻辑
        rsvp.validate()?;
        // 处理时间转换
        let start = abi::convert_to_utc_time(rsvp.start.unwrap());
        let end = abi::convert_to_utc_time(rsvp.end.unwrap());
        if start >= end {
            return Err(ReservationError::InvalidTime);
        }

        let timespan: PgRange<DateTime<Utc>> = (start..end).into();

        let status = abi::ReservationStatus::try_from(rsvp.status)
            .unwrap_or(abi::ReservationStatus::Pending);

        // execute the sql
        rsvp.id = sqlx::query_scalar(
            "INSERT INTO rsvp.reservations (user_id, resource_id, timespan, note, status) VALUES ($1, $2, $3, $4, $5::rsvp.reservation_status) RETURNING id"
        )
        .bind(rsvp.user_id.clone())
        .bind(rsvp.resource_id.clone())
        .bind(timespan)
        .bind(rsvp.note.clone())
        .bind(status.to_string())
        .fetch_one(&self.pool)
        .await?;

        Ok(rsvp)
    }

    async fn change_status(
        &self,
        _id: ReservationId,
    ) -> Result<abi::Reservation, ReservationError> {
        // 实现状态变更逻辑
        unimplemented!()
    }

    async fn update_note(
        &self,
        _id: ReservationId,
        _note: String,
    ) -> Result<abi::Reservation, ReservationError> {
        // 实现更新备注逻辑
        unimplemented!()
    }

    async fn get(&self, _id: ReservationId) -> Result<abi::Reservation, ReservationError> {
        // 实现获取单个预订信息逻辑
        unimplemented!()
    }

    async fn get_all(&self) -> Result<Vec<abi::Reservation>, ReservationError> {
        // 实现获取所有预订信息逻辑
        unimplemented!()
    }

    async fn query(
        &self,
        _query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, ReservationError> {
        // 实现查询指定用户的预订信息逻辑
        unimplemented!()
    }

    async fn cancel(&self, _id: ReservationId) -> Result<abi::Reservation, ReservationError> {
        // 实现取消预订逻辑
        unimplemented!()
    }
}

impl ReservationManager {
    pub fn new(pool: PgPool) -> Self {
        ReservationManager { pool }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use abi::Reservation;

    #[sqlx::test(migrations = "../migrations")]
    async fn reserve_should_work_for_valid_window(pool: PgPool) -> sqlx::Result<()> {
        let manager = ReservationManager::new(pool.clone());
        // let start = "2025-10-01T15:00:00Z".parse::<DateTime<Utc>>().unwrap();
        // let end = "2025-10-08T12:00:00Z".parse::<DateTime<Utc>>().unwrap();

        // let rsvp = abi::Reservation::new_pending(
        //     "jackluo".to_string(),
        //     "ocean-view-room-819".to_string(),
        //     start,
        //     end,
        //     "I'll arrive at 3pm.Please help to upgrade to execuitive room if possible.".to_string(),
        // );
        let rsvp = abi::Reservation::new_pending(
            "jackluo",
            "ocean-view-room-819",
            "2025-10-01T15:00:00Z".parse::<DateTime<Utc>>().unwrap(),
            "2025-10-08T12:00:00Z".parse::<DateTime<Utc>>().unwrap(),
            "I'll arrive at 3pm.Please help to upgrade to execuitive room if possible.".to_string(),
        );
        let rsvp = manager.reserve(rsvp).await.unwrap();

        assert_eq!(rsvp.id, 1); // 假设第一次插入的 ID 是 1
        Ok(())
    }
    #[sqlx::test(migrations = "../migrations")]
    async fn reserve_conflict_reservation_should_reject(pool: PgPool) -> sqlx::Result<()> {
        let manager = ReservationManager::new(pool.clone());
        let rsvp1 = Reservation::new_pending(
            "user1".to_string(),
            "room1".to_string(),
            "2025-10-01T15:00:00Z".parse::<DateTime<Utc>>().unwrap(),
            "2025-10-08T12:00:00Z".parse::<DateTime<Utc>>().unwrap(),
            "hello".to_string(),
        );
        let rsvp2 = Reservation::new_pending(
            "user2".to_string(),
            "room1".to_string(),
            "2025-10-09T16:00:00Z".parse::<DateTime<Utc>>().unwrap(),
            "2025-10-18T12:00:00Z".parse::<DateTime<Utc>>().unwrap(),
            "world".to_string(),
        );
        let _rsvp1 = manager.reserve(rsvp1).await.unwrap();
        let err = manager.reserve(rsvp2).await.unwrap();
        println!("Error: {:?}", err);
        Ok(())
    }
}
