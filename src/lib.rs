use tauri::{App, Manager};

fn main() {
    App::new("orbital-asset-manager")
        .invoke_handler(tauri::generate_handler![
            commands::test_orbital_folder_cleaning,
            commands::match_with_orbital_assets,
            commands::match_local_assets_with_orbital,
            commands::refresh_orbital_asset_details,
            commands::clear_database,
            commands::get_database_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 