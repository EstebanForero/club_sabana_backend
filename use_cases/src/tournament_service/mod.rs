pub mod err;
pub mod repository_trait;

use self::err::{Error, Result};
use entities::tournament::{
    Tournament, TournamentAttendance, TournamentCreation, TournamentRegistration,
};
use repository_trait::{
    TournamentAttendanceRepository, TournamentRegistrationRepository, TournamentRepository,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct TournamentService {
    tournament_repo: Arc<dyn TournamentRepository + Send + Sync>,
    registration_repo: Arc<dyn TournamentRegistrationRepository + Send + Sync>,
    attendance_repo: Arc<dyn TournamentAttendanceRepository + Send + Sync>,
}

impl TournamentService {
    pub fn new(
        tournament_repo: Arc<dyn TournamentRepository + Send + Sync>,
        registration_repo: Arc<dyn TournamentRegistrationRepository + Send + Sync>,
        attendance_repo: Arc<dyn TournamentAttendanceRepository + Send + Sync>,
    ) -> Self {
        Self {
            tournament_repo,
            registration_repo,
            attendance_repo,
        }
    }

    pub async fn create_tournament(&self, tournament: TournamentCreation) -> Result<()> {
        if tournament.start_datetime >= tournament.end_datetime {
            return Err(Error::InvalidDates);
        }

        self.tournament_repo
            .create_tournament(&tournament.to_tournament(Uuid::new_v4()))
            .await?;

        Ok(())
    }

    pub async fn get_tournament(&self, id: Uuid) -> Result<Tournament> {
        self.tournament_repo
            .get_tournament_by_id(id)
            .await?
            .ok_or(Error::TournamentNotFound)
            .map(|t| t.into())
    }

    pub async fn update_tournament(&self, tournament: Tournament) -> Result<()> {
        // Validate dates
        if tournament.start_datetime >= tournament.end_datetime {
            return Err(Error::InvalidDates);
        }

        // Check if tournament exists
        if self
            .tournament_repo
            .get_tournament_by_id(tournament.id_tournament)
            .await?
            .is_none()
        {
            return Err(Error::TournamentNotFound);
        }

        self.tournament_repo
            .update_tournament(&Tournament {
                id_tournament: tournament.id_tournament,
                name: tournament.name,
                id_category: tournament.id_category,
                start_datetime: tournament.start_datetime,
                end_datetime: tournament.end_datetime,
            })
            .await?;

        Ok(())
    }

    pub async fn delete_tournament(&self, id: Uuid) -> Result<()> {
        // Check if tournament exists
        if self
            .tournament_repo
            .get_tournament_by_id(id)
            .await?
            .is_none()
        {
            return Err(Error::TournamentNotFound);
        }

        self.tournament_repo.delete_tournament(id).await?;

        Ok(())
    }

    pub async fn list_tournaments(&self) -> Result<Vec<Tournament>> {
        let tournaments = self.tournament_repo.list_tournaments().await?;

        Ok(tournaments.into_iter().map(|t| t.into()).collect())
    }

    pub async fn register_user(&self, registration: TournamentRegistration) -> Result<()> {
        // Check if tournament exists
        if self
            .tournament_repo
            .get_tournament_by_id(registration.id_tournament)
            .await?
            .is_none()
        {
            return Err(Error::TournamentNotFound);
        }

        // Check if user is already registered
        let registrations = self
            .registration_repo
            .get_tournament_registrations(registration.id_tournament)
            .await?;
        if registrations
            .iter()
            .any(|r| r.id_user == registration.id_user)
        {
            return Err(Error::UserAlreadyRegistered);
        }

        self.registration_repo
            .register_user_for_tournament(&TournamentRegistration {
                id_tournament: registration.id_tournament,
                id_user: registration.id_user,
                registration_datetime: registration.registration_datetime,
            })
            .await?;

        Ok(())
    }

    pub async fn record_attendance(&self, attendance: TournamentAttendance) -> Result<()> {
        // Check if tournament exists
        if self
            .tournament_repo
            .get_tournament_by_id(attendance.id_tournament)
            .await?
            .is_none()
        {
            return Err(Error::TournamentNotFound);
        }

        // Check if user is registered
        let registrations = self
            .registration_repo
            .get_tournament_registrations(attendance.id_tournament)
            .await?;
        if !registrations
            .iter()
            .any(|r| r.id_user == attendance.id_user)
        {
            return Err(Error::UserNotRegistered);
        }

        self.attendance_repo
            .record_tournament_attendance(&TournamentAttendance {
                id_tournament: attendance.id_tournament,
                id_user: attendance.id_user,
                attendance_datetime: attendance.attendance_datetime,
                position: attendance.position,
            })
            .await?;

        Ok(())
    }

    pub async fn update_position(
        &self,
        tournament_id: Uuid,
        user_id: Uuid,
        position: i32,
    ) -> Result<()> {
        // Check if tournament exists
        if self
            .tournament_repo
            .get_tournament_by_id(tournament_id)
            .await?
            .is_none()
        {
            return Err(Error::TournamentNotFound);
        }

        // Check if user attended
        let attendance = self
            .attendance_repo
            .get_tournament_attendance(tournament_id)
            .await?;
        if !attendance.iter().any(|a| a.id_user == user_id) {
            return Err(Error::UserDidNotAttend);
        }

        if attendance.iter().any(|a| a.position == position) {
            return Err(Error::PositionAlreadyTaken);
        }

        // Validate position
        if position < 1 {
            return Err(Error::NegativePosition);
        }

        self.attendance_repo
            .update_tournament_position(tournament_id, user_id, position)
            .await?;

        Ok(())
    }
}
