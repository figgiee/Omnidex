use anyhow::{anyhow, Result};
use reqwest::{Client, ClientBuilder};
use ::scraper::{Html, Selector};
use std::time::Duration;
use tracing::{info, warn};
use url::Url;
use regex::Regex;
use tokio;
use std::sync::atomic::{AtomicU64, Ordering};
use serde::Deserialize;
use std::sync::Arc;
use std::collections::HashSet;
use lazy_static::lazy_static;

use super::models::*;
use super::api_models;
use super::scraper;

// Define ORBITAL_SELECTORS at the top using lazy_static!
lazy_static! {
    static ref ORBITAL_SELECTORS: Arc<OrbitalSelectors> = load_orbital_selectors();
}

const ORBITAL_BASE_URL: &str = "https://orbital-market.com";
const ORBITAL_API_BASE_URL: &str = "https://orbital-market.com/api/products";
const ORBITAL_SEARCH_URL: &str = "https://orbital-market.com/search";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);
const POOL_IDLE_TIMEOUT: Duration = Duration::from_secs(90);
const POOL_MAX_IDLE_PER_HOST: usize = 10;

static REQUEST_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Deserialize, Debug, Clone)]
pub struct OrbitalSelectors {
    pub search_result_item_selector: String,
    pub search_result_link_selector: String,
    pub search_result_title_selector: String,
    pub search_result_price_selector: String,
    pub search_result_seller_selector: String,
    pub search_result_image_selector: String,

    // Selectors for the product page
    pub product_description_selector: String,
    pub product_technical_details_selector: String,
    pub product_rating_selector: String,
    pub product_rating_count_selector: String,
}

impl Default for OrbitalSelectors {
    fn default() -> Self {
        OrbitalSelectors {
            search_result_item_selector: ".listing-card, .product-card, .asset-card, [class*='card']".to_string(),
            search_result_link_selector: "a[href*='/listings/'], a[href*='/products/']".to_string(),
            search_result_title_selector: "h3, h2, .title, .name, [class*='title'], [class*='name']".to_string(),
            search_result_price_selector: ".price, [class*='price']".to_string(),
            search_result_seller_selector: ".seller, .author, [class*='seller'], [class*='author']".to_string(),
            search_result_image_selector: "img".to_string(),
            
            // Default selectors for the product page
            product_description_selector: "#description, [class*='description'], #overview, [class*='overview']".to_string(),
            product_technical_details_selector: "#tech-details, [class*='tech-details'], #technical-details, [class*='technical-details']".to_string(),
            product_rating_selector: "[class*='rating'] [class*='star']".to_string(),
            product_rating_count_selector: "[class*='rating-count'], [class*='review-count']".to_string(),
        }
    }
}

fn load_orbital_selectors() -> Arc<OrbitalSelectors> {
    // Embed the selectors file content directly into the binary
    const SELECTORS_JSON: &str = include_str!("../../orbital_selectors.json");

    match serde_json::from_str::<OrbitalSelectors>(SELECTORS_JSON) {
        Ok(selectors) => {
            info!("Successfully loaded Orbital selectors from embedded configuration.");
            Arc::new(selectors)
        }
        Err(e) => {
            warn!("Failed to parse embedded orbital_selectors.json: {}. Using default selectors.", e);
            Arc::new(OrbitalSelectors::default())
        }
    }
}

fn parse_price(price_str: &str) -> Option<f64> {
    let cleaned_price = price_str
        .replace(['$', '€', '£', ',', ' '], "")
        .trim()
        .to_lowercase();
    if cleaned_price == "free" {
        return Some(0.0);
    }
    cleaned_price.parse::<f64>().ok()
}

#[derive(Debug, Clone)]
pub struct OrbitalApiClient {
    client: Client,
    pub selectors: Arc<OrbitalSelectors>,
}

impl OrbitalApiClient {
    pub fn new() -> Result<Self> {
        info!("🔧 Creating new OrbitalApiClient with respectful rate limiting");
        let client = ClientBuilder::new()
            .http1_only()
            .http1_title_case_headers()
            .pool_idle_timeout(POOL_IDLE_TIMEOUT)
            .pool_max_idle_per_host(POOL_MAX_IDLE_PER_HOST)
            .timeout(REQUEST_TIMEOUT)
            .connect_timeout(CONNECTION_TIMEOUT)
            .cookie_store(true)
            .tcp_keepalive(Duration::from_secs(60))
            .tcp_nodelay(true)
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()?;
        info!("✅ OrbitalApiClient created successfully");
        Ok(Self { 
            client,
            selectors: ORBITAL_SELECTORS.clone(),
        })
    }
    
    pub fn new_with_defaults() -> Result<Self> { // Added based on commands/mod.rs usage
        Self::new() 
    }

    pub fn from_client(client: Client, selectors: Arc<OrbitalSelectors>) -> Self {
        Self { client, selectors }
    }

    fn get_rate_limit_delay(&self) -> Duration {
        let count = REQUEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let base_delay = match count % 10 {
            0 => 1000, 1 => 1500, 2 => 2000, 3..=4 => 2500, 5..=6 => 3000, 7..=8 => 3500, _ => 4000,
        };
        let jitter = (rand::random::<f64>() - 0.5) * 0.4 * base_delay as f64;
        Duration::from_millis((base_delay as f64 + jitter).max(800.0) as u64)
    }

    fn get_user_agent(&self) -> &'static str {
        let agents = [
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:132.0) Gecko/20100101 Firefox/132.0",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:132.0) Gecko/20100101 Firefox/132.0",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15",
        ];
        agents[REQUEST_COUNTER.load(Ordering::SeqCst) as usize % agents.len()]
    }

    fn get_browser_headers(&self, url: &str) -> Vec<(&'static str, String)> {
        let user_agent = self.get_user_agent();
        let is_first_visit = !url.contains("orbital-market.com") || url == ORBITAL_BASE_URL;
        let mut headers: Vec<(&'static str, String)> = Vec::new();
        headers.push(("Accept", "application/json, text/plain, */*".to_string()));
        headers.push(("Accept-Language", "en-US,en;q=0.9".to_string()));
        headers.push(("Cache-Control", "no-cache".to_string()));
        if !is_first_visit { headers.push(("Referer", ORBITAL_BASE_URL.to_string())); }
        if user_agent.contains("Chrome") {
            headers.push(("Sec-Fetch-Dest", "empty".to_string()));
            headers.push(("Sec-Fetch-Mode", "cors".to_string()));
            headers.push(("Sec-Fetch-Site", "same-origin".to_string()));
        } else if user_agent.contains("Firefox") {
            headers.push(("Accept-Language", "en-US,en;q=0.5".to_string()));
            headers.push(("DNT", "1".to_string()));
        }
        headers
    }
    
    pub async fn make_request_with_retry(&self, url: &str, max_retries: u32) -> Result<String> {
        self.make_request_with_retry_internal(url, max_retries).await
    }

    async fn make_request_with_retry_internal(&self, url: &str, max_retries: u32) -> Result<String> {
        let mut attempts = 0;
        loop {
            attempts += 1;
            let delay = self.get_rate_limit_delay();
            tokio::time::sleep(delay).await;

            info!("Attempt {} to fetch URL: {}", attempts, url);
            let headers = self.get_browser_headers(url);
            let mut request_builder = self.client.get(url).header("User-Agent", self.get_user_agent());
            for (key, value) in headers {
                request_builder = request_builder.header(key, value);
            }

            match request_builder.send().await {
                Ok(response) => {
                    let status = response.status();
                    if status.is_success() {
                        info!("Successfully fetched URL: {} with status: {}", url, status);
                        return response.text().await.map_err(|e| anyhow!("Failed to read response text: {}", e));
                    } else if status.is_server_error() || status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                        warn!("Server error ({}) for URL: {}. Retrying if attempts < {}...", status, url, max_retries);
                        if attempts >= max_retries {
                            return Err(anyhow!("Failed to fetch URL {} after {} attempts, last status: {}", url, max_retries, status));
                        }
                    } else {
                        warn!("Client error ({}) for URL: {}. Not retrying.", status, url);
                        return Err(anyhow!("Failed to fetch URL {}: Client error status {}", url, status));
                    }
                }
                Err(e) => {
                    warn!("Network error fetching URL {}: {}. Retrying if attempts < {}...", url, e, max_retries);
                    if attempts >= max_retries {
                        return Err(anyhow!("Failed to fetch URL {} after {} retries: {}", url, max_retries, e));
                    }
                }
            }
        }
    }
    
    pub async fn fetch_asset_details_by_slug_or_id(&self, slug_or_id: &str) -> Result<OrbitalAsset> {
        self.fetch_asset_details_robust(slug_or_id).await
    }

    pub async fn test_public_access(&self) -> Result<bool> {
        info!("🧪 Testing public access to Orbital Market base URL...");
        match self.make_request_with_retry(ORBITAL_BASE_URL, 1).await {
            Ok(html) => {
                let is_valid = self.validate_orbital_content(&html);
                info!("Public access test result: valid content = {}", is_valid);
                Ok(is_valid)
            }
            Err(e) => {
                warn!("Public access test failed: {}", e);
                Err(e)
            }
        }
    }

    fn validate_orbital_content(&self, html: &str) -> bool {
        // Check for Orbital Market content markers
        html.contains("orbital-market.com") || // Domain references
        html.contains("Orbital Market") || // Site branding
        html.contains("Epic Games") || // Epic Games branding
        html.contains("Unreal Engine") || // UE references
        html.contains("marketplace") || // Marketplace references
        html.contains("asset") || // Asset references
        html.len() > 1000 // Basic length check - real pages are substantial
    }

    pub async fn search_assets(&self, query: &str, _limit: i32) -> Result<Vec<OrbitalAsset>> {
        let search_url = format!("{}?q={}", ORBITAL_SEARCH_URL, query);
        info!("Searching Orbital Market: {}", search_url);
        let html = self.make_request_with_retry(&search_url, 3).await?;
        self.parse_orbital_search_results(&html)
    }

    pub fn extract_product_id_from_url(&self, url_str: &str) -> Option<String> {
        Url::parse(url_str).ok().and_then(|url| {
            url.path_segments()?.last().map(String::from)
        })
    }
    pub async fn search_by_folder_name(&self, folder_name: &str) -> Result<Vec<OrbitalAsset>> {
        info!("▶️ Attempting to find match for folder: {}", folder_name);
    
        // Generate multiple slug variations to try
        let slug_variations = self.generate_slug_variations(folder_name);
        info!("🔍 Generated {} slug variations: {:?}", slug_variations.len(), slug_variations);
        
        // Try each slug variation until we find a match
        for (i, slug) in slug_variations.iter().enumerate() {
            info!("Trying slug variation {}: '{}'", i + 1, slug);
            if let Ok(Some(asset)) = self.try_fetch_product(slug).await {
                info!("✅ Successfully found direct product match with variation {}: {:?}", i + 1, asset.title);
                return Ok(vec![asset]);
            }
        }
    
        // Strategy 3: Fallback to web search
        info!("⚠️ No direct match found with any slug variations, falling back to web search for: {}", folder_name);
        let cleaned_name = self.clean_folder_name_for_search(folder_name);
        self.search_assets(&cleaned_name, 10).await
    }
    
    async fn try_fetch_product(&self, product_slug: &str) -> Result<Option<OrbitalAsset>> {
        let api_url = format!("{}/product/{}", ORBITAL_API_BASE_URL, product_slug);
        info!("Attempting direct product API URL: {}", api_url);
    
        match self.make_request_with_retry(&api_url, 1).await {
            Ok(json_content) => {
                info!("📦 Fetched raw data from Orbital: {}", json_content);
                match serde_json::from_str::<api_models::ApiResponse>(&json_content) {
                    Ok(api_response) => {
                        let asset = self.map_api_response_to_orbital_asset(api_response);
                        info!("✅ Successfully parsed API response for: {:?}", asset.title);
                        info!("🔎 Found asset data: {:?}", asset);
                        Ok(Some(asset))
                    },
                    Err(e) => {
                        warn!("Failed to parse JSON for {}: {}", product_slug, e);
                        // Optionally, save the problematic JSON for debugging
                        // self.save_debug_parsing_data(product_slug, &serde_json::Value::String(json_content)).await?;
                        Err(anyhow!("JSON parsing error: {}", e))
                    }
                }
            }
            Err(e) => {
                warn!("Direct API fetch failed for slug '{}': {}", product_slug, e);
                Ok(None)
            }
        }
    }
    
    fn map_api_response_to_orbital_asset(&self, api_response: api_models::ApiResponse) -> OrbitalAsset {
        let gallery_images: Vec<String> = api_response.media.images.clone().into_iter().collect();

        let rating = if api_response.review.count > 0 {
            // The API rating might be scaled (e.g., 45 for 4.5 stars) or direct (e.g., 4.5)
            // Let's handle both cases intelligently
            let raw_rating = api_response.review.rating as f32;
            let normalized_rating = if raw_rating > 5.0 {
                // If rating is > 5, assume it's scaled (e.g., 45 = 4.5 stars)
                raw_rating / 10.0
            } else {
                // If rating is <= 5, assume it's already in correct scale
                raw_rating
            };
            
            Some(OrbitalRating {
                average_rating: normalized_rating,
                total_ratings: api_response.review.count,
            })
        } else {
            None
        };

        let price = Some(OrbitalPrice {
            amount: (api_response.price.value as f64) / 100.0,
            currency: "USD".to_string(), // Assuming USD, API doesn't specify
        });
        
        OrbitalAsset {
            id: api_response.id.clone(),
            product_slug: Some(api_response.slug.clone()),
            title: Some(api_response.title.clone()),
            description: Some(api_response.description.long.clone()),
            technical_details: Some(api_response.description.technical.clone()),
            seller: Some(api_response.owner.name.clone()),
            categories: vec![api_response.category.clone()],
            supported_versions: vec![api_response.engine.min.clone(), api_response.engine.max.clone()],
            gallery_images,
            rating_average: rating.as_ref().map(|r| r.average_rating as f64),
            rating_count: rating.as_ref().map(|r| r.total_ratings),
            price: price.as_ref().map(|p| p.amount),
            release_date: Some(api_response.release_date.clone()),
            last_modified: Some(String::new()), // Not available in this API response
            raw_json: serde_json::to_value(&api_response).ok(),
            source_url: Some(format!("{}/product/{}", ORBITAL_BASE_URL, api_response.slug)),
            thumbnail_url: Some(api_response.media.thumbnail.clone()),
        }
    }

    /// Generate multiple slug variations to try for better matching
    fn generate_slug_variations(&self, folder_name: &str) -> Vec<String> {
        let mut variations = std::collections::HashSet::new();

        // 1. Slug from the original name
        variations.insert(self.folder_name_to_product_slug(folder_name));

        // 2. Slug from a fully normalized name
        let normalized_name = self.normalize_name(folder_name);
        variations.insert(self.folder_name_to_product_slug(&normalized_name));

        // 3. Slugs from variations with version patterns removed
        let cleaned_variations = self.remove_version_patterns(folder_name);
        for cleaned in cleaned_variations {
            variations.insert(self.folder_name_to_product_slug(&cleaned));
        }

        // 4. Slugs from additional heuristic-based variations
        let additional_variations = self.generate_additional_slug_variations(folder_name);
        for variation in additional_variations {
            variations.insert(variation); // These are already slug-formatted
        }

        // Remove empty strings and collect
        variations.into_iter()
            .filter(|s| !s.is_empty())
            .collect()
    }
    
    /// Remove various version patterns from folder names
    fn remove_version_patterns(&self, folder_name: &str) -> Vec<String> {
        let mut variations = Vec::new();
        
        // Comprehensive version patterns to remove
        let version_patterns = vec![
            // Engine versions in parentheses: (5 0), (4 18), (UE5.0), etc.
            r"\([Uu]?[Ee]?\s*\d+(?:\s*\.\s*\d+)*\s*\)",
            // Version numbers: v1.0, V2.3, etc.
            r"[vV]\d+(?:\.\d+)*",
            // Standalone parentheses with numbers/dots: (5 0), (4.27), etc.
            r"\(\s*\d+(?:[\s\.]\d+)*\s*\)",
            // UE version patterns: UE4, UE5, UE4.27, etc.
            r"[Uu][Ee]\d+(?:\.\d+)*",
            // Extra spaces or underscores at the end
            r"[\s_-]+$",
        ];
        
        for pattern in version_patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                let cleaned = re.replace_all(folder_name, "").trim().to_string();
                if !cleaned.is_empty() && cleaned != folder_name {
                    variations.push(cleaned);
                }
            }
        }
        
        // Also try removing anything after the first parenthesis
        if let Some(paren_pos) = folder_name.find('(') {
            let before_paren = folder_name[..paren_pos].trim().to_string();
            if !before_paren.is_empty() {
                variations.push(before_paren);
            }
        }
        
        variations
    }
    
    /// Generate additional slug variations using different approaches
    fn generate_additional_slug_variations(&self, folder_name: &str) -> Vec<String> {
        let mut variations = Vec::new();
        
        // Clean the folder name first
        let cleaned = folder_name
            .replace("_", " ")
            .replace("-", " ");
        
        // Extract just the main words (remove common suffixes/prefixes)
        let words: Vec<&str> = cleaned.split_whitespace()
            .filter(|word| {
                let w = word.to_lowercase();
                // Filter out common non-essential words
                !["ue4", "ue5", "unreal", "engine", "pack", "asset", "assets", "v", "version"].contains(&w.as_str())
                    && !w.chars().all(|c| c.is_numeric() || c == '.')
                    && !w.starts_with('v') && w.len() > 1
            })
            .collect();
        
        if !words.is_empty() {
            // Join main words with hyphens
            let main_words_slug = words.join("-").to_lowercase();
            let re = regex::Regex::new(r"[^a-z0-9-]").unwrap();
            let clean_main_slug = re.replace_all(&main_words_slug, "").to_string();
            variations.push(clean_main_slug);
            
            // Try without the last word (sometimes asset names have extra descriptors)
            if words.len() > 1 {
                let without_last = words[..words.len()-1].join("-").to_lowercase();
                let clean_without_last = re.replace_all(&without_last, "").to_string();
                variations.push(clean_without_last);
            }
            
            // Try without the first word (sometimes asset names have prefixes)
            if words.len() > 1 {
                let without_first = words[1..].join("-").to_lowercase();
                let clean_without_first = re.replace_all(&without_first, "").to_string();
                variations.push(clean_without_first);
            }
        }
        
        variations
    }

    /// Normalize folder name: remove UE tokens (ue4, ue5, ue5.3), trailing version/counter numbers etc.
    fn normalize_name(&self, raw: &str) -> String {
        // Lowercase for consistency
        let mut s = raw.to_lowercase();

        // Remove UE tokens like "ue5", "ue5.3", "ue4", "ue4.27"
        let re_ue = Regex::new(r"ue\d+(?:\.\d+)?").unwrap();
        s = re_ue.replace_all(&s, "").into_owned();

        // Preserve trailing number if it is part of "vol X" (e.g. "vol 1")
        let re_vol = Regex::new(r"\bvol[\s_-]*\d+$").unwrap();
        if !re_vol.is_match(&s) {
            // Remove standalone version numbers/counters at the end (e.g. " 3", " v2", " 1")
            let re_trailing_num = Regex::new(r"[\s_-]*\d+$").unwrap();
            s = re_trailing_num.replace_all(&s, "").into_owned();
        }

        // Collapse multiple spaces/underscores/dashes to single space
        let re_multi_space = Regex::new(r"[\s_-]+").unwrap();
        s = re_multi_space.replace_all(&s, " ").into_owned();

        s.trim().to_string()
    }

    fn folder_name_to_product_slug(&self, name: &str) -> String {
        let name = name.to_lowercase();
        // Replace separators with spaces for consistent splitting
        let name = name.replace('_', " ").replace('-', " ");
        // Remove all non-alphanumeric characters, but keep spaces
        let re = regex::Regex::new(r"[^a-z0-9\s]").unwrap();
        let name = re.replace_all(&name, "");
        // Join words with hyphens to form the final slug
        name.split_whitespace().collect::<Vec<&str>>().join("-")
    }

    fn clean_folder_name_for_search(&self, folder_name: &str) -> String {
        folder_name
            .replace("_", " ")
            .replace("-", " ")
    }
    
    fn extract_main_keywords(&self, cleaned_name: &str) -> Vec<String> {
        let name = cleaned_name.to_lowercase();
        let stop_words: HashSet<&str> = ["pack", "asset", "and", "ue4", "ue5", "unreal", "engine"].iter().cloned().collect();
        name.split_whitespace()
            .filter(|word| !stop_words.contains(word))
            .map(String::from)
            .collect()
    }

    pub fn test_folder_name_cleaning(&self, folder_name: &str) -> (String, Vec<String>) {
        let cleaned_name = self.clean_folder_name_for_search(folder_name);
        let keywords = self.extract_main_keywords(&cleaned_name);
        (cleaned_name, keywords)
    }

    /// Test function to see all generated slug variations for a given folder name
    pub fn test_slug_generation(&self, folder_name: &str) -> Vec<String> {
        info!("🧪 Testing slug generation for: '{}'", folder_name);
        let variations = self.generate_slug_variations(folder_name);
        for (i, variation) in variations.iter().enumerate() {
            info!("  Variation {}: '{}'", i + 1, variation);
        }
        variations
    }

    pub async fn fetch_asset_details(&self, product_id: &str) -> Result<OrbitalAsset> {
        self.fetch_asset_details_robust(product_id).await
    }
    
    fn extract_attribute_from_element(element: &::scraper::ElementRef, selector_str: &str, attr_name: &str) -> Option<String> {
        let selector = Selector::parse(selector_str).ok()?;
        element.select(&selector).next()?.value().attr(attr_name).map(str::to_owned)
    }

    async fn fetch_asset_details_robust(&self, product_id_or_url: &str) -> Result<OrbitalAsset> {
        info!("Fetching robust details for: {}", product_id_or_url);

        let product_slug = self.extract_product_id_from_url(product_id_or_url)
            .unwrap_or_else(|| product_id_or_url.to_string());

        info!("Fetching robust details for slug: {}", product_slug);

        // Try API call first
        if let Ok(Some(asset_details)) = self.try_fetch_product(&product_slug).await {
            return Ok(asset_details);
        }

        // If API call fails, fallback to scraping the web page
        let product_url = format!("{}/en-US/product/{}", ORBITAL_BASE_URL, &product_slug);
        match scraper::fetch_product_page_html(self, &product_url).await {
            Ok(html) => {
                let mut asset_details = scraper::parse_asset_details_from_html(self, &html);
                // We need to merge this with any info we got from the search results
                // For now, let's just populate the slug and url
                asset_details.product_slug = Some(product_slug);
                asset_details.source_url = Some(product_url);
                return Ok(asset_details);
            }
            Err(e) => {
                warn!("Failed to fetch or parse product page for {}: {}", product_slug, e);
                // Continue to try other methods if available
            }
        }


        // If scraping fails, try constructing a search query
        let search_query = self.clean_folder_name_for_search(product_id_or_url);
        match self.search_assets(&search_query, 1).await {
            Ok(mut results) => {
                if let Some(asset) = results.pop() {
                    return Ok(asset);
                }
            }
            Err(e) => {
                warn!("Failed to search for product {}: {}", product_id_or_url, e);
            }
        }

        Err(anyhow!("Failed to fetch asset details for {}", product_id_or_url))
    }

    /// Save detailed parsing data for debugging and analysis
    pub async fn save_debug_parsing_data(&self, asset_slug: &str, parsing_data: &serde_json::Value) -> Result<()> {
        let debug_dir = std::path::Path::new("debug_parsing");
        if !debug_dir.exists() {
            std::fs::create_dir_all(debug_dir)?;
        }
        
        let debug_file = debug_dir.join(format!("{}_parsing_debug.json", asset_slug));
        let pretty_json = serde_json::to_string_pretty(parsing_data)?;
        std::fs::write(debug_file, pretty_json)?;
        
        info!("💾 Saved parsing debug data for {} to debug_parsing/{}_parsing_debug.json", asset_slug, asset_slug);
        Ok(())
    }

    /// Extract all available data from a product page and save it for analysis
    pub async fn extract_full_product_data(&self, product_slug: &str) -> Result<serde_json::Value> {
        let product_url = format!("{}/product/{}", ORBITAL_API_BASE_URL, product_slug);
        let json_content = self.make_request_with_retry(&product_url, 3).await?;
        
        let data: serde_json::Value = serde_json::from_str(&json_content)?;
        Ok(data)
    }

    pub async fn get_asset_metadata(&self, asset_id: &str) -> Result<serde_json::Value> {
        let url = format!("{}/product/{}", ORBITAL_API_BASE_URL, asset_id);
        let response_text = self.make_request_with_retry(&url, 3).await?;
        serde_json::from_str(&response_text)
            .map_err(|e| anyhow!("Failed to parse asset metadata JSON: {}", e))
    }

    fn parse_orbital_search_results(&self, html_content: &str) -> Result<Vec<OrbitalAsset>> {
        let document = Html::parse_document(html_content);
        let mut assets = Vec::new();
        let item_selector = Selector::parse(&self.selectors.search_result_item_selector)
            .map_err(|e| anyhow!("Failed to parse item selector: {}", e))?;

        for element in document.select(&item_selector) {
            let mut asset = OrbitalAsset::default();

            if let Some(title) = scraper::extract_text_from_element(&element, &self.selectors.search_result_title_selector) {
                asset.title = Some(title);
            }
            if let Some(price_str) = scraper::extract_text_from_element(&element, &self.selectors.search_result_price_selector) {
                asset.price = parse_price(&price_str);
            }
            if let Some(seller) = scraper::extract_text_from_element(&element, &self.selectors.search_result_seller_selector) {
                asset.seller = Some(seller);
            }
            if let Some(image_url) = Self::extract_attribute_from_element(&element, &self.selectors.search_result_image_selector, "src") {
                asset.thumbnail_url = Some(image_url);
            }
            if let Some(asset_url) = Self::extract_attribute_from_element(&element, &self.selectors.search_result_link_selector, "href") {
                if let Some(slug) = self.extract_product_id_from_url(&asset_url) {
                    asset.product_slug = Some(slug);
                }
                asset.source_url = Some(asset_url);
            }

            if asset.title.is_some() {
                assets.push(asset);
            }
        }
        Ok(assets)
    }


} 