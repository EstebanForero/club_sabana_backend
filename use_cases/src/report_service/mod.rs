use crate::{
    category_service::CategoryService, request_service::RequestService,
    tournament_service::TournamentService, training_service::TrainingService,
    tuition_service::TuitionService, user_service::UserService,
};
use entities::{
    report::{
        Report, TournamentSummary, TrainingSummary, TuitionSummary, UserCategory, UserRequest,
    },
    user::UserInfo,
};
use err::ReportError;
use std::sync::Arc;
use uuid::Uuid;

pub mod err;
pub mod repository_trait;

#[derive(Clone)]
pub struct ReportService {
    user_service: UserService,
    category_service: CategoryService,
    training_service: TrainingService,
    tournament_service: TournamentService,
    tuition_service: TuitionService,
    request_service: RequestService,
}

impl ReportService {
    pub fn new(
        user_service: UserService,
        category_service: CategoryService,
        training_service: TrainingService,
        tournament_service: TournamentService,
        tuition_service: TuitionService,
        request_service: RequestService,
    ) -> Self {
        Self {
            user_service,
            category_service,
            training_service,
            tournament_service,
            tuition_service,
            request_service,
        }
    }

    pub async fn generate_user_report(&self, user_id: Uuid) -> Result<Report, ReportError> {
        let (
            user,
            categories,
            training_registrations,
            tournament_registrations,
            tournament_attendances,
            tuitions,
            requests,
        ) = tokio::try_join!(
            async {
                self.user_service
                    .get_user_by_id(user_id)
                    .await
                    .map_err(ReportError::from)
            },
            async {
                self.category_service
                    .get_user_categories(user_id)
                    .await
                    .map_err(ReportError::from)
            },
            async {
                self.training_service
                    .get_user_training_registrations(user_id)
                    .await
                    .map_err(ReportError::from)
            },
            async {
                self.tournament_service
                    .get_user_registrations(user_id)
                    .await
                    .map_err(ReportError::from)
            },
            async {
                self.tournament_service
                    .get_user_attendance(user_id)
                    .await
                    .map_err(ReportError::from)
            },
            async {
                self.tuition_service
                    .get_user_tuitions(user_id)
                    .await
                    .map_err(ReportError::from)
            },
            async {
                self.request_service
                    .list_user_requests(user_id)
                    .await
                    .map_err(ReportError::from)
            },
        )?;

        let categories = categories
            .into_iter()
            .map(|uc| UserCategory {
                category_name: uc.id_category.to_string(),
                user_level: uc.user_level.to_string(),
            })
            .collect();

        let training_summary = TrainingSummary {
            total_registrations: training_registrations.len() as u32,
            total_attendances: training_registrations.iter().filter(|r| r.attended).count() as u32,
            most_recent_attendance: training_registrations
                .iter()
                .filter_map(|r| r.attendance_datetime.map(|dt| dt.date()))
                .max(),
        };

        let tournament_summary = TournamentSummary {
            total_registrations: tournament_registrations.len() as u32,
            total_attendances: tournament_attendances.len() as u32,
            most_recent_attendance: tournament_attendances
                .iter()
                .map(|a| a.attendance_datetime.date())
                .max(),
            most_recent_registration: tournament_registrations
                .iter()
                .map(|r| r.registration_datetime.date())
                .max(),
        };

        let tuition_summary = if let Some(last_tuition) = tuitions.last() {
            TuitionSummary {
                last_payment_amount: last_tuition.amount,
                last_payment_date: last_tuition.payment_date.date(),
                days_until_next_payment: 30,
                total_payments: tuitions.iter().map(|t| t.amount).sum(),
            }
        } else {
            TuitionSummary {
                last_payment_amount: 0.0,
                last_payment_date: user.registration_date.date(),
                days_until_next_payment: 0,
                total_payments: 0.0,
            }
        };

        let user_requests = requests
            .into_iter()
            .map(|r| UserRequest {
                request_id: r.request_id,
                requested_command: r.requested_command,
                state: match r.approved {
                    Some(true) => "APPROVED".to_string(),
                    Some(false) => "REJECTED".to_string(),
                    None => "PENDING".to_string(),
                },
            })
            .collect();

        Ok(Report {
            full_name: format!("{} {}", user.first_name, user.last_name),
            email: user.email,
            phone_number: format!("{} {}", user.country_code, user.phone_number),
            birth_date: user.birth_date,
            registration_date: user.registration_date.date(),
            categories,
            training_summary,
            tournament_summary,
            tuition_summary,
            requests: user_requests,
        })
    }
}
