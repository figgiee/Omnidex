use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(FromRow)]
pub struct Asset {
    pub id: Option<i64>,
    pub name: String,
    pub file_path: String,
    pub asset_type: String,
    pub file_size: i64,
    pub created_date: String,
    pub modified_date: String,
    pub first_indexed_timestamp: Option<String>,
    pub thumbnail_path: Option<String>,
    pub tags: Option<String>, // JSON string of tags array
    pub description: Option<String>,
    pub scan_location_id: i64,
    pub is_favorite: bool,
    pub last_accessed: Option<String>,
    pub file_hash: Option<String>,
    pub metadata: Option<String>, // JSON string for additional metadata

    // Fields for Orbital asset matching & details
    pub orbital_title: Option<String>,
    pub orbital_description: Option<String>,
    pub orbital_technical_details: Option<String>,
    pub orbital_seller_name: Option<String>,
    pub orbital_price: Option<f64>,
    pub orbital_release_date: Option<String>,
    pub orbital_last_modified: Option<String>,
    pub orbital_rating_average: Option<f64>,
    pub orbital_rating_count: Option<i32>,
    pub orbital_categories: Option<String>, // JSON string
    pub orbital_supported_versions: Option<String>, // JSON string
    pub orbital_gallery_images: Option<String>, // JSON string
    pub orbital_thumbnail_url: Option<String>,
    pub orbital_source_url: Option<String>,
    pub orbital_raw_json: Option<String>,
    pub orbital_last_checked_timestamp: Option<String>,

    // Fields for linking to a matched Orbital asset
    pub matched_orbital_product_slug: Option<String>,
    pub orbital_match_confidence: Option<f64>,
    pub orbital_match_type: Option<String>, // e.g., "Exact", "High", "Manual"

    pub notes: Option<String>,
    pub orbital_manual_overrides: Option<String>, // JSON string for manual edits
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetFilter {
    pub asset_type: Option<String>,
    pub tags: Option<Vec<String>>,
    pub name_search: Option<String>,
    pub is_favorite: Option<bool>,
    pub scan_location_id: Option<i64>,
    pub date_range: Option<DateRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetStats {
    pub total_assets: i64,
    pub total_size: i64,
    pub asset_type_counts: Vec<AssetTypeCount>,
    pub recent_assets: Vec<Asset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(FromRow)]
pub struct AssetTypeCount {
    pub asset_type: String,
    pub count: i64,
}

 