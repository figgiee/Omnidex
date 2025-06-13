import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import type { Asset, AssetFilter, AssetStats, ScanLocation, ScanProgress } from '@/types'

export const useAssetStore = defineStore('assets', () => {
  // State
  const assets = ref<Asset[]>([])
  const currentAsset = ref<Asset | null>(null)
  const scanLocations = ref<ScanLocation[]>([])
  const scanProgress = ref<ScanProgress | null>(null)
  const isLoading = ref(false)
  const isScanning = ref(false)
  const error = ref<string | null>(null)
  const assetStats = ref<AssetStats | null>(null)
  const selectedAssets = ref<number[]>([])
  
  // Loading state specifically for FAB refresh, mapping assetId to boolean
  const isRefreshingFabDetails = ref<Record<number, boolean>>({})

  // Getters
  const totalAssets = computed(() => assets.value.length)
  const favoriteAssets = computed(() => assets.value.filter(asset => asset.is_favorite))

  // Actions

  // Initialize event listeners
  // This should be called once, perhaps in App.vue or a main layout component,
  // or ensure this store is initialized early.
  // For simplicity, adding it here. Ensure it's only set up once.
  let unlistenAssetUpdated: (() => void) | null = null;

  function getAssetById(id: number): Asset | undefined {
    return assets.value.find(a => a.id === id);
  }

  function updateFavoriteStatus(assetIds: number[], isFavorite: boolean) {
    assetIds.forEach(id => {
      const asset = getAssetById(id);
      if (asset) {
        asset.is_favorite = isFavorite;
      }
    });
  }

  async function setupEventListeners() {
    if (unlistenAssetUpdated) {
      unlistenAssetUpdated(); // Clear previous listener if any
    }
    unlistenAssetUpdated = await listen<number>('asset_updated', async (event) => {
      const assetId = event.payload;
      
      try {
        // Fetch the updated asset details
        const updatedAsset = await invoke<Asset>('get_asset_details', { id: assetId });
        
        // Update the asset in the list
        updateAssetInList(updatedAsset);
        
        // Clear loading state for this specific asset if it was refreshing
        if (isRefreshingFabDetails.value[assetId]) {
          isRefreshingFabDetails.value[assetId] = false;
        }
        
      } catch (err) {
        let message = 'An unknown error occurred while updating asset after event';
        if (err instanceof Error) {
          message = err.message;
        } else if (typeof err === 'string') {
          message = err;
        }
        console.error(`Error updating asset ${assetId} after event: ${message}`, err);
      }
    });
    
  }

  // Call setupEventListeners when store is initialized or in an init action
  // setupEventListeners(); // Consider calling this from an appropriate lifecycle hook in a Vue component


  function updateAssetInList(updatedAsset: Asset) {
    const index = assets.value.findIndex(a => a.id === updatedAsset.id);
    if (index !== -1) {
      assets.value[index] = { ...assets.value[index], ...updatedAsset };
    } else {
      // Optionally add if not found, though update implies it exists
      // assets.value.push(updatedAsset); 
    }
  }

  async function fetchAssets(filter?: AssetFilter, limit?: number, offset?: number) {
    isLoading.value = true
    error.value = null
    try {
      const result = await invoke<Asset[]>('get_assets', { filter, limit, offset })
      assets.value = result
    } catch (err: unknown) {
      let message = 'Failed to fetch assets';
      if (err instanceof Error) {
        message = err.message;
      } else if (typeof err === 'string') {
        message = err;
      }
      error.value = message;
    } finally {
      isLoading.value = false
    }
  }

  async function fetchAssetById(id: number) {
    isLoading.value = true // Or a specific loader for currentAsset
    error.value = null
    try {
      // Assuming a command 'get_asset_details' or similar exists
      const result = await invoke<Asset>('get_asset_details', { id: id }) 
      currentAsset.value = result
      // Also update in the main list if present
      updateAssetInList(result);
    } catch (err: unknown) {
      let message = `Failed to fetch asset ${id}`;
      if (err instanceof Error) {
        message = err.message;
      } else if (typeof err === 'string') {
        message = err;
      }
      error.value = message;
      currentAsset.value = null
    } finally {
      isLoading.value = false
    }
  }
  
  async function refreshFabAssetDetails(assetId: number) {
    isRefreshingFabDetails.value = { ...isRefreshingFabDetails.value, [assetId]: true };
    error.value = null; // Clear previous errors specifically for this operation
    try {
      await invoke<void>('refresh_fab_asset_details', { assetId });
      // Success is primarily handled by the 'asset_updated' event listener
      // which will then re-fetch or update the asset.
      // We can leave the isRefreshingFabDetails[assetId] = false to the event handler,
      // or set a timeout here as a fallback if the event doesn't fire.
      
    } catch (err: unknown) {
      let message = `Failed to refresh FAB details for asset ${assetId}`;
      if (err instanceof Error) {
        message = err.message;
      } else if (typeof err === 'string') {
        message = err;
      }
      error.value = message;
      isRefreshingFabDetails.value = { ...isRefreshingFabDetails.value, [assetId]: false };
    }
    // Note: isLoading.value might not be appropriate here if it's a global page loader.
    // isRefreshingFabDetails handles per-asset loading state.
  }


  async function fetchAssetStats() {
    isLoading.value = true
    error.value = null
    try {
      assetStats.value = await invoke<AssetStats>('get_asset_stats')
    } catch (err: unknown) {
      let message = 'Failed to fetch asset stats';
      if (err instanceof Error) {
        message = err.message;
      } else if (typeof err === 'string') {
        message = err;
      }
      error.value = message;
    } finally {
      isLoading.value = false
    }
  }

  async function fetchScanLocations() {
    isLoading.value = true
    error.value = null
    try {
      scanLocations.value = await invoke<ScanLocation[]>('get_scan_locations')
    } catch (err: unknown) {
      let message = 'Failed to fetch scan locations';
      if (err instanceof Error) {
        message = err.message;
      } else if (typeof err === 'string') {
        message = err;
      }
      error.value = message;
    } finally {
      isLoading.value = false
    }
  }


  async function toggleFavorite(assetId: number, isFavorite: boolean) {
    error.value = null;
    try {
      const asset = assets.value.find(a => a.id === assetId);
      if (asset) {
        asset.is_favorite = isFavorite;
        await invoke('toggle_favorite', { assetId, isFavorite });
      }
    } catch (err: unknown) {
      let message = 'Failed to update favorite status';
      if (err instanceof Error) {
        message = err.message;
      } else if (typeof err === 'string') {
        message = err;
      }
      error.value = message;
    }
  }
  
  async function updateAssetDescription(assetId: number, newDescription: string) {
    error.value = null;
    try {
      await invoke('update_asset_description', { id: assetId, description: newDescription });
      const index = assets.value.findIndex(a => a.id === assetId);
      if (index !== -1) {
        assets.value[index].description = newDescription;
      }
      if (currentAsset.value && currentAsset.value.id === assetId) {
        currentAsset.value.description = newDescription;
      }
    } catch (err: unknown) {
      let message = 'Failed to update description';
      if (err instanceof Error) {
        message = err.message;
      } else if (typeof err === 'string') {
        message = err;
      }
      error.value = message;
    }
  }
  
  function toggleAssetSelection(assetId: number) {
    const index = selectedAssets.value.indexOf(assetId)
    if (index > -1) {
      selectedAssets.value.splice(index, 1)
    } else {
      selectedAssets.value.push(assetId)
    }
  }

  function clearSelection() {
    selectedAssets.value = []
  }

  function clearError() {
    error.value = null
  }
  
  // Make sure to call setupEventListeners when the store is initialized.
  // This is often done in App.vue or a main plugin setup.
  // If this store is setup immediately, calling it here might work,
  // but can lead to issues if store is initialized multiple times or too early.
  // A dedicated init action or plugin pattern is usually safer for event listeners.
  
  // Example of an init action if you prefer that pattern:
  async function initializeStore() {
    await setupEventListeners();
    // Fetch initial data if needed
    // await fetchAssets();
    // await fetchScanLocations();
  }


  return {
    assets,
    currentAsset,
    scanLocations,
    scanProgress,
    isLoading,
    isScanning,
    isRefreshingFabDetails, // Expose new loading state
    error,
    assetStats,
    selectedAssets,
    totalAssets,
    favoriteAssets,
    fetchAssets,
    fetchAssetStats,
    fetchAssetById,
    refreshFabAssetDetails, // Expose new action
    fetchScanLocations,
    toggleFavorite,
    updateAssetDescription,
    toggleAssetSelection,
    clearSelection,
    clearError,
    // Call initializeStore if using that pattern, or ensure setupEventListeners is called
    initializeStore, // Expose init action
    setupEventListeners, // Exposing for manual setup if preferred from outside
    updateAssetInList, // Exposing for direct use if needed
    getAssetById,
    updateFavoriteStatus
  }
}) 