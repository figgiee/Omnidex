use sqlx::{sqlite::SqlitePool, Pool, Sqlite, Row};
use std::sync::OnceLock;
use anyhow::Result;
use crate::models::{Asset, ScanLocation};
use crate::orbital::models::OrbitalAsset;
use chrono::Utc;

static DB_POOL: OnceLock<Pool<Sqlite>> = OnceLock::new();

// Define DatabaseManager struct
#[derive(Clone)]
pub struct DatabaseManager {
    pool: &'static Pool<Sqlite>,
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

    pub async fn get_scan_location_by_id(&self, location_id: i64) -> Result<ScanLocation> {
        sqlx::query_as::<_, ScanLocation>("SELECT * FROM scan_locations WHERE id = ?")
            .bind(location_id)
            .fetch_one(self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("ScanLocation with id {} not found: {}", location_id, e))
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

    pub async fn update_asset_match_details(
        &self,
        asset_id: i64,
        product_slug: Option<String>,
        confidence: Option<f64>,
        match_type_str: Option<String>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE assets
            SET matched_fab_product_slug = ?,
                match_confidence = ?,
                match_type = ?
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
        let _result = sqlx::query(
            r#"
            INSERT INTO assets (
                name, file_path, asset_type, file_size, created_date, modified_date,
                first_indexed_timestamp, thumbnail_path, tags, description,
                scan_location_id, is_favorite, last_accessed, file_hash, metadata,
                matched_fab_product_slug, match_confidence, match_type
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&asset.name)
        .bind(&asset.file_path)
        .bind(&asset.asset_type)
        .bind(asset.file_size)
        .bind(&asset.created_date)
        .bind(&asset.modified_date)
        .bind(&asset.first_indexed_timestamp)
        .bind(&asset.thumbnail_path)
        .bind(&asset.tags)
        .bind(&asset.description)
        .bind(asset.scan_location_id)
        .bind(asset.is_favorite)
        .bind(&asset.last_accessed)
        .bind(&asset.file_hash)
        .bind(&asset.metadata)
        .bind(&asset.matched_fab_product_slug)
        .bind(asset.match_confidence)
        .bind(&asset.match_type)
        .execute(self.pool)
        .await?;

        let row = sqlx::query("SELECT last_insert_rowid() as id")
            .fetch_one(self.pool)
            .await?;
        let id: i64 = row.get("id");
        Ok(id)
    }

    pub async fn get_assets_without_fab_matches(&self) -> Result<Vec<Asset>> {
        let assets = sqlx::query_as::<_, Asset>(
            "SELECT * FROM assets WHERE matched_fab_product_slug IS NULL OR matched_fab_product_slug = ''"
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch unmatched assets: {}", e))?;

        Ok(assets)
    }

    pub async fn update_asset_with_fab_details(&self, asset_id: i64, fab_data: &OrbitalAsset) -> Result<()> {
        // Convert string dates to Option<String> for database storage  
        let release_date_str = if fab_data.release_date.is_empty() { None } else { Some(fab_data.release_date.clone()) };
        let last_modified_str = if fab_data.last_modified.is_empty() { None } else { Some(fab_data.last_modified.clone()) };

        // Handle Vec fields - OrbitalAsset has categories and gallery_images
        let categories_json = if !fab_data.categories.is_empty() {
            serde_json::to_string(&fab_data.categories).ok()
        } else {
            None
        };
        
        let gallery_images_json = if !fab_data.gallery_images.is_empty() {
            serde_json::to_string(&fab_data.gallery_images).ok()
        } else {
            None
        };

        // OrbitalAsset doesn't have tags or compatible_apps directly, so set to None
        let tags_json: Option<String> = None;
        let compatible_apps_json: Option<String> = None;

        let now_timestamp_str = Utc::now().to_rfc3339();

        // Handle price (Option<OrbitalPrice>)
        let price_value = fab_data.price.as_ref().map(|p| p.amount);

        // Handle rating (Option<OrbitalRating>)
        let rating_average_db = fab_data.rating.as_ref().map(|r| r.average_rating);
        let rating_count_db = fab_data.rating.as_ref().map(|r| r.total_ratings);

        // SQL update query
        sqlx::query(
            r#"
            UPDATE assets
            SET fab_title = ?,
                fab_description = ?,
                fab_technical_details = ?,
                fab_seller_name = ?,
                fab_price = ?,
                fab_release_date = ?,
                fab_last_modified = ?,
                fab_rating_average = ?,
                fab_rating_count = ?,
                fab_categories = ?,
                fab_tags = ?,
                fab_compatible_apps = ?,
                fab_gallery_images = ?,
                fab_thumbnail_url = ?,
                fab_source_url = ?,
                fab_raw_json = ?,
                fab_last_checked_timestamp = ?
            WHERE id = ?
            "#
        )
        .bind(&fab_data.title)
        .bind(&fab_data.description)
        .bind(&fab_data.technical_details)
        .bind(&fab_data.seller.name)
        .bind(price_value)
        .bind(release_date_str)
        .bind(last_modified_str)
        .bind(rating_average_db)
        .bind(rating_count_db)
        .bind(categories_json)
        .bind(tags_json)
        .bind(compatible_apps_json)
        .bind(gallery_images_json)
        .bind(None::<String>) // No thumbnail_url in OrbitalAsset
        .bind(&fab_data.source_url)
        .bind(fab_data.raw_json.as_ref())
        .bind(now_timestamp_str)
        .bind(asset_id)
        .execute(self.pool)
        .await?;

        Ok(())
    }
}

pub async fn init_database() -> Result<()> {
    // Create database directory in user's data directory
    let app_data_dir = if cfg!(target_os = "windows") {
        std::env::var("APPDATA").unwrap_or_else(|_| std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string()))
    } else if cfg!(target_os = "macos") {
        std::env::var("HOME").map(|home| format!("{}/Library/Application Support", home)).unwrap_or_else(|_| ".".to_string())
    } else {
        std::env::var("HOME").map(|home| format!("{}/.local/share", home)).unwrap_or_else(|_| ".".to_string())
    };
    
    let db_dir = std::path::Path::new(&app_data_dir).join("Omnidex");
    if !db_dir.exists() {
        std::fs::create_dir_all(&db_dir)?;
        tracing::info!("Created database directory: {}", db_dir.display());
    }

    let database_path = db_dir.join("omnidex.db");
    
    // Try to create an empty file first to ensure permissions work
    if !database_path.exists() {
        std::fs::File::create(&database_path)?;
        tracing::info!("Created empty database file: {}", database_path.display());
    }
    
    let database_url = format!("sqlite:{}", database_path.display());
    tracing::info!("Connecting to database: {}", database_url);
    
    let pool = SqlitePool::connect(&database_url).await?;
    
    // Run migrations
    create_tables(&pool).await?;
    
    // Store the pool globally
    DB_POOL.set(pool).map_err(|_| anyhow::anyhow!("Failed to set database pool"))?;
    
    tracing::info!("Database initialized successfully");
    Ok(())
}

pub fn get_db() -> &'static Pool<Sqlite> {
    DB_POOL.get().expect("Database not initialized")
}

async fn create_tables(pool: &SqlitePool) -> Result<()> {
    // Create scan_locations table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS scan_locations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            path TEXT NOT NULL UNIQUE,
            is_active BOOLEAN NOT NULL DEFAULT 1,
            last_scan TEXT,
            scan_recursive BOOLEAN NOT NULL DEFAULT 1,
            file_extensions TEXT,
            created_date TEXT NOT NULL,
            description TEXT
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create assets table
    sqlx::query(
        r#"
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
            matched_fab_product_slug TEXT,
            match_confidence REAL,
            match_type TEXT,
            fab_title TEXT,
            fab_description TEXT,
            fab_technical_details TEXT,
            fab_seller_name TEXT,
            fab_price REAL,
            fab_release_date TEXT,
            fab_last_modified TEXT,
            fab_rating_average REAL,
            fab_rating_count INTEGER,
            fab_categories TEXT,
            fab_tags TEXT,
            fab_compatible_apps TEXT,
            fab_gallery_images TEXT,
            fab_thumbnail_url TEXT,
            fab_source_url TEXT,
            fab_raw_json TEXT,
            fab_last_checked_timestamp TEXT,
            notes TEXT,
            orbital_manual_overrides TEXT,
            FOREIGN KEY (scan_location_id) REFERENCES scan_locations(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create app_settings table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS app_settings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL UNIQUE,
            value TEXT NOT NULL,
            setting_type TEXT NOT NULL,
            description TEXT,
            created_date TEXT NOT NULL,
            modified_date TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create indexes for better performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_assets_type ON assets(asset_type)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_assets_location ON assets(scan_location_id)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_assets_favorite ON assets(is_favorite)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_assets_file_hash ON assets(file_hash)")
        .execute(pool)
        .await?;

    tracing::info!("Database tables created successfully");
    Ok(())
} 