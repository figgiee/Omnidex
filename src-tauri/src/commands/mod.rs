pub mod asset_commands;
pub mod orbital_commands;
pub mod scan_commands;
pub mod system_commands;

use crate::database::get_db;
use crate::models::asset::{Asset, AssetFilter};
use crate::models::asset_card::{AssetCardData, SortOption};
use crate::models::asset_details::AssetForFrontend;
use std::collections::HashMap;

// Re-export commands from other modules
pub use asset_commands::{toggle_favorite_status, add_tags_to_assets, delete_assets, match_asset_manually};
pub use orbital_commands::*;
pub use scan_commands::*;
pub use system_commands::*;

// Asset querying and search commands that remain in mod.rs
#[tauri::command]
pub async fn get_assets(
    filter: Option<AssetFilter>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Asset>, String> {
    let db = get_db();
    let limit = limit.unwrap_or(100);
    let offset = offset.unwrap_or(0);
    
    let mut query = "SELECT * FROM assets WHERE 1=1".to_string();
    let mut params: Vec<String> = Vec::new();
    
    if let Some(filter) = filter {
        if let Some(asset_type) = filter.asset_type {
            query.push_str(" AND asset_type = ?");
            params.push(asset_type);
        }
        
        if let Some(name_search) = filter.name_search {
            query.push_str(" AND name LIKE ?");
            params.push(format!("%{}%", name_search));
        }
        
        if let Some(is_favorite) = filter.is_favorite {
            query.push_str(" AND is_favorite = ?");
            params.push(is_favorite.to_string());
        }
        
        if let Some(scan_location_id) = filter.scan_location_id {
            query.push_str(" AND scan_location_id = ?");
            params.push(scan_location_id.to_string());
        }
    }
    
    query.push_str(" ORDER BY created_date DESC LIMIT ? OFFSET ?");
    params.push(limit.to_string());
    params.push(offset.to_string());

    let mut sqlx_query = sqlx::query_as::<_, Asset>(&query);
    for param in params {
        sqlx_query = sqlx_query.bind(param);
    }

    let assets = sqlx_query
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to fetch assets: {}", e))?;

    Ok(assets)
}

#[tauri::command]
pub async fn update_asset_favorite(id: i64, is_favorite: bool) -> Result<(), String> {
    let db = get_db();
    
    sqlx::query("UPDATE assets SET is_favorite = ? WHERE id = ?")
        .bind(is_favorite)
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to update asset favorite status: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn update_asset_tags(id: i64, tags: Vec<String>) -> Result<(), String> {
    let db = get_db();
    let tags_str = tags.join(",");
    
    sqlx::query("UPDATE assets SET tags = ? WHERE id = ?")
        .bind(&tags_str)
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to update asset tags: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn update_asset_description(id: i64, description: Option<String>) -> Result<(), String> {
    let db = get_db();
    
    sqlx::query("UPDATE assets SET description = ? WHERE id = ?")
        .bind(&description)
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to update asset description: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn update_asset_metadata(id: i64, metadata: Option<String>) -> Result<(), String> {
    let db = get_db();
    
    sqlx::query("UPDATE assets SET metadata = ? WHERE id = ?")
        .bind(&metadata)
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to update asset metadata: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_category_counts() -> Result<HashMap<String, i64>, String> {
    let db = get_db();
    
    let rows: Vec<(String, i64)> = sqlx::query_as(
        "SELECT asset_type, COUNT(*) as count FROM assets GROUP BY asset_type ORDER BY count DESC"
    )
    .fetch_all(db)
    .await
    .map_err(|e| format!("Failed to get category counts: {}", e))?;

    let mut counts = HashMap::new();
    for (category, count) in rows {
        counts.insert(category, count);
    }

    Ok(counts)
}

#[tauri::command]
pub async fn get_assets_by_category(
    category: String,
    sort_by: Option<SortOption>,
    limit: u32,
    offset: u32,
) -> Result<Vec<AssetCardData>, String> {
    let db = get_db();
    
    let sort_clause = match sort_by.unwrap_or(SortOption::NameAsc) {
        SortOption::NameAsc => "ORDER BY name ASC",
        SortOption::NameDesc => "ORDER BY name DESC",
        SortOption::DateAddedAsc => "ORDER BY created_date ASC",
        SortOption::DateAddedDesc => "ORDER BY created_date DESC",
        SortOption::DateAsc => "ORDER BY modified_date ASC",
        SortOption::DateDesc => "ORDER BY modified_date DESC",
    };
    
    let (query, bind_category) = if category == "All" {
        // For "All" category, don't filter by asset_type
        (format!(
            "SELECT id, name, asset_type, file_size, thumbnail_path, orbital_thumbnail_url, orbital_rating_average, orbital_rating_count, is_favorite, created_date 
             FROM assets 
             {} 
             LIMIT ? OFFSET ?",
            sort_clause
        ), false)
    } else {
        // For specific categories, filter by asset_type
        (format!(
            "SELECT id, name, asset_type, file_size, thumbnail_path, orbital_thumbnail_url, orbital_rating_average, orbital_rating_count, is_favorite, created_date 
             FROM assets 
             WHERE asset_type = ? 
             {} 
             LIMIT ? OFFSET ?",
            sort_clause
        ), true)
    };

    let assets: Vec<AssetCardData> = if bind_category {
        sqlx::query_as(&query)
            .bind(&category)
            .bind(limit)
            .bind(offset)
            .fetch_all(db)
            .await
            .map_err(|e| format!("Failed to fetch assets by category: {}", e))?
    } else {
        sqlx::query_as(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(db)
            .await
            .map_err(|e| format!("Failed to fetch assets: {}", e))?
    };

    Ok(assets)
}

#[tauri::command]
pub async fn get_asset_details(id: i64) -> Result<AssetForFrontend, String> {
    let db = get_db();
    
    let asset: Asset = sqlx::query_as("SELECT * FROM assets WHERE id = ?")
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(|e| format!("Failed to fetch asset details: {}", e))?;

    // Convert Asset to AssetForFrontend
    let asset_for_frontend = AssetForFrontend {
        id: asset.id,
        name: asset.name,
        file_path: asset.file_path,
        asset_type: asset.asset_type,
        file_size: asset.file_size,
        created_date: asset.created_date,
        modified_date: asset.modified_date,
        first_indexed_timestamp: asset.first_indexed_timestamp,
        thumbnail_path: asset.thumbnail_path,
        tags: asset.tags.map(|t| t.split(',').map(|s| s.trim().to_string()).collect()).unwrap_or_default(),
        description: asset.description,
        scan_location_id: asset.scan_location_id,
        is_favorite: asset.is_favorite,
        last_accessed: asset.last_accessed,
        file_hash: asset.file_hash,
        metadata: asset.metadata,
        orbital_title: asset.orbital_title,
        orbital_description: asset.orbital_description,
        orbital_technical_details: asset.orbital_technical_details,
        orbital_seller_name: asset.orbital_seller_name,
        orbital_price: asset.orbital_price,
        orbital_release_date: asset.orbital_release_date,
        orbital_last_modified: asset.orbital_last_modified,
        orbital_rating_average: asset.orbital_rating_average,
        orbital_rating_count: asset.orbital_rating_count,
        orbital_categories: asset.orbital_categories,
        orbital_supported_versions: asset.orbital_supported_versions,
        orbital_gallery_images: asset.orbital_gallery_images,
        orbital_thumbnail_url: asset.orbital_thumbnail_url,
        orbital_source_url: asset.orbital_source_url,
        orbital_raw_json: asset.orbital_raw_json,
        orbital_last_checked_timestamp: asset.orbital_last_checked_timestamp,
        matched_orbital_product_slug: asset.matched_orbital_product_slug,
        orbital_match_confidence: asset.orbital_match_confidence,
        orbital_match_type: asset.orbital_match_type,
        notes: asset.notes,
        orbital_manual_overrides: asset.orbital_manual_overrides,
    };

    Ok(asset_for_frontend)
}

#[tauri::command]
pub async fn search_assets(
    query: String,
    limit: u32,
    offset: u32,
) -> Result<Vec<AssetCardData>, String> {
    let db = get_db();
    
    let search_query = format!("%{}%", query);
    
    let assets: Vec<AssetCardData> = sqlx::query_as(
        r#"
        SELECT id, name, asset_type, file_size, thumbnail_path, orbital_thumbnail_url, orbital_rating_average, orbital_rating_count, is_favorite, created_date 
        FROM assets 
        WHERE name LIKE ? 
           OR description LIKE ? 
           OR tags LIKE ?
           OR orbital_title LIKE ?
           OR orbital_description LIKE ?
        ORDER BY 
            CASE WHEN name LIKE ? THEN 1 ELSE 2 END,
            name ASC
        LIMIT ? OFFSET ?
        "#
    )
    .bind(&search_query)
    .bind(&search_query)
    .bind(&search_query)
    .bind(&search_query)
    .bind(&search_query)
    .bind(&search_query)
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await
    .map_err(|e| format!("Failed to search assets: {}", e))?;

    Ok(assets)
}

 