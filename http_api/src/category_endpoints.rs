use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use entities::{
    category::{Category, CategoryCreation, CategoryRequirement, LevelName},
    user::UserCategory,
};
use tracing::error;
use use_cases::category_service::{err::Error, CategoryService};
use uuid::Uuid;

use super::err::{HttpError, ToErrResponse};
use crate::err::HttpResult;

pub fn category_router(category_service: CategoryService) -> Router {
    Router::new()
        .route("/health-category", get(alive))
        .route(
            "/categories",
            post(create_category)
                .get(list_categories)
                .put(update_category),
        )
        .route(
            "/categories/{id}",
            get(get_category).delete(delete_category),
        )
        .route(
            "/categories/{id}/requirements",
            post(add_requirement).get(get_requirements),
        )
        .route(
            "/categories/{id}/requirements/{requirement_id}",
            delete(remove_requirement),
        )
        .route(
            "/categories/{id}/user/{id}/level/{level}",
            put(update_user_category_level),
        )
        .route(
            "/categories/{category_id}/user/{user_id}",
            delete(delete_user_from_category_endpoint),
        )
        .route(
            "/categories/{category_id}/users/{user_id}",
            post(register_user_in_category).get(get_user_category),
        )
        .route(
            "/categories/{category_id}/users/{user_id}/eligible",
            get(check_user_eligibility),
        )
        .route("/categories/user/{user_id}", get(get_user_categories))
        .with_state(category_service)
}

async fn alive() -> &'static str {
    "Category service is alive"
}

async fn check_user_eligibility(
    State(category_service): State<CategoryService>,
    Path((category_id, user_id)): Path<(Uuid, Uuid)>,
) -> HttpResult<Json<bool>> {
    category_service
        .is_user_eligible_for_category(user_id, category_id)
        .await
        .http_err("check user eligibility")?;

    Ok(Json(true))
}

async fn delete_user_from_category_endpoint(
    State(category_service): State<CategoryService>,
    Path((category_id, user_id)): Path<(Uuid, Uuid)>,
) -> HttpResult<impl IntoResponse> {
    category_service
        .delete_user_from_category(user_id, category_id)
        .await
        .http_err("Failed to remove user from category")?;

    Ok((StatusCode::OK, "User successfully removed from category."))
}

async fn update_user_category_level(
    State(category_service): State<CategoryService>,
    Path((category_id, user_id, level_name)): Path<(Uuid, Uuid, LevelName)>,
) -> HttpResult<impl IntoResponse> {
    category_service
        .update_user_category_level(user_id, category_id, level_name)
        .await
        .http_err("error, update user category level")?;

    Ok((StatusCode::OK, "User registered in category successfully"))
}

async fn register_user_in_category(
    State(category_service): State<CategoryService>,
    Path((category_id, user_id)): Path<(Uuid, Uuid)>,
) -> HttpResult<impl IntoResponse> {
    category_service
        .add_user_to_category(user_id, category_id)
        .await
        .http_err("register user in category")?;

    Ok((StatusCode::OK, "User registered in category successfully"))
}

async fn create_category(
    State(category_service): State<CategoryService>,
    Json(category): Json<CategoryCreation>,
) -> HttpResult<impl IntoResponse> {
    category_service
        .add_category(category)
        .await
        .http_err("create category")?;

    Ok((StatusCode::OK, "Category created successfully"))
}

async fn get_category(
    State(category_service): State<CategoryService>,
    Path(id): Path<Uuid>,
) -> HttpResult<Json<Category>> {
    let category = category_service
        .get_category_by_id(id)
        .await
        .http_err("get category")?;

    Ok(Json(category))
}

async fn update_category(
    State(category_service): State<CategoryService>,
    Json(category): Json<Category>,
) -> Result<Json<Category>, Response> {
    category_service
        .update_category(&category)
        .await
        .http_err("update category")?;

    Ok(Json(category))
}

async fn delete_category(
    State(category_service): State<CategoryService>,
    Path(id): Path<Uuid>,
) -> Result<Json<String>, Response> {
    category_service
        .delete_category(id)
        .await
        .http_err("delete category")?;

    Ok(Json("Category deleted successfully".to_string()))
}

async fn list_categories(
    State(category_service): State<CategoryService>,
) -> Result<Json<Vec<Category>>, Response> {
    let categories = category_service
        .get_all_categories()
        .await
        .http_err("list categories")?;

    Ok(Json(categories))
}

async fn add_requirement(
    State(category_service): State<CategoryService>,
    Json(requirement): Json<CategoryRequirement>,
) -> Result<Json<CategoryRequirement>, Response> {
    category_service
        .add_category_requirement(&requirement)
        .await
        .http_err("add requirement")?;

    Ok(Json(requirement))
}

async fn remove_requirement(
    State(category_service): State<CategoryService>,
    Path((category_id, category_requirement_id)): Path<(Uuid, Uuid)>,
) -> Result<(), Response> {
    category_service
        .delete_category_requirement(&category_requirement_id, &category_id)
        .await
        .http_err("add requirement")?;

    Ok(())
}

async fn get_requirements(
    State(category_service): State<CategoryService>,
    Path(category_id): Path<Uuid>,
) -> Result<Json<Vec<CategoryRequirement>>, Response> {
    let requirements = category_service
        .get_category_requirements(category_id)
        .await
        .http_err("get requirements")?;

    Ok(Json(requirements))
}

async fn get_user_category(
    State(category_service): State<CategoryService>,
    Path((category_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Option<UserCategory>>, Response> {
    let user_category = category_service
        .get_user_category(user_id, category_id)
        .await
        .http_err("get user category")?;

    Ok(Json(user_category))
}

async fn get_user_categories(
    State(category_service): State<CategoryService>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<UserCategory>>, Response> {
    let user_category = category_service
        .get_user_categories(user_id)
        .await
        .http_err("get user categories")?;

    Ok(Json(user_category))
}

impl<T> HttpError<T> for use_cases::category_service::err::Result<T> {
    fn http_err(self, endpoint_name: &str) -> crate::err::HttpResult<T> {
        self.map_err(|err| {
            error!("Error in: {endpoint_name}");
            match err {
                Error::UnknownDatabaseError(error) => {
                    error!("{error}");
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
                Error::InvalidUserAge => {
                    "The age of the user is not between the specified category range"
                }
                Error::InvalidRequirementLevel => {
                    "The user don't have the necesary level in one of it's category requirements"
                }
                Error::UserServiceError(_) => "Error with the user service",
            }
            .to_err_response()
        })
    }
}
