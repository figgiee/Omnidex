<template>
  <div class="asset-detail">
    <div v-if="asset" class="detail-content">
      <!-- Hero Section -->
      <div class="asset-hero">
        <div class="asset-identity">
          <div class="asset-icon">{{ getAssetIcon(asset.asset_type) }}</div>
          <div class="asset-title">
            <h1>{{ asset.name }}</h1>
            <div class="asset-subtitle">
              <span class="asset-type-badge">{{ formatAssetType(asset.asset_type) }}</span>
              <span class="asset-size">{{ formatFileSize(asset.file_size) }}</span>
            </div>
          </div>
        </div>
        <div class="hero-actions">
          <button 
            @click="toggleFavorite"
            class="btn btn-secondary"
            :class="{ 'btn-favorite': asset.is_favorite }"
          >
            ⭐ {{ asset.is_favorite ? 'Remove from Favorites' : 'Add to Favorites' }}
          </button>
          <button @click="openFileLocation" class="btn btn-primary">
            📂 Open Location
          </button>
          <button @click="copyPath" class="btn btn-secondary">
            📋 Copy Path
          </button>
        </div>
      </div>

      <!-- orbital Marketplace Information (Main Content) -->
      <div v-if="hasOrbitalData" class="orbital-marketplace-main">
        <div class="orbital-header">
          <h2>🛒 Marketplace Information</h2>
          <span v-if="asset.orbital_match_confidence" class="match-badge" :class="getConfidenceClass(asset.orbital_match_confidence)">
            {{ formatConfidence(asset.orbital_match_confidence) }} Match
          </span>
        </div>
        
        <div class="orbital-content-main">
          <!-- Marketplace Overview -->
          <div class="orbital-overview-main">
            <div class="orbital-meta-row">
              <div v-if="asset.orbital_seller_name" class="orbital-item-main">
                <span class="orbital-label-main">Seller</span>
                <span class="orbital-value-main orbital-seller-main">{{ decodeHtmlEntities(asset.orbital_seller_name) }}</span>
              </div>
              <div v-if="asset.orbital_price !== null && asset.orbital_price !== undefined" class="orbital-item-main">
                <span class="orbital-label-main">Price</span>
                <span class="orbital-value-main orbital-price-main">{{ formatPrice(asset.orbital_price) }}</span>
              </div>
            </div>
          </div>

          <!-- Description -->
          <div v-if="asset.orbital_description" class="orbital-description-main">
            <h4>📄 Description</h4>
            <p class="orbital-description-text-main" v-html="decodeHtmlEntities(asset.orbital_description)"></p>
          </div>

          <!-- Additional Details Grid -->
          <div class="orbital-details-grid">
            <!-- Rating Information -->
            <div v-if="asset.orbital_rating_average || asset.orbital_rating_count" class="orbital-detail-card">
              <h4>⭐ Rating</h4>
              <div class="rating-info">
                <StarRating 
                  :rating="asset.orbital_rating_average || 0"
                  :review-count="asset.orbital_rating_count || 0"
                  :show-text="true"
                  size="medium"
                />
              </div>
            </div>

            <!-- Gallery Images -->
            <div v-if="orbitalGalleryImages.length > 0" class="orbital-detail-card">
              <h4>🖼️ Gallery</h4>
              <div class="gallery-grid">
                <div 
                  v-for="(image, index) in orbitalGalleryImages.slice(0, 6)" 
                  :key="index" 
                  class="gallery-item"
                  @click="openImageModal(image, index)"
                >
                  <img 
                    :src="image.url" 
                    :alt="image.alt_text || `Gallery image ${index + 1}`"
                    class="gallery-image"
                    loading="lazy"
                    @error="onGalleryImageError($event)"
                  />
                  <div v-if="orbitalGalleryImages.length > 6 && index === 5" class="gallery-overlay">
                    +{{ orbitalGalleryImages.length - 6 }} more
                  </div>
                </div>
              </div>
            </div>

            <!-- Technical Details -->
            <div v-if="asset.orbital_technical_details" class="orbital-detail-card">
              <h4>⚙️ Technical Details</h4>
              <p class="orbital-technical-text-main" v-html="decodeHtmlEntities(asset.orbital_technical_details)"></p>
            </div>

            <!-- Compatible Applications -->
            <div v-if="orbitalCompatibleApps.length > 0" class="orbital-detail-card">
              <h4>🔧 Compatible With</h4>
              <div class="compatible-apps">
                <span v-for="app in orbitalCompatibleApps" :key="app" class="compatible-app">
                  {{ app }}
                </span>
              </div>
            </div>

            <!-- Categories and Tags -->
            <div v-if="orbitalCategories.length > 0 || orbitalTags.length > 0" class="orbital-detail-card">
              <h4>🏷️ Categories & Tags</h4>
              <div v-if="orbitalCategories.length > 0" class="categories-section">
                <span class="section-label">Categories:</span>
                <div class="categories-container">
                  <span v-for="category in orbitalCategories" :key="category" class="category-tag">
                    {{ category }}
                  </span>
                </div>
              </div>
              <div v-if="orbitalTags.length > 0" class="tags-section">
                <span class="section-label">Tags:</span>
                <div class="orbital-tags-container">
                  <span v-for="tag in orbitalTags" :key="tag" class="orbital-tag">
                    {{ tag }}
                  </span>
                </div>
              </div>
            </div>

            <!-- Release Information -->
            <div v-if="asset.orbital_release_date || asset.orbital_last_modified" class="orbital-detail-card">
              <h4>📅 Release Info</h4>
              <div class="orbital-date-items">
                <div v-if="asset.orbital_release_date" class="orbital-date-item">
                  <span class="orbital-date-label">Released</span>
                  <span class="orbital-date-value">{{ formatFabDate(asset.orbital_release_date) }}</span>
                </div>
                <div v-if="asset.orbital_last_modified" class="orbital-date-item">
                  <span class="orbital-date-label">Updated</span>
                  <span class="orbital-date-value">{{ formatFabDate(asset.orbital_last_modified) }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Marketplace Actions -->
          <div v-if="asset.orbital_source_url" class="orbital-actions">
            <button @click="openOrbitalUrl" class="btn btn-primary orbital-action-btn">
              🛒 View on Marketplace
            </button>
            <button @click="copyOrbitalUrl" class="btn btn-secondary orbital-action-btn">
              📋 Copy Marketplace URL
            </button>
          </div>

          <!-- Last Updated -->
          <div v-if="asset.orbital_last_checked_timestamp" class="orbital-last-updated">
            <span class="update-label">Data last updated: </span>
            <span class="update-time">{{ formatFabDate(asset.orbital_last_checked_timestamp) }}</span>
          </div>
        </div>
      </div>

      <!-- Local Asset Information (Secondary) -->
      <div class="local-asset-section">
        <div class="collapsible-header" @click="toggleLocalInfo">
          <h3>📁 Local Asset Details</h3>
          <span class="collapse-icon" :class="{ expanded: showLocalInfo }">▼</span>
        </div>
        
        <div v-if="showLocalInfo" class="local-content">
          <!-- Key Information Cards -->
          <div class="info-cards">
            <!-- Quick Stats -->
            <div class="info-card stats-card">
              <h3>📊 Quick Stats</h3>
              <div class="stats-grid">
                <div class="stat-item">
                  <span class="stat-label">Size</span>
                  <span class="stat-value">{{ formatFileSize(asset.file_size) }}</span>
                </div>
                <div class="stat-item">
                  <span class="stat-label">Type</span>
                  <span class="stat-value">{{ formatAssetType(asset.asset_type) }}</span>
                </div>
                <div class="stat-item">
                  <span class="stat-label">Modified</span>
                  <span class="stat-value">{{ formatRelativeDate(asset.modified_date) }}</span>
                </div>
                <div class="stat-item">
                  <span class="stat-label">Status</span>
                  <span class="stat-value" :class="{ 'favorite': asset.is_favorite }">
                    {{ asset.is_favorite ? '⭐ Favorited' : '📄 Standard' }}
                  </span>
                </div>
              </div>
            </div>

            <!-- Location Info -->
            <div class="info-card location-card">
              <h3>📍 Location</h3>
              <div class="location-info">
                <div class="location-item">
                  <span class="location-label">Directory</span>
                  <div class="location-path">
                    <span class="path-text">{{ getDirectoryPath(asset.file_path) }}</span>
                    <button @click="copyDirectoryPath" class="copy-btn" title="Copy directory path">📋</button>
                  </div>
                </div>
                <div class="location-item">
                  <span class="location-label">Full Path</span>
                  <div class="location-path">
                    <span class="path-text monospace">{{ asset.file_path }}</span>
                    <button @click="copyPath" class="copy-btn" title="Copy full path">📋</button>
                  </div>
                </div>
                <div v-if="scanLocation" class="location-item">
                  <span class="location-label">Scan Source</span>
                  <span class="scan-source">{{ scanLocation.name }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Timeline -->
          <div class="detail-group">
            <h3>🕐 Timeline</h3>
            <div class="timeline-items">
              <div class="timeline-item">
                <span class="timeline-label">Created</span>
                <span class="timeline-value">{{ formatFullDate(asset.created_date) }}</span>
              </div>
              <div class="timeline-item">
                <span class="timeline-label">Last Modified</span>
                <span class="timeline-value">{{ formatFullDate(asset.modified_date) }}</span>
              </div>
              <div v-if="asset.last_accessed" class="timeline-item">
                <span class="timeline-label">Last Accessed</span>
                <span class="timeline-value">{{ formatFullDate(asset.last_accessed) }}</span>
              </div>
            </div>
          </div>

          <!-- Tags -->
          <div v-if="asset.tags" class="detail-group">
            <h3>🏷️ Local Tags</h3>
            <div class="tags-container">
              <span 
                v-for="tag in parseTags(asset.tags)" 
                :key="tag"
                class="tag"
              >
                {{ tag }}
              </span>
            </div>
          </div>

          <!-- Technical Details -->
          <div class="detail-group">
            <div class="collapsible-header" @click="toggleTechnicalDetails">
              <h3>🔧 Technical Details</h3>
              <span class="collapse-icon" :class="{ expanded: showTechnicalDetails }">▼</span>
            </div>
            <div v-if="showTechnicalDetails" class="technical-details">
              <div v-if="asset.file_hash" class="tech-item">
                <span class="tech-label">File Hash</span>
                <div class="tech-value-row">
                  <code class="tech-value">{{ asset.file_hash }}</code>
                  <button @click="copyHash" class="copy-btn" title="Copy hash">📋</button>
                </div>
              </div>
              <div class="tech-item">
                <span class="tech-label">Scan Location ID</span>
                <span class="tech-value">{{ asset.scan_location_id }}</span>
              </div>
              <div class="tech-item">
                <span class="tech-label">Asset ID</span>
                <span class="tech-value">{{ asset.id }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Description Section -->
      <div class="description-section">
        <h3>📝 Local Description</h3>
        <div v-if="!editingDescription" class="description-display">
          <p v-if="asset.description" class="description-text">{{ asset.description }}</p>
          <p v-else class="no-description">No local description available.</p>
          <button @click="startEditingDescription" class="btn btn-secondary btn-sm">
            ✏️ {{ asset.description ? 'Edit' : 'Add' }} Description
          </button>
        </div>
        <div v-else class="description-edit">
          <textarea 
            v-model="editedDescription"
            class="description-textarea"
            placeholder="Enter asset description..."
            rows="4"
          ></textarea>
          <div class="edit-actions">
            <button @click="saveDescription" class="btn btn-primary btn-sm">
              💾 Save
            </button>
            <button @click="cancelEditingDescription" class="btn btn-secondary btn-sm">
              ❌ Cancel
            </button>
          </div>
        </div>
      </div>

      <!-- Additional Metadata (Structured) -->
      <div v-if="asset.metadata && parsedMetadata" class="metadata-section">
        <div class="collapsible-header" @click="toggleMetadata">
          <h3>📋 Additional Metadata</h3>
          <span class="collapse-icon" :class="{ expanded: showMetadata }">▼</span>
        </div>
        <div v-if="showMetadata" class="structured-metadata">
          <div v-for="(value, key) in parsedMetadata" :key="key" class="metadata-item">
            <span class="metadata-key">{{ formatMetadataKey(key) }}</span>
            <span class="metadata-value">{{ formatMetadataValue(value) }}</span>
          </div>
        </div>
      </div>

      <!-- Thumbnail Section (if available) -->
      <div v-if="asset.thumbnail_path" class="thumbnail-section">
        <h3>🖼️ Preview</h3>
        <div class="thumbnail-container">
          <img :src="asset.thumbnail_path" :alt="asset.name" class="asset-thumbnail" />
        </div>
      </div>
    </div>

    <div v-else class="no-asset">
      <div class="empty-state">
        <div class="empty-icon">📄</div>
        <p>No asset selected</p>
      </div>
    </div>

    <!-- Image Modal -->
    <div v-if="showImageModal && selectedGalleryImage" class="image-modal" @click="closeImageModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h3>Gallery Image</h3>
          <button @click="closeImageModal" class="close-btn">✕</button>
        </div>
        <div class="modal-body">
          <img 
            :src="selectedGalleryImage.url" 
            :alt="selectedGalleryImage.alt_text || 'Gallery image'"
            class="modal-image"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useAssetStore } from '@/stores/assetStore'
import { useScanLocationStore } from '@/stores/scanLocationStore'
import { invoke } from '@tauri-apps/api/core'
import type { Asset } from '@/types'
import StarRating from '@/components/StarRating.vue'
import { useToast } from 'vue-toastification'

interface Props {
  assetId?: number
  asset?: Asset
}

const props = defineProps<Props>()

const assetStore = useAssetStore()
const scanLocationStore = useScanLocationStore()
const toast = useToast()

// State
const editingDescription = ref(false)
const editedDescription = ref('')
const showTechnicalDetails = ref(false)
const showMetadata = ref(false)
const showLocalInfo = ref(false)
const selectedGalleryImage = ref<{ url: string; alt_text?: string } | null>(null)
const showImageModal = ref(false)

// Computed
const asset = computed(() => {
  if (props.asset) return props.asset
  if (props.assetId) {
    // Trigger fetch if needed
    if (!assetStore.currentAsset || assetStore.currentAsset.id !== props.assetId) {
      assetStore.fetchAssetById(props.assetId)
    }
    return assetStore.currentAsset
  }
  return null
})

const scanLocation = computed(() => {
  if (!asset.value) return null
  return scanLocationStore.getLocationById(asset.value.scan_location_id)
})

const parsedMetadata = computed(() => {
  if (!asset.value?.metadata) return null
  try {
    const parsed = JSON.parse(asset.value.metadata) as Record<string, unknown>
    // Filter out complex nested objects for better display
    const filtered: Record<string, unknown> = {}
    for (const [key, value] of Object.entries(parsed)) {
      if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        filtered[key] = value
      }
    }
    return Object.keys(filtered).length > 0 ? filtered : null
  } catch {
    return null
  }
})

const hasOrbitalData = computed(() => {
  if (!asset.value) return false
  return !!(
    asset.value.orbital_title || 
    asset.value.orbital_description || 
    asset.value.orbital_seller_name || 
    asset.value.orbital_price !== null || 
    asset.value.orbital_source_url ||
    asset.value.matched_orbital_product_slug
  )
})

const orbitalCategories = computed(() => {
  if (!asset.value?.orbital_categories) return []
  try {
    const parsed = JSON.parse(asset.value.orbital_categories)
    return Array.isArray(parsed) ? parsed : []
  } catch {
    console.log('Failed to parse orbital_categories as JSON, attempting split. Value:', asset.value?.orbital_categories, '| Type:', typeof asset.value?.orbital_categories);
    if (typeof asset.value?.orbital_categories === 'string') {
      return asset.value.orbital_categories.split(',').map((cat: string) => cat.trim())
    }
    console.error('orbital_categories is not a string:', asset.value?.orbital_categories);
    return [];
  }
})

const orbitalTags = computed(() => {
  if (!asset.value?.orbital_tags) return []
  try {
    const parsed = JSON.parse(asset.value.orbital_tags)
    return Array.isArray(parsed) ? parsed : []
  } catch {
    console.log('Failed to parse orbital_tags as JSON, attempting split. Value:', asset.value?.orbital_tags, '| Type:', typeof asset.value?.orbital_tags);
    if (typeof asset.value?.orbital_tags === 'string') {
      return asset.value.orbital_tags.split(',').map((tag: string) => tag.trim())
    }
    console.error('orbital_tags is not a string:', asset.value?.orbital_tags);
    return [];
  }
})

const orbitalCompatibleApps = computed(() => {
  if (!asset.value?.orbital_compatible_apps) return []
  try {
    const parsed = JSON.parse(asset.value.orbital_compatible_apps)
    return Array.isArray(parsed) ? parsed : []
  } catch {
    console.log('Failed to parse orbital_compatible_apps as JSON, attempting split. Value:', asset.value?.orbital_compatible_apps, '| Type:', typeof asset.value?.orbital_compatible_apps);
    if (typeof asset.value?.orbital_compatible_apps === 'string') {
      return asset.value.orbital_compatible_apps.split(',').map((app: string) => app.trim())
    }
    console.error('orbital_compatible_apps is not a string:', asset.value?.orbital_compatible_apps);
    return [];
  }
})

const orbitalGalleryImages = computed(() => {
  if (!asset.value?.orbital_gallery_images) return []
  try {
    const parsed = JSON.parse(asset.value.orbital_gallery_images)
    return Array.isArray(parsed) ? parsed : []
  } catch {
    return []
  }
})

// Methods
function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${Number.parseFloat((bytes / k ** i).toFixed(2))} ${sizes[i]}`
}

function formatAssetType(assetType: string): string {
  // Convert snake_case to Title Case
  return assetType.replace(/_/g, ' ')
    .replace(/\b\w/g, l => l.toUpperCase())
}

function formatFullDate(dateString: string): string {
  const date = new Date(Number.parseInt(dateString, 10) * 1000)
  return `${date.toLocaleDateString()} at ${date.toLocaleTimeString()}`
}

function formatRelativeDate(dateString: string): string {
  const date = new Date(Number.parseInt(dateString, 10) * 1000)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))
  
  if (diffDays === 0) return 'Today'
  if (diffDays === 1) return 'Yesterday'
  if (diffDays < 7) return `${diffDays} days ago`
  if (diffDays < 30) return `${Math.floor(diffDays / 7)} weeks ago`
  if (diffDays < 365) return `${Math.floor(diffDays / 30)} months ago`
  return `${Math.floor(diffDays / 365)} years ago`
}

function getDirectoryPath(fullPath: string): string {
  const lastSlash = fullPath.lastIndexOf('/')
  const lastBackslash = fullPath.lastIndexOf('\\')
  const lastSeparator = Math.max(lastSlash, lastBackslash)
  return lastSeparator > 0 ? fullPath.substring(0, lastSeparator) : fullPath
}

function getAssetIcon(assetType: string): string {
  const icons: Record<string, string> = {
    'Blueprint': '🔧',
    'Level': '🗺️',
    'Texture': '🖼️',
    'StaticMesh': '🧊',
    'Sound': '🔊',
    'Video': '🎬',
    'Material': '🎨',
    'Animation': '🎭',
    'Document': '📄',
    'Project': '📦',
    'Plugin': '🔌',
    'FAB_Asset': '🛒',
    'Unknown': '❓'
  }
  return icons[assetType] || icons.Unknown
}

const parseTags = (tags: string | string[]): string[] => {
  if (Array.isArray(tags)) {
    return tags;
  }
  if (typeof tags === 'string') {
    return tags.split(',').map(tag => tag.trim()).filter(t => t.length > 0);
  }
  return [];
};

function formatMetadataKey(key: string): string {
  return key.replace(/_/g, ' ')
    .replace(/\b\w/g, l => l.toUpperCase())
}

function formatMetadataValue(value: unknown): string {
  if (Array.isArray(value)) {
    return value.join(', ')
  }
  if (typeof value === 'boolean') {
    return value ? 'Yes' : 'No'
  }
  if (typeof value === 'number') {
    return value.toLocaleString()
  }
  // Decode HTML entities
  return decodeHtmlEntities(String(value))
}

function decodeHtmlEntities(text: string): string {
  // Create a temporary element to decode HTML entities
  const temp = document.createElement('div')
  temp.innerHTML = text
  return temp.textContent || temp.innerText || text
}

function formatPrice(price: number): string {
  return `$${price.toLocaleString()}`
}

function getConfidenceClass(confidence: number): string {
  if (confidence < 0.33) return 'low-confidence'
  if (confidence < 0.66) return 'medium-confidence'
  return 'high-confidence'
}

function formatConfidence(confidence: number): string {
  return `${(confidence * 100).toFixed(0)}%`
}

function formatFabDate(dateString: string): string {
  const date = new Date(Number.parseInt(dateString, 10) * 1000)
  return `${date.toLocaleDateString()} at ${date.toLocaleTimeString()}`
}

function toggleTechnicalDetails() {
  showTechnicalDetails.value = !showTechnicalDetails.value
}

function toggleMetadata() {
  showMetadata.value = !showMetadata.value
}

function toggleLocalInfo() {
  showLocalInfo.value = !showLocalInfo.value
}

async function toggleFavorite() {
  if (asset.value) {
    await assetStore.toggleFavorite(asset.value.id, !asset.value.is_favorite)
  }
}

async function openFileLocation() {
  if (asset.value) {
    try {
      await invoke('open_file_location', { filePath: asset.value.file_path })
    } catch (error) {
      console.error('Failed to open file location:', error)
      toast.error('Could not open file location. The file may have been moved or deleted.')
    }
  }
}

async function copyPath() {
  if (!asset.value) return
  
  try {
    await navigator.clipboard.writeText(asset.value.file_path)
    console.log('Path copied to clipboard')
  } catch (error) {
    console.error('Failed to copy path:', error)
  }
}

async function copyDirectoryPath() {
  if (!asset.value) return
  
  try {
    const dirPath = getDirectoryPath(asset.value.file_path)
    await navigator.clipboard.writeText(dirPath)
    console.log('Directory path copied to clipboard')
  } catch (error) {
    console.error('Failed to copy directory path:', error)
  }
}

async function copyHash() {
  if (!asset.value?.file_hash) return
  
  try {
    await navigator.clipboard.writeText(asset.value.file_hash)
    console.log('Hash copied to clipboard')
  } catch (error) {
    console.error('Failed to copy hash:', error)
  }
}

async function openOrbitalUrl() {
  if (!asset.value?.orbital_source_url) return
  try {
    await invoke('open_url', { url: asset.value.orbital_source_url })
  } catch (e) {
    console.error('Failed to open URL', e)
    toast.error('Could not open the marketplace URL.')
  }
}

async function copyOrbitalUrl() {
  if (!asset.value?.orbital_source_url) return
  try {
    await navigator.clipboard.writeText(asset.value.orbital_source_url)
    toast.success('Marketplace URL copied to clipboard!')
  } catch (e) {
    console.error('Failed to copy URL', e)
    toast.error('Could not copy the marketplace URL.')
  }
}

function startEditingDescription() {
  editedDescription.value = asset.value?.description || ''
  editingDescription.value = true
}

function cancelEditingDescription() {
  editingDescription.value = false
  editedDescription.value = ''
}

async function saveDescription() {
  if (!asset.value?.id) return
  
  try {
    await assetStore.updateAssetDescription(asset.value.id, editedDescription.value)
    editingDescription.value = false
  } catch (error) {
    console.error('Failed to save description:', error)
  }
}

function openImageModal(image: { url: string; alt_text?: string }, index: number) {
  selectedGalleryImage.value = image
  showImageModal.value = true
}

function closeImageModal() {
  showImageModal.value = false
  selectedGalleryImage.value = null
}

function onGalleryImageError(event: Event) {
  const img = event.target as HTMLImageElement
  img.style.display = 'none'
}

// Watchers
watch(() => props.assetId, () => {
  editingDescription.value = false
  showTechnicalDetails.value = false
  showMetadata.value = false
  showLocalInfo.value = false
})
</script>

<style scoped>
.asset-detail {
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.1), rgba(255, 255, 255, 0.05));
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 1rem;
  padding: 0;
  max-height: 80vh;
  overflow-y: auto;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
}

.detail-content {
  padding: 2rem;
}

/* Hero Section */
.asset-hero {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 2rem;
  padding-bottom: 2rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  gap: 2rem;
}

.asset-identity {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  flex: 1;
}

.asset-icon {
  font-size: 3rem;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 1rem;
  padding: 1rem;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.asset-title h1 {
  margin: 0 0 0.5rem 0;
  color: #fff;
  font-size: 1.75rem;
  font-weight: 600;
  line-height: 1.2;
}

.asset-subtitle {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.asset-type-badge {
  background: linear-gradient(135deg, #4ade80, #22c55e);
  color: #000;
  padding: 0.25rem 0.75rem;
  border-radius: 1rem;
  font-size: 0.875rem;
  font-weight: 500;
}

.asset-size {
  color: rgba(255, 255, 255, 0.7);
  font-weight: 500;
}

.hero-actions {
  display: flex;
  gap: 0.75rem;
  flex-wrap: wrap;
}

/* Info Cards */
.info-cards {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.info-card {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 1rem;
  padding: 1.5rem;
}

.info-card h3 {
  margin: 0 0 1rem 0;
  color: #4ade80;
  font-size: 1rem;
  font-weight: 600;
}

.stats-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.stat-item {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.stat-label {
  color: rgba(255, 255, 255, 0.6);
  font-size: 0.875rem;
}

.stat-value {
  color: #fff;
  font-weight: 500;
}

.stat-value.favorite {
  color: #fbbf24;
}

.location-info {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.location-item {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.location-label {
  color: rgba(255, 255, 255, 0.6);
  font-size: 0.875rem;
}

.location-path {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.path-text {
  flex: 1;
  color: #fff;
  font-size: 0.875rem;
  word-break: break-all;
}

.monospace {
  font-family: 'Courier New', monospace;
}

.scan-source {
  color: #4ade80;
  font-weight: 500;
}

.copy-btn {
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 0.5rem;
  padding: 0.25rem 0.5rem;
  color: rgba(255, 255, 255, 0.7);
  cursor: pointer;
  font-size: 0.75rem;
  transition: all 0.2s ease;
}

.copy-btn:hover {
  background: rgba(255, 255, 255, 0.2);
  color: #fff;
}

/* Details Section */
.details-section {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.detail-group {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 1rem;
  padding: 1.5rem;
}

.detail-group h3 {
  margin: 0 0 1rem 0;
  color: #4ade80;
  font-size: 1rem;
  font-weight: 600;
}

.collapsible-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  cursor: pointer;
  margin: -1.5rem -1.5rem 0 -1.5rem;
  padding: 1.5rem;
  border-radius: 1rem 1rem 0 0;
  transition: background-color 0.2s ease;
}

.collapsible-header:hover {
  background: rgba(255, 255, 255, 0.05);
}

.collapsible-header h3 {
  margin: 0;
}

.collapse-icon {
  color: rgba(255, 255, 255, 0.6);
  transition: transform 0.2s ease;
}

.collapse-icon.expanded {
  transform: rotate(180deg);
}

.timeline-items {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.timeline-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.timeline-label {
  color: rgba(255, 255, 255, 0.6);
  font-size: 0.875rem;
}

.timeline-value {
  color: #fff;
  font-weight: 500;
}

.tags-container {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.tag {
  background: rgba(74, 222, 128, 0.2);
  color: #4ade80;
  padding: 0.25rem 0.75rem;
  border-radius: 1rem;
  font-size: 0.875rem;
  border: 1px solid rgba(74, 222, 128, 0.3);
}

.technical-details {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin-top: 1rem;
}

.tech-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
}

.tech-label {
  color: rgba(255, 255, 255, 0.6);
  font-size: 0.875rem;
  min-width: 120px;
}

.tech-value {
  color: #fff;
  font-family: 'Courier New', monospace;
  font-size: 0.875rem;
  background: rgba(0, 0, 0, 0.3);
  padding: 0.25rem 0.5rem;
  border-radius: 0.25rem;
}

.tech-value-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex: 1;
  justify-content: flex-end;
}

/* Metadata Section */
.metadata-section {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 1rem;
  padding: 0;
  margin-bottom: 2rem;
}

.structured-metadata {
  padding: 1.5rem;
  padding-top: 0;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.metadata-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.metadata-item:last-child {
  border-bottom: none;
}

.metadata-key {
  color: rgba(255, 255, 255, 0.6);
  font-size: 0.875rem;
  min-width: 140px;
}

.metadata-value {
  color: #fff;
  font-weight: 500;
  text-align: right;
}

/* Description Section */
.description-section {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 1rem;
  padding: 1.5rem;
  margin-bottom: 2rem;
}

.description-section h3 {
  margin: 0 0 1rem 0;
  color: #4ade80;
  font-size: 1rem;
  font-weight: 600;
}

.description-display {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.description-text {
  margin: 0;
  color: #fff;
  line-height: 1.6;
}

.no-description {
  margin: 0;
  color: rgba(255, 255, 255, 0.5);
  font-style: italic;
}

.description-edit {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.description-textarea {
  width: 100%;
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 0.5rem;
  padding: 1rem;
  color: #fff;
  font-family: inherit;
  resize: vertical;
  min-height: 100px;
}

.description-textarea::placeholder {
  color: rgba(255, 255, 255, 0.5);
}

.edit-actions {
  display: flex;
  gap: 0.5rem;
}

/* Thumbnail Section */
.thumbnail-section {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 1rem;
  padding: 1.5rem;
  margin-bottom: 2rem;
}

.thumbnail-section h3 {
  margin: 0 0 1rem 0;
  color: #4ade80;
  font-size: 1rem;
  font-weight: 600;
}

.thumbnail-container {
  display: flex;
  justify-content: center;
}

.asset-thumbnail {
  max-width: 100%;
  max-height: 400px;
  border-radius: 0.5rem;
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
}

/* Empty State */
.no-asset {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 300px;
  color: rgba(255, 255, 255, 0.5);
}

.empty-state {
  text-align: center;
}

.empty-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
  opacity: 0.5;
}

/* Buttons */
.btn {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 0.5rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.875rem;
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
}

.btn-sm {
  padding: 0.375rem 0.75rem;
  font-size: 0.8rem;
}

.btn-primary {
  background: linear-gradient(135deg, #3b82f6, #1d4ed8);
  color: white;
}

.btn-primary:hover {
  background: linear-gradient(135deg, #1d4ed8, #1e40af);
  transform: translateY(-1px);
}

.btn-secondary {
  background: rgba(255, 255, 255, 0.1);
  color: rgba(255, 255, 255, 0.9);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.btn-secondary:hover {
  background: rgba(255, 255, 255, 0.2);
  color: white;
  transform: translateY(-1px);
}

.btn-favorite {
  background: linear-gradient(135deg, #fbbf24, #f59e0b);
  color: #000;
}

.btn-favorite:hover {
  background: linear-gradient(135deg, #f59e0b, #d97706);
}

/* orbital Marketplace Section */
.orbital-marketplace-main {
  background-color: var(--bg-surface-softer);
  padding: var(--spacing-lg);
  border-radius: var(--border-radius-large);
  margin-bottom: var(--spacing-xl);
  border: 1px solid var(--border-color-soft);
}

.orbital-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-md);
}

.orbital-header h2 {
  margin: 0;
  color: #4ade80;
  font-size: 1.5rem;
  font-weight: 700;
}

.match-badge {
  padding: 0.375rem 1rem;
  border-radius: 1rem;
  font-size: 0.875rem;
  font-weight: 600;
}

.orbital-content-main {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.orbital-overview-main {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.orbital-item-main {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.orbital-label-main {
  color: rgba(255, 255, 255, 0.7);
  font-size: 0.875rem;
  font-weight: 500;
}

.orbital-value-main {
  color: #fff;
  font-weight: 600;
  font-size: 1rem;
}

.orbital-title-main {
  margin: 0;
  color: #4ade80;
  font-size: 1.75rem;
  font-weight: 700;
  line-height: 1.2;
}

.orbital-meta-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
}

.orbital-seller-main {
  color: #fbbf24;
  font-size: 1.1rem;
}

.orbital-price-main {
  color: #10b981;
  font-size: 1.25rem;
  font-weight: 700;
}

.orbital-description-main {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 0.75rem;
  padding: 1.5rem;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.orbital-description-main h4 {
  margin: 0 0 1rem 0;
  color: #4ade80;
  font-size: 1.1rem;
  font-weight: 600;
}

.orbital-description-text-main,
.orbital-technical-text-main {
  font-size: 0.95rem;
  color: #cccccc;
  line-height: 1.6em;
  white-space: pre-wrap;
  word-wrap: break-word;
  padding: 10px;
  background-color: #2c2c2c;
  border-radius: 6px;
  border: 1px solid #444;
}

.orbital-details-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1.5rem;
}

.orbital-detail-card {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 0.75rem;
  padding: 1.5rem;
}

.orbital-detail-card h4 {
  margin: 0 0 1rem 0;
  color: #4ade80;
  font-size: 1rem;
  font-weight: 600;
}

.orbital-technical-text-main {
  margin: 0;
  color: rgba(255, 255, 255, 0.9);
  line-height: 1.6;
  word-wrap: break-word;
  overflow-wrap: break-word;
  white-space: pre-wrap;
}

.categories-section,
.tags-section {
  margin-bottom: 1rem;
}

.categories-section:last-child,
.tags-section:last-child {
  margin-bottom: 0;
}

.section-label {
  display: block;
  color: rgba(255, 255, 255, 0.7);
  font-size: 0.875rem;
  margin-bottom: 0.5rem;
  font-weight: 500;
}

.orbital-actions {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}

.orbital-action-btn {
  flex: 1;
  min-width: 200px;
  padding: 0.75rem 1.5rem;
  font-size: 1rem;
  font-weight: 600;
}

.orbital-last-updated {
  text-align: center;
  padding: 1rem;
  background: rgba(255, 255, 255, 0.03);
  border-radius: 0.5rem;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.update-label {
  color: rgba(255, 255, 255, 0.6);
  font-size: 0.875rem;
}

.update-time {
  color: #4ade80;
  font-weight: 500;
}

/* Local Asset Section */
.local-asset-section {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 1rem;
  padding: 0;
  margin-bottom: 2rem;
}

.local-content {
  padding: 1.5rem;
  padding-top: 0;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

/* Confidence Classes */
.high-confidence {
  background: linear-gradient(135deg, #10b981, #059669);
  color: white;
}

.medium-confidence {
  background: linear-gradient(135deg, #fbbf24, #f59e0b);
  color: #000;
}

.low-confidence {
  background: linear-gradient(135deg, #ef4444, #dc2626);
  color: white;
}

/* Rating Information */
.rating-info {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.rating-average {
  color: #fbbf24;
  font-weight: 600;
}

.rating-count {
  color: rgba(255, 255, 255, 0.7);
}

/* Tags and Categories */
.categories-container,
.orbital-tags-container,
.compatible-apps {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.category-tag,
.orbital-tag,
.compatible-app {
  background: rgba(74, 222, 128, 0.2);
  color: #4ade80;
  padding: 0.25rem 0.75rem;
  border-radius: 1rem;
  font-size: 0.875rem;
  border: 1px solid rgba(74, 222, 128, 0.3);
}

/* Gallery Styles */
.gallery-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 0.75rem;
}

.gallery-item {
  position: relative;
  aspect-ratio: 1;
  border-radius: 0.5rem;
  overflow: hidden;
  cursor: pointer;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.gallery-item:hover {
  transform: scale(1.05);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
}

.gallery-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: filter 0.2s ease;
}

.gallery-item:hover .gallery-image {
  filter: brightness(1.1);
}

.gallery-overlay {
  position: absolute;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-weight: 600;
  font-size: 0.875rem;
}

/* FAB Date Items */
.orbital-date-items {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.orbital-date-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.orbital-date-label {
  color: rgba(255, 255, 255, 0.6);
  font-size: 0.875rem;
}

.orbital-date-value {
  color: #fff;
  font-weight: 500;
}

/* Responsive Design */
@media (max-width: 768px) {
  .asset-hero {
    flex-direction: column;
    gap: 1rem;
  }
  
  .hero-actions {
    width: 100%;
    justify-content: center;
  }

  .orbital-marketplace-main {
    padding: 1.5rem;
  }

  .orbital-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }

  .orbital-meta-row {
    grid-template-columns: 1fr;
    gap: 1rem;
  }

  .orbital-details-grid {
    grid-template-columns: 1fr;
  }

  .orbital-actions {
    flex-direction: column;
  }

  .orbital-action-btn {
    min-width: 0;
  }
  
  .info-cards {
    grid-template-columns: 1fr;
  }
  
  .stats-grid {
    grid-template-columns: 1fr;
  }
  
  .timeline-item,
  .tech-item,
  .metadata-item,
  .orbital-date-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }
  
  .tech-value-row {
    justify-content: flex-start;
  }

  .gallery-grid {
    grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
  }
}

/* Image Modal */
.image-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.9);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(10px);
}

.image-modal .modal-content {
  background: var(--color-background-primary);
  border-radius: 1rem;
  max-width: 90vw;
  max-height: 90vh;
  overflow: hidden;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.image-modal .modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.image-modal .modal-header h3 {
  margin: 0;
  color: var(--color-text-primary);
}

.image-modal .close-btn {
  background: none;
  border: none;
  color: var(--color-text-secondary);
  font-size: 1.5rem;
  cursor: pointer;
  padding: 0.5rem;
  border-radius: 0.25rem;
  transition: background-color 0.2s ease;
}

.image-modal .close-btn:hover {
  background: rgba(255, 255, 255, 0.1);
}

.image-modal .modal-body {
  padding: 1rem;
  display: flex;
  justify-content: center;
  align-items: center;
}

.modal-image {
  max-width: 100%;
  max-height: 70vh;
  object-fit: contain;
  border-radius: 0.5rem;
}
</style> 