<template>
  <div class="action-bar" :class="{ 'visible': assetGridStore.hasSelection }">
    <div class="action-bar-content">
      <div class="selection-info">
        <span>{{ assetGridStore.selectedAssetIds.length }} item(s) selected</span>
        <AppButton variant="link" @click="clearSelection">Clear</AppButton>
      </div>
      <div class="actions">
        <AppButton
          variant="secondary"
          @click="toggleFavorite"
          :title="areAllSelectedFavorited ? 'Unfavorite items' : 'Favorite items'"
        >
          {{ areAllSelectedFavorited ? 'Unfavorite' : 'Favorite' }}
        </AppButton>
        <AppButton variant="secondary" @click="openTagModal">Tag</AppButton>
        <div class="more-actions">
          <AppButton variant="ghost" @click="showMore = !showMore">More...</AppButton>
          <div v-if="showMore" class="more-dropdown">
            <button @click="openDeleteModal">Delete...</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useAssetGridStore } from '@/stores/assetGridStore';
import { useModalStore } from '@/stores/modalStore';
import { useInspectorStore } from '@/stores/inspectorStore';
import { useSourceListStore } from '@/stores/sourceListStore';
import AppButton from '@/components/AppButton.vue';
import TaggingModal from '@/components/TaggingModal.vue';
import DeleteConfirmationModal from '@/components/DeleteConfirmationModal.vue';
import { invoke } from '@tauri-apps/api/core';

const assetGridStore = useAssetGridStore();
const modalStore = useModalStore();
const inspectorStore = useInspectorStore();
const sourceListStore = useSourceListStore();

const showMore = ref(false);

const areAllSelectedFavorited = computed(() => {
  if (assetGridStore.selectedAssetIds.length === 0) return false;
  return assetGridStore.selectedAssetIds.every(id => {
    // This is a bit of a hack, we should have a proper way to get asset details
    const asset = assetGridStore.assets.find(a => a.id === id);
    return asset?.is_favorite;
  });
});

const clearSelection = () => {
  assetGridStore.clearSelection();
}

const toggleFavorite = async () => {
  const shouldFavorite = !areAllSelectedFavorited.value;
  try {
    await invoke('toggle_favorite_status', {
      assetIds: assetGridStore.selectedAssetIds,
      isFavorited: shouldFavorite
    });
    // Refresh both the grid and the source list
    const currentCategory = assetGridStore.currentCategory || 'All';
    await Promise.all([
      assetGridStore.fetchAssets(currentCategory, true),
      sourceListStore.fetchCategoryCounts()
    ]);
    
    // Don't clear the selection so users can perform more actions on the same assets
  } catch (e) {
    console.error("Failed to toggle favorite status", e);
  }
};

const openTagModal = () => {
  modalStore.openModal(TaggingModal, { assetIds: assetGridStore.selectedAssetIds });
};

const openDeleteModal = () => {
  showMore.value = false;
  modalStore.openModal(DeleteConfirmationModal, { assetIds: assetGridStore.selectedAssetIds });
};
</script>

<style scoped>
.action-bar {
  position: fixed;
  bottom: 2rem;
  left: 50%;
  transform: translateX(-50%) translateY(200%);
  width: auto;
  min-width: 400px;
  max-width: 600px;
  background-color: var(--bg-surface);
  border-radius: var(--border-radius-large);
  border: 1px solid var(--border-color);
  box-shadow: var(--shadow-strong);
  padding: 1rem 1.5rem;
  transition: transform 0.3s ease-in-out;
  z-index: 100;
}

.action-bar.visible {
  transform: translateX(-50%) translateY(0);
}

.action-bar-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.selection-info {
  color: var(--text-secondary);
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.selection-info .app-button {
  font-size: 0.9rem;
}

.actions {
  display: flex;
  gap: 0.75rem;
}

.more-actions {
  position: relative;
}

.more-dropdown {
  position: absolute;
  bottom: calc(100% + 0.5rem);
  right: 0;
  background-color: var(--bg-surface);
  border-radius: var(--border-radius-medium);
  border: 1px solid var(--border-color);
  box-shadow: var(--shadow-medium);
  padding: 0.5rem;
  min-width: 150px;
  z-index: 110;
}

.more-dropdown button {
  background: none;
  border: none;
  color: var(--text-primary);
  padding: 0.5rem 1rem;
  width: 100%;
  text-align: left;
  border-radius: var(--border-radius-small);
  cursor: pointer;
}

.more-dropdown button:hover {
  background-color: var(--bg-primary);
}
</style> 