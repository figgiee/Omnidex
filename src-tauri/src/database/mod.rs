pub mod database_manager;
pub use database_manager::DatabaseManager;
pub mod orbital_helpers;

use anyhow::Result;
use sqlx::{Pool, Sqlite, sqlite::SqlitePool, Row};

use crate::database::database_manager::DB_POOL;

/// Initialize the global SQLite connection pool and create tables if they do not exist.
///
/// This mirrors the old implementation that lived in `mod_temp.rs`, but now reflects the
/// current schema that contains Orbital-related columns (rating, version, overrides, …).
pub async fn init_database() -> Result<()> {
    // Determine a writable directory within the user profile
    let app_data_dir = if cfg!(target_os = "windows") {
        std::env::var("APPDATA").unwrap_or_else(|_| std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string()))
    } else if cfg!(target_os = "macos") {
        std::env::var("HOME").map(|home| format!("{home}/Library/Application Support")).unwrap_or_else(|_| ".".to_string())
    } else {
        std::env::var("HOME").map(|home| format!("{home}/.local/share")).unwrap_or_else(|_| ".".to_string())
    };

    let db_dir = std::path::Path::new(&app_data_dir).join("Omnidex");
    if !db_dir.exists() {
        std::fs::create_dir_all(&db_dir)?;
    }

    let database_path = db_dir.join("omnidex.db");
    if !database_path.exists() {
        std::fs::File::create(&database_path)?;
    }

    let database_url = format!("sqlite:{}", database_path.display());
    let pool = SqlitePool::connect(&database_url).await?;

    // Run migrations (create tables, indexes)
    create_tables(&pool).await?;

    // Store globally so other parts of the backend can reuse the connection.
    DB_POOL.set(pool).map_err(|_| anyhow::anyhow!("Failed to set database pool"))?;

    tracing::info!("Database initialised at {}", database_path.display());
    Ok(())
}

/// Low-level helper that creates (or upgrades) the SQLite schema.
/// Only additive changes are performed; destructive migrations should be handled separately.
pub(super) async fn create_tables(pool: &Pool<Sqlite>) -> Result<()> {
    // scan_locations
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS scan_locations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            path TEXT NOT NULL UNIQUE,
            is_active BOOLEAN NOT NULL DEFAULT 1,
            last_scan TEXT,
            scan_recursive BOOLEAN NOT NULL DEFAULT 0,
            file_extensions TEXT,
            created_date TEXT NOT NULL,
            description TEXT
        )
    "#).execute(pool).await?;

    // assets – unified columns (basic + Orbital)
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS assets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            file_path TEXT NOT NULL UNIQUE,
            asset_type TEXT NOT NULL,
            file_size INTEGER NOT NULL,
            created_date TEXT NOT NULL,
            modified_date TEXT NOT NULL,
            first_indexed_timestamp TEXT,
            thumbnail_path TEXT,
            tags TEXT,
            description TEXT,
            scan_location_id INTEGER NOT NULL,
            is_favorite BOOLEAN NOT NULL DEFAULT 0,
            last_accessed TEXT,
            file_hash TEXT,
            metadata TEXT,

            -- Orbital metadata
            orbital_title TEXT,
            orbital_description TEXT,
            orbital_technical_details TEXT,
            orbital_seller_name TEXT,
            orbital_price REAL,
            orbital_release_date TEXT,
            orbital_last_modified TEXT,
            orbital_rating_average REAL,
            orbital_rating_count INTEGER,
            orbital_categories TEXT,
            orbital_supported_versions TEXT,
            orbital_gallery_images TEXT,
            orbital_thumbnail_url TEXT,
            orbital_source_url TEXT,
            orbital_raw_json TEXT,
            orbital_last_checked_timestamp TEXT,

            -- Matching information
            matched_orbital_product_slug TEXT,
            orbital_match_confidence REAL,
            orbital_match_type TEXT,

            -- User additions
            notes TEXT,
            orbital_manual_overrides TEXT,

            FOREIGN KEY (scan_location_id) REFERENCES scan_locations(id) ON DELETE CASCADE
        )
    "#).execute(pool).await?;

    // app_settings (unchanged)
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS app_settings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL UNIQUE,
            value TEXT NOT NULL,
            setting_type TEXT NOT NULL,
            description TEXT,
            created_date TEXT NOT NULL,
            modified_date TEXT NOT NULL
        )
    "#).execute(pool).await?;

    // indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_assets_type ON assets(asset_type)").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_assets_location ON assets(scan_location_id)").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_assets_favorite ON assets(is_favorite)").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_assets_file_hash ON assets(file_hash)").execute(pool).await?;

    // --- Lightweight migration: ensure recently added columns exist -------------------------
    // This avoids breaking older user databases created before new columns were introduced.
    // We currently only need to guarantee the `orbital_manual_overrides` column.
    // If more columns are added later, extend this list.

    let existing_columns: Vec<String> = sqlx::query("PRAGMA table_info(assets)")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| row.get::<String, _>("name"))
        .collect();

    if !existing_columns.iter().any(|c| c == "orbital_manual_overrides") {
        // Add the column. SQLite allows a simple ALTER TABLE for this.
        sqlx::query("ALTER TABLE assets ADD COLUMN orbital_manual_overrides TEXT")
            .execute(pool)
            .await?;
        tracing::info!("🔄 Migrated assets table: added missing column 'orbital_manual_overrides'.");
    }

    Ok(())
}

/// Convenience accessor used by command handlers that prefer raw `sqlx` queries.
/// Panics if the pool has not been initialised yet (should only happen very early in startup).
pub fn get_db() -> &'static Pool<Sqlite> {
    DB_POOL.get().expect("Database has not been initialised. Call init_database() first")
}
