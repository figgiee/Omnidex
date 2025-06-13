use std::fs;
use std::path::Path;

pub fn determine_asset_type_from_path(folder_path: &Path) -> String {
    const DEFAULT_CATEGORY: &str = "uncategorized";
    
    // Define the valid categories that match the frontend
    const VALID_CATEGORIES: &[&str] = &[
        "2d-asset",
        "3d-model", 
        "animation",
        "audio",
        "education-tutorial",
        "environment",
        "game-system",
        "game-template",
        "hdri",
        "material",
        "smart-asset",
        "tool-and-plugin",
        "ui",
        "vfx",
    ];

    // Find the first .json file in the directory
    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                // Try to read and parse the json file
                if let Ok(file_content) = fs::read_to_string(&path) {
                    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&file_content) {
                        // Look for a 'category' or 'asset_type' field
                        if let Some(category) = json_value.get("category").and_then(|v| v.as_str()) {
                            let normalized = normalize_category_name(category);
                            return map_to_valid_category(&normalized, VALID_CATEGORIES);
                        }
                        if let Some(asset_type) = json_value.get("asset_type").and_then(|v| v.as_str()) {
                            let normalized = normalize_category_name(asset_type);
                            return map_to_valid_category(&normalized, VALID_CATEGORIES);
                        }
                    }
                }
            }
        }
    }

    // Fallback to parent folder name if no json category is found
    let parent_dir = folder_path.parent().and_then(|p| p.file_name()).and_then(|s| s.to_str());
    let category = parent_dir.map_or(DEFAULT_CATEGORY.to_string(), |s| normalize_category_name(s));
    map_to_valid_category(&category, VALID_CATEGORIES)
}

fn normalize_category_name(name: &str) -> String {
    name.to_lowercase().replace([' ', '_'], "-")
}

fn map_to_valid_category(category: &str, valid_categories: &[&str]) -> String {
    // Direct match first
    if valid_categories.contains(&category) {
        return category.to_string();
    }
    
    // Try to map common variations to valid categories
    match category {
        // 2D related
        "2d" | "2d-assets" | "2d-asset" | "2d-graphics" | "graphics" | "images" => "2d-asset",
        
        // 3D related  
        "3d" | "3d-assets" | "3d-model" | "3d-models" | "models" | "mesh" | "meshes" => "3d-model",
        
        // Materials and textures
        "texture" | "textures" | "textures-materials" | "textures-&-materials" | "materials" => "material",
        
        // Animation related
        "animations" | "anim" | "anims" | "motion" | "mocap" => "animation",
        
        // Audio related
        "sound" | "sounds" | "music" | "sfx" | "audio-files" => "audio",
        
        // Environment related
        "env" | "environment-assets" | "environments" | "landscape" | "terrain" => "environment",
        
        // VFX related
        "effects" | "particle" | "particles" | "visual-effects" => "vfx",
        
        // UI related
        "interface" | "gui" | "hud" | "menu" | "menus" => "ui",
        
        // Game systems
        "gameplay" | "mechanics" | "systems" => "game-system",
        
        // Tools and plugins
        "tool" | "tools" | "plugin" | "plugins" | "utility" | "utilities" => "tool-and-plugin",
        
        // Templates
        "template" | "templates" | "blueprint" | "blueprints" => "game-template",
        
        // Education
        "tutorial" | "tutorials" | "learning" | "course" | "courses" => "education-tutorial",
        
        // Smart assets
        "smart" | "intelligent" | "procedural" => "smart-asset",
        
        // HDRI
        "hdr" | "hdri-images" | "skybox" | "skyboxes" => "hdri",
        
        // Default fallback
        _ => "material", // Most generic fallback for unknown categories
    }.to_string()
} 