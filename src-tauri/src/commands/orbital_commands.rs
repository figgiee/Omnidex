use tauri::State;
use tracing::{info, warn};

use crate::{
    database::{DatabaseManager, orbital_helpers},
    orbital::models::OrbitalAsset,
};

#[tauri::command]
pub async fn reprocess_cached_orbital_data(
    db_manager: State<'_, DatabaseManager>,
) -> Result<String, String> {
    info!("Reprocessing cached Orbital data for all assets...");

    let assets = match db_manager.get_all_assets_with_raw_orbital_json().await {
        Ok(assets) => assets,
        Err(e) => return Err(format!("Failed to fetch assets: {}", e)),
    };

    info!("Found {} assets with cached data to reprocess.", assets.len());
    let mut processed_count = 0;
    let mut error_count = 0;

    for asset in assets {
        let Some(asset_id) = asset.id else { continue };

        if let Some(raw_json_str) = asset.orbital_raw_json {
            match serde_json::from_str::<OrbitalAsset>(&raw_json_str) {
                Ok(orbital_data) => {
                    if let Err(e) = orbital_helpers::update_asset_with_orbital_details(&db_manager, asset_id, &orbital_data).await {
                        warn!("Failed to update asset {} with reprocessed Orbital details: {}", asset_id, e);
                        error_count += 1;
                    } else {
                        processed_count += 1;
                    }
                }
                Err(e) => {
                    warn!("Failed to deserialize raw_json for asset {}: {}", asset_id, e);
                    error_count += 1;
                }
            }
        }
    }

    let result_message = format!("Reprocessed {} assets. {} errors.", processed_count, error_count);
    info!("{}", &result_message);
    Ok(result_message)
}

#[tauri::command]
pub async fn update_asset_manual_overrides(
    asset_id: i64,
    overrides_json: String,
    db_manager: State<'_, DatabaseManager>,
) -> Result<(), String> {
    info!("Updating manual overrides for asset ID: {}", asset_id);

    // Optional: Validate that overrides_json is valid JSON
    if serde_json::from_str::<serde_json::Value>(&overrides_json).is_err() {
        return Err("Invalid JSON format for overrides".to_string());
    }

    db_manager
        .update_asset_manual_overrides(asset_id, &overrides_json)
        .await
        .map_err(|e| format!("Failed to update manual overrides: {}", e))
} 