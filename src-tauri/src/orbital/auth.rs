use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    // Add other fields as necessary
}

#[derive(Debug, Clone)]
pub struct OrbitalAuth {
    client: Client,
    _config: Option<OrbitalAuthConfig>, // Underscore indicates it might not be used yet
}

impl OrbitalAuth {
    pub fn new(client: Client) -> Self {
        info!("🔐 Initializing OrbitalAuth manager.");
        Self {
            client,
            _config: None, // Load config from a file or env var if needed
        }
    }

    /// Check if we have a valid authentication token
    pub fn is_authenticated(&self) -> bool {
        self._config.is_some()
    }

    /// Get the current access token if available
    pub fn get_access_token(&self) -> Option<&str> {
        self._config.as_ref().map(|config| config.client_id.as_str())
    }

    /// Set authentication token manually (for testing or if user provides it)
    pub fn set_token(&mut self, config: OrbitalAuthConfig) {
        info!("Setting Orbital authentication token");
        self._config = Some(config);
    }

    /// Clear current authentication
    pub fn clear_auth(&mut self) {
        info!("Clearing Orbital authentication");
        self._config = None;
    }

    /// Store token securely using keyring
    pub fn store_token_securely(&self, config: &OrbitalAuthConfig) -> Result<()> {
        use keyring::Entry;
        
        let entry = Entry::new("omnidex", "orbital_access_token")?;
        let config_json = serde_json::to_string(config)?;
        entry.set_password(&config_json)?;
        
        info!("Orbital token stored securely in keyring");
        Ok(())
    }

    /// Load token from secure storage
    pub fn load_token_from_storage(&mut self) -> Result<()> {
        use keyring::Entry;
        
        let entry = Entry::new("omnidex", "orbital_access_token")?;
        match entry.get_password() {
            Ok(config_json) => {
                let config: OrbitalAuthConfig = serde_json::from_str(&config_json)?;
                self._config = Some(config);
                info!("Orbital token loaded from secure storage");
                Ok(())
            }
            Err(_) => {
                warn!("No stored Orbital token found");
                Err(anyhow!("No stored token found"))
            }
        }
    }

    /// Validate current token by making a test API call
    pub async fn validate_token(&self) -> Result<bool> {
        if let Some(config) = &self._config {
            // Make a simple authenticated request to validate the token
            let response = self.client
                .post("https://graphql.epicgames.com/ue/marketplace/graphql")
                .header("Authorization", format!("Bearer {}", config.client_id))
                .header("Content-Type", "application/json")
                .json(&serde_json::json!({
                    "query": "query { viewer { id } }"
                }))
                .send()
                .await?;

            Ok(response.status().is_success())
        } else {
            Ok(false)
        }
    }

    /// Instructions for manual token acquisition
    pub async fn get_auth_instructions(&self) -> Result<String> {
        Ok("Authentication with Orbital Market is currently handled by extracting the bearer token directly from your browser's developer tools. Future updates may automate this process.".to_string())
    }

    pub async fn test_token(&self, token: &str) -> Result<bool> {
        let test_url = "https://www.unrealengine.com/marketplace/api/assets";
        let response = self.client
            .get(test_url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        
        info!("Token test response status: {}", response.status());
        Ok(response.status().is_success())
    }
} 