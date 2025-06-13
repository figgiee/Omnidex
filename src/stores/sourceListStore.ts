import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';

const initializeCategories = (): Record<string, number> => {
  const orbitalCategories = [
    '2d-asset',
    '3d-model',
    'animation',
    'audio',
    'education-tutorial',
    'environment',
    'game-system',
    'game-template',
    'hdri',
    'material',
    'smart-asset',
    'tool-and-plugin',
    'ui',
    'vfx',
  ];
  const categories: Record<string, number> = {};
  for (const category of orbitalCategories) {
    categories[category] = 0;
  }
  return categories;
}

export const useSourceListStore = defineStore('sourceList', {
  state: () => ({
    categories: initializeCategories(),
    favoritesCount: 0,
    loading: false,
    error: null as string | null,
    selectedCategory: 'All Assets' as string,
    isPanelOpen: true,
  }),

  getters: {
    justCategories(state): { name: string, count: number }[] {
      return Object.entries(state.categories)
        .map(([name, count]) => ({ name, count }))
        .sort((a, b) => a.name.localeCompare(b.name));
    },
    displayCategories(state): { name: string, count: number | string }[] {
      const allAssetsCount = Object.values(state.categories).reduce((sum, count) => sum + count, 0);
      
      return [
        { name: 'All Assets', count: allAssetsCount },
        { name: 'Favorites', count: state.favoritesCount },
        ...this.justCategories,
      ];
    },
    totalAssetCount(state): number {
      return Object.values(state.categories).reduce((sum, count) => sum + count, 0);
    },
  },

  actions: {
    async fetchCategoryCounts() {
      this.loading = true;
      this.error = null;
      try {
        const counts: Record<string, number> = await invoke('get_category_counts');
        this.favoritesCount = await invoke('get_favorite_assets_count');
        // Merge fetched counts with existing categories
        const newCategories = initializeCategories();
        for (const category in counts) {
          if (Object.prototype.hasOwnProperty.call(newCategories, category)) {
            newCategories[category] = counts[category];
          }
        }
        this.categories = newCategories;
      } catch (e: any) {
        this.error = e.toString();
      } finally {
        this.loading = false;
      }
    },
    selectCategory(category: string) {
      this.selectedCategory = category;
    },
    togglePanel() {
      this.isPanelOpen = !this.isPanelOpen;
    }
  },
}); 