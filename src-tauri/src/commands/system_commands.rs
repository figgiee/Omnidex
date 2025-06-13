use crate::database::{get_db, DatabaseManager};
use crate::models::asset::AssetStats;
use std::path::Path;
use tauri::{AppHandle, State, Emitter};

#[cfg(target_os = "linux")]
use std::path::PathBuf;

#[tauri::command]
pub async fn get_asset_stats() -> Result<AssetStats, String> {
    let db = get_db();
    
    let total_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM assets")
        .fetch_one(db)
        .await
        .map_err(|e| format!("Failed to get total asset count: {}", e))?;
        

        
    let total_size: Option<i64> = sqlx::query_scalar("SELECT SUM(file_size) FROM assets")
        .fetch_one(db)
        .await
        .map_err(|e| format!("Failed to get total size: {}", e))?;
        
    Ok(AssetStats {
        total_assets: total_count,
        total_size: total_size.unwrap_or(0),
        asset_type_counts: vec![], // Empty for now, could be filled later
        recent_assets: vec![], // Empty for now, could be filled later
    })
}

#[tauri::command]
pub async fn clear_all_assets() -> Result<String, String> {
    let db = get_db();
    
    let result = sqlx::query("DELETE FROM assets")
        .execute(db)
        .await
        .map_err(|e| format!("Failed to clear all assets: {}", e))?;
    
    Ok(format!("Successfully deleted {} assets", result.rows_affected()))
}

#[tauri::command]
pub async fn clear_all_scan_locations() -> Result<String, String> {
    let db = get_db();
    
    let result = sqlx::query("DELETE FROM scan_locations")
        .execute(db)
        .await
        .map_err(|e| format!("Failed to clear all scan locations: {}", e))?;
    
    Ok(format!("Successfully deleted {} scan locations", result.rows_affected()))
}

#[tauri::command]
pub async fn wipe_entire_database(
    app_handle: AppHandle,
    db_manager_state: State<'_, DatabaseManager>
) -> Result<String, String> {
    let db_manager = db_manager_state.inner();
    
    match db_manager.wipe_database().await {
        Ok(_) => {
            // Emit an event to inform the frontend about the database wipe
            if let Err(e) = app_handle.emit("database-wiped", ()) {
                eprintln!("Failed to emit database-wiped event: {}", e);
            }
            Ok("Database wiped successfully".to_string())
        }
        Err(e) => Err(format!("Failed to wipe database: {}", e)),
    }
}

#[tauri::command]
pub async fn get_database_stats() -> Result<serde_json::Value, String> {
    let db = get_db();
    
    let asset_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM assets")
        .fetch_one(db)
        .await
        .map_err(|e| format!("Failed to get asset count: {}", e))?;
    
    let location_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM scan_locations")
        .fetch_one(db)
        .await
        .map_err(|e| format!("Failed to get scan location count: {}", e))?;
    
    let total_size: Option<i64> = sqlx::query_scalar("SELECT SUM(file_size) FROM assets")
        .fetch_one(db)
        .await
        .map_err(|e| format!("Failed to get total file size: {}", e))?;
    
    let favorite_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM assets WHERE is_favorite = 1")
        .fetch_one(db)
        .await
        .map_err(|e| format!("Failed to get favorite count: {}", e))?;
    
    Ok(serde_json::json!({
        "asset_count": asset_count,
        "location_count": location_count,
        "total_size": total_size.unwrap_or(0),
        "favorite_count": favorite_count
    }))
}

#[tauri::command]
pub async fn open_file_location(path: String) -> Result<(), String> {
    let path = Path::new(&path);
    
    if !path.exists() {
        return Err(format!("Path does not exist: {}", path.display()));
    }
    
    let parent = path.parent().unwrap_or(path);
    
    #[cfg(target_os = "windows")]
    {
        let path_str = parent.to_string_lossy().replace('/', "\\");
        std::process::Command::new("explorer")
            .arg(&path_str)
            .spawn()
            .map_err(|e| format!("Failed to open file location: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("-R")
            .arg(path)
            .spawn()
            .map_err(|e| format!("Failed to open file location: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        // Try various file managers
        let file_managers = ["nautilus", "dolphin", "thunar", "pcmanfm", "nemo"];
        let mut success = false;
        
        for fm in &file_managers {
            if let Ok(_) = std::process::Command::new(fm)
                .arg(parent)
                .spawn()
            {
                success = true;
                break;
            }
        }
        
        if !success {
            // Fallback to xdg-open
            std::process::Command::new("xdg-open")
                .arg(parent)
                .spawn()
                .map_err(|e| format!("Failed to open file location: {}", e))?;
        }
    }
    
    Ok(())
}

#[tauri::command]
pub async fn open_url(url: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", &url])
            .spawn()
            .map_err(|e| format!("Failed to open URL: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("Failed to open URL: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("Failed to open URL: {}", e))?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn get_duplicate_assets() -> Result<Vec<crate::models::asset::Asset>, String> {
    let db = get_db();
    
    let assets = sqlx::query_as::<_, crate::models::asset::Asset>(
        r#"
        SELECT * FROM assets 
        WHERE file_hash IN (
            SELECT file_hash 
            FROM assets 
            WHERE file_hash IS NOT NULL 
            GROUP BY file_hash 
            HAVING COUNT(*) > 1
        )
        ORDER BY file_hash, name
        "#
    )
    .fetch_all(db)
    .await
    .map_err(|e| format!("Failed to fetch duplicate assets: {}", e))?;
    
    Ok(assets)
}

#[tauri::command]
pub async fn get_favorite_assets_count() -> Result<i64, String> {
    let db = get_db();
    
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM assets WHERE is_favorite = 1")
        .fetch_one(db)
        .await
        .map_err(|e| format!("Failed to get favorite assets count: {}", e))?;

    Ok(count)
}

#[tauri::command]
pub async fn post_process_asset_categories() -> Result<i64, String> {
    let db = get_db();
    
    let assets: Vec<(i64, String)> = sqlx::query_as("SELECT id, name FROM assets")
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to fetch assets for category processing: {}", e))?;

    let mut updated_count = 0;

    for (id, name) in assets {
        let category = crate::scanner::utils::determine_asset_type_from_path(Path::new(&name));
        
        sqlx::query("UPDATE assets SET asset_type = ? WHERE id = ?")
            .bind(&category)
            .bind(id)
            .execute(db)
            .await
            .map_err(|e| format!("Failed to update asset category: {}", e))?;
        
        updated_count += 1;
    }

    Ok(updated_count)
} 