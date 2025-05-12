pub mod err;
pub mod repository_trait;

use crate::{
    category_service::CategoryService,
    court_service::{self, CourtService}, // Added CourtService
};

use self::err::{Error, Result};
use chrono::{Duration, NaiveDateTime, Utc};
use entities::{
    court::CourtReservationCreation, // Added
    tournament::{
        Tournament, TournamentAttendance, TournamentAttendanceRequest, TournamentCreation,
        TournamentRegistration, TournamentRegistrationRequest,
    },
};
use repository_trait::{
    TournamentAttendanceRepository, TournamentRegistrationRepository, TournamentRepository,
};
use std::sync::Arc;
use uuid::Uuid;

const MIN_EVENT_DURATION_MINUTES: i64 = 10;
const MAX_EVENT_DURATION_HOURS: i64 = 5;

#[derive(Clone)]
pub struct TournamentService {
    tournament_repo: Arc<dyn TournamentRepository>,
    registration_repo: Arc<dyn TournamentRegistrationRepository>,
    attendance_repo: Arc<dyn TournamentAttendanceRepository>,
    category_service: CategoryService,
    court_service: CourtService, // Added
}

impl TournamentService {
    pub fn new(
        tournament_repo: Arc<dyn TournamentRepository>,
        registration_repo: Arc<dyn TournamentRegistrationRepository>,
        attendance_repo: Arc<dyn TournamentAttendanceRepository>,
        category_service: CategoryService,
        court_service: CourtService, // Added
    ) -> Self {
        Self {
            tournament_repo,
            registration_repo,
            attendance_repo,
            category_service,
            court_service, // Added
        }
    }

    pub async fn create_tournament(
        &self,
        tournament_creation: TournamentCreation,
        id_court_to_reserve: Option<Uuid>,
    ) -> Result<Tournament> {
        validate_event_duration(
            tournament_creation.start_datetime,
            tournament_creation.end_datetime,
        )?;

        // Validate category exists
        let _ = self
            .category_service
            .get_category_by_id(tournament_creation.id_category)
            .await?;

        let tournament_id = Uuid::new_v4();
        let tournament = tournament_creation.to_tournament(tournament_id);

        self.tournament_repo.create_tournament(&tournament).await?;

        if let Some(id_court) = id_court_to_reserve {
            let reservation_creation = CourtReservationCreation {
                id_court,
                start_reservation_datetime: tournament.start_datetime,
                end_reservation_datetime: tournament.end_datetime,
                id_training: None,
                id_tournament: Some(tournament.id_tournament),
            };
            if let Err(e) = self
                .court_service
                .create_reservation(reservation_creation)
                .await
            {
                // Rollback or inform. For now, delete tournament.
                self.tournament_repo.delete_tournament(tournament.id_tournament).await.unwrap_or_else(|del_err| {
                    tracing::error!("Failed to rollback tournament creation after court reservation failure: {}", del_err);
                });
                return Err(Error::CourtServiceError(e));
            }
        }
        Ok(tournament)
    }

    pub async fn update_tournament(
        &self,
        tournament_id: Uuid,
        tournament_update_payload: TournamentCreation,
        id_court_to_reserve: Option<Uuid>,
    ) -> Result<Tournament> {
        let mut tournament = self.get_tournament(tournament_id).await?;

        validate_event_duration(
            tournament_update_payload.start_datetime,
            tournament_update_payload.end_datetime,
        )?;

        if tournament.id_category != tournament_update_payload.id_category {
            let _ = self
                .category_service
                .get_category_by_id(tournament_update_payload.id_category)
                .await?;
        }

        tournament.name = tournament_update_payload.name;
        tournament.id_category = tournament_update_payload.id_category;
        tournament.start_datetime = tournament_update_payload.start_datetime;
        tournament.end_datetime = tournament_update_payload.end_datetime;

        // Handle court reservation change
        if let Some(existing_res) = self
            .court_service
            .get_reservation_for_tournament(tournament_id)
            .await // Now returns Option
            .map_err(Error::CourtServiceError)?
        {
            let times_changed = existing_res.start_reservation_datetime
                != tournament.start_datetime
                || existing_res.end_reservation_datetime != tournament.end_datetime;
            let court_changed =
                id_court_to_reserve.is_some() && Some(existing_res.id_court) != id_court_to_reserve;

            if times_changed || court_changed || id_court_to_reserve.is_none() {
                self.court_service
                    .delete_reservation_for_event(tournament_id, "tournament")
                    .await
                    .map_err(Error::CourtServiceError)?;
            }
        }

        if let Some(id_court) = id_court_to_reserve {
            if self
                .court_service
                .get_reservation_for_tournament(tournament_id)
                .await?
                .is_none()
            {
                let reservation_creation = CourtReservationCreation {
                    id_court,
                    start_reservation_datetime: tournament.start_datetime,
                    end_reservation_datetime: tournament.end_datetime,
                    id_training: None,
                    id_tournament: Some(tournament.id_tournament),
                };
                if let Err(e) = self
                    .court_service
                    .create_reservation(reservation_creation)
                    .await
                {
                    return Err(Error::CourtServiceError(e));
                }
            }
        }

        self.tournament_repo.update_tournament(&tournament).await?;
        Ok(tournament)
    }

    pub async fn get_tournament(&self, id: Uuid) -> Result<Tournament> {
        self.tournament_repo
            .get_tournament_by_id(id)
            .await?
            .ok_or(Error::TournamentNotFound)
    }

    pub async fn delete_tournament(&self, id: Uuid) -> Result<()> {
        let _ = self.get_tournament(id).await?; // Ensures tournament exists

        if let Err(e) = self
            .court_service
            .delete_reservation_for_event(id, "tournament")
            .await
        {
            tracing::warn!(
                "Could not delete court reservation for tournament {}: {}",
                id,
                e
            );
        }
        self.tournament_repo.delete_tournament(id).await
    }

    pub async fn list_tournaments(&self) -> Result<Vec<Tournament>> {
        self.tournament_repo.list_tournaments().await
    }

    pub async fn register_user(
        &self,
        registration_payload: TournamentRegistrationRequest,
        tournament_id: Uuid,
    ) -> Result<TournamentRegistration> {
        let tournament = self.get_tournament(tournament_id).await?;

        if !self
            .category_service
            .user_has_category(registration_payload.id_user, tournament.id_category)
            .await?
        {
            return Err(Error::UserDoesNotMeetCategoryRequirements);
        }

        if self
            .registration_repo
            .get_tournament_registration(tournament_id, registration_payload.id_user)
            .await?
            .is_some()
        {
            return Err(Error::UserAlreadyRegistered);
        }

        // Use registration_payload directly as it now contains all necessary fields including id_tournament
        let registration_to_create = TournamentRegistration {
            id_tournament: tournament_id,
            id_user: registration_payload.id_user,
            registration_datetime: Utc::now().naive_utc(), // Set current time
        };

        self.registration_repo
            .register_user_for_tournament(&registration_to_create) // Corrected variable name
            .await?;

        Ok(registration_to_create) // Return the object that was actually created
    }

    pub async fn record_attendance(
        &self,
        attendance_payload: TournamentAttendanceRequest,
        tournament_id: Uuid,
    ) -> Result<TournamentAttendance> {
        let tournament = self.get_tournament(tournament_id).await?;

        let now = Utc::now().naive_utc();

        if tournament.start_datetime >= now {
            return Err(Error::InvalidAssistanceDate);
        }

        if tournament.end_datetime <= now {
            return Err(Error::InvalidAssistanceDate);
        }

        if self
            .registration_repo
            .get_tournament_registration(tournament_id, attendance_payload.id_user)
            .await?
            .is_none()
        {
            return Err(Error::UserNotRegistered);
        }

        if self
            .attendance_repo
            .get_tournament_attendance_by_user(tournament_id, attendance_payload.id_user)
            .await?
            .is_some()
        {
            return Err(Error::UnknownDatabaseError(
                "User attendance already recorded for this tournament.".to_string(),
            ));
        }

        if attendance_payload.position <= 0 {
            return Err(Error::NegativePosition);
        }

        let existing_attendance = self
            .attendance_repo
            .get_tournament_attendance(tournament_id)
            .await?;
        if existing_attendance
            .iter()
            .any(|a| a.position == attendance_payload.position)
        {
            return Err(Error::PositionAlreadyTaken);
        }

        let attendance = TournamentAttendance {
            id_tournament: tournament_id,
            id_user: attendance_payload.id_user,
            attendance_datetime: Utc::now().naive_utc(),
            position: attendance_payload.position,
        };

        self.attendance_repo
            .record_tournament_attendance(&attendance)
            .await?;

        Ok(attendance)
    }

    pub async fn get_user_registrations(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<TournamentRegistration>> {
        self.registration_repo.get_user_registrations(user_id).await
    }

    pub async fn get_tournament_registrations(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<TournamentRegistration>> {
        self.registration_repo
            .get_tournament_registrations(tournament_id)
            .await
    }

    pub async fn update_position(
        &self,
        tournament_id: Uuid,
        user_id: Uuid,
        new_position: i32,
    ) -> Result<()> {
        let _ = self.get_tournament(tournament_id).await?;

        // Ensure user attended
        let _ = self
            .attendance_repo
            .get_tournament_attendance_by_user(tournament_id, user_id)
            .await?
            .ok_or(Error::UserDidNotAttend)?;

        if new_position <= 0 {
            return Err(Error::NegativePosition);
        }

        // Check if new_position is already taken by another user in the same tournament
        let existing_attendance = self
            .attendance_repo
            .get_tournament_attendance(tournament_id)
            .await?;
        if existing_attendance
            .iter()
            .any(|a| a.id_user != user_id && a.position == new_position)
        {
            return Err(Error::PositionAlreadyTaken);
        }

        self.attendance_repo
            .update_tournament_position(tournament_id, user_id, new_position)
            .await
    }

    pub async fn get_eligible_tournaments(&self, user_id: Uuid) -> Result<Vec<Tournament>> {
        let all_tournaments = self.tournament_repo.list_tournaments().await?;

        let user_categories = self.category_service.get_user_categories(user_id).await?;
        let user_category_ids: Vec<Uuid> = user_categories
            .into_iter()
            .map(|uc| uc.id_category)
            .collect();

        let eligible_tournaments = all_tournaments
            .into_iter()
            .filter(|t| user_category_ids.contains(&t.id_category))
            .collect();

        Ok(eligible_tournaments)
    }

    pub async fn get_tournament_attendance(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<TournamentAttendance>> {
        self.attendance_repo
            .get_tournament_attendance(tournament_id)
            .await
    }

    pub async fn delete_attendance(&self, tournament_id: Uuid, user_id: Uuid) -> Result<()> {
        let _ = self.get_tournament(tournament_id).await?;
        // Check if attendance exists before deleting
        let _ = self
            .attendance_repo
            .get_tournament_attendance_by_user(tournament_id, user_id)
            .await?
            .ok_or(Error::UserDidNotAttend)?;
        self.attendance_repo
            .delete_attendance(tournament_id, user_id)
            .await
    }

    pub async fn delete_registration(&self, tournament_id: Uuid, user_id: Uuid) -> Result<()> {
        let _ = self.get_tournament(tournament_id).await?;
        // Check if registration exists before deleting
        let _ = self
            .registration_repo
            .get_tournament_registration(tournament_id, user_id)
            .await?
            .ok_or(Error::UserNotRegistered)?;
        self.registration_repo
            .delete_registration(tournament_id, user_id)
            .await
    }

    pub async fn get_user_attendance(&self, user_id: Uuid) -> Result<Vec<TournamentAttendance>> {
        let all_tournaments = self.tournament_repo.list_tournaments().await?;
        let mut user_attendance_list = Vec::new();

        for tournament in all_tournaments {
            if let Some(att) = self
                .attendance_repo
                .get_tournament_attendance_by_user(tournament.id_tournament, user_id)
                .await?
            {
                user_attendance_list.push(att);
            }
        }
        Ok(user_attendance_list)
    }
}

fn validate_event_duration(start_time: NaiveDateTime, end_time: NaiveDateTime) -> Result<()> {
    if start_time >= end_time {
        return Err(Error::InvalidDates);
    }
    // It's okay for tournaments to be in the past for record keeping or if they are ongoing.
    // if start_time <= Utc::now().naive_utc() {
    //     return Err(Error::InvalidDates);
    // }
    let duration = end_time - start_time;
    if duration < Duration::minutes(MIN_EVENT_DURATION_MINUTES) {
        return Err(Error::InvalidDates); // Or a more specific error like "EventDurationTooShort"
    }
    if duration > Duration::hours(MAX_EVENT_DURATION_HOURS) {
        return Err(Error::InvalidDates); // Or "EventDurationTooLong"
    }
    Ok(())
}
