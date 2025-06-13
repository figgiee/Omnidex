use anyhow::Result;
use log::{info, warn};
use urlencoding;
use crate::database::DatabaseManager;
use crate::models::asset::Asset;

use crate::orbital::scraper::{fetch_product_page_html, parse_asset_details_from_html, find_first_product_link};

use std::sync::Arc;
// Note: Arc and Mutex imports removed as browser automation is not needed for Orbital Market

pub mod api;
pub mod auth;
pub mod matching;
pub mod models;
pub mod api_models;
pub mod scraper;

pub use api::OrbitalApiClient;
pub use auth::OrbitalAuth;
// Note: OrbitalBrowserClient not needed for Orbital Market
pub use matching::AssetMatcher;
pub use models::*;

/// Main Orbital Market integration manager
#[derive(Debug)]
pub struct OrbitalManager {
    api_client: OrbitalApiClient,
    auth_manager: OrbitalAuth,
    asset_matcher: AssetMatcher,
}

impl OrbitalManager {
    pub fn new() -> Result<Self> {
        info!("🚀 Initializing Orbital Market Manager with respectful rate limiting");
        
        let api_client = OrbitalApiClient::new()?;
        let auth_manager = OrbitalAuth::new(reqwest::Client::new());
        let asset_matcher = AssetMatcher::new(api_client.clone());
        
        // Note: Browser automation is not needed for Orbital Market (they're developer-friendly!)
        
        Ok(Self {
            api_client,
            auth_manager,
            asset_matcher,
        })
    }

    /// Test Orbital Market API access
    pub async fn test_api_access(&self) -> Result<bool> {
        info!("🧪 Testing Orbital Market API access");
        
        // Test Orbital Market access (should be straightforward!)
        match self.api_client.test_public_access().await {
            Ok(success) => {
                if success {
                    info!("✅ Orbital Market access successful");
                } else {
                    warn!("⚠️ Orbital Market access failed");
                }
                Ok(success)
            }
            Err(e) => {
                warn!("❌ Orbital Market access error: {}", e);
                Ok(false)
            }
        }
    }

    /// Fetch asset details from Orbital Market
    pub async fn fetch_asset_details(&self, product_id: &str) -> Result<OrbitalAsset> {
        info!("📦 Fetching asset details for: {}", product_id);
        
        self.api_client.fetch_asset_details(product_id).await
    }

    /// Search assets on Orbital Market
    pub async fn search_assets(&self, query: &str, limit: i32) -> Result<Vec<OrbitalAsset>> {
        info!("🔍 Searching Orbital Market for: {}", query);
        
        self.api_client.search_assets(query, limit).await
    }

    /// Search assets by folder name using direct URL construction
    pub async fn search_by_folder_name(&self, folder_name: &str) -> Result<Vec<OrbitalAsset>> {
        info!("📁 Searching by folder name: {}", folder_name);
        
        // Use the API client's direct URL approach
        self.api_client.search_by_folder_name(folder_name).await
    }

    /// Match local assets with Orbital marketplace
    pub async fn match_local_assets(&self, local_assets: Vec<crate::models::Asset>) -> Result<Vec<AssetMatch>> {
        info!("🔗 Matching {} local assets with Orbital marketplace", local_assets.len());
        
        let mut matches = Vec::new();
        
        for asset in local_assets {
            let asset_match = self.asset_matcher.match_single_asset(&asset).await?;
            if asset_match.match_type != MatchType::NoMatch {
                info!("✅ Found match for '{}' with confidence: {:.2}", asset.name, asset_match.match_confidence);
                matches.push(asset_match);
            } else {
                info!("❌ No matches found for '{}'", asset.name);
            }
            
            // Add delay between searches to be respectful
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
        
        info!("🎉 Matching complete: {} total matches found", matches.len());
        Ok(matches)
    }

    /// Set authentication token
    pub fn set_auth_token(&mut self, token: String) -> Result<()> {
        // For now, just log that we received a token
        // In a full implementation, we'd parse and store it properly
        info!("Orbital authentication token received: {}", token);
        Ok(())
    }

    /// Get authentication instructions
    pub async fn get_auth_instructions(&self) -> String {
        self.auth_manager.get_auth_instructions().await.unwrap_or_else(|e| e.to_string())
    }

    /// Test folder name cleaning
    pub fn test_folder_cleaning(&self, folder_name: &str) -> (String, Vec<String>) {
        self.api_client.test_folder_name_cleaning(folder_name)
    }

    /// Test slug generation variations
    #[cfg(test)]
    pub fn test_slug_generation(&self, folder_name: &str) -> Vec<String> {
        self.api_client.generate_slug_variations(folder_name)
    }

    /// Test browser automation access specifically
    pub async fn test_browser_access(&self) -> Result<bool> {
        // Browser automation not needed for Orbital Market
        Ok(false)
    }

    pub async fn test_token(&self, token: &str) -> Result<bool> {
        self.auth_manager.test_token(token).await
    }

    /// This function is intended for testing purposes.
    #[cfg(test)]
    pub async fn test_slug_generation(&self, name: &str) -> Vec<String> {
        self.api_client.generate_slug_variations(name)
    }
}

pub async fn refresh_orbital_metadata_for_asset(
    asset: &Asset,
    db_manager: &DatabaseManager,
    client: Arc<api::OrbitalApiClient>,
) -> Result<()> {
    info!("Fetching Orbital data for asset: {}", asset.name);

    let mut orbital_asset = None;

    if let Some(slug) = &asset.matched_orbital_product_slug {
        info!("▶️ Asset has a matched slug, fetching from: {}", slug);
        let product_url = format!("https://orbital-market.com/product/{}", slug);
        match fetch_product_page_html(&client, &product_url).await {
            Ok(html) => {
                let mut parsed_asset = parse_asset_details_from_html(&client, &html);
                parsed_asset.title = Some(asset.name.clone());
                parsed_asset.source_url = Some(product_url);
                orbital_asset = Some(parsed_asset);
            },
            Err(e) => {
                warn!("Failed to fetch matched product page for slug '{}': {}", slug, e);
            }
        }
    }

    if orbital_asset.is_none() {
        info!("▶️ Attempting to find match for folder: {}", asset.name);

        // Fallback to web scraping the search page
        info!("⚠️ No direct match found, falling back to web search for: {}", &asset.name);
        let search_query = urlencoding::encode(&asset.name);
        let search_url = format!("https://orbital-market.com/search?q={}", search_query);
        
        info!("Searching Orbital Market: {}", search_url);

        match fetch_product_page_html(&client, &search_url).await {
            Ok(html_content) => {
                if let Some(product_url) = find_first_product_link(&html_content, &client.selectors.search_result_item_selector) {
                    info!("Found product link on search page: {}", product_url);
                    if let Ok(product_html) = fetch_product_page_html(&client, &product_url).await {
                        let mut parsed_asset = parse_asset_details_from_html(&client, &product_html);
                        parsed_asset.title = Some(asset.name.clone());
                        parsed_asset.source_url = Some(product_url);
                        orbital_asset = Some(parsed_asset);
                    }
                }
            },
            Err(e) => {
                warn!("Failed to fetch search results for '{}': {}", asset.name, e);
            }
        }
    }

    if let Some(orbital_data) = orbital_asset {
        info!("✅ Successfully scraped data for: {}", asset.name);
        if let Err(e) = db_manager.update_asset_with_orbital_details(asset.id.unwrap(), &orbital_data).await {
            warn!("Failed to update database for asset {}: {}", asset.id.unwrap(), e);
        }
    } else {
        warn!("❌ Could not find or scrape any Orbital Market data for: {}", asset.name);
    }

    Ok(())
} 