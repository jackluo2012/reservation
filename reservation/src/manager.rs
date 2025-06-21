use crate::{ReservationError, ReservationManager, Rsvp};
use abi::{self, ReservationId};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::postgres::types::PgRange;
#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(
        &self,
        mut rsvp: abi::Reservation,
    ) -> Result<abi::Reservation, ReservationError> {
        // 实现预订逻辑
        // 首先检查预订的时间是否有效
        if rsvp.start.is_none() || rsvp.end.is_none() {
            return Err(ReservationError::InvalidTime);
        }

        // 处理时间转换
        let start = abi::convert_to_utc_time(rsvp.start.unwrap());
        let end = abi::convert_to_utc_time(rsvp.end.unwrap());
        if start >= end {
            return Err(ReservationError::InvalidTime);
        }

        let timespan: PgRange<DateTime<Utc>> = (start..end).into();

        // generate a insert sql for the reservation
        // 使用 sqlx 或其他数据库库执行插入操作
        // 这里需要使用数据库连接池 self.pool 来执行 SQL 查询
        // sqlx::query!(
        //     r#"INSERT INTO rsvp.reservations (user_id, resource_id, timespan, note, status) VALUES ($1, $2, $3, $4, $5::rsvp.reservation_status) RETURNING id"#,
        //     rsvp.user_id.clone(),
        //     rsvp.resource_id,
        //     timespan,
        //     status,
        //     rsvp.note
        // );
        // execute the sql
        rsvp.id = sqlx::query_scalar(
            "INSERT INTO rsvp.reservations (user_id, resource_id, timespan, note, status) VALUES ($1, $2, $3, $4, $5::rsvp.reservation_status) RETURNING id"
        )
        .bind(rsvp.user_id.clone())
        .bind(rsvp.resource_id.clone())
        .bind(timespan)
        .bind(rsvp.note.clone())
        .bind(rsvp.status)
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
