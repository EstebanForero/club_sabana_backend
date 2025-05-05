use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct UserCategory {
    pub category_name: String,
    pub user_level: String, // e.g., "BEGINNER", "AMATEUR", "PROFESSIONAL"
}

#[derive(Debug, Serialize)]
pub struct TrainingSummary {
    pub total_registrations: u32,
    pub total_attendances: u32,
    pub most_recent_attendance: Option<NaiveDate>,
}

#[derive(Debug, Serialize)]
pub struct TournamentSummary {
    pub total_registrations: u32,
    pub total_attendances: u32,
    pub most_recent_attendance: Option<NaiveDate>,
    pub most_recent_registration: Option<NaiveDate>,
}

#[derive(Debug, Serialize)]
pub struct TuitionSummary {
    pub last_payment_amount: f64,
    pub last_payment_date: NaiveDate,
    pub days_until_next_payment: i64, // e.g., 30 days remaining
    pub total_payments: f64,
}

#[derive(Debug, Serialize)]
pub struct UserRequest {
    pub request_id: Uuid,
    pub requested_command: String,
    pub state: String, // e.g., "PENDING", "APPROVED", "REJECTED"
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub full_name: String,
    pub email: String,
    pub phone_number: String, // Includes country code (e.g., "+57 1234567890")
    pub birth_date: NaiveDate,
    pub registration_date: NaiveDate,

    pub categories: Vec<UserCategory>,
    pub training_summary: TrainingSummary,
    pub tournament_summary: TournamentSummary,
    pub tuition_summary: TuitionSummary,
    pub requests: Vec<UserRequest>,
}
