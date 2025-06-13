import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { SortOption, type AssetCardData } from '@/types/asset';
import { ErrorHandler } from '@/types/error';

export const useAssetGridStore = defineStore('assetGrid', {
  state: () => ({
    assets: [] as AssetCardData[],
    loading: false,
    error: null as string | null,
    selectedAssetIds: [] as number[],
    page: 0,
    hasMore: true,
    sortBy: (localStorage.getItem('sortBy') as SortOption) || SortOption.DateDesc,
    viewPerPage: Number(localStorage.getItem('viewPerPage')) || 50,
    currentCategory: 'All' as string,
  }),

  getters: {
    hasSelection(state): boolean {
      return state.selectedAssetIds.length > 0;
    },
    // The main selected asset for the inspector is the last one selected
    selectedAssetId(state): number | null {
      return state.selectedAssetIds.length > 0 ? state.selectedAssetIds[state.selectedAssetIds.length - 1] : null;
    }
  },

  actions: {
    async fetchAssets(category: string, reset = false) {
      if (this.loading && !reset) return;

      this.currentCategory = category;

      if (reset) {
        this.page = 0;
        this.assets = [];
        this.hasMore = true;
      }
      if (!this.hasMore) return;

      this.loading = true;
      this.error = null;

      const isAllAssets = category === 'All Assets';
      const invokeCategory = isAllAssets ? 'All' : category;
      const invokeSortBy = this.sortBy;

      try {
  
        const newAssets: AssetCardData[] = await invoke('get_assets_by_category', {
          category: invokeCategory,
          sortBy: invokeSortBy,
          limit: this.viewPerPage,
          offset: this.page * this.viewPerPage,
        });

        if (newAssets.length < this.viewPerPage) {
          this.hasMore = false;
        }
        this.assets.push(...newAssets);
        this.page++;
      } catch (e: any) {
        console.error(`❌ Failed to fetch assets for category "${invokeCategory}":`, e);
        const errorInfo = ErrorHandler.showErrorToast(e, 'Failed to fetch assets');
        this.error = errorInfo.userMessage;
      } finally {
        this.loading = false;
      }
    },
    selectAsset(id: number) {
      this.toggleSelection(id);
    },
    toggleSelection(id: number, event?: MouseEvent) {
      if (event?.ctrlKey) {
        const index = this.selectedAssetIds.indexOf(id);
        if (index > -1) {
          this.selectedAssetIds.splice(index, 1);
        } else {
          this.selectedAssetIds.push(id);
        }
      } else if (event?.shiftKey) {
        const lastSelectedId = this.selectedAssetId;
        if (lastSelectedId) {
          const lastIndex = this.assets.findIndex(a => a.id === lastSelectedId);
          const currentIndex = this.assets.findIndex(a => a.id === id);
          const start = Math.min(lastIndex, currentIndex);
          const end = Math.max(lastIndex, currentIndex);
          const idsToSelect = this.assets.slice(start, end + 1).map(a => a.id);
          this.selectedAssetIds = [...new Set([...this.selectedAssetIds, ...idsToSelect])];
        } else {
          this.selectedAssetIds = [id];
        }
      } else {
        this.selectedAssetIds = [id];
      }
    },
    clearSelection() {
      this.selectedAssetIds = [];
    },
    selectAll() {
      this.selectedAssetIds = this.assets.map(a => a.id);
    },
    setSortBy(sortBy: SortOption) {
      this.sortBy = sortBy;
      localStorage.setItem('sortBy', sortBy);
      this.fetchAssets(this.currentCategory, true); // Re-fetch with new sort
    },
    setViewPerPage(viewPerPage: number) {
      this.viewPerPage = viewPerPage;
      localStorage.setItem('viewPerPage', viewPerPage.toString());
      this.fetchAssets(this.currentCategory, true); // Re-fetch with new view per page
    },
  },
}); 