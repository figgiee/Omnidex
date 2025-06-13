use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub title: String,
    pub category: String,
    pub computed: Computed,
    pub discount: i32,
    pub engine: Engine,
    pub media: Media,
    pub meta: Meta,
    pub owner: Owner,
    pub price: Price,
    #[serde(rename = "releaseDate")]
    pub release_date: String,
    pub review: Review,
    pub slug: String,
    pub description: Description,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Computed {
    #[serde(rename = "embeddedContent")]
    pub embedded_content: Vec<String>,
    #[serde(rename = "isBoosted")]
    pub is_boosted: bool,
    pub score: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Engine {
    pub max: String,
    pub min: String,
    #[serde(rename = "_id")]
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Media {
    pub thumbnail: String,
    pub images: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    #[serde(rename = "fabId")]
    pub fab_id: String,
    #[serde(rename = "unrealId")]
    pub unreal_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Owner {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub name: String,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Price {
    pub history: Vec<PriceHistory>,
    pub value: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceHistory {
    pub date: String,
    pub value: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Review {
    pub count: i32,
    pub rating: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Description {
    pub long: String,
    pub technical: String,
} 