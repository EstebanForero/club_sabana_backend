use super::err::Result;
use entities::tournament::{Tournament, TournamentAttendance, TournamentRegistration};
use uuid::Uuid;

/// Trait defining tournament-related operations
pub trait TournamentRepository {
    fn create_tournament(&self, tournament: &Tournament) -> Result<()>;
    fn get_tournament_by_id(&self, id: Uuid) -> Result<Option<Tournament>>;
    fn update_tournament(&self, tournament: &Tournament) -> Result<()>;
    fn delete_tournament(&self, id: Uuid) -> Result<()>; // Soft delete
    fn list_tournaments(&self) -> Result<Vec<Tournament>>;
}

pub trait TournamentRegistrationRepository {
    fn register_user_for_tournament(&self, registration: &TournamentRegistration) -> Result<()>;
    fn get_tournament_registrations(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<TournamentRegistration>>;
}

pub trait TournamentAttendanceRepository {
    fn record_tournament_attendance(&self, attendance: &TournamentAttendance) -> Result<()>;
    fn get_tournament_attendance(&self, tournament_id: Uuid) -> Result<Vec<TournamentAttendance>>;
    fn update_tournament_position(
        &self,
        tournament_id: Uuid,
        user_id: Uuid,
        position: i32,
    ) -> Result<()>;
}
