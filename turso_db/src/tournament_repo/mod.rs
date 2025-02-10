use async_trait::async_trait;
use entities::tournament::{Tournament, TournamentAttendance, TournamentRegistration};
use libsql::{de, params};
use use_cases::tournament_service::err::{Error, Result};
use use_cases::tournament_service::repository_trait::{
    TournamentAttendanceRepository, TournamentRegistrationRepository, TournamentRepository,
};
use uuid::{uuid, Uuid};

use crate::TursoDb;

#[async_trait]
impl TournamentRepository for TursoDb {
    async fn create_tournament(&self, tournament: &Tournament) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        conn.execute(
            "INSERT INTO tournament (
                id_tournament, name, id_category, start_datetime, end_datetime, deleted
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                tournament.id_tournament.to_string(),
                tournament.name.to_string(),
                tournament.id_category.to_string(),
                tournament
                    .start_datetime
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                tournament
                    .end_datetime
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                tournament.deleted as i32,
            ],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        Ok(())
    }

    async fn get_tournament_by_id(&self, id: Uuid) -> Result<Option<Tournament>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        let mut rows = conn
            .query(
                "SELECT id_tournament, name, id_category, start_datetime, end_datetime, deleted 
                 FROM tournament 
                 WHERE id_tournament = ?1 AND deleted = 0",
                params![id.to_string()],
            )
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        if let Some(row_result) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?
        {
            let tournament = de::from_row::<Tournament>(&row_result)
                .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;
            Ok(Some(tournament))
        } else {
            Ok(None)
        }
    }

    async fn update_tournament(&self, tournament: &Tournament) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        conn.execute(
            "UPDATE tournament SET 
                name = ?1, 
                id_category = ?2, 
                start_datetime = ?3, 
                end_datetime = ?4, 
                deleted = ?5 
             WHERE id_tournament = ?6",
            params![
                tournament.name.to_string(),
                tournament.id_category.to_string(),
                tournament
                    .start_datetime
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                tournament
                    .end_datetime
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                tournament.deleted as i32,
                tournament.id_tournament.to_string(),
            ],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        Ok(())
    }

    async fn delete_tournament(&self, id: Uuid) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        conn.execute(
            "UPDATE tournament SET deleted = 1 WHERE id_tournament = ?1",
            params![id.to_string()],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        Ok(())
    }

    async fn list_tournaments(&self) -> Result<Vec<Tournament>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        let mut rows = conn
            .query(
                "SELECT id_tournament, name, id_category, start_datetime, end_datetime, deleted 
                 FROM tournament 
                 WHERE deleted = 0",
                params![],
            )
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        let mut tournaments = Vec::new();
        while let Some(row_result) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?
        {
            let tournament = de::from_row::<Tournament>(&row_result)
                .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;
            tournaments.push(tournament);
        }

        Ok(tournaments)
    }
}

#[async_trait]
impl TournamentRegistrationRepository for TursoDb {
    async fn register_user_for_tournament(
        &self,
        registration: &TournamentRegistration,
    ) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        conn.execute(
            "INSERT INTO tournament_registration (
                id_tournament, id_user, registration_datetime, deleted
            ) VALUES (?1, ?2, ?3, ?4)",
            params![
                registration.id_tournament.to_string(),
                registration.id_user.to_string(),
                registration
                    .registration_datetime
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                registration.deleted as i32,
            ],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        Ok(())
    }

    async fn get_tournament_registrations(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<TournamentRegistration>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        let mut rows = conn
            .query(
                "SELECT id_tournament, id_user, registration_datetime, deleted 
                 FROM tournament_registration 
                 WHERE id_tournament = ?1 AND deleted = 0",
                params![tournament_id.to_string()],
            )
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        let mut registrations = Vec::new();
        while let Some(row_result) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?
        {
            let registration = de::from_row::<TournamentRegistration>(&row_result)
                .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;
            registrations.push(registration);
        }

        Ok(registrations)
    }
}

#[async_trait]
impl TournamentAttendanceRepository for TursoDb {
    async fn record_tournament_attendance(&self, attendance: &TournamentAttendance) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        conn.execute(
            "INSERT INTO tournament_attendance (
                id_tournament, id_user, attendance_datetime, position, deleted
            ) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                attendance.id_tournament.to_string(),
                attendance.id_user.to_string(),
                attendance
                    .attendance_datetime
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                attendance.position,
                attendance.deleted as i32,
            ],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        Ok(())
    }

    async fn get_tournament_attendance(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<TournamentAttendance>> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        let mut rows = conn
            .query(
                "SELECT id_tournament, id_user, attendance_datetime, position, deleted 
                 FROM tournament_attendance 
                 WHERE id_tournament = ?1 AND deleted = 0",
                params![tournament_id.to_string()],
            )
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        let mut attendances = Vec::new();
        while let Some(row_result) = rows
            .next()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?
        {
            let attendance = de::from_row::<TournamentAttendance>(&row_result)
                .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;
            attendances.push(attendance);
        }

        Ok(attendances)
    }

    async fn update_tournament_position(
        &self,
        tournament_id: Uuid,
        user_id: Uuid,
        position: i32,
    ) -> Result<()> {
        let conn = self
            .get_connection()
            .await
            .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        conn.execute(
            "UPDATE tournament_attendance SET position = ?1 
             WHERE id_tournament = ?2 AND id_user = ?3",
            params![position, tournament_id.to_string(), user_id.to_string()],
        )
        .await
        .map_err(|err| Error::UnknownDatabaseError(format!("{err}")))?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::future::Future;

    use entities::{
        tournament::{Tournament, TournamentAttendance, TournamentRegistration},
        user::User,
    };
    use libsql::params;
    use rstest::{fixture, rstest};
    use use_cases::{
        tournament_service::repository_trait::{
            TournamentAttendanceRepository, TournamentRegistrationRepository, TournamentRepository,
        },
        user_service::repository_trait::UserRepository,
    };
    use uuid::{uuid, Uuid};

    use crate::{TestDbBuilder, TursoDb};

    #[fixture]
    async fn repository() -> TursoDb {
        let db = TestDbBuilder::create()
            .await
            .apply_doc_types()
            .await
            .apply_user_roles()
            .await
            .build();

        // Create a test user
        let user = User {
            id_user: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            phone_number: "1234567890".to_string(),
            identification_number: "ID123456".to_string(),
            ..User::default()
        };

        db.create_user(&user)
            .await
            .expect("Failed to create test user");

        // Create a test category
        let conn = db.get_connection().await.expect("Failed to get connection");
        let category_id = uuid!("123e4567-e89b-12d3-a456-426614174000");
        conn.execute(
            "INSERT OR IGNORE INTO category (id_category, name, min_age, max_age, deleted) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                category_id.to_string(),
                "Test Category".to_string(),
                10,
                20,
                0
            ],
        )
        .await
        .expect("Failed to create test category");

        db
    }

    #[rstest]
    #[tokio::test]
    async fn test_create_tournament(repository: impl Future<Output = TursoDb>) {
        let tournament_id = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
        let category_id = uuid!("123e4567-e89b-12d3-a456-426614174000");
        let db = repository.await;

        let tournament = Tournament {
            id_tournament: tournament_id,
            name: "Test Tournament".to_string(),
            id_category: category_id,
            start_datetime: chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            end_datetime: chrono::DateTime::from_timestamp(86400, 0)
                .unwrap()
                .naive_utc(),
            deleted: false,
        };

        db.create_tournament(&tournament)
            .await
            .expect("Error creating tournament");

        let tournament_db = db
            .get_tournament_by_id(tournament_id)
            .await
            .expect("Error getting tournament by id")
            .expect("Tournament was not added");

        assert_eq!(tournament, tournament_db);
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_tournament_by_id(repository: impl Future<Output = TursoDb>) {
        let tournament_id = Uuid::new_v4();
        let category_id = Uuid::new_v4();
        let db = repository.await;

        let tournament = Tournament {
            id_tournament: tournament_id,
            name: "Test Tournament".to_string(),
            id_category: category_id,
            start_datetime: chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            end_datetime: chrono::DateTime::from_timestamp(86400, 0)
                .unwrap()
                .naive_utc(),
            deleted: false,
        };

        db.create_tournament(&tournament)
            .await
            .expect("Error creating tournament");

        let tournament_db = db
            .get_tournament_by_id(tournament_id)
            .await
            .expect("Error getting tournament by id")
            .expect("Tournament was not added");

        assert_eq!(tournament, tournament_db);

        let tournament_db = db
            .get_tournament_by_id(Uuid::new_v4())
            .await
            .expect("Error getting tournament by id");

        assert!(tournament_db.is_none());
    }

    #[rstest]
    #[tokio::test]
    async fn test_update_tournament(repository: impl Future<Output = TursoDb>) {
        let tournament_id = Uuid::new_v4();
        let category_id = Uuid::new_v4();
        let db = repository.await;

        let mut tournament = Tournament {
            id_tournament: tournament_id,
            name: "Original Name".to_string(),
            id_category: category_id,
            start_datetime: chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            end_datetime: chrono::DateTime::from_timestamp(86400, 0)
                .unwrap()
                .naive_utc(),
            deleted: false,
        };

        db.create_tournament(&tournament)
            .await
            .expect("Error creating tournament");

        // Update some fields
        tournament.name = "Updated Name".to_string();
        tournament.end_datetime = chrono::DateTime::from_timestamp(172800, 0)
            .unwrap()
            .naive_utc();

        db.update_tournament(&tournament)
            .await
            .expect("Error updating tournament");

        let updated_tournament = db
            .get_tournament_by_id(tournament_id)
            .await
            .expect("Error fetching updated tournament")
            .expect("Tournament not found after update");

        assert_eq!(tournament, updated_tournament);
    }

    #[rstest]
    #[tokio::test]
    async fn test_delete_tournament(repository: impl Future<Output = TursoDb>) {
        let tournament_id = Uuid::new_v4();
        let category_id = Uuid::new_v4();
        let db = repository.await;

        let tournament = Tournament {
            id_tournament: tournament_id,
            name: "Test Tournament".to_string(),
            id_category: category_id,
            start_datetime: chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            end_datetime: chrono::DateTime::from_timestamp(86400, 0)
                .unwrap()
                .naive_utc(),
            deleted: false,
        };

        db.create_tournament(&tournament)
            .await
            .expect("Error creating tournament");

        db.delete_tournament(tournament_id)
            .await
            .expect("Error deleting tournament");

        let deleted_tournament = db
            .get_tournament_by_id(tournament_id)
            .await
            .expect("Error fetching tournament after delete");

        assert!(deleted_tournament.is_none());
    }

    #[rstest]
    #[tokio::test]
    async fn test_list_tournaments(repository: impl Future<Output = TursoDb>) {
        let db = repository.await;

        // Get initial list of tournaments
        let initial_tournaments = db
            .list_tournaments()
            .await
            .expect("Error listing initial tournaments");
        let initial_count = initial_tournaments.len();

        // Create two tournaments
        let tournament1 = Tournament {
            id_tournament: Uuid::new_v4(),
            name: "Tournament 1".to_string(),
            id_category: uuid!("123e4567-e89b-12d3-a456-426614174000"),
            start_datetime: chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            end_datetime: chrono::DateTime::from_timestamp(86400, 0)
                .unwrap()
                .naive_utc(),
            deleted: false,
        };

        let tournament2 = Tournament {
            id_tournament: Uuid::new_v4(),
            name: "Tournament 2".to_string(),
            id_category: Uuid::new_v4(),
            start_datetime: chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            end_datetime: chrono::DateTime::from_timestamp(86400, 0)
                .unwrap()
                .naive_utc(),
            deleted: false,
        };

        db.create_tournament(&tournament1)
            .await
            .expect("Error creating tournament1");
        db.create_tournament(&tournament2)
            .await
            .expect("Error creating tournament2");

        let tournaments_after_insert = db
            .list_tournaments()
            .await
            .expect("Error listing tournaments after insert");
        assert_eq!(tournaments_after_insert.len(), initial_count + 2);

        // Delete one tournament
        db.delete_tournament(tournament1.id_tournament)
            .await
            .expect("Error deleting tournament1");

        let tournaments_after_delete = db
            .list_tournaments()
            .await
            .expect("Error listing tournaments after deletion");
        assert_eq!(tournaments_after_delete.len(), initial_count + 1);

        // Ensure that tournament1 is no longer listed
        for tournament in tournaments_after_delete {
            assert_ne!(tournament.id_tournament, tournament1.id_tournament);
        }
    }

    #[rstest]
    #[tokio::test]
    async fn test_register_user_for_tournament(repository: impl Future<Output = TursoDb>) {
        let tournament_id = Uuid::new_v4();
        let db = repository.await;

        // Get the test user ID
        let user_id = db
            .get_user_id_by_email("test@example.com")
            .await
            .expect("Failed to get test user")
            .expect("Test user not found");

        let registration = TournamentRegistration {
            id_tournament: tournament_id,
            id_user: user_id,
            registration_datetime: chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            deleted: false,
        };

        db.register_user_for_tournament(&registration)
            .await
            .expect("Error registering user for tournament");

        let registrations = db
            .get_tournament_registrations(tournament_id)
            .await
            .expect("Error getting tournament registrations");

        assert_eq!(registrations.len(), 1);
        assert_eq!(registrations[0], registration);
    }

    #[rstest]
    #[tokio::test]
    async fn test_record_tournament_attendance(repository: impl Future<Output = TursoDb>) {
        let tournament_id = Uuid::new_v4();
        let db = repository.await;

        // Get the test user ID
        let user_id = db
            .get_user_id_by_email("test@example.com")
            .await
            .expect("Failed to get test user")
            .expect("Test user not found");

        let attendance = TournamentAttendance {
            id_tournament: tournament_id,
            id_user: user_id,
            attendance_datetime: chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            position: 1,
            deleted: false,
        };

        db.record_tournament_attendance(&attendance)
            .await
            .expect("Error recording tournament attendance");

        let attendances = db
            .get_tournament_attendance(tournament_id)
            .await
            .expect("Error getting tournament attendance");

        assert_eq!(attendances.len(), 1);
        assert_eq!(attendances[0], attendance);
    }

    #[rstest]
    #[tokio::test]
    async fn test_update_tournament_position(repository: impl Future<Output = TursoDb>) {
        let tournament_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let db = repository.await;

        let attendance = TournamentAttendance {
            id_tournament: tournament_id,
            id_user: user_id,
            attendance_datetime: chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            position: 1,
            deleted: false,
        };

        db.record_tournament_attendance(&attendance)
            .await
            .expect("Error recording tournament attendance");

        db.update_tournament_position(tournament_id, user_id, 2)
            .await
            .expect("Error updating tournament position");

        let attendances = db
            .get_tournament_attendance(tournament_id)
            .await
            .expect("Error getting tournament attendance");

        assert_eq!(attendances.len(), 1);
        assert_eq!(attendances[0].position, 2);
    }
}
