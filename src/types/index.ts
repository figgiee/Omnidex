// Asset types
export interface Asset {
  id: number
  name: string
  filePath: string
  assetType: string
  fileSize: number
  createdDate: string
  modifiedDate: string
  firstIndexedTimestamp?: string | null
  thumbnailPath?: string | null
  tags: string[]
  description?: string | null
  scanLocationId: number
  isFavorite: boolean
  lastAccessed?: string | null
  fileHash?: string | null
  metadata?: string | null

  fabTitle?: string | null
  fabDescription?: string | null
  fabTechnicalDetails?: string | null
  fabSellerName?: string | null
  fabPrice?: number | null
  fabReleaseDate?: string | null
  fabLastModified?: string | null
  fabRatingAverage?: number | null
  fabRatingCount?: number | null
  fabCategories?: string | null
  fabTags: string[]
  fabCompatibleApps?: string | null
  fabGalleryImages?: string | null
  fabThumbnailUrl?: string | null
  fabSourceUrl?: string | null
  fabRawJson?: string | null
  fabLastCheckedTimestamp?: string | null

  matchedFabProductSlug?: string | null
  matchConfidence?: number | null
  matchType?: string | null
  
  orbitalTitle?: string | null
  orbitalDescription?: string | null
  orbitalTechnicalDetails?: string | null
  orbitalSellerName?: string | null
  orbitalPrice?: number | null
  orbitalReleaseDate?: string | null
  orbitalLastModified?: string | null
  orbitalRatingAverage?: number | null
  orbitalRatingCount?: number | null
  orbitalCategories?: string | null
  orbitalSupportedVersions?: string | null
  orbitalGalleryImages?: string | null
  orbitalThumbnailUrl?: string | null
  orbitalSourceUrl?: string | null
  orbitalRawJson?: string | null
  orbitalLastCheckedTimestamp?: string | null

  matchedOrbitalProductSlug?: string | null
  orbitalMatchConfidence?: number | null
  orbitalMatchType?: string | null

  notes?: string | null
}

export interface AssetDetails {
  id: number
  name: string
  thumbnail_url?: string
  gallery_images?: GalleryItem[] // Can be URLs or media objects
  creator_name?: string
  rating?: number
  reviews_count?: number
  price?: number
  source_url?: string
  release_date?: string
  description?: string
  technical_description?: string
  tags: string[]
  scraped_tags: string[]
  file_path: string
  file_size: number
  date_created: string
  engine_version?: string[]
  is_matched: boolean
  folder_name?: string
}

export interface AssetFilter {
  assetType?: string
  nameSearch?: string
  isFavorite?: boolean
  scanLocationId?: number
}

export interface AssetStats {
  totalAssets: number
  totalSize: number
  assetTypeCounts: AssetTypeCount[]
  recentAssets: Asset[]
}

export interface AssetTypeCount {
  assetType: string
  count: number
}

// Scan Location types
export interface ScanLocation {
  id?: number
  name: string
  path: string
  isActive: boolean
  lastScan?: string
  scanRecursive: boolean
  fileExtensions?: string
  createdDate: string
  description?: string
}

export interface ScanProgress {
  locationId: number
  status: string
  currentPath: string
  processedItems: number
  totalItems: number
  completedSuccessfully: boolean
  error?: string | null
}

// App Settings types
export interface AppSetting {
  id?: number
  key: string
  value: string
  settingType: string
  description?: string
  createdDate: string
  modifiedDate: string
}

export interface AppSettings {
  theme: string
  autoScan: boolean
  scanInterval: number
  maxFileSize: number
  excludedExtensions: string[]
}

// UI State types
export interface UIState {
  loading: boolean
  error?: string
  selectedAssets: number[]
  viewMode: 'grid' | 'list'
  sortBy: 'name' | 'size' | 'date' | 'type'
  sortOrder: 'asc' | 'desc'
}

// API Response types
export interface ApiResponse<T> {
  data?: T
  error?: string
  success: boolean
}

// Adding helper interfaces for potentially parsed FAB data

export interface FabSellerInfo {
  name: string
  // id?: string
  // displayName?: string
}

export interface FabPriceInfo {
  amount: number
  currency: string
  discountPercentage?: number
}

export interface FabRatingInfo {
  average: number
  count: number
}

export interface FabImageInfo {
  url: string
  alt_text?: string
  width?: number
  height?: number
}

// Gallery Media Types for mixed content (images + videos)
export type GalleryMediaType = 'image' | 'youtube'

export interface GalleryImageMedia {
  type: 'image'
  url: string
  alt_text?: string
  width?: number
  height?: number
  source?: 'fab' | 'epicgames' | 'orbital' // fab.com, cdn1.epicgames.com, etc.
}

export interface GalleryVideoMedia {
  type: 'youtube'
  url: string // Full embed URL or video ID
  video_id?: string
  title?: string
  thumbnail_url?: string
}

export type GalleryMedia = GalleryImageMedia | GalleryVideoMedia

// Backward compatibility - can be string URL or media object
export type GalleryItem = string | GalleryMedia 