<template>
  <div class="inspector-panel">
    <!-- Multi-select state -->
    <div v-if="assetGridStore.selectedAssetIds.length > 1" class="multi-select-state">
      <h3 class="multi-select-title">{{ assetGridStore.selectedAssetIds.length }} items selected</h3>
      <div class="tags-section">
        <h4>Add Tags</h4>
        <p>Add tags to all selected items. Press Enter to apply.</p>
        <div class="tags-input-container">
          <input
            ref="tagInput"
            v-model="newTag"
            class="tag-input-field"
            placeholder="e.g. character, sci-fi, low-poly"
            @keydown="handleMultiTagInput"
          />
        </div>
      </div>
    </div>

    <!-- Single-select state (or no selection) -->
    <template v-else>
      <div v-if="!assetGridStore.selectedAssetId" class="prompt-state">
        <p>Select an asset to see details.</p>
      </div>
      <div v-else-if="inspectorStore.loading" class="loading-state">
        <SkeletonLoader height="12rem" width="100%" style="margin-bottom: 1.5rem;" />
        <SkeletonLoader height="2rem" width="60%" style="margin-bottom: 0.5rem;" />
        <SkeletonLoader height="1rem" width="40%" style="margin-bottom: 1rem;" />
        <SkeletonLoader height="1rem" width="80%" style="margin-bottom: 0.5rem;" />
        <SkeletonLoader height="1rem" width="70%" style="margin-bottom: 0.5rem;" />
        <SkeletonLoader height="1rem" width="75%" style="margin-bottom: 1rem;" />
        <SkeletonLoader height="2rem" width="30%" style="margin-bottom: 0.5rem;" />
        <SkeletonLoader height="4rem" width="100%" />
      </div>
      <div v-else-if="inspectorStore.error" class="error-state">
        <p>{{ inspectorStore.error }}</p>
        <AppButton @click="retryFetch" variant="secondary">Retry</AppButton>
      </div>
      <div v-else-if="inspectorStore.details && editableDetails" class="display-state">
        <!-- Matched Asset View -->
        <template v-if="inspectorStore.details.is_matched">
          <div class="preview-section">
            <swiper-container
              v-if="inspectorStore.details.gallery_images && inspectorStore.details.gallery_images.length"
              :key="`main-swiper-${inspectorStore.details.id}`"
              slides-per-view="1"
              space-between="0"
              navigation="true"
              :pagination="{clickable: true}"
              :keyboard="{enabled: true}"
              observer="true"
              observe-parents="true"
              @slidechange="onMainCarouselSlideChange"
              class="main-swiper"
            >
              <swiper-slide v-for="(media, index) in inspectorStore.details.gallery_images" :key="index" @click="openSlideshow(index)">
                <div class="slide-wrapper">
                  <!-- YouTube Video Embed -->
                  <iframe
                    v-if="getMediaType(media) === 'youtube'"
                    :src="getMediaUrl(media)"
                    title="YouTube video player"
                    class="preview-video"
                    allow="accelerometer; encrypted-media; gyroscope; picture-in-picture"
                    allowfullscreen
                    @load="onMediaLoad($event, index, 'video')"
                    @error="onMediaError($event, index, media, 'video')"
                  />
                  
                  <!-- Image -->
                  <img 
                    v-else
                    :src="getMediaUrl(media)" 
                    alt="Asset gallery image" 
                    class="preview-image"
                    @load="onMediaLoad($event, index, 'image')"
                    @error="onMediaError($event, index, media, 'image')"
                  />
                  
                  <!-- Expand icon overlay -->
                  <div class="expand-overlay">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="expand-icon">
                      <path d="M7 14H5v5h5v-2H7v-3zm-2-4h2V7h3V5H5v5zm12 7h-3v2h5v-5h-2v3zM14 5v2h3v3h2V5h-5z"/>
                    </svg>
                  </div>
                </div>
              </swiper-slide>
            </swiper-container>
            <img v-else-if="thumbnailSrc" :src="thumbnailSrc" alt="Asset preview" class="preview-image" />
             <div v-else class="preview-fallback">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M21 15a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2zM5 20h14a1 1 0 0 0 1-1v-2H4v2a1 1 0 0 0 1 1z"/></svg>
            </div>
          </div>

          <div class="marketplace-data-section">
            <div class="title-header">
              <input v-if="isEditMode" v-model="editableDetails.name" class="title-input" />
              <h3 v-else class="asset-title">{{ inspectorStore.details.name }}</h3>
              
              <div v-if="!isEditMode" class="edit-actions">
                <button @click="toggleEditMode" class="edit-button" title="Edit Asset Details">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M20.71,7.04C21.1,6.65 21.1,6 20.71,5.63L18.37,3.29C18,2.9 17.35,2.9 16.96,3.29L15.13,5.12L18.88,8.87M3,17.25V21H6.75L17.81,9.94L14.06,6.19L3,17.25Z" /></svg>
                </button>
              </div>
            </div>
            <p class="creator-name" v-if="inspectorStore.details.creator_name">
              by {{ inspectorStore.details.creator_name }}
            </p>
            <div class="rating-container" v-if="inspectorStore.details.rating">
              <StarRating :rating="inspectorStore.details.rating" />
              <span class="rating-text">{{ inspectorStore.details.rating.toFixed(1) }}/5</span>
              <span class="reviews-count">({{ inspectorStore.details.reviews_count }} reviews)</span>
            </div>
          </div>
          
          <div class="description-section" v-if="inspectorStore.details.description || isEditMode">
            <h4>Description</h4>
            <textarea v-if="isEditMode" v-model="editableDetails.description" class="description-textarea"></textarea>
            <div v-else class="description-content" v-html="decodedDescription"></div>
          </div>

          <div class="description-section" v-if="inspectorStore.details.technical_description">
            <h4>Technical Details</h4>
            <div class="description-content" v-html="inspectorStore.details.technical_description"></div>
          </div>
          
          <div class="tags-properties-container">
            <div class="tags-section">
              <h4>My Tags</h4>
              <div class="tags-input-container">
                <TagPill
                  v-for="tag in inspectorStore.details.tags"
                  :key="tag"
                  :text="tag"
                  is-editable
                  @remove="removeTag(tag)"
                />
                <input
                  ref="tagInput"
                  v-model="newTag"
                  class="tag-input-field"
                  placeholder="Add a tag..."
                  @keydown="handleTagInput"
                />
              </div>
            </div>

            <details class="properties-section" open>
              <summary>Properties</summary>
              <ul class="properties-list">
                <li v-if="inspectorStore.details.source_url"><strong>Source:</strong> <button @click="openSourceUrl" class="marketplace-link">View on Marketplace</button></li>
                <li v-if="inspectorStore.details.price"><strong>Price:</strong> ${{ inspectorStore.details.price.toFixed(2) }}</li>
                <li v-if="inspectorStore.details.release_date"><strong>Release Date:</strong> {{ new Date(inspectorStore.details.release_date).toLocaleDateString() }}</li>
                <li v-if="inspectorStore.details.engine_version?.length">
                  <strong>Supported Versions:</strong> {{ inspectorStore.details.engine_version.join(', ') }}
                </li>
                <li><strong>File Path:</strong> {{ inspectorStore.details.file_path }}</li>
                <li><strong>File Size:</strong> {{ formatBytes(inspectorStore.details.file_size) }}</li>
              </ul>
            </details>
          </div>
        </template>
        
        <!-- Unmatched Asset View -->
        <div v-else class="unmatched-state">
          <h3>{{ inspectorStore.details.name }}</h3>
          <p>This asset hasn't been matched with marketplace data.</p>
          <div class="tags-properties-container">
            <div class="tags-section">
              <h4>My Tags</h4>
              <div class="tags-input-container">
                <TagPill
                  v-for="tag in inspectorStore.details.tags"
                  :key="tag"
                  :text="tag"
                  is-editable
                  @remove="removeTag(tag)"
                />
                <input
                  ref="tagInput"
                  v-model="newTag"
                  class="tag-input-field"
                  placeholder="Add a tag..."
                  @keydown="handleTagInput"
                />
              </div>
            </div>
          </div>
          <AppButton variant="primary" class="match-button" @click="openManualMatchModal">Match Manually</AppButton>
        </div>

        <div v-if="isEditMode" class="edit-mode-controls">
          <AppButton @click="saveChanges" :is-loading="isSaving" :disabled="!isDirty" variant="primary">Save</AppButton>
          <AppButton @click="cancelEdit" variant="secondary">Cancel</AppButton>
        </div>
      </div>
    </template>

    <SlideshowModal
      v-if="isSlideshowVisible"
      :key="`slideshow-${slideshowStartIndex}`"
      :media-items="inspectorStore.details?.gallery_images || []"
      :start-index="slideshowStartIndex"
      @close="closeSlideshow"
    />
    
    <Teleport to="body">
      <div v-if="showCancelConfirm" class="modal-overlay">
        <ConfirmationModal
          title="Discard Changes?"
          message="You have unsaved changes. Are you sure you want to discard them?"
          confirm-button-text="Discard"
          confirm-button-variant="danger"
          @cancel="showCancelConfirm = false"
          @confirm="confirmCancel"
        />
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, computed } from 'vue';
import { register } from 'swiper/element/bundle';
import 'swiper/css';
import 'swiper/css/navigation';
import 'swiper/css/pagination';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { useInspectorStore } from '@/stores/inspectorStore';
import { useAssetGridStore } from '@/stores/assetGridStore';
import SkeletonLoader from '@/components/SkeletonLoader.vue';
import AppButton from '@/components/AppButton.vue';
import TagPill from '@/components/TagPill.vue';
import StarRating from '@/components/StarRating.vue';
import ManualMatchModal from '@/components/ManualMatchModal.vue';
import SlideshowModal from '@/components/SlideshowModal.vue';
import ConfirmationModal from '@/components/ConfirmationModal.vue';
import { useModalStore } from '@/stores/modalStore';
import { sendNotification } from '@tauri-apps/plugin-notification';

const inspectorStore = useInspectorStore();
const assetGridStore = useAssetGridStore();
const modalStore = useModalStore();

const newTag = ref('');
const tagInput = ref<HTMLInputElement | null>(null);

const isSlideshowVisible = ref(false);
const slideshowStartIndex = ref(0);
const currentMainCarouselIndex = ref(0);

const isEditMode = ref(false);
const isSaving = ref(false);
const editableDetails = ref<any>(null);
const showCancelConfirm = ref(false);

const decodedDescription = computed(() => {
    if (inspectorStore.details?.description) {
        const d = document.createElement('div');
        d.innerHTML = inspectorStore.details.description;
        return d.innerText;
    }
    return '';
});

const isDirty = computed(() => {
  if (!isEditMode.value || !inspectorStore.details || !editableDetails.value) {
    return false;
  }
  return (
    inspectorStore.details.name !== editableDetails.value.name ||
    inspectorStore.details.description !== editableDetails.value.description
  );
});

// Computed property for proper thumbnail handling
const thumbnailSrc = computed(() => {
  if (!inspectorStore.details) return null;
  
  // If there's a thumbnail_url (from orbital), use it directly
  if (inspectorStore.details.thumbnail_url) {
    return inspectorStore.details.thumbnail_url;
  }
  
  return null;
});

// Register Swiper web components
onMounted(() => {
  register();
  window.addEventListener('keydown', handleKeyDown);
});

const openSlideshow = (index?: number) => {
  // Use the provided index or fall back to the current carousel position
  const startIndex = index !== undefined ? index : currentMainCarouselIndex.value;

  
  isSlideshowVisible.value = true;
  slideshowStartIndex.value = startIndex;
};

const closeSlideshow = () => {
  isSlideshowVisible.value = false;
};

const onMainCarouselSlideChange = (event: any) => {
  const swiper = event.detail[0];
  currentMainCarouselIndex.value = swiper.activeIndex;
  
};

const retryFetch = () => {
  if (assetGridStore.selectedAssetId) {
    inspectorStore.fetchAssetDetails(assetGridStore.selectedAssetId);
  }
};

const openManualMatchModal = () => {
  if (inspectorStore.details?.id) {
    modalStore.openModal(ManualMatchModal, { assetId: inspectorStore.details.id });
  }
};

const handleTagInput = (event: KeyboardEvent) => {
  if (event.key === 'Enter' || event.key === ',') {
    event.preventDefault();
    addTag();
  }
};

const handleMultiTagInput = (event: KeyboardEvent) => {
  if (event.key === 'Enter') {
    event.preventDefault();
    addTagsToSelectedAssets();
  }
};

const addTagsToSelectedAssets = () => {
  const tags = newTag.value.split(',').map(t => t.trim()).filter(t => t.length > 0);
  if (tags.length > 0) {
    inspectorStore.updateMultipleAssetTags(assetGridStore.selectedAssetIds, tags);
    newTag.value = '';
  }
};

const addTag = () => {
  const tagValue = newTag.value.trim().replace(/,$/, '');
  if (tagValue && inspectorStore.details) {
    const updatedTags = [...new Set([...inspectorStore.details.tags, tagValue])];
    inspectorStore.updateAssetTags(inspectorStore.details.id, updatedTags);
    newTag.value = '';
  }
};

const removeTag = (tagToRemove: string) => {
  if (inspectorStore.details) {
    const updatedTags = inspectorStore.details.tags.filter(tag => tag !== tagToRemove);
    inspectorStore.updateAssetTags(inspectorStore.details.id, updatedTags);
  }
};

const openSourceUrl = async () => {
  if (inspectorStore.details?.source_url) {
    try {
      await invoke('open_url', { url: inspectorStore.details.source_url });
    } catch (error) {
      console.error('Failed to open marketplace URL:', error);
    }
  }
};

const onMediaLoad = (event: Event, index: number, type: 'image' | 'video') => {
  const element = event.target as HTMLImageElement | HTMLIFrameElement;
  
};

const onMediaError = (event: Event, index: number, media: any, type: 'image' | 'video') => {
  console.error(`❌ ${type} ${index} failed to load:`, media);
  console.error('Error event:', event);
};

const getMediaType = (media: any): 'youtube' | 'image' => {
  const url = getMediaUrl(media);
  
  // Check if it's a YouTube URL
  if (url.includes('youtube.com/embed/') || url.includes('youtu.be/')) {
    return 'youtube';
  }
  
  return 'image';
};

const getMediaUrl = (media: any): string => {
  // Handle different possible media formats
  if (typeof media === 'string') {
    return media;
  } else if (typeof media === 'object' && media !== null) {
    // For mixed media objects:
    // Images: {url: string, alt_text?: string, width?: number, height?: number, type: 'image'}
    // Videos: {url: string, type: 'youtube', video_id?: string}
    
    // Try common property names for URLs
    const url = media.url || media.src || media.href || media.link || media.embed_url || media.video_url || String(media);
    
    // If it's a YouTube video ID, convert to embed URL
    if (media.type === 'youtube' && media.video_id && !url.includes('youtube.com/embed/')) {
      return `https://www.youtube.com/embed/${media.video_id}`;
    }
    
    return url;
  }
  return '';
};

const formatBytes = (bytes: number, decimals = 2) => {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}

watch(() => assetGridStore.selectedAssetId, (newId, oldId) => {
  // Only fetch if the single selected ID changes
  if (newId && newId !== oldId && assetGridStore.selectedAssetIds.length <= 1) {
    inspectorStore.fetchAssetDetails(newId);
  } else if (!newId) {
    inspectorStore.clearDetails();
  }
}, { immediate: true });

// Watch for gallery images changes and log for debugging
watch(() => inspectorStore.details?.gallery_images, (newImages) => {
  if (newImages) {
    // Reset carousel position when images change
    currentMainCarouselIndex.value = 0;
  }
}, { immediate: true });

// When selection mode changes, clear the tag input
watch(() => assetGridStore.selectedAssetIds.length, () => {
  newTag.value = '';
});

watch(() => inspectorStore.details, (newDetails) => {
  if (newDetails) {
    editableDetails.value = JSON.parse(JSON.stringify(newDetails));
  } else {
    editableDetails.value = null;
  }
  // Always exit edit mode when selection changes
  if (isEditMode.value) {
    isEditMode.value = false;
  }
}, { deep: true, immediate: true });

const toggleEditMode = () => {
  if (!isEditMode.value) {
    // Entering edit mode, create a fresh copy
    editableDetails.value = JSON.parse(JSON.stringify(inspectorStore.details));
  }
  isEditMode.value = !isEditMode.value;
};

const cancelEdit = () => {
  if (isDirty.value) {
    showCancelConfirm.value = true;
  } else {
    isEditMode.value = false;
  }
};

const confirmCancel = () => {
  isEditMode.value = false;
  showCancelConfirm.value = false;
};

const saveChanges = async () => {
  if (!isDirty.value || !inspectorStore.details) return;
  isSaving.value = true;

  const original = inspectorStore.details;
  const edited = editableDetails.value;
  const overrides: Record<string, any> = {};

  // Compare fields and find changes
  if (original.name !== edited.name) {
    overrides.name = edited.name;
  }
  if (original.description !== edited.description) {
    overrides.description = edited.description;
  }
  
  if (Object.keys(overrides).length === 0) {
    isSaving.value = false;
    isEditMode.value = false;
    return; // No changes to save
  }

  try {
    await invoke('update_asset_manual_overrides', {
      assetId: original.id,
      overridesJson: JSON.stringify(overrides),
    });

    await inspectorStore.fetchAssetDetails(original.id);
    
    await sendNotification({
      title: 'Save Successful',
      body: `Changes to ${original.name} have been saved.`,
    });
    
    isEditMode.value = false;

  } catch (error) {
    console.error('Failed to save changes:', error);
    await sendNotification({
      title: 'Save Failed',
      body: 'Could not save changes. Please check the logs.',
    });
  } finally {
    isSaving.value = false;
  }
};

const handleKeyDown = (e: KeyboardEvent) => {
  if (isEditMode.value) {
    if (e.key === 'Escape') {
      e.preventDefault();
      cancelEdit();
    }
    if ((e.ctrlKey || e.metaKey) && e.key === 's') {
      e.preventDefault();
      saveChanges();
    }
  }
};

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown);
  register();

});

onUnmounted(() => {
    window.removeEventListener('keydown', handleKeyDown);
});
</script>

<style scoped>
.inspector-panel {
  padding: 1.5rem;
  height: 100%;
  color: var(--text-primary);
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}

.prompt-state, .error-state, .loading-state, .unmatched-state, .multi-select-state {
  display: flex;
  flex-direction: column;
  height: 100%;
  color: var(--text-secondary);
}

.prompt-state, .error-state {
  justify-content: center;
  align-items: center;
  text-align: center;
}

.multi-select-state {
  padding-top: 2rem;
}

.multi-select-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 1.5rem 0;
}

.multi-select-state .tags-section p {
  font-size: 0.9rem;
  margin-bottom: 0.75rem;
}

.error-state p {
  margin-bottom: 1rem;
}

.unmatched-state {
  padding-top: 2rem;
}
.unmatched-state h3 {
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
}
.unmatched-state p {
  margin-bottom: 1.5rem;
}
.match-button {
  margin-top: auto;
}

.display-state {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.preview-section {
  width: 100%;
  padding-top: 56.25%; /* 16:9 Aspect Ratio */
  position: relative;
  border-radius: var(--border-radius-large);
  overflow: hidden;
  background-color: var(--bg-secondary);
  margin-bottom: 1.5rem;
  flex-shrink: 0;
}

.preview-fallback {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  color: var(--text-secondary);
}

.preview-fallback svg {
  width: 4rem;
  height: 4rem;
}

.marketplace-data-section {
  padding: 0 1.5rem;
  margin-bottom: 1rem;
}

.title-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 0.5rem;
  margin-bottom: 0.25rem;
}

.asset-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  line-height: 1.3;
  
  /* --- Word wrapping --- */
  word-wrap: break-word; /* Legacy */
  overflow-wrap: break-word; /* Standard */
  word-break: break-word; /* More aggressive */
  hyphens: auto; /* Optional: for better looking breaks */
}

.edit-actions {
  margin-left: 1rem;
}

.edit-button {
  background: none;
  border: none;
  color: var(--text-tertiary);
  cursor: pointer;
  padding: 4px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.edit-button:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.edit-button svg {
  width: 20px;
  height: 20px;
}

.title-input,
.description-textarea {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  border-radius: var(--border-radius-medium);
  font-family: inherit;
}

.title-input {
  flex-grow: 1;
}

.description-textarea {
  min-height: 150px;
  resize: vertical;
  font-size: 0.95rem;
}

.edit-mode-controls {
  position: sticky;
  bottom: 0;
  background-color: var(--bg-surface);
  padding: 1rem 1.5rem;
  border-top: 1px solid var(--border-color);
  display: flex;
  gap: 1rem;
  z-index: 10;
}

.rating-container {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.9rem;
  color: var(--text-secondary);
}

.rating-text, .reviews-count {
  line-height: 1;
}

.description-section {
  margin-bottom: 1.5rem;
}

.description-section h4 {
  margin-bottom: 0.5rem;
  color: var(--text-secondary);
  font-size: 0.9rem;
  font-weight: 600;
  text-transform: uppercase;
}

.description-content {
  color: var(--text-primary);
  line-height: 1.6;
  font-size: 0.95rem;
  max-height: 250px;
  overflow-y: auto;
  padding-right: 0.5rem;

  /* --- Word wrapping for long strings like URLs --- */
  word-wrap: break-word;
  overflow-wrap: break-word;
  word-break: break-word;
}

.description-content a {
  color: var(--accent);
  text-decoration: none;
}

.tags-properties-container {
  display: flex;
  flex-direction: column;
  gap: 0;
}

.tags-section {
  padding: 1rem 0;
  margin-top: 0;
}

.tags-section h4 {
  margin-bottom: 0.75rem;
}

.tags-input-container {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  align-items: center;
}

.tag-input-field {
  flex-grow: 1;
  background: none;
  border: none;
  color: var(--text-primary);
  padding: 0.25rem 0;
  min-width: 100px;
}

.tag-input-field:focus {
  outline: none;
}

.properties-section {
  margin-top: 0;
  padding: 1rem 0;
}

.properties-section summary {
  cursor: pointer;
  font-weight: bold;
}

.properties-list {
  list-style: none;
  padding: 0.5rem 0 0 0;
  margin: 0;
  font-size: 0.9rem;
}

.properties-list li {
  display: grid;
  grid-template-columns: max-content 1fr;
  gap: 1rem;
  padding: 0.5rem 0;
  align-items: baseline;
}

.properties-list strong {
  color: var(--text-secondary);
  font-weight: 400;
}

.marketplace-link {
  background: none;
  border: none;
  color: var(--accent, #007acc);
  text-decoration: underline;
  cursor: pointer;
  font-family: inherit;
  font-size: inherit;
  padding: 0;
  margin: 0;
}

.marketplace-link:hover {
  color: var(--accent-hover, #005aa3);
  text-decoration: none;
}

.preview-section .main-swiper {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 1;
  overflow: hidden;
}

.preview-section .main-swiper :deep(.swiper-wrapper) {
  border-radius: 8px;
  overflow: hidden;
  width: 100%;
  height: 100%;
  display: flex;
  transition-property: transform;
  transition-duration: 0.3s;
  align-items: center;
  transform-style: preserve-3d;
}

.preview-section .main-swiper .preview-image {
  width: 100%;
  height: 100%;
  object-fit: contain;
  border-radius: var(--border-radius-large);
  display: block;
}

.preview-section .main-swiper .preview-video {
  width: 100%;
  height: 100%;
  border: none;
  border-radius: var(--border-radius-large);
  display: block;
}

.preview-section .preview-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  border-radius: var(--border-radius-large);
  display: block;
}

/* Customize swiper controls to match the app theme */
.preview-section :deep(.swiper-button-prev),
.preview-section :deep(.swiper-button-next) {
  background-color: rgba(0, 0, 0, 0.7);
  color: white;
  border-radius: 6px;
  width: 36px;
  height: 36px;
  margin-top: -18px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  backdrop-filter: blur(10px);
  transition: all 0.2s ease;
}

.preview-section :deep(.swiper-button-prev) {
  left: 10px;
}

.preview-section :deep(.swiper-button-next) {
  right: 10px;
}

.preview-section :deep(.swiper-button-prev):hover,
.preview-section :deep(.swiper-button-next):hover {
  background-color: rgba(0, 0, 0, 0.9);
}

.preview-section :deep(.swiper-pagination) {
  position: absolute;
  bottom: 10px;
  z-index: 10;
}

.preview-section :deep(.swiper-pagination-bullet) {
  background-color: rgba(255, 255, 255, 0.6);
  border: none;
  border-radius: 50%;
  width: 10px;
  height: 10px;
  margin: 0 4px;
  transition: all 0.3s ease;
}

.preview-section :deep(.swiper-pagination-bullet-active) {
  background-color: var(--accent, #007acc);
}

.preview-section :deep(.swiper-slide) {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100% !important;
  height: 100%;
  position: relative;
  cursor: pointer;
  transition: transform 0.2s ease, opacity 0.2s ease;
  flex-shrink: 0;
}

.preview-section .main-swiper :deep(.swiper-slide) {
  width: 100% !important;
  max-width: 100% !important;
  min-width: 100% !important;
  flex-shrink: 0;
  opacity: 1;
  visibility: visible;
}

.preview-section :deep(.swiper-slide:hover) {
  transform: scale(1.02);
  opacity: 0.9;
}

.slide-wrapper {
  position: relative;
  width: 100%;
  height: 100%;
}

.expand-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: rgba(0, 0, 0, 0.3);
  opacity: 0;
  transition: opacity 0.2s ease;
  border-radius: var(--border-radius-large);
}

.slide-wrapper:hover .expand-overlay {
  opacity: 1;
}

.expand-icon {
  width: 32px;
  height: 32px;
  fill: white;
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.5));
}
</style>