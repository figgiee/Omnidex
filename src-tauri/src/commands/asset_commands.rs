use tauri::State;
use std::collections::HashSet;
use crate::orbital::api::OrbitalApiClient;
use crate::models::asset::Asset;
use url;

/// Toggles the favorite status for multiple assets
/// 
/// # Arguments
/// * `asset_ids` - Vector of asset IDs to update
/// * `is_favorited` - Whether to mark assets as favorites (true) or remove favorite status (false)
/// 
/// # Returns
/// * `Ok(())` on success
/// * `Err(String)` containing error message on failure
/// 
/// # Security
/// Uses parameterized queries to prevent SQL injection attacks
#[tauri::command]
pub async fn toggle_favorite_status(
    asset_ids: Vec<i64>,
    is_favorited: bool,
) -> Result<(), String> {
    if asset_ids.is_empty() {
        return Ok(());
    }

    let db = crate::database::get_db();
    
    // Use parameterized query with ? placeholders to prevent SQL injection
    let placeholders = asset_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "UPDATE assets SET is_favorite = ? WHERE id IN ({})",
        placeholders
    );

    let mut sqlx_query = sqlx::query(&query);
    sqlx_query = sqlx_query.bind(if is_favorited { 1 } else { 0 });
    
    for id in asset_ids {
        sqlx_query = sqlx_query.bind(id);
    }

    sqlx_query
        .execute(db)
        .await
        .map_err(|e| format!("Failed to update favorite status: {}", e))?;

    Ok(())
}

/// Adds tags to multiple assets, merging with existing tags
/// 
/// # Arguments
/// * `asset_ids` - Vector of asset IDs to add tags to
/// * `tags` - Vector of tag strings to add
/// 
/// # Returns
/// * `Ok(())` on success
/// * `Err(String)` containing error message on failure
/// 
/// # Behavior
/// - Existing tags are preserved and merged with new tags
/// - Duplicate tags are automatically removed
/// - Tags are trimmed of whitespace
/// - Empty asset_ids or tags vectors are handled gracefully
/// 
/// # Security
/// Uses parameterized queries to prevent SQL injection attacks
#[tauri::command]
pub async fn add_tags_to_assets(
    asset_ids: Vec<i64>,
    tags: Vec<String>,
) -> Result<(), String> {
    if asset_ids.is_empty() || tags.is_empty() {
        return Ok(());
    }

    let db = crate::database::get_db();
    
    // Use parameterized query with ? placeholders to prevent SQL injection
    let placeholders = asset_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!("SELECT id, tags FROM assets WHERE id IN ({})", placeholders);

    let mut sqlx_query = sqlx::query_as::<_, (i64, Option<String>)>(&query);
    for id in &asset_ids {
        sqlx_query = sqlx_query.bind(id);
    }

    let assets: Vec<(i64, Option<String>)> = sqlx_query
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to fetch assets for tagging: {}", e))?;

    for (id, current_tags) in assets {
        let mut existing_tags: HashSet<String> = current_tags
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        for tag in &tags {
            existing_tags.insert(tag.trim().to_string());
        }

        let new_tags = existing_tags.into_iter().collect::<Vec<String>>().join(",");

        sqlx::query("UPDATE assets SET tags = ? WHERE id = ?")
            .bind(&new_tags)
            .bind(id)
            .execute(db)
            .await
            .map_err(|e| format!("Failed to update tags for asset {}: {}", id, e))?;
    }

    Ok(())
}

/// Manually matches an asset with an Orbital marketplace product
/// 
/// # Arguments
/// * `asset_id` - ID of the local asset to match
/// * `url` - Orbital marketplace URL for the product (must be from orbital-market.com)
/// * `orbital_client` - Orbital API client for fetching product details
/// 
/// # Returns
/// * `Ok(())` on success
/// * `Err(String)` containing error message on failure
/// 
/// # Security
/// - Validates URL format and domain to ensure it's from orbital-market.com only
/// - Prevents potential security issues from malicious URLs
/// 
/// # Behavior
/// - Extracts product ID from the provided URL
/// - Fetches detailed product information from Orbital API
/// - Updates local asset with Orbital product details
/// - Sets match confidence to 100% since it's a manual match
/// - Marks the match type as "Manual"
#[tauri::command]
pub async fn match_asset_manually(
    asset_id: i64,
    url: String,
    orbital_client: State<'_, OrbitalApiClient>,
) -> Result<(), String> {
    // Validate URL to ensure it's from orbital-market.com
    let parsed_url = url::Url::parse(&url)
        .map_err(|_| "Invalid URL format".to_string())?;
    
    if parsed_url.host_str() != Some("orbital-market.com") {
        return Err("URL must be from orbital-market.com".to_string());
    }

    let product_id = orbital_client
        .extract_product_id_from_url(&url)
        .ok_or("Could not extract product ID from URL".to_string())?;

    let orbital_asset = orbital_client
        .fetch_asset_details(&product_id)
        .await
        .map_err(|e| format!("Failed to fetch asset details from Orbital: {}", e))?;

    let db = crate::database::get_db();

    // Check if the asset exists
    let asset: Option<Asset> = sqlx::query_as("SELECT * FROM assets WHERE id = ?")
        .bind(asset_id)
        .fetch_optional(db)
        .await
        .map_err(|e| format!("Failed to check if asset exists: {}", e))?;

    if asset.is_none() {
        return Err(format!("Asset with ID {} not found", asset_id));
    }

    // Get the database manager for the orbital updates
    let db_manager = crate::database::DatabaseManager::new().await
        .map_err(|e| format!("Failed to get database manager: {}", e))?;

    // Update the asset with orbital details
    db_manager
        .update_asset_with_orbital_details(asset_id, &orbital_asset)
        .await
        .map_err(|e| format!("Failed to update asset with Orbital details: {}", e))?;

    // Mark the asset as manually matched
    db_manager
        .update_asset_orbital_match_details(
            asset_id,
            orbital_asset.product_slug.clone(),
            Some(1.0), // Manual matches get 100% confidence
            Some("Manual".to_string()),
        )
        .await
        .map_err(|e| format!("Failed to update asset with match details: {}", e))?;

    Ok(())
}

/// Permanently deletes multiple assets from the database
/// 
/// # Arguments
/// * `asset_ids` - Vector of asset IDs to delete
/// 
/// # Returns
/// * `Ok(())` on success
/// * `Err(String)` containing error message on failure
/// 
/// # Warning
/// This operation is permanent and cannot be undone. All asset data will be lost.
/// 
/// # Security
/// Uses parameterized queries to prevent SQL injection attacks
#[tauri::command]
pub async fn delete_assets(
    asset_ids: Vec<i64>,
) -> Result<(), String> {
    if asset_ids.is_empty() {
        return Ok(());
    }

    let db = crate::database::get_db();
    
    // Use parameterized query with ? placeholders to prevent SQL injection
    let placeholders = asset_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "DELETE FROM assets WHERE id IN ({})",
        placeholders
    );

    let mut sqlx_query = sqlx::query(&query);
    for id in asset_ids {
        sqlx_query = sqlx_query.bind(id);
    }

    sqlx_query
        .execute(db)
        .await
        .map_err(|e| format!("Failed to delete assets: {}", e))?;

    Ok(())
} 