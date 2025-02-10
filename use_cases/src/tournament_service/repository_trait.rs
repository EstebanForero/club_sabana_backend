use super::err::Result;
use async_trait::async_trait;
use entities::tournament::{Tournament, TournamentAttendance, TournamentRegistration};
use uuid::Uuid;

/// Trait defining tournament-related operations
#[async_trait]
pub trait TournamentRepository: Send + Sync {
    async fn create_tournament(&self, tournament: &Tournament) -> Result<()>;
    async fn get_tournament_by_id(&self, id: Uuid) -> Result<Option<Tournament>>;
    async fn update_tournament(&self, tournament: &Tournament) -> Result<()>;
    async fn delete_tournament(&self, id: Uuid) -> Result<()>; // Soft delete
    async fn list_tournaments(&self) -> Result<Vec<Tournament>>;
}

#[async_trait]
pub trait TournamentRegistrationRepository: Send + Sync {
    async fn register_user_for_tournament(
        &self,
        registration: &TournamentRegistration,
    ) -> Result<()>;
    async fn get_tournament_registrations(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<TournamentRegistration>>;
}

#[async_trait]
pub trait TournamentAttendanceRepository: Send + Sync {
    async fn record_tournament_attendance(&self, attendance: &TournamentAttendance) -> Result<()>;
    async fn get_tournament_attendance(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<TournamentAttendance>>;
    async fn update_tournament_position(
        &self,
        tournament_id: Uuid,
        user_id: Uuid,
        position: i32,
    ) -> Result<()>;
}
