use crate::{ReservationManager, Rsvp};
// use abi::Validate; // Bring the trait with `validate` into scope
use abi::{self, ReservationId, error::Error as ReservationError};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::types::PgRange;

impl ReservationManager {
    pub fn new(pool: PgPool) -> Self {
        ReservationManager { pool }
    }

    pub async fn from_env() -> Result<Self, abi::Error> {
        dotenvy::dotenv().ok();
        let max_connections = std::env::var("MAX_CONNECTIONS")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&std::env::var("DATABASE_URL").unwrap())
            .await?;

        PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;
        Ok(Self::new(pool))
    }
}

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

    async fn change_status(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError> {
        // 实现状态变更逻辑
        // if current status is pending, change to confirmed,otherwise, change to pending
        let rsvp: abi::Reservation = sqlx::query_as(
            "UPDATE rsvp.reservations SET status = 'confirmed' WHERE id = $1 AND status = 'pending' RETURNING *"
        ).bind(id).fetch_one(&self.pool).await?;
        Ok(rsvp)
    }

    async fn update_note(
        &self,
        id: ReservationId,
        note: String,
    ) -> Result<abi::Reservation, ReservationError> {
        // 实现更新备注逻辑

        let rsvp: abi::Reservation =
            sqlx::query_as("UPDATE rsvp.reservations SET note = $1 WHERE id = $2 RETURNING *")
                .bind(note)
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        Ok(rsvp)
    }

    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError> {
        // 实现获取单个预订信息逻辑
        // id.validate()?;
        let rsvp: abi::Reservation =
            sqlx::query_as("SELECT * FROM rsvp.reservations WHERE id = $1")
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        Ok(rsvp)
    }

    async fn query(
        &self,
        query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, ReservationError> {
        // 实现查询指定用户的预订信息逻辑
        let rsvps = sqlx::query_as("SELECT * FROM rsvp.query($1,$2,$3,$4,$5,$6) ")
            .bind(query.user_id)
            .bind(query.resource_id)
            // .bind(query.start)
            // .bind(query.end)
            // .bind(query.page)
            // .bind(query.page_size)
            .fetch_all(&self.pool)
            .await?;
        Ok(rsvps)
    }

    async fn cancel(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError> {
        // 实现取消预订逻辑
        let rsvp: abi::Reservation =
            sqlx::query_as("DELETE FROM rsvp.reservations WHERE id = $1 RETURNING *")
                .bind(id)
                .fetch_one(&self.pool)
                .await?;
        Ok(rsvp)
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
        println!("Error: {err:?}");
        Ok(())
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn reserve_change_status_should_work(pool: PgPool) -> sqlx::Result<()> {
        let manager = ReservationManager::new(pool);
        let changersvp = Reservation::new_pending(
            "user1".to_string(),
            "room1".to_string(),
            "2025-10-09T16:00:00Z".parse::<DateTime<Utc>>().unwrap(),
            "2025-10-18T12:00:00Z".parse::<DateTime<Utc>>().unwrap(),
            "world".to_string(),
        );
        let rsvp = manager.reserve(changersvp).await.unwrap();

        let changersvp = manager.change_status(rsvp.id).await.unwrap();
        assert_eq!(changersvp.status, abi::ReservationStatus::Confirmed as i32);
        Ok(())
    }
    #[tokio::test]
    async fn reservation_manager_should_work() -> Result<(), Box<dyn std::error::Error>> {
        let manager = ReservationManager::from_env().await.unwrap();
        // 清理表，避免冲突
        sqlx::query("DELETE FROM rsvp.reservations")
            .execute(&manager.pool)
            .await
            .unwrap();

        //随机插入10条数据
        for i in 0..10 {
            let rsvp = abi::Reservation::new_pending(
                format!("user{i}"),
                format!("room{i}"),
                "2025-10-01T15:00:00Z".parse::<DateTime<Utc>>().unwrap(),
                "2025-10-08T12:00:00Z".parse::<DateTime<Utc>>().unwrap(),
                "I'll arrive at 3pm.Please help to upgrade to execuitive room if possible."
                    .to_string(),
            );
            manager.reserve(rsvp).await.unwrap();
        }
        Ok(())
    }
}
