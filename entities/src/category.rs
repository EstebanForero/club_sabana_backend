use enum2str::EnumStr;
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
    pub level_name: LevelName,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryRequirement {
    pub id_category_requirement: Uuid,
    pub id_category: Uuid,
    pub requirement_description: String,
    pub required_level: LevelName,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, EnumStr)]
pub enum LevelName {
    BEGGINER,
    AMATEUR,
    PROFESSIONAL,
}
