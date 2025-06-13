// Module declarations
pub mod commands;
pub mod database;
pub mod orbital;
pub mod scanner;
pub mod models;

use crate::database::DatabaseManager;
use crate::orbital::OrbitalApiClient;
use tauri::{Manager};
use tracing::{info, error};
use tauri_plugin_log::{Target, TargetKind, Builder as LogBuilder};
use log::LevelFilter;

// Import the scanner initializer
use crate::scanner::init_scanner_manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(
            LogBuilder::new()
                .targets([
                    // Log to a file in the app log directory
                    Target::new(TargetKind::LogDir { file_name: Some("omnidex.log".into()) }),
                    // Log to stdout
                    Target::new(TargetKind::Stdout),
                    // Log to webview console
                    Target::new(TargetKind::Webview),
                ])
                .level(LevelFilter::Info)
                .build(),
        )
        .setup(|app| {
            // Initialize scanner manager
            init_scanner_manager();
            info!("Scanner manager initialized.");

            // Initialize database on startup
            let app_handle_clone = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = database::init_database().await {
                    error!("Failed to initialize database: {}", e);
                    return;
                }
                
                match DatabaseManager::new().await {
                    Ok(db_manager) => {
                        info!("DatabaseManager created successfully.");
                        app_handle_clone.manage(db_manager);
                    }
                    Err(e) => {
                        error!("Failed to create DatabaseManager: {}", e);
                    }
                }
            });
            
            // Manage OrbitalApiClient
            let orbital_client = OrbitalApiClient::new().unwrap();
            app.manage(orbital_client);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Scan Location Commands
            commands::create_scan_location,
            commands::get_scan_locations,
            commands::update_scan_location,
            commands::delete_scan_location,
            commands::update_scan_recursive_setting,
            commands::clear_assets_from_scan_location,
            // Asset Commands
            commands::get_assets,
            commands::update_asset_favorite,
            commands::update_asset_description,
            commands::update_asset_metadata,
            commands::get_asset_stats,
            commands::get_category_counts,
            commands::get_assets_by_category,
            commands::get_favorite_assets_count,
            commands::get_asset_details,
            commands::update_asset_tags,
            commands::search_assets,
            commands::post_process_asset_categories,
            // New Asset Commands
            commands::asset_commands::toggle_favorite_status,
            commands::asset_commands::add_tags_to_assets,
            commands::asset_commands::delete_assets,
            commands::asset_commands::match_asset_manually,
            // Scanning Commands
            commands::scan_all_locations,
            commands::start_scan,
            commands::cancel_scan,
            commands::cancel_all_scans,
            commands::get_duplicate_assets,
            // File System Commands
            commands::open_file_location,
            commands::open_url,
            // Orbital Integration Commands
            commands::orbital_commands::reprocess_cached_orbital_data,
            commands::orbital_commands::update_asset_manual_overrides,
            // Database Management Commands
            commands::clear_all_assets,
            commands::clear_all_scan_locations,
            commands::wipe_entire_database,
            commands::get_database_stats
        ])
        .run(context)
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orbital::OrbitalManager;
    use tracing::info;

    #[test]
    fn test_slug_generation() {
        info!("🧪 Testing slug generation with problematic cases...");
        
        if let Ok(manager) = OrbitalManager::new() {
            let test_cases = vec![
                "MC Skydive (5 0 )",
                "Mage Animation Set",
                "Some Asset (4 18)",
                "Character Pack UE5.3",
                "Environment Assets v2.1",
                "Building Kit (UE4.27)",
            ];
            
            for case in test_cases {
                info!("Testing: '{}'", case);
                let variations = manager.test_slug_generation(case);
                info!("Generated {} variations: {:?}", variations.len(), variations);
            }
        }
    }
}
