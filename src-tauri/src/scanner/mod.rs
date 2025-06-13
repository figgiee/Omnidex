use crate::models::{Asset, ScanLocation, ScanProgress};
use anyhow::Result;
use jwalk::WalkDir;
use std::path::Path;
use std::time::{Instant};
use tauri::{AppHandle, Emitter};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::database::DatabaseManager;
use strsim::{jaro_winkler, levenshtein};
use crate::orbital::models::OrbitalAsset;
use crate::orbital::api::OrbitalApiClient;
use chrono;

pub mod utils;

// Global scanner manager
static SCANNER_MANAGER: Mutex<Option<ScannerManager>> = Mutex::new(None);

pub struct ScannerManager {
    active_scanners: HashMap<i64, Arc<AtomicBool>>,
}

impl ScannerManager {
    pub fn new() -> Self {
        Self {
            active_scanners: HashMap::new(),
        }
    }

    pub fn is_scan_active(&self, location_id: i64) -> bool {
        self.active_scanners.contains_key(&location_id)
    }

    pub fn register_scanner(&mut self, location_id: i64, cancel_token: Arc<AtomicBool>) {
        self.active_scanners.insert(location_id, cancel_token);
    }

    pub fn cancel_scan(&mut self, location_id: i64) -> bool {
        if let Some(cancel_token) = self.active_scanners.get(&location_id) {
            cancel_token.store(true, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    pub fn remove_scanner(&mut self, location_id: i64) {
        self.active_scanners.remove(&location_id);
    }

    pub fn cancel_all_scans(&mut self) -> usize {
        let cancelled_count = self.active_scanners.len();
        for (_, cancel_token) in &self.active_scanners {
            cancel_token.store(true, Ordering::Relaxed);
        }
        cancelled_count
    }
}

pub fn get_scanner_manager() -> &'static Mutex<Option<ScannerManager>> {
    &SCANNER_MANAGER
}

pub fn init_scanner_manager() {
    let mut manager = SCANNER_MANAGER.lock().unwrap();
    if manager.is_none() {
        *manager = Some(ScannerManager::new());
    }
}

// Constants for matching thresholds
const EXACT_MATCH_THRESHOLD: f64 = 0.95;
const HIGH_MATCH_THRESHOLD: f64 = 0.85;
const MEDIUM_MATCH_THRESHOLD: f64 = 0.70;
const LOW_MATCH_THRESHOLD: f64 = 0.50;

// Weights for combining similarity scores
const JARO_WINKLER_WEIGHT: f64 = 0.7; // Original: 0.7
const LEVENSHTEIN_WEIGHT: f64 = 0.3; // Original: 0.3
const COMMON_PREFIX_BONUS: f64 = 0.05; // Small bonus if first few words match
const COMMON_PREFIX_WORDS: usize = 2;    // Number of words to check for common prefix

#[derive(Debug, Clone, serde::Serialize)]
pub enum MatchStrength {
    NoMatch,
    Low,
    Medium,
    High,
    Exact,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MatchResult {
    pub local_asset_name: String,
    pub fab_asset_name: String,
    pub similarity_score: f64,
    pub strength: MatchStrength,
    pub fab_asset_url: Option<String>, // Optional: URL to the FAB asset page
}

pub struct AssetMatcher;

impl AssetMatcher {
    pub fn new() -> Self {
        AssetMatcher
    }

    fn clean_name(name: &str) -> String {
        name.to_lowercase()
            .replace("_", " ")
            .replace("-", " ")
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }

    pub fn calculate_similarity(&self, name1: &str, name2: &str) -> (f64, MatchStrength) {
        let clean_name1 = Self::clean_name(name1);
        let clean_name2 = Self::clean_name(name2);

        if clean_name1.is_empty() || clean_name2.is_empty() {
            return (0.0, MatchStrength::NoMatch);
        }

        let jw_score = jaro_winkler(&clean_name1, &clean_name2);
        
        let lev_distance = levenshtein(&clean_name1, &clean_name2) as f64;
        let max_len = clean_name1.len().max(clean_name2.len()) as f64;
        let lev_similarity = if max_len == 0.0 { 1.0 } else { (max_len - lev_distance) / max_len };

        let mut combined_score = (jw_score * JARO_WINKLER_WEIGHT) + (lev_similarity * LEVENSHTEIN_WEIGHT);

        // Add bonus for common prefix
        let words1: Vec<&str> = clean_name1.split_whitespace().collect();
        let words2: Vec<&str> = clean_name2.split_whitespace().collect();
        let prefix_len = words1.len().min(words2.len()).min(COMMON_PREFIX_WORDS);
        if prefix_len > 0 {
            if words1[..prefix_len] == words2[..prefix_len] {
                combined_score += COMMON_PREFIX_BONUS;
            }
        }
        // Ensure score does not exceed 1.0 due to bonus
        combined_score = combined_score.min(1.0);

        let strength = if combined_score >= EXACT_MATCH_THRESHOLD {
            MatchStrength::Exact
        } else if combined_score >= HIGH_MATCH_THRESHOLD {
            MatchStrength::High
        } else if combined_score >= MEDIUM_MATCH_THRESHOLD {
            MatchStrength::Medium
        } else if combined_score >= LOW_MATCH_THRESHOLD {
            MatchStrength::Low
        } else {
            MatchStrength::NoMatch
        };

        (combined_score, strength)
    }
}

pub struct AssetScanner {
    db_manager: DatabaseManager,
    cancel_token: Arc<AtomicBool>,
    asset_matcher: AssetMatcher,
    fab_api_client: OrbitalApiClient,
}

impl AssetScanner {
    pub fn new(db_manager: DatabaseManager, fab_api_client: OrbitalApiClient, cancel_token: Arc<AtomicBool>) -> Self {
        Self {
            db_manager,
            cancel_token,
            asset_matcher: AssetMatcher::new(),
            fab_api_client,
        }
    }

    pub fn get_cancel_token(&self) -> Arc<AtomicBool> {
        self.cancel_token.clone()
    }

    pub fn cancel(&self) {
        self.cancel_token.store(true, Ordering::Relaxed);
    }

    pub fn match_local_asset_to_fab_candidates(
        &self,
        local_asset_name: &str,
        fab_candidates: &[OrbitalAsset],
    ) -> Option<(OrbitalAsset, MatchStrength, f64)> {
        if fab_candidates.is_empty() {
            return None;
        }

        let mut best_match_fab_asset: Option<OrbitalAsset> = None;
        let mut best_match_strength: Option<MatchStrength> = None;
        let mut highest_score = 0.0;

        for fab_asset in fab_candidates {
            if let Some(title) = &fab_asset.title {
                let (score, strength) = self.asset_matcher.calculate_similarity(
                    local_asset_name,
                    title,
                );

                if score > highest_score && !matches!(strength, MatchStrength::NoMatch) {
                    highest_score = score;
                    best_match_fab_asset = Some(fab_asset.clone());
                    best_match_strength = Some(strength);
                }
            }
        }

        if let (Some(asset), Some(strength)) = (best_match_fab_asset, best_match_strength) {
            if highest_score > 0.0 {
                return Some((asset, strength, highest_score));
            }
        }
        None
    }

    pub async fn scan_directory_and_process_folders(
        &self, 
        scan_location: &ScanLocation, 
        app_handle_for_emit: &AppHandle
    ) -> Result<Vec<Asset>> {
        let mut assets = Vec::new();
        let mut errors: Vec<String> = Vec::new();
        let path = Path::new(&scan_location.path);
        let start_time = Instant::now();

        if !path.exists() {
            return Err(anyhow::anyhow!("Scan location does not exist: {}", scan_location.path));
        }

        tracing::info!("Starting folder scan of location: {}", scan_location.path);

        let walker = if scan_location.scan_recursive {
            WalkDir::new(path)
        } else {
            WalkDir::new(path).max_depth(1)
        };

        let mut folders_processed = 0i64;
        let total_folders = self.count_folders(path, scan_location.scan_recursive)?;
        tracing::info!("Found {} folders to process in {}", total_folders, scan_location.path);

        let initial_progress = ScanProgress {
            location_id: scan_location.id.unwrap_or(0),
            status: "Initializing Scan".to_string(),
            total_items: total_folders as u64,
            processed_items: 0,
            current_path: scan_location.path.clone(),
            error: None,
            completed_successfully: false,
        };
        if let Err(e) = app_handle_for_emit.emit("scan-progress", &initial_progress) {
            tracing::error!("Failed to emit initial scan progress: {}", e);
        }

        for entry_result in walker {
            if self.cancel_token.load(Ordering::Relaxed) {
                tracing::info!("Scan cancelled for location: {}", scan_location.path);
                let cancel_progress = ScanProgress {
                    location_id: scan_location.id.unwrap_or(0),
                    status: "Cancelled".to_string(),
                    total_items: total_folders as u64,
                    processed_items: folders_processed as u64,
                    current_path: "".to_string(),
                    error: None,
                    completed_successfully: false,
                };
                let _ = app_handle_for_emit.emit("scan-progress", &cancel_progress);
                return Err(anyhow::anyhow!("Scan cancelled by user for location: {}", scan_location.path));
            }

            match entry_result {
                Ok(entry) => {
                    if entry.file_type().is_dir() && entry.path() != path {
                        let current_folder_path = entry.path();
                        match self.process_folder_and_save_asset(&current_folder_path, scan_location.id.unwrap_or(0)).await {
                            Ok(Some(asset)) => {
                                assets.push(asset);
                            }
                            Ok(None) => {
                                tracing::info!("Folder {} processed but not saved as new asset.", current_folder_path.display());
                            }
                            Err(e) => {
                                let error_msg = format!("Failed to process folder {}: {}", current_folder_path.display(), e);
                                tracing::warn!("{}", error_msg);
                                errors.push(error_msg);
                            }
                        }
                        folders_processed += 1;
                        
                        let progress = ScanProgress {
                            location_id: scan_location.id.unwrap_or(0),
                            status: "Scanning".to_string(),
                            total_items: total_folders as u64,
                            processed_items: folders_processed as u64,
                            current_path: current_folder_path.to_string_lossy().into_owned(),
                            error: None,
                            completed_successfully: false,
                        };
                        if let Err(e) = app_handle_for_emit.emit("scan-progress", &progress) {
                            tracing::error!("Failed to emit scan progress update: {}", e);
                        }
                    }
                }
                Err(e) => {
                    let error_msg = format!("Error walking directory entry in {}: {}", scan_location.path, e);
                    tracing::warn!("{}", error_msg);
                    errors.push(error_msg.clone());
                    let error_progress = ScanProgress {
                        location_id: scan_location.id.unwrap_or(0),
                        status: "Error".to_string(),
                        total_items: total_folders as u64,
                        processed_items: folders_processed as u64,
                        current_path: scan_location.path.clone(),
                        error: Some(error_msg),
                        completed_successfully: false,
                    };
                    if let Err(e_emit) = app_handle_for_emit.emit("scan-progress", &error_progress) {
                        tracing::error!("Failed to emit error scan progress: {}", e_emit);
                    }
                }
            }
        }

        let final_progress = ScanProgress {
            location_id: scan_location.id.unwrap_or(0),
            status: "Completed".to_string(),
            total_items: total_folders as u64,
            processed_items: folders_processed as u64,
            current_path: "".to_string(),
            error: if errors.is_empty() { None } else { Some(errors.join("; ")) },
            completed_successfully: errors.is_empty(),
        };
        if let Err(e) = app_handle_for_emit.emit("scan-progress", &final_progress) {
            tracing::error!("Failed to emit final scan progress: {}", e);
        }
        
        let elapsed = start_time.elapsed();
        tracing::info!("Scan of {} completed in {:?}. Processed {}/{} folders. Found {} assets. Errors: {}", 
                      scan_location.path, elapsed, folders_processed, total_folders, assets.len(), errors.len());

        Ok(assets)
    }

    async fn process_folder_and_save_asset(&self, folder_path: &Path, scan_location_id: i64) -> Result<Option<Asset>> {
        if self.cancel_token.load(Ordering::Relaxed) {
            return Ok(None);
        }

        let folder_path_str = folder_path.to_str().unwrap().to_string();
        
        // Check if asset already exists
        if let Some(mut existing_asset) = self.db_manager.get_asset_by_path(&folder_path_str).await? {
            // If asset exists, check if it needs metadata refresh
            if existing_asset.orbital_description.is_none() || existing_asset.orbital_title.is_none() {
                tracing::info!("Asset {} found, attempting to refresh metadata.", existing_asset.name);
                if let Err(e) = self.fetch_and_update_orbital_data(&mut existing_asset).await {
                    tracing::warn!("Failed to refresh metadata for asset {}: {}", existing_asset.name, e);
                }
            }
            return Ok(Some(existing_asset));
        }

        let folder_name = folder_path.file_name().unwrap().to_str().unwrap().to_string();
        let created_date_system_time = folder_path.metadata()?.created()?;
        let modified_date_system_time = folder_path.metadata()?.modified()?;

        let created_date = chrono::DateTime::<chrono::Utc>::from(created_date_system_time).to_rfc3339();
        let modified_date = chrono::DateTime::<chrono::Utc>::from(modified_date_system_time).to_rfc3339();

        let folder_size = self.calculate_folder_size(folder_path).await.unwrap_or(0);
        let asset_type = self.determine_asset_type_from_folder(folder_path);
        
        // This is a simplified hash for demonstration. A more robust solution might be needed.
        let file_hash = self.calculate_folder_hash(&folder_name);
        let first_indexed_timestamp = chrono::Utc::now().to_rfc3339();

        let new_asset = Asset {
            id: None,
            name: folder_name.clone(),
            file_path: folder_path_str,
            asset_type,
            file_size: folder_size,
            created_date,
            modified_date,
            first_indexed_timestamp: Some(first_indexed_timestamp),
            thumbnail_path: None,
            tags: None,
            description: None,
            scan_location_id,
            is_favorite: false,
            last_accessed: None,
            file_hash: Some(file_hash),
            metadata: None, // Can be extended to store more data
            orbital_title: None,
            orbital_description: None,
            orbital_technical_details: None,
            orbital_seller_name: None,
            orbital_price: None,
            orbital_release_date: None,
            orbital_last_modified: None,
            orbital_rating_average: None,
            orbital_rating_count: None,
            orbital_categories: None,
            orbital_supported_versions: None,
            orbital_gallery_images: None,
            orbital_thumbnail_url: None,
            orbital_source_url: None,
            orbital_raw_json: None,
            orbital_last_checked_timestamp: None,
            matched_orbital_product_slug: None,
            orbital_match_confidence: None,
            orbital_match_type: None,
            notes: None,
            orbital_manual_overrides: None,
        };
        
        match self.db_manager.insert_asset(&new_asset).await {
            Ok(asset_id) => {
                let mut saved_asset = new_asset;
                saved_asset.id = Some(asset_id);

                // Immediately attempt to enrich the new asset with Orbital marketplace data
                if let Err(e) = self.fetch_and_update_orbital_data(&mut saved_asset).await {
                    tracing::warn!(
                        "Failed to fetch Orbital metadata for new asset {} (id {}): {}",
                        saved_asset.name,
                        asset_id,
                        e
                    );
                }

                Ok(Some(saved_asset))
            },
            Err(e) => {
                tracing::error!("Failed to insert asset for folder {}: {}", folder_name, e);
                Err(e.into())
            }
        }
    }

    #[tracing::instrument(skip(self, folder_path), fields(path = %folder_path.display()))]
    async fn calculate_folder_size(&self, folder_path: &Path) -> Result<i64> {
        let mut total_size = 0;
        for entry in WalkDir::new(folder_path) {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_file() {
                        match entry.metadata() {
                            Ok(metadata) => {
                                total_size += metadata.len();
                            }
                            Err(e) => {
                                tracing::warn!("Couldn't get metadata for file {}: {}", entry.path().display(), e);
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Error walking directory for size calculation {}: {}", folder_path.display(), e);
                }
            }
        }
        Ok(total_size as i64)
    }

    fn calculate_folder_hash(&self, folder_name: &str) -> String {
        let mut hasher = DefaultHasher::new();
        folder_name.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    fn determine_asset_type_from_folder(&self, folder_path: &Path) -> String {
        utils::determine_asset_type_from_path(folder_path)
    }

    fn count_folders(&self, path: &Path, recursive: bool) -> Result<i64> {
        if !path.is_dir() {
            return Ok(0);
        }

        let walker = if recursive {
            WalkDir::new(path)
        } else {
            WalkDir::new(path).max_depth(1)
        };

        let mut count = 0i64;
        for entry in walker {
            if let Ok(entry) = entry {
                if entry.file_type().is_dir() && entry.path() != path {
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    async fn fetch_and_update_orbital_data(&self, asset: &mut Asset) -> Result<()> {
        tracing::info!("Fetching Orbital data for asset: {}", asset.name);
        
        let fab_candidates = self.fab_api_client.search_by_folder_name(&asset.name).await?;
        
        if let Some((best_match, _, _)) = self.match_local_asset_to_fab_candidates(&asset.name, &fab_candidates) {
            tracing::info!("Found best match for {}: {:?}", asset.name, best_match.title);
            
            if let Some(asset_id) = asset.id {
                self.db_manager.update_asset_with_orbital_details(asset_id, &best_match).await?;
                tracing::info!("Successfully updated asset {} with new Orbital data.", asset.name);

                // Update the local asset object with the new data
                asset.orbital_title = best_match.title;
                asset.orbital_description = best_match.description;
                asset.orbital_technical_details = best_match.technical_details;
                asset.orbital_seller_name = best_match.seller;
                asset.orbital_price = best_match.price;
                asset.orbital_release_date = best_match.release_date;
                asset.orbital_last_modified = best_match.last_modified;
                asset.orbital_rating_average = best_match.rating_average;
                asset.orbital_rating_count = best_match.rating_count;
                asset.orbital_categories = Some(serde_json::to_string(&best_match.categories)?);
                asset.orbital_supported_versions = Some(serde_json::to_string(&best_match.supported_versions)?);
                asset.orbital_gallery_images = Some(serde_json::to_string(&best_match.gallery_images)?);
                asset.orbital_thumbnail_url = best_match.thumbnail_url;
                asset.orbital_source_url = best_match.source_url;
                asset.orbital_raw_json = best_match.raw_json.map(|v| serde_json::to_string(&v)).transpose()?;
                asset.matched_orbital_product_slug = best_match.product_slug;
            }
        } else {
            tracing::warn!("Could not find a match for asset: {}", asset.name);
        }
        
        Ok(())
    }
} 
