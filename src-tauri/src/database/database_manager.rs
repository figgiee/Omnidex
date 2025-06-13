use sqlx::{Pool, Sqlite};
use std::sync::OnceLock;
use anyhow::Result;
use crate::models::{Asset, ScanLocation};
use crate::orbital::models::OrbitalAsset;
use chrono::Utc;
use super::create_tables;

pub static DB_POOL: OnceLock<Pool<Sqlite>> = OnceLock::new();

// Define DatabaseManager struct
#[derive(Clone)]
pub struct DatabaseManager {
    pub pool: &'static Pool<Sqlite>,
}

impl DatabaseManager {
    pub async fn new() -> Result<Self, String> {
        match DB_POOL.get() {
            Some(pool) => Ok(Self { pool }),
            None => Err("Database pool not initialized. Call init_database first.".to_string()),
        }
    }

    pub async fn get_asset_by_id(&self, asset_id: i64) -> Result<Asset> {
        sqlx::query_as::<_, Asset>("SELECT * FROM assets WHERE id = ?")
            .bind(asset_id)
            .fetch_one(self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Asset with id {} not found: {}", asset_id, e))
    }

    pub async fn get_scan_location_by_id(&self, location_id: i64) -> Result<Option<ScanLocation>> {
        sqlx::query_as::<_, ScanLocation>("SELECT * FROM scan_locations WHERE id = ?")
            .bind(location_id)
            .fetch_optional(self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Error fetching ScanLocation with id {}: {}", location_id, e))
    }

    pub async fn get_asset_by_path(&self, file_path: &str) -> Result<Option<Asset>> {
        sqlx::query_as::<_, Asset>("SELECT * FROM assets WHERE file_path = ?")
            .bind(file_path)
            .fetch_optional(self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Error fetching asset by path {}: {}", file_path, e))
    }

    pub async fn wipe_database(&self) -> Result<()> {
        sqlx::query("DROP TABLE IF EXISTS assets")
            .execute(self.pool)
            .await?;
        sqlx::query("DROP TABLE IF EXISTS scan_locations")
            .execute(self.pool)
            .await?;
        sqlx::query("DROP TABLE IF EXISTS app_settings")
            .execute(self.pool)
            .await?;

        create_tables(self.pool).await?;

        Ok(())
    }

    pub async fn update_scan_location_last_scanned(&self, location_id: i64) -> Result<()> {
        let now_timestamp = chrono::Utc::now().to_rfc3339();
        sqlx::query("UPDATE scan_locations SET last_scan = ? WHERE id = ?")
            .bind(now_timestamp)
            .bind(location_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_asset_orbital_match_details(
        &self,
        asset_id: i64,
        product_slug: Option<String>,
        confidence: Option<f64>,
        match_type_str: Option<String>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE assets
            SET matched_orbital_product_slug = ?,
                orbital_match_confidence = ?,
                orbital_match_type = ?
            WHERE id = ?
            "#
        )
        .bind(product_slug)
        .bind(confidence)
        .bind(match_type_str)
        .bind(asset_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_asset(&self, asset: &Asset) -> Result<i64> {
        let result = sqlx::query(
            r#"
            INSERT INTO assets (
                name, file_path, asset_type, file_size, created_date, modified_date,
                first_indexed_timestamp, thumbnail_path, tags, description,
                scan_location_id, is_favorite, last_accessed, file_hash, metadata,
                orbital_title, orbital_description, orbital_technical_details, orbital_seller_name,
                orbital_price, orbital_release_date, orbital_last_modified, orbital_rating_average,
                orbital_rating_count, orbital_categories, orbital_supported_versions, orbital_gallery_images,
                orbital_thumbnail_url, orbital_source_url, orbital_raw_json, orbital_last_checked_timestamp,
                matched_orbital_product_slug, orbital_match_confidence, orbital_match_type,
                notes, orbital_manual_overrides
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            "#
        )
        .bind(&asset.name)
        .bind(&asset.file_path)
        .bind(&asset.asset_type)
        .bind(asset.file_size)
        .bind(&asset.created_date)
        .bind(&asset.modified_date)
        .bind(asset.first_indexed_timestamp.as_deref())
        .bind(asset.thumbnail_path.as_deref())
        .bind(asset.tags.as_deref())
        .bind(asset.description.as_deref())
        .bind(asset.scan_location_id)
        .bind(asset.is_favorite)
        .bind(asset.last_accessed.as_deref())
        .bind(asset.file_hash.as_deref())
        .bind(asset.metadata.as_deref())
        .bind(asset.orbital_title.as_deref())
        .bind(asset.orbital_description.as_deref())
        .bind(asset.orbital_technical_details.as_deref())
        .bind(asset.orbital_seller_name.as_deref())
        .bind(asset.orbital_price)
        .bind(asset.orbital_release_date.as_deref())
        .bind(asset.orbital_last_modified.as_deref())
        .bind(asset.orbital_rating_average)
        .bind(asset.orbital_rating_count)
        .bind(asset.orbital_categories.as_deref())
        .bind(asset.orbital_supported_versions.as_deref())
        .bind(asset.orbital_gallery_images.as_deref())
        .bind(asset.orbital_thumbnail_url.as_deref())
        .bind(asset.orbital_source_url.as_deref())
        .bind(asset.orbital_raw_json.as_deref())
        .bind(asset.orbital_last_checked_timestamp.as_deref())
        .bind(asset.matched_orbital_product_slug.as_deref())
        .bind(asset.orbital_match_confidence)
        .bind(asset.orbital_match_type.as_deref())
        .bind(asset.notes.as_deref())
        .bind(asset.orbital_manual_overrides.as_deref())
        .execute(self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Fetch all assets that have cached raw Orbital JSON data
    pub async fn get_all_assets_with_raw_orbital_json(&self) -> Result<Vec<Asset>> {
        sqlx::query_as::<_, Asset>(
            "SELECT * FROM assets WHERE orbital_raw_json IS NOT NULL"
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Error fetching assets with raw JSON: {}", e))
    }

    /// Update the `orbital_manual_overrides` column for a specific asset
    pub async fn update_asset_manual_overrides(&self, asset_id: i64, overrides_json: &str) -> Result<()> {
        sqlx::query(
            "UPDATE assets SET orbital_manual_overrides = ?, modified_date = ? WHERE id = ?"
        )
        .bind(overrides_json)
        .bind(Utc::now().to_rfc3339())
        .bind(asset_id)
        .execute(self.pool)
        .await
        .map(|_| ())
        .map_err(|e| anyhow::anyhow!("Failed to update manual overrides: {}", e))
    }

    /// Delegate to the helper to update all Orbital-derived columns for an asset
    pub async fn update_asset_with_orbital_details(&self, asset_id: i64, orbital_data: &OrbitalAsset) -> Result<()> {
        crate::database::orbital_helpers::update_asset_with_orbital_details(self, asset_id, orbital_data).await
    }
}