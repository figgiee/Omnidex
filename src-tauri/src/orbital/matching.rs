use anyhow::Result;
use strsim::jaro_winkler;
use tracing::{info, debug, warn};

use crate::models::Asset;
use super::models::{AssetMatch, MatchType, OrbitalAsset};
use super::api::OrbitalApiClient;

#[derive(Debug, Clone)]
pub struct AssetMatcher {
    // Configuration for matching thresholds
    exact_match_threshold: f64,
    high_confidence_threshold: f64,
    medium_confidence_threshold: f64,
    low_confidence_threshold: f64,
    orbital_client: OrbitalApiClient,
}

impl AssetMatcher {
    pub fn new(orbital_client: OrbitalApiClient) -> Self {
        Self {
            exact_match_threshold: 0.95,
            high_confidence_threshold: 0.85,
            medium_confidence_threshold: 0.70,
            low_confidence_threshold: 0.50,
            orbital_client,
        }
    }

    /// Match local assets against Orbital marketplace using web scraping
    pub async fn match_assets(&self, local_assets: &[Asset]) -> Result<Vec<AssetMatch>> {
        info!("Starting asset matching for {} local assets", local_assets.len());
        
        let mut matches = Vec::new();
        
        for asset in local_assets {
            let asset_match = self.match_single_asset(asset).await?;
            matches.push(asset_match);
        }
        
        info!("Completed asset matching, found {} matches", matches.len());
        Ok(matches)
    }

    /// Match a single local asset using Orbital search
    pub async fn match_single_asset(&self, local_asset: &Asset) -> Result<AssetMatch> {
        debug!("Matching asset: {}", local_asset.name);
        
        // Search Orbital marketplace for similar assets
        let search_results = match self.orbital_client.search_by_folder_name(&local_asset.name).await {
            Ok(results) => results,
            Err(e) => {
                warn!("Failed to search Orbital for asset '{}': {}", local_asset.name, e);
                return Ok(AssetMatch {
                    local_asset_id: local_asset.id.unwrap_or(0),
                    orbital_asset: None,
                    match_confidence: 0.0,
                    match_type: MatchType::NoMatch,
                    match_reasons: vec![format!("Search failed: {}", e)],
                });
            }
        };

        if search_results.is_empty() {
            return Ok(AssetMatch {
                local_asset_id: local_asset.id.unwrap_or(0),
                orbital_asset: None,
                match_confidence: 0.0,
                match_type: MatchType::NoMatch,
                match_reasons: vec!["No matching assets found on Orbital marketplace".to_string()],
            });
        }

        // Find the best match from search results
        let mut best_match: Option<(OrbitalAsset, f64)> = None;
        
        for orbital_asset in search_results {
            let confidence = self.calculate_match_confidence(local_asset, &orbital_asset);
            
            if let Some((_, best_confidence)) = &best_match {
                if confidence > *best_confidence {
                    best_match = Some((orbital_asset, confidence));
                }
            } else {
                best_match = Some((orbital_asset, confidence));
            }
        }

        if let Some((orbital_asset, confidence)) = best_match {
            let match_type = self.determine_match_type(confidence);
            let match_reasons = self.generate_match_reasons(local_asset, &orbital_asset, confidence);
            
            Ok(AssetMatch {
                local_asset_id: local_asset.id.unwrap_or(0),
                orbital_asset: Some(orbital_asset),
                match_confidence: confidence,
                match_type,
                match_reasons,
            })
        } else {
            Ok(AssetMatch {
                local_asset_id: local_asset.id.unwrap_or(0),
                orbital_asset: None,
                match_confidence: 0.0,
                match_type: MatchType::NoMatch,
                match_reasons: vec!["No suitable matches found".to_string()],
            })
        }
    }

    /// Calculate overall match confidence between local and Orbital asset
    fn calculate_match_confidence(&self, local_asset: &Asset, orbital_asset: &OrbitalAsset) -> f64 {
        let mut total_score = 0.0;
        let mut weight_sum = 0.0;

        // Name similarity (highest weight)
        if let Some(orbital_title) = &orbital_asset.title {
            let name_similarity = self.calculate_name_similarity(&local_asset.name, orbital_title);
            total_score += name_similarity * 0.6;
            weight_sum += 0.6;
        }

        // Asset type compatibility
        if self.asset_types_compatible(&local_asset.asset_type, &orbital_asset.categories) {
            total_score += 1.0 * 0.2;
        }
        weight_sum += 0.2;

        // Description/technical details similarity
        if let Some(orbital_description) = &orbital_asset.description {
            let desc_similarity = self.calculate_description_similarity(local_asset, orbital_description);
            total_score += desc_similarity * 0.1;
            weight_sum += 0.1;
        }

        // Keywords in tags (assuming tags are part of description or another field)
        // This part might need adjustment depending on where tags are stored in OrbitalAsset
        // For now, we'll re-use the description similarity as a proxy
        if let Some(orbital_description) = &orbital_asset.description {
            let keyword_score = self.calculate_description_similarity(local_asset, orbital_description);
            total_score += keyword_score * 0.1;
            weight_sum += 0.1;
        }

        // Normalize by total weight
        if weight_sum > 0.0 {
            total_score / weight_sum
        } else {
            0.0
        }
    }

    /// Calculate similarity between local asset and Orbital asset description
    fn calculate_description_similarity(&self, local_asset: &Asset, orbital_description: &str) -> f64 {
        let local_keywords = self.extract_keywords(&local_asset.name);
        let orbital_description_lower = orbital_description.to_lowercase();
        
        let mut matches = 0;
        for keyword in &local_keywords {
            if orbital_description_lower.contains(&keyword.to_lowercase()) {
                matches += 1;
            }
        }
        
        if local_keywords.is_empty() {
            0.0
        } else {
            matches as f64 / local_keywords.len() as f64
        }
    }

    /// Calculate similarity between two asset names
    pub fn calculate_name_similarity(&self, local_name: &str, orbital_name: &str) -> f64 {
        let clean_local_name = self.clean_string(local_name);
        let clean_orbital_name = self.clean_string(orbital_name);
        jaro_winkler(&clean_local_name, &clean_orbital_name)
    }

    /// Clean asset names for better comparison
    fn clean_string(&self, name: &str) -> String {
        name.to_lowercase()
            .replace("_", " ")
            .replace("-", " ")
            .replace("  ", " ")
            .trim()
            .to_string()
    }

    /// Determine match type based on confidence score
    pub fn determine_match_type(&self, confidence: f64) -> MatchType {
        if confidence >= self.exact_match_threshold {
            MatchType::Exact
        } else if confidence >= self.high_confidence_threshold {
            MatchType::HighConfidence
        } else if confidence >= self.medium_confidence_threshold {
            MatchType::MediumConfidence
        } else if confidence >= self.low_confidence_threshold {
            MatchType::LowConfidence
        } else {
            MatchType::NoMatch
        }
    }

    /// Generate match reasons based on comparison
    pub fn generate_match_reasons(&self, local_asset: &Asset, orbital_asset: &OrbitalAsset, confidence: f64) -> Vec<String> {
        let mut reasons = Vec::new();
        
        if let Some(orbital_title) = &orbital_asset.title {
            let name_similarity = self.calculate_name_similarity(&local_asset.name, orbital_title);
            if name_similarity > 0.8 {
                reasons.push(format!("High name similarity: {:.1}%", name_similarity * 100.0));
            }
        }
        
        // Check asset type compatibility
        if self.asset_types_compatible(&local_asset.asset_type, &orbital_asset.categories) {
            reasons.push("Compatible asset type".to_string());
        }
        
        // Add overall confidence
        reasons.push(format!("Overall confidence: {:.1}%", confidence * 100.0));
        
        reasons
    }

    /// Check if local asset type is compatible with Orbital categories
    fn asset_types_compatible(&self, local_type: &str, orbital_categories: &[String]) -> bool {
        let local_type_lower = local_type.to_lowercase();
        orbital_categories.iter().any(|cat| cat.to_lowercase().contains(&local_type_lower))
    }
    
    // Extracts keywords from an asset name
    pub fn extract_keywords(&self, name: &str) -> Vec<String> {
        name.split_whitespace()
            .map(|s| s.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|s| !s.is_empty() && s.len() > 2) // Filter out short/empty words
            .collect()
    }
} 