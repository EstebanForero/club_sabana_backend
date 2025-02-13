use enum2str::EnumStr;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Partial)]
#[partial(
    "CategoryCreation",
    derive(Debug, Serialize, Deserialize, Partial),
    omit(id_category)
)]
pub struct Category {
    pub id_category: Uuid,
    pub name: String,
    pub min_age: i32,
    pub max_age: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Level {
    pub level_name: LevelName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryRequirement {
    pub id_category_requirement: Uuid,
    pub id_category: Uuid,
    pub requirement_description: String,
    pub required_level: LevelName,
}

#[derive(Debug, Serialize, Deserialize, EnumStr)]
pub enum LevelName {
    BEGGINER,
    AMATEUR,
    PROFESSIONAL,
}
