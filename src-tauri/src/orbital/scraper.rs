use anyhow::Result;
use scraper::{Html, Selector, ElementRef};
use tracing::info;

use super::models::OrbitalAsset;
use super::api::OrbitalApiClient;

pub async fn fetch_product_page_html(client: &OrbitalApiClient, product_url: &str) -> Result<String> {
    info!("Fetching product page: {}", product_url);
    client.make_request_with_retry(product_url, 3).await
}

pub fn parse_asset_details_from_html(client: &OrbitalApiClient, html_content: &str) -> OrbitalAsset {
    let document = Html::parse_document(html_content);
    let mut asset = OrbitalAsset::default();

    let tab_selector = Selector::parse(".tabs-bar .tab").unwrap();
    let content_selector = Selector::parse(".tabs-content > div").unwrap();

    let tabs: Vec<String> = document.select(&tab_selector).map(|el| el.text().collect::<String>().trim().to_lowercase()).collect();
    let contents: Vec<ElementRef> = document.select(&content_selector).collect();

    let mut description_html = String::new();
    let mut tech_details_html = String::new();

    if let Some(index) = tabs.iter().position(|t| t == "detailed description") {
        if let Some(content_el) = contents.get(index) {
            description_html = content_el.inner_html();
        }
    }

    if let Some(index) = tabs.iter().position(|t| t == "technical description") {
        if let Some(content_el) = contents.get(index) {
            tech_details_html = content_el.inner_html();
        }
    }

    if !description_html.is_empty() {
        asset.description = Some(description_html);
    }
    
    if !tech_details_html.is_empty() {
        asset.technical_details = Some(tech_details_html);
    }

    // Fallback for pages without tabs
    if asset.description.is_none() {
        if let Some(description) = extract_text_from_element(&document.root_element(), &client.selectors.product_description_selector) {
            asset.description = Some(description);
        }
    }
    if asset.technical_details.is_none() {
        if let Some(tech_details) = extract_text_from_element(&document.root_element(), &client.selectors.product_technical_details_selector) {
            asset.technical_details = Some(tech_details);
        }
    }

    // Extract rating count from various possible selectors
    let rating_count_selectors = vec![
        ".product-header__rating .total",
        ".rating-count",
        ".review-count", 
        "[class*='rating'] [class*='count']",
        "[class*='review'] [class*='count']"
    ];
    
    for selector in rating_count_selectors {
        if let Some(rating_count_str) = extract_text_from_element(&document.root_element(), selector) {
            let numeric_part = rating_count_str.trim_matches(|p: char| !p.is_numeric());
            if let Ok(count) = numeric_part.parse::<i32>() {
                asset.rating_count = Some(count);
                break;
            }
        }
    }
    
    // Enhanced rating extraction from star widths with multiple selector patterns
    let rating_selectors = vec![
        ".rating.hasRatings.stars .star .front", // Based on the HTML you provided
        ".product-header__rating .rating .star .front", // Original selector
        ".rating .star .front", // Simpler fallback
        "[class*='rating'] [class*='star'] .front", // Generic pattern
        ".stars .star .front" // Another fallback
    ];
    
    for selector_str in rating_selectors {
        if let Ok(star_selector) = Selector::parse(selector_str) {
            let stars: Vec<f64> = document.select(&star_selector)
                .filter_map(|star| {
                    star.value().attr("style")
                        .and_then(|style| {
                            // Extract width percentage from style="width: 80%;" or style="width:80%"
                            extract_width_percentage(style)
                        })
                })
                .collect();
            
            if !stars.is_empty() {
                let total_rating: f64 = stars.iter().sum();
                asset.rating_average = Some(total_rating);
                info!("✅ Successfully extracted {} star rating: {:.2} from {} stars", 
                      selector_str, total_rating, stars.len());
                break; // Stop after first successful extraction
            }
        }
    }
    
    // Fallback: try to extract rating from text content if star parsing failed
    if asset.rating_average.is_none() {
        let text_rating_selectors = vec![
            ".rating-value",
            ".average-rating", 
            ".product-rating",
            "[class*='rating'] [class*='value']"
        ];
        
        for selector in text_rating_selectors {
            if let Some(rating_text) = extract_text_from_element(&document.root_element(), selector) {
                // Try to parse rating like "4.7", "4.7/5", "4.7 stars", etc.
                if let Some(rating_value) = extract_rating_from_text(&rating_text) {
                    asset.rating_average = Some(rating_value);
                    info!("✅ Extracted rating from text: {:.2}", rating_value);
                    break;
                }
            }
        }
    }

    asset
}

pub fn extract_text_from_element(element: &ElementRef, selector_str: &str) -> Option<String> {
    let selector = Selector::parse(selector_str).ok()?;
    let text: String = element.select(&selector).flat_map(|el| el.text()).collect();
    if text.trim().is_empty() {
        None
    } else {
        Some(text.trim().to_string())
    }
}

pub fn find_first_product_link(html_content: &str, item_selector_str: &str) -> Option<String> {
    let document = Html::parse_document(html_content);
    let item_selector = Selector::parse(item_selector_str).ok()?;
    let link_selector = Selector::parse("a[href*='/product/']").ok()?;
    
    // Find the first product item, then find a link within it
    document.select(&item_selector).next()
        .and_then(|item| item.select(&link_selector).next())
        .and_then(|link| link.value().attr("href"))
        .map(|href| {
            if href.starts_with("http") {
                href.to_string()
            } else {
                format!("https://orbital-market.com{}", href)
            }
        })
}

/// Extract width percentage from CSS style attribute
/// Handles formats like "width: 80%", "width:80%", "width: 100%;", etc.
fn extract_width_percentage(style: &str) -> Option<f64> {
    // Look for width property in style attribute
    if let Some(width_start) = style.find("width") {
        let width_part = &style[width_start..];
        
        // Find the colon after "width"
        if let Some(colon_pos) = width_part.find(':') {
            let value_part = &width_part[colon_pos + 1..];
            
            // Find the percentage sign
            if let Some(percent_pos) = value_part.find('%') {
                let width_str = value_part[..percent_pos].trim();
                if let Ok(width_value) = width_str.parse::<f64>() {
                    // Convert percentage to decimal (e.g., 80% -> 0.8)
                    return Some(width_value / 100.0);
                }
            }
        }
    }
    None
}

/// Extract rating value from text content
/// Handles formats like "4.7", "4.7/5", "4.7 stars", "Rating: 4.7", etc.
fn extract_rating_from_text(text: &str) -> Option<f64> {
    // Simple approach: find first number that looks like a rating
    let cleaned_text = text.trim().to_lowercase();
    
    // Split by common separators and look for valid rating numbers
    let words: Vec<&str> = cleaned_text.split(&[' ', ':', '/', '\t', '\n']).collect();
    
    for word in words {
        // Try to parse each word as a number
        if let Ok(rating) = word.parse::<f64>() {
            // Check if it's in a reasonable rating range (0.0 to 5.0)
            if rating >= 0.0 && rating <= 5.0 {
                return Some(rating);
            }
        }
    }
    
    None
} 