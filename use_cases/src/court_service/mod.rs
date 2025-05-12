pub mod err;
pub mod repository_trait;

use chrono::NaiveDateTime;
use entities::court::{Court, CourtCreation, CourtReservation, CourtReservationCreation};
use repository_trait::{CourtRepository, CourtReservationRepository};
use std::sync::Arc;
use uuid::Uuid;

use self::err::{Error, Result};

#[derive(Clone)]
pub struct CourtService {
    court_repo: Arc<dyn CourtRepository>,
    reservation_repo: Arc<dyn CourtReservationRepository>,
}

impl CourtService {
    pub fn new(
        court_repo: Arc<dyn CourtRepository>,
        reservation_repo: Arc<dyn CourtReservationRepository>,
    ) -> Self {
        Self {
            court_repo,
            reservation_repo,
        }
    }

    pub async fn create_court(&self, court_creation: CourtCreation) -> Result<Court> {
        if self
            .court_repo
            .get_court_by_name(&court_creation.court_name)
            .await?
            .is_some()
        {
            return Err(Error::CourtNameExists);
        }
        let court_id = Uuid::new_v4();
        let court = court_creation.to_court(court_id);
        self.court_repo.create_court(&court).await?;
        Ok(court)
    }

    pub async fn get_court(&self, id_court: Uuid) -> Result<Court> {
        self.court_repo
            .get_court_by_id(id_court)
            .await?
            .ok_or(Error::CourtNotFound)
    }

    pub async fn list_courts(&self) -> Result<Vec<Court>> {
        self.court_repo.list_courts().await
    }

    pub async fn delete_court(&self, id_court: Uuid) -> Result<()> {
        let reservation_exists = self
            .reservation_repo
            .court_has_reservations(id_court)
            .await?;

        if reservation_exists {
            return Err(Error::ReservationExists);
        }

        self.court_repo.delete_court(id_court).await
    }

    pub async fn create_reservation(
        &self,
        reservation_creation: CourtReservationCreation,
    ) -> Result<CourtReservation> {
        if reservation_creation.start_reservation_datetime
            >= reservation_creation.end_reservation_datetime
        {
            return Err(Error::InvalidReservationTime);
        }

        let _ = self
            .court_repo
            .get_court_by_id(reservation_creation.id_court)
            .await?
            .ok_or(Error::CourtNotFound)?;

        match (
            reservation_creation.id_training,
            reservation_creation.id_tournament,
        ) {
            (Some(_), Some(_)) => return Err(Error::ReservationPurposeConflict),
            (None, None) => return Err(Error::ReservationPurposeMissing),
            (Some(training_id), None) => {
                if self
                    .reservation_repo
                    .get_reservation_for_training(training_id)
                    .await?
                    .is_some()
                {
                    return Err(Error::CourtUnavailable);
                }
            }
            (None, Some(tournament_id)) => {
                if self
                    .reservation_repo
                    .get_reservation_for_tournament(tournament_id)
                    .await?
                    .is_some()
                {
                    return Err(Error::CourtUnavailable);
                }
            }
        }

        if !self
            .is_court_available(
                reservation_creation.id_court,
                reservation_creation.start_reservation_datetime,
                reservation_creation.end_reservation_datetime,
                None,
            )
            .await?
        {
            return Err(Error::CourtUnavailable);
        }

        let reservation_id = Uuid::new_v4();
        let reservation = reservation_creation.to_court_reservation(reservation_id);
        self.reservation_repo
            .create_reservation(&reservation)
            .await?;
        Ok(reservation)
    }

    pub async fn get_reservation(&self, id_reservation: Uuid) -> Result<CourtReservation> {
        self.reservation_repo
            .get_reservation_by_id(id_reservation)
            .await?
            .ok_or(Error::ReservationNotFound)
    }

    pub async fn get_reservations_for_court(
        &self,
        id_court: Uuid,
        start_datetime_filter: Option<NaiveDateTime>,
        end_datetime_filter: Option<NaiveDateTime>,
    ) -> Result<Vec<CourtReservation>> {
        let start = start_datetime_filter.unwrap_or_else(|| {
            NaiveDateTime::parse_from_str("1970-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        });
        let end = end_datetime_filter.unwrap_or_else(|| {
            NaiveDateTime::parse_from_str("9999-12-31 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap()
        });

        self.reservation_repo
            .get_reservations_for_court_in_range(id_court, start, end)
            .await
    }

    pub async fn is_court_available(
        &self,
        id_court: Uuid,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
        exclude_reservation_id: Option<Uuid>,
    ) -> Result<bool> {
        let overlapping_reservations = self
            .reservation_repo
            .get_reservations_for_court_in_range(id_court, start_time, end_time)
            .await?;

        for r in overlapping_reservations {
            if Some(r.id_court_reservation) == exclude_reservation_id {
                continue;
            }
            if start_time < r.end_reservation_datetime && end_time > r.start_reservation_datetime {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub async fn delete_reservation_for_event(
        &self,
        event_id: Uuid,
        event_type: &str,
    ) -> Result<()> {
        self.reservation_repo
            .delete_reservation_by_event_id(event_id, event_type)
            .await
    }

    pub async fn get_reservation_for_training(
        &self,
        training_id: Uuid,
    ) -> Result<Option<CourtReservation>> {
        self.reservation_repo
            .get_reservation_for_training(training_id)
            .await
    }

    pub async fn get_reservation_for_tournament(
        &self,
        tournament_id: Uuid,
    ) -> Result<Option<CourtReservation>> {
        self.reservation_repo
            .get_reservation_for_tournament(tournament_id)
            .await
    }
}
