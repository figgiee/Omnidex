<template>
  <div class="source-list">
    <div class="list-header" :class="{ collapsed: !store.isPanelOpen }">
      <h3 v-show="store.isPanelOpen">Library</h3>
      <button @click="store.togglePanel" class="toggle-button">
        <svg v-if="store.isPanelOpen" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>
        <svg v-else xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>
      </button>
    </div>
    <template v-if="store.isPanelOpen">
      <div v-if="store.loading" class="skeleton-list">
        <SkeletonLoader v-for="i in 10" :key="i" height="2rem" style="margin-bottom: 0.5rem;" />
      </div>
      <div v-else-if="store.error" class="error-state">
        <p>Error loading sources.</p>
        <AppButton @click="store.fetchCategoryCounts" variant="secondary">Retry</AppButton>
      </div>
      <ul v-else class="navigation-list">
        <!-- Static Items -->
        <li
          class="nav-item"
          :class="{ active: store.selectedCategory === 'All Assets' }"
          @click="store.selectCategory('All Assets')"
        >
          <span class="nav-name">All Assets</span>
          <span class="nav-count">{{ store.totalAssetCount }}</span>
        </li>
        <li
          class="nav-item"
          :class="{ active: store.selectedCategory === 'Favorites' }"
          @click="store.selectCategory('Favorites')"
        >
          <span class="nav-name">Favorites</span>
          <span class="nav-count">{{ store.favoritesCount }}</span>
        </li>

        <!-- Collapsible Categories -->
        <li class="collapsible-section">
          <div class="section-header" @click="toggleCategories">
            <span class="arrow" :class="{ expanded: categoriesOpen }">▶</span>
            <span class="section-title">Categories</span>
          </div>
          <ul v-show="categoriesOpen" class="category-sublist">
            <li
              v-for="category in store.justCategories"
              :key="category.name"
              class="category-item"
              :class="{ active: category.name === store.selectedCategory }"
              @click="store.selectCategory(category.name)"
            >
              <span class="category-name">{{ formatCategoryName(category.name) }}</span>
              <span class="category-count">{{ category.count }}</span>
            </li>
          </ul>
        </li>
      </ul>
    </template>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { useSourceListStore } from '@/stores/sourceListStore';
import { useAssetGridStore } from '@/stores/assetGridStore';
import SkeletonLoader from '@/components/SkeletonLoader.vue';
import AppButton from '@/components/AppButton.vue';
import { formatCategoryName } from '@/utils/formatting';

const store = useSourceListStore();
const assetGridStore = useAssetGridStore();
const categoriesOpen = ref(true);

const toggleCategories = () => {
  categoriesOpen.value = !categoriesOpen.value;
};

onMounted(() => {
  store.fetchCategoryCounts();
});

watch(
  () => store.selectedCategory,
  (newCategory) => {
    if (newCategory) {
      assetGridStore.fetchAssets(newCategory, true);
    }
  },
  { immediate: true }
);
</script>

<style scoped>
.source-list {
  display: flex;
  flex-direction: column;
  height: 100%;
  transition: width 0.3s ease;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 1rem 1rem 1rem;
  border-bottom: 1px solid var(--bg-surface);
  transition: padding 0.3s ease;
}

.list-header.collapsed {
  justify-content: center;
  padding: 1rem 0;
}

.list-header h3 {
  margin: 0;
  color: var(--text-primary);
}

.toggle-button {
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0.25rem;
  border-radius: var(--border-radius-small);
}

.toggle-button:hover {
  background-color: var(--bg-surface);
}

.toggle-button svg {
  width: 1.2rem;
  height: 1.2rem;
}

.skeleton-list, .error-state {
  padding: 1rem;
}

.error-state {
  text-align: center;
  color: var(--text-secondary);
}

.navigation-list {
  list-style: none;
  padding: 1rem;
  margin: 0;
  overflow-y: auto;
  flex-grow: 1;
}

.nav-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1rem;
  border-radius: var(--border-radius-medium);
  cursor: pointer;
  transition: background-color 0.2s ease;
  margin-bottom: 0.25rem;
}

.nav-item:hover {
  background-color: var(--bg-surface);
}

.nav-item.active {
  background-color: var(--accent);
  color: var(--text-primary);
  font-weight: 600;
}

.nav-name {
  font-weight: 500;
}

.nav-count {
  font-size: 0.9rem;
  color: var(--text-secondary);
  background-color: var(--bg-surface);
  padding: 0.1rem 0.4rem;
  border-radius: var(--border-radius-small);
}

.nav-item.active .nav-count {
  background-color: rgba(255, 255, 255, 0.2);
  color: var(--text-primary);
}

.collapsible-section {
  margin-top: 1rem;
}

.section-header {
  display: flex;
  align-items: center;
  cursor: pointer;
  padding: 0.5rem 0;
  color: var(--text-secondary);
}

.section-header:hover {
  color: var(--text-primary);
}

.arrow {
  transition: transform 0.2s ease;
  font-size: 0.8rem;
  margin-right: 0.5rem;
}

.arrow.expanded {
  transform: rotate(90deg);
}

.section-title {
  font-weight: 600;
  text-transform: uppercase;
  font-size: 0.8rem;
  letter-spacing: 0.05em;
}

.category-sublist {
  list-style: none;
  padding: 0;
  margin-left: 1.5rem; /* Indent sublist */
}

.category-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 1rem;
  border-radius: var(--border-radius-medium);
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.category-item:hover {
  background-color: var(--bg-surface);
}

.category-item.active {
  background-color: var(--accent-secondary); /* A different color for distinction maybe */
  color: var(--text-primary);
}

.category-name {
  font-weight: 500;
}

.category-count {
  font-size: 0.9rem;
  color: var(--text-secondary);
}

.category-item.active .category-count {
  color: var(--text-primary);
  opacity: 0.8;
}

/* Remove old .category-list if it exists */
.category-list {
  display: none;
}
</style> 