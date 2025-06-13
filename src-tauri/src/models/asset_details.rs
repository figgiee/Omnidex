use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetDetails {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetForFrontend {
    pub id: Option<i64>,
    pub name: String,
    pub file_path: String,
    pub asset_type: String,
    pub file_size: i64,
    pub created_date: String,
    pub modified_date: String,
    pub first_indexed_timestamp: Option<String>,
    pub thumbnail_path: Option<String>,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub scan_location_id: i64,
    pub is_favorite: bool,
    pub last_accessed: Option<String>,
    pub file_hash: Option<String>,
    pub metadata: Option<String>,

    pub orbital_title: Option<String>,
    pub orbital_description: Option<String>,
    pub orbital_technical_details: Option<String>,
    pub orbital_seller_name: Option<String>,
    pub orbital_price: Option<f64>,
    pub orbital_release_date: Option<String>,
    pub orbital_last_modified: Option<String>,
    pub orbital_rating_average: Option<f64>,
    pub orbital_rating_count: Option<i32>,
    pub orbital_categories: Option<String>,
    pub orbital_supported_versions: Option<String>,
    pub orbital_gallery_images: Option<String>,
    pub orbital_thumbnail_url: Option<String>,
    pub orbital_source_url: Option<String>,
    pub orbital_raw_json: Option<String>,
    pub orbital_last_checked_timestamp: Option<String>,

    pub matched_orbital_product_slug: Option<String>,
    pub orbital_match_confidence: Option<f64>,
    pub orbital_match_type: Option<String>,

    pub notes: Option<String>,
    pub orbital_manual_overrides: Option<String>,
} 