use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use entities::{
    category::{Category, CategoryRequirement},
    user::UserCategory,
};
use tracing::error;
use use_cases::category_service::{err::Error, CategoryService};
use uuid::Uuid;

fn internal_error_response(message: &str) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, message.to_string()).into_response()
}

pub fn category_router(category_service: CategoryService) -> Router {
    Router::new()
        .route("/health-category", get(alive))
        .route("/categories", post(create_category).get(list_categories))
        .route(
            "/categories/:id",
            get(get_category)
                .put(update_category)
                .delete(delete_category),
        )
        .route(
            "/categories/:id/requirements",
            post(add_requirement).get(get_requirements),
        )
        .route("/categories/:id/users/:user_id", get(get_user_category))
        .with_state(category_service)
}

async fn alive() -> &'static str {
    "Category service is alive"
}

async fn create_category(
    State(category_service): State<CategoryService>,
    Json(category): Json<Category>,
) -> Result<(), Response> {
    category_service
        .add_category(category)
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "create category")))?;

    Ok(())
}

async fn get_category(
    State(category_service): State<CategoryService>,
    Path(id): Path<Uuid>,
) -> Result<Json<Category>, Response> {
    let category = category_service
        .get_category_by_id(id)
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "get category")))?;

    Ok(Json(category))
}

async fn update_category(
    State(category_service): State<CategoryService>,
    Path(id): Path<Uuid>,
    Json(category): Json<Category>,
) -> Result<Json<Category>, Response> {
    category_service
        .update_category(&category)
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "update category")))?;

    Ok(Json(category))
}

async fn delete_category(
    State(category_service): State<CategoryService>,
    Path(id): Path<Uuid>,
) -> Result<Json<String>, Response> {
    category_service
        .delete_category(id)
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "delete category")))?;

    Ok(Json("Category deleted successfully".to_string()))
}

async fn list_categories(
    State(category_service): State<CategoryService>,
) -> Result<Json<Vec<Category>>, Response> {
    let categories = category_service
        .get_all_categories()
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "list categories")))?;

    Ok(Json(categories))
}

async fn add_requirement(
    State(category_service): State<CategoryService>,
    Path(category_id): Path<Uuid>,
    Json(requirement): Json<CategoryRequirement>,
) -> Result<Json<CategoryRequirement>, Response> {
    category_service
        .add_category_requirement(&requirement)
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "add requirement")))?;

    Ok(Json(requirement))
}

async fn get_requirements(
    State(category_service): State<CategoryService>,
    Path(category_id): Path<Uuid>,
) -> Result<Json<Vec<CategoryRequirement>>, Response> {
    let requirements = category_service
        .get_category_requirements(category_id)
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "get requirements")))?;

    Ok(Json(requirements))
}

async fn get_user_category(
    State(category_service): State<CategoryService>,
    Path((category_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Option<UserCategory>>, Response> {
    let user_category = category_service
        .get_user_category(user_id, category_id)
        .await
        .map_err(|err| internal_error_response(&message_from_err(err, "get user category")))?;

    Ok(Json(user_category))
}

fn message_from_err(err: Error, endpoint_name: &str) -> String {
    let error_msg = match err {
        Error::UnknownDatabaseError(error) => {
            error!("{endpoint_name}: {error}");
            "We are having problems in the server, try again"
        }
        Error::CategoryNotFound => "Category not found",
        Error::CategoryAlreadyExists => "Category already exists",
        Error::InvalidAgeRange => "Invalid age range",
        Error::MissingName => "Category name is required",
        Error::RequirementNotFound => "Category requirement not found",
        Error::UserAlreadyHasCategory => "User already has this category",
        Error::UserDoesNotMeetRequirements => "User does not meet category requirements",
        Error::LevelNotFound => "Level not found",
    };

    error!("error in {endpoint_name}: {error_msg}");

    error_msg.to_string()
}
