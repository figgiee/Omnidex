use serde::{Deserialize, Serialize};

/// Orbital Market Asset data structure
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OrbitalAsset {
    pub id: Option<String>,
    pub product_slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub technical_details: Option<String>,
    pub seller: Option<String>,
    pub categories: Vec<String>,
    pub supported_versions: Vec<String>,
    pub gallery_images: Vec<String>,
    pub rating_average: Option<f64>,
    pub rating_count: Option<i32>,
    pub price: Option<f64>,
    pub release_date: Option<String>,
    pub last_modified: Option<String>,
    pub raw_json: Option<serde_json::Value>,
    pub source_url: Option<String>,
    pub thumbnail_url: Option<String>,
}

/// Represents the seller of an asset on Orbital Market
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrbitalSeller {
    pub id: String,
    pub name: String,
    pub display_name: String,
}

impl Default for OrbitalSeller {
    fn default() -> Self {
        Self {
            id: "unknown".to_string(),
            name: "Unknown Seller".to_string(),
            display_name: "Unknown Seller".to_string(),
        }
    }
}

/// Represents a category on Orbital Market
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrbitalCategory {
    pub id: String,
    pub name: String,
}

/// Represents an image in the asset gallery
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrbitalImage {
    pub url: String,
    pub alt_text: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

/// Represents the rating of an asset
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrbitalRating {
    pub average_rating: f32,
    pub total_ratings: i32,
}

/// Represents the price of an asset
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrbitalPrice {
    pub amount: f64,
    pub currency: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatchResult {
    pub local_asset_id: i32,
    pub orbital_asset: Option<OrbitalAsset>,
    pub match_strength: Option<String>,
    pub similarity_score: Option<f64>,
}

/// Asset matching result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMatch {
    pub local_asset_id: i64,
    pub orbital_asset: Option<OrbitalAsset>,
    pub match_confidence: f64,
    pub match_type: MatchType,
    pub match_reasons: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub enum MatchType {
    Exact,
    HighConfidence,
    MediumConfidence,
    LowConfidence,
    NoMatch,
}

/// GraphQL response structures
#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    pub locations: Option<Vec<GraphQLLocation>>,
    pub path: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLLocation {
    pub line: i32,
    pub column: i32,
}

/// Product details query response
#[derive(Debug, Deserialize)]
pub struct ProductDetailsResponse {
    pub product: Option<ProductDetails>,
}

#[derive(Debug, Deserialize)]
pub struct ProductDetails {
    pub id: String,
    #[serde(rename = "productSlug")]
    pub product_slug: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "technicalDetails")]
    pub technical_details: Option<String>,
    pub seller: SellerDetails,
    pub categories: Vec<CategoryDetails>,
    #[serde(rename = "compatibleApps")]
    pub compatible_apps: Vec<String>,
    #[serde(rename = "galleryImages")]
    pub gallery_images: Vec<ImageDetails>,
    pub rating: Option<RatingDetails>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    #[serde(rename = "lastModified")]
    pub last_modified: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct SellerDetails {
    pub id: String,
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CategoryDetails {
    pub id: String,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct ImageDetails {
    pub url: String,
    #[serde(rename = "type")]
    pub image_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct RatingDetails {
    #[serde(rename = "averageRating")]
    pub average_rating: f64,
    #[serde(rename = "totalRatings")]
    pub total_ratings: i32,
} 