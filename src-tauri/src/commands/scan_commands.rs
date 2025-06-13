use crate::database::{get_db, DatabaseManager};
use crate::models::scan_location::ScanLocation;
use crate::orbital::OrbitalApiClient;
use crate::scanner::{get_scanner_manager, AssetScanner};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tauri::{AppHandle, State};
use tracing::{error, info, warn};

/// Creates a new scan location in the database
/// 
/// # Arguments
/// * `name` - Human-readable name for the scan location
/// * `path` - File system path to scan for assets
/// * `scan_recursive` - Whether to scan subdirectories recursively
/// * `file_extensions` - Optional comma-separated list of file extensions to scan (e.g., "uasset,umap")
/// * `description` - Optional description of the scan location
/// 
/// # Returns
/// * `Ok(ScanLocation)` - The created scan location with assigned ID
/// * `Err(String)` - Error message if creation fails
/// 
/// # Behavior
/// - Automatically sets `is_active` to true for new locations
/// - Records creation timestamp in RFC3339 format
/// - Returns the complete scan location record including assigned ID
#[tauri::command]
pub async fn create_scan_location(
    name: String,
    path: String,
    scan_recursive: bool,
    file_extensions: Option<String>,
    description: Option<String>,
) -> Result<ScanLocation, String> {
    let db = get_db();
    let created_date = chrono::Utc::now().to_rfc3339();
    
    let result = sqlx::query_as::<_, ScanLocation>(
        r#"
        INSERT INTO scan_locations (name, path, is_active, scan_recursive, file_extensions, created_date, description)
        VALUES (?, ?, 1, ?, ?, ?, ?)
        RETURNING *
        "#,
    )
    .bind(&name)
    .bind(&path)
    .bind(scan_recursive)
    .bind(&file_extensions)
    .bind(&created_date)
    .bind(&description)
    .fetch_one(db)
    .await
    .map_err(|e| format!("Failed to create scan location: {}", e))?;

    Ok(result)
}

#[tauri::command]
pub async fn get_scan_locations() -> Result<Vec<ScanLocation>, String> {
    let db = get_db();
    
    let locations = sqlx::query_as::<_, ScanLocation>("SELECT * FROM scan_locations ORDER BY created_date DESC")
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to fetch scan locations: {}", e))?;

    Ok(locations)
}

#[tauri::command]
pub async fn update_scan_location(
    id: i64,
    name: String,
    path: String,
    is_active: bool,
    scan_recursive: bool,
    file_extensions: Option<String>,
    description: Option<String>,
) -> Result<ScanLocation, String> {
    let db = get_db();
    
    let result = sqlx::query_as::<_, ScanLocation>(
        r#"
        UPDATE scan_locations 
        SET name = ?, path = ?, is_active = ?, scan_recursive = ?, file_extensions = ?, description = ?
        WHERE id = ?
        RETURNING *
        "#,
    )
    .bind(&name)
    .bind(&path)
    .bind(is_active)
    .bind(scan_recursive)
    .bind(&file_extensions)
    .bind(&description)
    .bind(id)
    .fetch_one(db)
    .await
    .map_err(|e| format!("Failed to update scan location: {}", e))?;

    Ok(result)
}

#[tauri::command]
pub async fn delete_scan_location(id: i64) -> Result<(), String> {
    let db = get_db();
    
    sqlx::query("DELETE FROM scan_locations WHERE id = ?")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to delete scan location: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn update_scan_recursive_setting(
    location_id: i64,
    scan_recursive: bool
) -> Result<ScanLocation, String> {
    let db = get_db();
    
    let result = sqlx::query_as::<_, ScanLocation>(
        r#"
        UPDATE scan_locations 
        SET scan_recursive = ?
        WHERE id = ?
        RETURNING *
        "#,
    )
    .bind(scan_recursive)
    .bind(location_id)
    .fetch_one(db)
    .await
    .map_err(|e| format!("Failed to update scan recursive setting: {}", e))?;

    Ok(result)
}

#[tauri::command]
pub async fn clear_assets_from_scan_location(location_id: i64) -> Result<i64, String> {
    let db = get_db();
    
    let result = sqlx::query("DELETE FROM assets WHERE scan_location_id = ?")
        .bind(location_id)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to clear assets from scan location: {}", e))?;

    Ok(result.rows_affected() as i64)
}

#[tauri::command]
pub async fn scan_all_locations(
    app_handle: AppHandle,
    db_manager_state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = get_db();
    let locations = sqlx::query_as::<_, ScanLocation>("SELECT * FROM scan_locations WHERE is_active = 1")
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to fetch scan locations: {}", e))?;

    for location in locations {
        if let Some(location_id) = location.id {
            let mut scanner_manager_guard = get_scanner_manager().lock().unwrap();
            if let Some(manager) = scanner_manager_guard.as_mut() {
                if manager.is_scan_active(location_id) {
                    warn!(
                        "Scan is already in progress for location ID: {}",
                        location_id
                    );
                    continue; // Skip to the next location
                }
            } else {
                error!("ScannerManager is not initialized.");
                return Err("ScannerManager is not initialized.".to_string());
            }

            let cancel_token = Arc::new(AtomicBool::new(false));
            if let Some(manager) = scanner_manager_guard.as_mut() {
                manager.register_scanner(location_id, cancel_token.clone());
            }

            // Drop the lock before starting the async task
            drop(scanner_manager_guard);

            let app_handle_clone = app_handle.clone();
            let db_manager = db_manager_state.inner().clone();

            tauri::async_runtime::spawn(async move {
                let orbital_api_client = OrbitalApiClient::new()
                    .map_err(|e| format!("Failed to create Orbital API client: {}", e));

                if let Err(e) = orbital_api_client {
                    error!("{}", e);
                    // Optionally emit an error event to the frontend here
                    return;
                }

                let asset_scanner = AssetScanner::new(
                    db_manager,
                    orbital_api_client.unwrap(),
                    cancel_token.clone(),
                );

                info!("Starting scan for location: {}", location.name);
                match asset_scanner.scan_directory_and_process_folders(&location, &app_handle_clone).await {
                    Ok(_) => info!("Scan completed for location: {}", location.name),
                    Err(e) => error!("Scan failed for location: {}: {}", location.name, e),
                }

                // Clean up after scan completion or failure
                if let Some(manager) = get_scanner_manager().lock().unwrap().as_mut() {
                    manager.remove_scanner(location_id);
                }
            });
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn start_scan(
    location_id: i64,
    app_handle: AppHandle,
    db_manager_state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = get_db();
    
    // Fetch the scan location
    let location: ScanLocation = sqlx::query_as("SELECT * FROM scan_locations WHERE id = ?")
        .bind(location_id)
        .fetch_one(db)
        .await
        .map_err(|e| format!("Failed to fetch scan location: {}", e))?;

    // Check if a scan is already in progress for this location
    let mut scanner_manager_guard = get_scanner_manager().lock().unwrap();
    if let Some(manager) = scanner_manager_guard.as_mut() {
        if manager.is_scan_active(location_id) {
            return Err("Scan is already in progress for this location.".to_string());
        }
    } else {
        return Err("ScannerManager is not initialized.".to_string());
    }

    // Create the cancel token and register the scanner
    let cancel_token = Arc::new(AtomicBool::new(false));
    if let Some(manager) = scanner_manager_guard.as_mut() {
        manager.register_scanner(location_id, cancel_token.clone());
    }

    // Drop the lock before starting the async task
    drop(scanner_manager_guard);

    let app_handle_clone = app_handle.clone();
    let db_manager = db_manager_state.inner().clone();

    tauri::async_runtime::spawn(async move {
        let orbital_api_client = OrbitalApiClient::new()
            .map_err(|e| format!("Failed to create Orbital API client: {}", e));

        if let Err(e) = orbital_api_client {
            error!("{}", e);
            // Optionally emit an error event to the frontend here
            return;
        }

        let asset_scanner = AssetScanner::new(
            db_manager,
            orbital_api_client.unwrap(),
            cancel_token.clone(),
        );

        info!("Starting scan for location: {}", location.name);
        match asset_scanner.scan_directory_and_process_folders(&location, &app_handle_clone).await {
            Ok(_) => {
                info!("Scan completed for location: {}", location.name);
                // Update the last scan timestamp for the location
                let db = get_db();
                let now_timestamp = chrono::Utc::now().to_rfc3339();
                if let Err(e) = sqlx::query("UPDATE scan_locations SET last_scan = ? WHERE id = ?")
                    .bind(now_timestamp)
                    .bind(location_id)
                    .execute(db)
                    .await
                {
                    error!("Failed to update last scan timestamp: {}", e);
                }
            }
            Err(e) => error!("Scan failed for location: {}: {}", location.name, e),
        }

        // Clean up after scan completion or failure
        if let Some(manager) = get_scanner_manager().lock().unwrap().as_mut() {
            manager.remove_scanner(location_id);
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn cancel_scan(location_id: i64) -> Result<bool, String> {
    let mut scanner_manager_guard = get_scanner_manager().lock().unwrap();
    if let Some(manager) = scanner_manager_guard.as_mut() {
        if manager.cancel_scan(location_id) {
            // The scanner manager handles the removal after the cancel is complete
            Ok(true)
        } else {
            Ok(false) // No scan was running for this location
        }
    } else {
        Err("ScannerManager is not initialized.".to_string())
    }
}

/// Cancels all active scans
/// 
/// # Returns
/// * `Ok(usize)` - Number of scans that were cancelled
/// * `Err(String)` - Error message if cancellation fails
/// 
/// # Behavior
/// - Iterates through all active scans and cancels them
/// - Returns the count of successfully cancelled scans
/// - Does not fail if no scans are running
#[tauri::command]
pub async fn cancel_all_scans() -> Result<usize, String> {
    let mut scanner_manager_guard = get_scanner_manager().lock().unwrap();
    if let Some(manager) = scanner_manager_guard.as_mut() {
        let cancelled_count = manager.cancel_all_scans();
        Ok(cancelled_count)
    } else {
        Err("ScannerManager is not initialized.".to_string())
    }
} 