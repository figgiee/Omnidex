export type Asset = {
    id: number;
    name: string;
    file_path: string;
    asset_type: string;
    file_size: number;
    created_date: string;
    modified_date: string;
    first_indexed_timestamp?: string;
    thumbnail_path?: string;
    tags: string[];
    description?: string;
    scan_location_id: number;
    is_favorite: boolean;
    last_accessed?: string;
    file_hash?: string;
    metadata?: string;
    orbital_title?: string;
    orbital_description?: string;
    orbital_technical_details?: string;
    orbital_seller_name?: string;
    orbital_price?: number;
    orbital_release_date?: string;
    orbital_last_modified?: string;
    orbital_rating_average?: number;
    orbital_rating_count?: number;
    orbital_categories?: string;
    orbital_supported_versions?: string;
    orbital_gallery_images?: string;
    orbital_tags?: string;
    orbital_compatible_apps?: string;
    orbital_thumbnail_url?: string;
    orbital_source_url?: string;
    orbital_raw_json?: string;
    orbital_last_checked_timestamp?: string;
    matched_orbital_product_slug?: string;
    orbital_match_confidence?: number;
    orbital_match_type?: string;
    notes?: string;
};

export type AssetDetails = {
    id: number;
    name: string;
    thumbnail_url?: string;
    gallery_images?: string[];
    creator_name?: string;
    rating?: number;
    reviews_count?: number;
    price?: number;
    source_url?: string;
    release_date?: string;
    description?: string;
    technical_description?: string;
    tags: string[];
    scraped_tags: string[];
    file_path: string;
    file_size: number;
    date_created: string;
    engine_version?: string[];
    is_matched: boolean;
    folder_name?: string;
};

export interface LogEntry {
  timestamp: string;
  level: 'info' | 'warn' | 'error' | 'debug';
  message: string;
}

export interface ScanLocation {
  id: number;
  name: string;
  path: string;
  is_active: boolean;
  scan_recursive?: boolean;
  file_extensions?: string;
  last_scanned?: string;
  asset_count?: number;
  description?: string;
  created_date?: string;
}

export interface ScanProgress {
  locationId?: number;
  total_items: number;
  processed_items: number;
  current_path: string;
  error?: string;
  status?: 'Scanning' | 'Completed' | 'Error' | 'Cancelled';
  error_count?: number;
  completed_successfully?: boolean;
}

export interface AssetStats {
  total_assets: number;
  total_size_gb: number;
  unmatched_assets: number;
  supported_engine_versions: Record<string, number>;
  top_categories: Record<string, number>;
}

export interface AssetFilter {
  searchTerm?: string;
  tags?: string[];
  categories?: string[];
  engineVersions?: string[];
  rating?: number;
  isFavorite?: boolean;
}

export interface GalleryImageMedia {
  type: 'image';
  src: string;
  alt: string;
}

export interface GalleryVideoMedia {
  type: 'video';
  src: string;
post: string;
}

export type GalleryItem = GalleryImageMedia | GalleryVideoMedia; 