use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id_category: Uuid,
    pub name: String,
    pub min_age: i32,
    pub max_age: i32,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Level {
    pub level_name: Uuid,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryRequirement {
    pub id_category_requirement: Uuid,
    pub id_category: Uuid,
    pub requirement_description: String,
    pub required_level: Uuid,
    pub deleted: bool,
}
