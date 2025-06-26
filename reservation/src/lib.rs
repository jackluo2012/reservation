mod manager;

use abi::{self, ReservationId, error::Error as ReservationError};
use async_trait::async_trait;
use sqlx::PgPool;
use tokio::sync::mpsc;

// 定义一个struct来表示预
#[derive(Debug, Clone)]
pub struct ReservationManager {
    pool: PgPool,
}

// 定义一个trait来表示预订的行为
#[async_trait]
pub trait Rsvp {
    // make a reservation
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError>;
    // 改变预订的状态 (if current status is pending ,change it to confirmed)
    async fn change_status(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError>;
    //修改预订内容
    async fn update_note(
        &self,
        id: ReservationId,
        note: String,
    ) -> Result<abi::Reservation, ReservationError>;
    // 获取预订信息
    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError>;
    // 获取指定用户的预订信息
    async fn query(
        &self,
        query: abi::ReservationQuery,
    ) -> mpsc::Receiver<Result<abi::Reservation, abi::Error>>;
    // 取消预订
    async fn cancel(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError>;
}
