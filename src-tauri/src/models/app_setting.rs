use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AppSetting {
    pub id: Option<i64>,
    pub key: String,
    pub value: String,
    pub setting_type: String, // "string", "number", "boolean", "json"
    pub description: Option<String>,
    pub created_date: String,
    pub modified_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub auto_scan: bool,
    pub scan_interval_minutes: i32,
    pub thumbnail_size: String,
    pub show_hidden_files: bool,
    pub default_view: String,
    pub backup_enabled: bool,
    pub backup_location: Option<String>,
} 