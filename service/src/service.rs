use crate::{ReservationStream, RsvpService};
use abi::{
    CancelRequest, CancelResponse, ConfirmRequest, ConfirmResponse, FilterRequest, FilterResponse,
    GetRequest, GetResponse, ListenRequest, QueryRequest, ReserveRequest, ReserveResponse,
    UpdateRequest, UpdateResponse, reservation_service_server::ReservationService,
};

use reservation::Rsvp;
use tonic::{self, Request, Response, Status, async_trait};

//解引用 获取ReservationManager

#[async_trait]
impl ReservationService for RsvpService {
    /// make a reservation
    async fn reserve(
        &self,
        request: Request<ReserveRequest>,
    ) -> Result<Response<ReserveResponse>, Status> {
        let request = request.into_inner();
        if request.reservation.is_none() {
            return Err(Status::invalid_argument("reservation is empty"));
        }
        let reservation = self
            .manager
            .reserve(request.reservation.unwrap())
            .await
            .unwrap();
        Ok(Response::new(ReserveResponse {
            reservation: Some(reservation),
        }))
    }
    /// confirm a pending reservation, if reservation is not pending, do nothing
    async fn confirm(
        &self,
        request: Request<ConfirmRequest>,
    ) -> Result<Response<ConfirmResponse>, Status> {
        let request = request.into_inner();
        let reservation = self.manager.change_status(request.id).await.unwrap();
        Ok(Response::new(ConfirmResponse {
            reservation: Some(reservation),
        }))
    }
    /// update the reservation note
    async fn update(
        &self,
        request: Request<UpdateRequest>,
    ) -> Result<Response<UpdateResponse>, Status> {
        let request = request.into_inner();
        let reservation = self
            .manager
            .update_note(request.id, request.note)
            .await
            .unwrap();
        Ok(Response::new(UpdateResponse {
            reservation: Some(reservation),
        }))
    }
    /// cancel a reservation
    async fn cancel(
        &self,
        request: Request<CancelRequest>,
    ) -> Result<Response<CancelResponse>, Status> {
        let request = request.into_inner();
        let reservation = self.manager.cancel(request.id).await.unwrap();
        Ok(Response::new(CancelResponse {
            reservation: Some(reservation),
        }))
    }
    /// get a reservation by id
    async fn get(&self, _request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        todo!()
    }
    /// Server streaming response type for the query method.
    type queryStream = ReservationStream;
    /// query reservations by resource id, user id, status, start time, end time
    async fn query(
        &self,
        _request: Request<QueryRequest>,
    ) -> Result<Response<Self::queryStream>, Status> {
        todo!()
    }
    /// filter reservations, order by reservation id
    async fn filter(
        &self,
        _request: Request<FilterRequest>,
    ) -> Result<Response<FilterResponse>, Status> {
        todo!()
    }

    /// Server streaming response type for the listen method.
    type listenStream = ReservationStream;
    /// another system could monitor newly added/confirmed/cancelled reservations
    async fn listen(
        &self,
        _request: Request<ListenRequest>,
    ) -> Result<Response<Self::listenStream>, Status> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn rpc_reserve_should_work() {
        // let service = RsvpService::from_config().await.unwrap();
        // let reservation = Rsvp::new(1, 1, "".to_string(), "".to_string(), "".to_string());
    }
}
