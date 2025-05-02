use std::cmp::Ordering;

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

#[derive(Debug, Serialize, Deserialize, EnumStr, PartialEq)]
pub enum LevelName {
    BEGGINER,
    AMATEUR,
    PROFESSIONAL,
}

impl LevelName {
    fn value(&self) -> u8 {
        match self {
            LevelName::BEGGINER => 0,
            LevelName::AMATEUR => 1,
            LevelName::PROFESSIONAL => 2,
        }
    }
}

impl PartialOrd for LevelName {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other)) // Delegate to the full Ord implementation
    }
}

impl Eq for LevelName {}

impl Ord for LevelName {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}
