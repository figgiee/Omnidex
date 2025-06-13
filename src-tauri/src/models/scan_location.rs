use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(FromRow)]
pub struct ScanLocation {
    pub id: Option<i64>,
    pub name: String,
    pub path: String,
    pub is_active: bool,
    pub last_scan: Option<String>,
    pub scan_recursive: bool,
    pub file_extensions: Option<String>, // JSON string of extensions array
    pub created_date: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub location_id: i64,
    pub status: String,
    pub current_path: String,
    pub processed_items: u64,
    pub total_items: u64,
    pub completed_successfully: bool,
    pub error: Option<String>,
} 