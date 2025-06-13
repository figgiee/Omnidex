<template>
  <div class="asset-grid-container">
    <div class="grid-toolbar">
      <div class="toolbar-left">
        <label for="sort-by">Sort by:</label>
        <AppSelect
          id="sort-by"
          :model-value="store.sortBy"
          @update:model-value="onSortChange"
        >
          <option value="name_asc">Name (A-Z)</option>
          <option value="name_desc">Name (Z-A)</option>
          <option value="date_desc">Date (Newest)</option>
          <option value="date_asc">Date (Oldest)</option>
        </AppSelect>
      </div>
      <div class="toolbar-right">
        <label for="view-per-page">View:</label>
        <AppSelect
          id="view-per-page"
          :model-value="store.viewPerPage.toString()"
          @update:model-value="onViewChange"
        >
          <option value="20">20</option>
          <option value="50">50</option>
          <option value="100">100</option>
        </AppSelect>
      </div>
    </div>
    <div ref="scrollContainer" class="asset-grid-scroll-area">
      <div v-if="store.loading && store.assets.length === 0" class="asset-grid skeleton-grid">
        <AssetCardSkeleton v-for="i in 20" :key="i" />
      </div>
      <div v-else-if="store.error" class="error-state">
        <p>{{ store.error }}</p>
        <AppButton @click="retryFetch" variant="secondary">Retry</AppButton>
      </div>
      <div 
        v-else-if="store.assets.length > 0" 
        class="asset-grid"
        @click.self="clearSelection"
      >
        <AssetCard
          v-for="asset in store.assets"
          :key="asset.id"
          :asset-data="asset"
          :selected="store.selectedAssetIds.includes(asset.id)"
        />
      </div>
      <div v-if="store.assets.length === 0 && !store.loading" class="empty-state">
        <div v-if="sourceListStore.totalAssetCount === 0" class="onboarding-message">
          <h2>Welcome to Omnidex!</h2>
          <p>To get started, add an asset folder to scan.</p>
          <p>Click the <strong>Settings</strong> icon (⚙️) in the top right.</p>
        </div>
        <p v-else>No assets found in this category.</p>
      </div>
      <div ref="sentinel" class="sentinel"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { useAssetGridStore } from '@/stores/assetGridStore';
import { useSourceListStore } from '@/stores/sourceListStore';
import AssetCard from '@/components/AssetCard.vue';
import AssetCardSkeleton from '@/components/AssetCardSkeleton.vue';
import AppSelect from '@/components/AppSelect.vue';
import AppButton from '@/components/AppButton.vue';
import { SortOption } from '@/types/asset';

const store = useAssetGridStore();
const sourceListStore = useSourceListStore();

const scrollContainer = ref<HTMLElement | null>(null);
const sentinel = ref<HTMLElement | null>(null);

const clearSelection = () => {
  store.clearSelection();
};

const onAssetSelect = (id: number) => {
  store.selectAsset(id);
};

const onSortChange = (value: string) => {
  store.setSortBy(value as SortOption);
};

const onViewChange = (value: string) => {
  store.setViewPerPage(Number(value));
};

const retryFetch = () => {
  store.fetchAssets(sourceListStore.selectedCategory, true);
};

let observer: IntersectionObserver;

onMounted(() => {
  store.fetchAssets(sourceListStore.selectedCategory, true);
  
  observer = new IntersectionObserver((entries) => {
    if (entries[0].isIntersecting && store.hasMore) {
      store.fetchAssets(sourceListStore.selectedCategory);
    }
  }, {
    root: scrollContainer.value,
    threshold: 0.1,
  });

  if (sentinel.value) {
    observer.observe(sentinel.value);
  }
});

watch(() => sourceListStore.selectedCategory, (newCategory) => {
  store.fetchAssets(newCategory, true);
});

onUnmounted(() => {
  if (observer) {
    observer.disconnect();
  }
});

</script>

<style scoped>
.asset-grid-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: var(--bg-primary);
}

.grid-toolbar {
  padding: 1rem;
  border-bottom: 1px solid var(--bg-surface);
  flex-shrink: 0;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.toolbar-left, .toolbar-right {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

label {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.asset-grid-scroll-area {
  flex-grow: 1;
  overflow-y: auto;
  padding: 1rem;
}

.asset-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 1rem;
}

.skeleton-grid {
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
}

.empty-state, .error-state {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100%;
  color: var(--text-secondary);
  text-align: center;
  padding: 2rem;
}

.onboarding-message h2 {
  color: var(--text-primary);
  font-size: 1.8rem;
  margin-bottom: 1rem;
}

.onboarding-message p {
  font-size: 1.1rem;
  line-height: 1.6;
  max-width: 400px;
}

.onboarding-message strong {
  color: var(--accent);
}

.error-state p {
  margin-bottom: 1rem;
}

.sentinel {
  height: 1px;
}
</style> 