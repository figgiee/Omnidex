use anyhow::Result;
use chrono::Utc;

use crate::database::DatabaseManager;
use crate::orbital::models::OrbitalAsset;

/// Update a single asset row with all metadata extracted from an OrbitalAsset struct.
/// Vector fields are stored as JSON strings, timestamps are refreshed, and the raw JSON is cached.
pub async fn update_asset_with_orbital_details(
    db_manager: &DatabaseManager,
    asset_id: i64,
    orbital_data: &OrbitalAsset,
) -> Result<()> {
    // Convert Vec<String> fields to JSON strings so we can persist them in TEXT columns
    let categories_json = if !orbital_data.categories.is_empty() {
        serde_json::to_string(&orbital_data.categories).ok()
    } else {
        None
    };
    let versions_json = if !orbital_data.supported_versions.is_empty() {
        serde_json::to_string(&orbital_data.supported_versions).ok()
    } else {
        None
    };
    let gallery_images_json = if !orbital_data.gallery_images.is_empty() {
        serde_json::to_string(&orbital_data.gallery_images).ok()
    } else {
        None
    };

    let now_timestamp = Utc::now().to_rfc3339();
    let raw_json = serde_json::to_string(orbital_data).ok();

    sqlx::query(
        r#"UPDATE assets SET
            orbital_title = ?,
            orbital_description = ?,
            orbital_technical_details = ?,
            orbital_seller_name = ?,
            orbital_price = ?,
            orbital_release_date = ?,
            orbital_last_modified = ?,
            orbital_rating_average = ?,
            orbital_rating_count = ?,
            orbital_categories = ?,
            orbital_supported_versions = ?,
            orbital_gallery_images = ?,
            orbital_thumbnail_url = ?,
            orbital_source_url = ?,
            orbital_raw_json = ?,
            orbital_last_checked_timestamp = ?,
            matched_orbital_product_slug = ?,
            orbital_match_confidence = ?,
            orbital_match_type = ?
        WHERE id = ?"#,
    )
    .bind(orbital_data.title.as_deref())
    .bind(orbital_data.description.as_deref())
    .bind(orbital_data.technical_details.as_deref())
    .bind(orbital_data.seller.as_deref())
    .bind(orbital_data.price)
    .bind(orbital_data.release_date.as_deref())
    .bind(orbital_data.last_modified.as_deref())
    .bind(orbital_data.rating_average)
    .bind(orbital_data.rating_count)
    .bind(categories_json)
    .bind(versions_json)
    .bind(gallery_images_json)
    .bind(orbital_data.thumbnail_url.as_deref())
    .bind(orbital_data.source_url.as_deref())
    .bind(raw_json)
    .bind(now_timestamp)
    .bind(orbital_data.product_slug.as_deref())
    .bind(None::<f64>)
    .bind(None::<String>)
    .bind(asset_id)
    .execute(db_manager.pool)
    .await?;

    Ok(())
} 