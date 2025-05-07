use async_trait::async_trait;
use chrono::NaiveDateTime;
use entities::court::{Court, CourtReservation};
use libsql::params;
use use_cases::court_service::{
    err::{Error, Result},
    repository_trait::{CourtRepository, CourtReservationRepository},
};
use uuid::Uuid;

use crate::TursoDb;

#[async_trait]
impl CourtRepository for TursoDb {
    async fn create_court(&self, court: &Court) -> Result<()> {
        self.execute_with_error(
            "INSERT INTO court (id_court, court_name, deleted) VALUES (?1, ?2, 0)",
            params![court.id_court.to_string(), court.court_name.clone()],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn get_court_by_id(&self, id_court: Uuid) -> Result<Option<Court>> {
        self.query_one_with_error(
            "SELECT id_court, court_name FROM court WHERE id_court = ?1 AND deleted = 0",
            params![id_court.to_string()],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn get_court_by_name(&self, court_name: &str) -> Result<Option<Court>> {
        self.query_one_with_error(
            "SELECT id_court, court_name FROM court WHERE court_name = ?1 AND deleted = 0",
            params![court_name],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn list_courts(&self) -> Result<Vec<Court>> {
        self.query_many_with_error(
            "SELECT id_court, court_name FROM court WHERE deleted = 0",
            params![],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn delete_court(&self, id_court: Uuid) -> Result<()> {
        self.execute_with_error(
            "UPDATE court SET deleted = 1 WHERE id_court = ?1",
            params![id_court.to_string()],
            Error::UnknownDatabaseError,
        )
        .await
    }
}

#[async_trait]
impl CourtReservationRepository for TursoDb {
    async fn create_reservation(&self, reservation: &CourtReservation) -> Result<()> {
        self.execute_with_error(
            "INSERT INTO court_reservation (id_court_reservation, id_court, start_reservation_datetime, end_reservation_datetime, id_training, id_tournament, deleted) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0)",
            params![
                reservation.id_court_reservation.to_string(),
                reservation.id_court.to_string(),
                reservation.start_reservation_datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
                reservation.end_reservation_datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
                reservation.id_training.map(|id| id.to_string()),
                reservation.id_tournament.map(|id| id.to_string()),
            ],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn get_reservations_for_court_in_range(
        &self,
        id_court: Uuid,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    ) -> Result<Vec<CourtReservation>> {
        self.query_many_with_error(
            "SELECT id_court_reservation, id_court, start_reservation_datetime, end_reservation_datetime, id_training, id_tournament 
             FROM court_reservation 
             WHERE id_court = ?1 AND deleted = 0 
             AND (
                 (start_reservation_datetime < ?3 AND end_reservation_datetime > ?2) -- Overlaps
             )",
            params![
                id_court.to_string(),
                start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                end_time.format("%Y-%m-%d %H:%M:%S").to_string()
            ],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn get_reservation_by_id(
        &self,
        id_reservation: Uuid,
    ) -> Result<Option<CourtReservation>> {
        self.query_one_with_error(
            "SELECT id_court_reservation, id_court, start_reservation_datetime, end_reservation_datetime, id_training, id_tournament
             FROM court_reservation
             WHERE id_court_reservation = ?1 AND deleted = 0",
            params![id_reservation.to_string()],
            Error::UnknownDatabaseError,
        ).await
    }

    async fn delete_reservation_by_event_id(&self, event_id: Uuid, event_type: &str) -> Result<()> {
        let sql = match event_type {
            "training" => "UPDATE court_reservation SET deleted = 1 WHERE id_training = ?1",
            "tournament" => "UPDATE court_reservation SET deleted = 1 WHERE id_tournament = ?1",
            _ => {
                return Err(Error::UnknownDatabaseError(
                    "Invalid event type for deletion".to_string(),
                ))
            }
        };
        self.execute_with_error(
            sql,
            params![event_id.to_string()],
            Error::UnknownDatabaseError,
        )
        .await
    }

    async fn get_reservations_for_training(
        &self,
        training_id: Uuid,
    ) -> Result<Vec<CourtReservation>> {
        self.query_many_with_error(
            "SELECT id_court_reservation, id_court, start_reservation_datetime, end_reservation_datetime, id_training, id_tournament 
             FROM court_reservation 
             WHERE id_training = ?1 AND deleted = 0",
            params![training_id.to_string()],
            Error::UnknownDatabaseError,
        ).await
    }

    async fn get_reservations_for_tournament(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<CourtReservation>> {
        self.query_many_with_error(
            "SELECT id_court_reservation, id_court, start_reservation_datetime, end_reservation_datetime, id_training, id_tournament 
             FROM court_reservation 
             WHERE id_tournament = ?1 AND deleted = 0",
            params![tournament_id.to_string()],
            Error::UnknownDatabaseError,
        ).await
    }
}
