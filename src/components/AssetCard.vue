<template>
  <div 
    class="asset-card" 
    :class="{ 'selected': selected }"
    @click.prevent="handleClick"
  >
    <div class="thumbnail">
      <img v-if="thumbnailSrc" :src="thumbnailSrc" :alt="assetData.name" />
      <div v-else class="no-thumbnail">?</div>
    </div>
    <div class="info">
      <h4 class="asset-name">{{ assetData.name }}</h4>
      <div class="rating-display">
        <StarRating 
          :rating="rating"
          :star-size="16"
          :read-only="true"
        />
        <span class="review-count-text">
          ({{ reviewCount }})
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/core';
import { useAssetGridStore } from '@/stores/assetGridStore';
import type { AssetCardData } from '@/types/asset';
import StarRating from '@/components/StarRating.vue';

const props = withDefaults(defineProps<{
  assetData: AssetCardData;
  selected?: boolean;
}>(), {
  selected: false,
});

const assetGridStore = useAssetGridStore();

// Use orbital ratings
const rating = computed(() => {
  return props.assetData.orbital_rating_average ?? 0;
});

const reviewCount = computed(() => {
  return props.assetData.orbital_rating_count ?? 0;
});

// Convert local file path to Tauri-accessible URL, fallback to orbital thumbnail
const thumbnailSrc = computed(() => {
  // Try local thumbnail first
  if (props.assetData.thumbnail_path) {
    try {
      return convertFileSrc(props.assetData.thumbnail_path);
    } catch (error) {
      // Silently fall through to orbital thumbnail
    }
  }
  
  // Fall back to orbital thumbnail URL
  if (props.assetData.orbital_thumbnail_url) {
    return props.assetData.orbital_thumbnail_url;
  }
  
  return null;
});

const handleClick = (event: MouseEvent) => {
  assetGridStore.toggleSelection(props.assetData.id, event);
};
</script>

<style scoped>
.asset-card {
  display: flex;
  flex-direction: column;
  border-radius: var(--border-radius-large);
  background-color: var(--bg-surface);
  padding: 1rem;
  overflow: hidden;
  transition: all 0.2s ease-in-out;
  cursor: pointer;
  border: 1px solid var(--border-color);
}

.asset-card.selected {
  border-color: var(--accent);
  box-shadow: 0 0 0 2px var(--accent);
}

.asset-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2), 0 2px 4px rgba(0, 0, 0, 0.1);
  border-color: var(--accent);
}

.thumbnail {
  width: 100%;
  padding-top: 56.25%; /* 16:9 Aspect Ratio */
  position: relative;
  border-radius: var(--border-radius-small);
  background-color: #333;
  margin-bottom: 1rem;
  overflow: hidden;
}

.thumbnail img {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.no-thumbnail {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 2rem;
  color: var(--text-subtle);
}

.info {
  display: flex;
  flex-direction: column;
}

.asset-name {
  margin: 0 0 0.25rem 0;
  color: var(--text-primary);
  font-weight: 600;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.rating-display {
  display: flex;
  align-items: center;
  gap: 0.5rem; /* spacing between stars and review count */
  margin-top: 0.5rem;
}

.review-count-text {
  font-size: 0.8rem;
  color: var(--text-secondary);
}

.asset-rating {
  margin-top: 0.5rem;
}

.asset-meta {
  margin: 0;
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.card-checkbox {
  position: absolute;
  top: 0.5rem;
  left: 0.5rem;
  z-index: 2;
}

.thumbnail-container {
  width: 100%;
  padding-top: 56.25%; /* 16:9 Aspect Ratio */
}
</style> 