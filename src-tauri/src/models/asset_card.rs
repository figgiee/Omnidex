use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(FromRow)]
pub struct AssetCardData {
    pub id: i64,
    pub name: String,
    pub asset_type: String,
    pub file_size: i64,
    pub thumbnail_path: Option<String>,
    pub orbital_thumbnail_url: Option<String>,
    pub orbital_rating_average: Option<f64>,
    pub orbital_rating_count: Option<i32>,
    pub is_favorite: bool,
    pub created_date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SortOption {
    NameAsc,
    NameDesc,
    DateAsc,
    DateDesc,
    DateAddedAsc,
    DateAddedDesc,
}

impl Default for SortOption {
    fn default() -> Self {
        SortOption::DateDesc
    }
}

impl SortOption {
    pub fn to_sql(&self) -> &'static str {
        match self {
            SortOption::NameAsc => "name ASC",
            SortOption::NameDesc => "name DESC",
            SortOption::DateAsc => "modified_date ASC",
            SortOption::DateDesc => "modified_date DESC",
            SortOption::DateAddedAsc => "created_date ASC",
            SortOption::DateAddedDesc => "created_date DESC",
        }
    }
} 