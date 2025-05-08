use async_trait::async_trait;
use chrono::NaiveDateTime;
use entities::court::{Court, CourtReservation};
use uuid::Uuid;

use super::err::Result;

#[async_trait]
pub trait CourtRepository: Send + Sync {
    async fn create_court(&self, court: &Court) -> Result<()>;
    async fn get_court_by_id(&self, id_court: Uuid) -> Result<Option<Court>>;
    async fn get_court_by_name(&self, court_name: &str) -> Result<Option<Court>>;
    async fn list_courts(&self) -> Result<Vec<Court>>;
    async fn delete_court(&self, id_court: Uuid) -> Result<()>;
}

#[async_trait]
pub trait CourtReservationRepository: Send + Sync {
    async fn create_reservation(&self, reservation: &CourtReservation) -> Result<()>;
    async fn get_reservations_for_court_in_range(
        &self,
        id_court: Uuid,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    ) -> Result<Vec<CourtReservation>>;
    async fn get_reservation_by_id(&self, id_reservation: Uuid)
        -> Result<Option<CourtReservation>>;
    async fn delete_reservation_by_event_id(&self, event_id: Uuid, event_type: &str) -> Result<()>;
    async fn get_reservation_for_training(
        &self,
        training_id: Uuid,
    ) -> Result<Option<CourtReservation>>;

    async fn get_reservation_for_tournament(
        &self,
        tournament_id: Uuid,
    ) -> Result<Option<CourtReservation>>;
}
