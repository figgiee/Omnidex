import type { ScanLocation, ScanProgress } from '@/types'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { ErrorHandler } from '@/types/error'

export const useScanLocationStore = defineStore('scanLocations', () => {
  // State
  const scanLocations = ref<ScanLocation[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const scanProgress = ref<ScanProgress | null>(null)
  const isScanning = ref(false)
  const scanCompleted = ref(false)

  // Actions
  async function fetchScanLocations() {
    loading.value = true
    error.value = null

    try {
      const result = await invoke<ScanLocation[]>('get_scan_locations')
      scanLocations.value = result
    } catch (err) {
      const errorInfo = ErrorHandler.showErrorToast(err, 'Failed to fetch scan locations')
      error.value = errorInfo.userMessage
    } finally {
      loading.value = false
    }
  }

  async function createScanLocation(
    name: string,
    path: string,
    scanRecursive: boolean,
    fileExtensions?: string,
    description?: string,
  ) {
    loading.value = true
    error.value = null
    try {
      const args = {
        name,
        path,
        scanRecursive: scanRecursive,
        fileExtensions,
        description,
      };
      const result = await invoke<ScanLocation>('create_scan_location', args)
      await fetchScanLocations()
      return result
    } catch (err) {
      const errorInfo = ErrorHandler.showErrorToast(err, 'Failed to create scan location')
      error.value = errorInfo.userMessage
      throw err
    } finally {
      loading.value = false
    }
  }

  async function updateScanLocation(
    id: number,
    name: string,
    path: string,
    isActive: boolean,
    scanRecursive: boolean,
    fileExtensions?: string,
    description?: string
  ) {
    loading.value = true
    error.value = null

    try {
      const result = await invoke<ScanLocation>('update_scan_location', {
        id,
        name,
        path,
        isActive,
        scanRecursive,
        fileExtensions,
        description
      })
      
      // Update local state
      const index = scanLocations.value.findIndex(loc => loc.id === id)
      if (index > -1) {
        scanLocations.value[index] = result
      }
      
      return result
    } catch (err) {
      error.value = err as string
      console.error('Failed to update scan location:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  async function updateScanRecursive(id: number, scanRecursive: boolean) {
    loading.value = true
    error.value = null

    try {
      const result = await invoke<ScanLocation>('update_scan_recursive_setting', {
        locationId: id,
        scanRecursive
      })
      
      // Update local state
      const index = scanLocations.value.findIndex(loc => loc.id === id)
      if (index > -1) {
        scanLocations.value[index] = result
      }
      
      return result
    } catch (err) {
      error.value = err as string
      console.error('Failed to update scan recursive setting:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  async function clearAssetsFromScanLocation(locationId: number) {
    loading.value = true
    error.value = null

    try {
      const deletedCount = await invoke<number>('clear_assets_from_scan_location', {
        locationId
      })
      
      return deletedCount
    } catch (err) {
      error.value = err as string
      console.error('Failed to clear assets from scan location:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  async function deleteScanLocation(id: number) {
    loading.value = true
    error.value = null

    try {
      await invoke('delete_scan_location', { id })
      
      // Remove from local state
      const index = scanLocations.value.findIndex(loc => loc.id === id)
      if (index > -1) {
        scanLocations.value.splice(index, 1)
      }
    } catch (err) {
      error.value = err as string
      console.error('Failed to delete scan location:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  async function startScan(locationId: number) {
    if (isScanning.value) {
      console.warn('A scan is already in progress.');
      return;
    }

    isScanning.value = true;
    error.value = null;
    scanProgress.value = null; // Reset progress
    

    try {
      await invoke('start_scan', { locationId });
    } catch (err) {
      error.value = err as string;
      isScanning.value = false; // Reset on error
      console.error('Failed to start scan:', err);
      throw err;
    }
  }

  async function scanAll() {
    if (isScanning.value) {
      console.warn('A scan is already in progress.');
      return;
    }

    isScanning.value = true;
    error.value = null;
    scanProgress.value = null;

    try {
      await invoke('scan_all_locations');
    } catch (err) {
      error.value = err as string;
      isScanning.value = false;
      console.error('Failed to start all scans:', err);
      throw err;
    }
  }

  let unlisten: UnlistenFn | null = null;
  
  async function initializeScanListener() {
    if (unlisten) return; // Already initialized

    try {
      unlisten = await listen<ScanProgress>('scan-progress', (event) => {
        const { status } = event.payload;

        if (status === 'Scanning') {
          if (!isScanning.value) {
            isScanning.value = true;
            ErrorHandler.showInfoToast('Scan started', 'Scanning Assets');
          }
          scanProgress.value = event.payload;
        } else if (status === 'Completed' || status === 'Error' || status === 'Cancelled') {
          if (isScanning.value) isScanning.value = false;
          scanProgress.value = event.payload; // Keep final progress
          if (status === 'Completed') {
            scanCompleted.value = true;
            // Show completion toast with detailed information
            const processedCount = event.payload.processed_items || 0;
            const totalCount = event.payload.total_items || 0;
            const errorCount = event.payload.error_count || 0;
            
            let message = `Successfully processed ${processedCount.toLocaleString()}`;
            if (totalCount > 0) {
              message += ` of ${totalCount.toLocaleString()}`;
            }
            message += ' items';
            
            if (errorCount > 0) {
              message += ` (${errorCount} errors)`;
            }
            
            ErrorHandler.showSuccessToast(message, 'Scan Complete');
          } else if (status === 'Error') {
            ErrorHandler.showErrorToast(
              event.payload.error || 'An error occurred during scanning', 
              'Scan Failed'
            );
          } else if (status === 'Cancelled') {
            ErrorHandler.showInfoToast('Scan was cancelled', 'Scan Cancelled');
          }
        } else {
          // For 'Initializing' or other states, just update progress
          scanProgress.value = event.payload;
          
          if (status === 'Initializing' && !isScanning.value) {
            ErrorHandler.showInfoToast('Preparing to scan...', 'Initializing');
          }
        }
      });
  
    } catch (e) {
      console.error('Failed to initialize scan listener:', e);
    }
  }

  function destroyScanListener() {
    if (unlisten) {
      unlisten();
      unlisten = null;
  
    }
  }

  function clearScanProgress() {
    scanProgress.value = null
  }

  function clearError() {
    error.value = null
  }

  // Computed getters
  function getActiveLocations() {
    return scanLocations.value.filter(loc => loc.is_active)
  }

  function getLocationById(id: number) {
    return scanLocations.value.find(loc => loc.id === id)
  }

  function getLocationsByPath(path: string) {
    return scanLocations.value.filter(loc => 
      loc.path.toLowerCase().includes(path.toLowerCase())
    )
  }

  return {
    // State
    scanLocations,
    loading,
    error,
    scanProgress,
    isScanning,
    scanCompleted,
    
    // Actions
    fetchScanLocations,
    createScanLocation,
    updateScanLocation,
    updateScanRecursive,
    clearAssetsFromScanLocation,
    deleteScanLocation,
    startScan,
    scanAll,
    clearScanProgress,
    clearError,
    
    // Getters
    getActiveLocations,
    getLocationById,
    getLocationsByPath,
    initializeScanListener,
    destroyScanListener,
  }
}) 