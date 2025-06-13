import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import type { AssetCardData } from '@/types/asset';
import { useAssetGridStore } from './assetGridStore';

export const useSearchStore = defineStore('search', {
  state: () => ({
    isSearching: false,
    query: '',
    results: [] as AssetCardData[],
    loading: false,
    error: null as string | null,
  }),

  actions: {
    async performSearch(query: string) {
      if (!query) {
        this.isSearching = false;
        this.query = '';
        this.results = [];
        const assetGridStore = useAssetGridStore();
        assetGridStore.fetchAssets(assetGridStore.currentCategory, true); // Re-fetch current category
        return;
      }

      this.isSearching = true;
      this.query = query;
      this.loading = true;
      this.error = null;
      try {
        this.results = await invoke('search_assets', { query, limit: 100, offset: 0 });
        const assetGridStore = useAssetGridStore();
        assetGridStore.assets = this.results; // Overwrite asset grid with search results
      } catch (e: any) {
        this.error = e.toString();
      } finally {
        this.loading = false;
      }
    },
  },
}); 