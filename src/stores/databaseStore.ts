import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export const useDatabaseStore = defineStore('database', () => {
  // State
  const loading = ref(false)
  const error = ref<string | null>(null)
  const stats = ref<{
    assets: number
    scan_locations: number
    total_records: number
  } | null>(null)

  // Actions
  async function clearAllAssets() {
    loading.value = true
    error.value = null

    try {
      const result = await invoke<string>('clear_all_assets')
      await fetchDatabaseStats() // Refresh stats
      return result
    } catch (err) {
      error.value = err as string
      console.error('Failed to clear all assets:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  async function clearAllScanLocations() {
    loading.value = true
    error.value = null

    try {
      const result = await invoke<string>('clear_all_scan_locations')
      await fetchDatabaseStats() // Refresh stats
      return result
    } catch (err) {
      error.value = err as string
      console.error('Failed to clear all scan locations:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  async function wipeEntireDatabase() {
    loading.value = true
    error.value = null

    try {
      const result = await invoke<string>('wipe_entire_database')
      await fetchDatabaseStats() // Refresh stats
      return result
    } catch (err) {
      error.value = err as string
      console.error('Failed to wipe database:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  async function fetchDatabaseStats() {
    try {
      const result = await invoke<{
        assets: number
        scan_locations: number
        total_records: number
      }>('get_database_stats')
      stats.value = result
      return result
    } catch (err) {
      error.value = err as string
      console.error('Failed to fetch database stats:', err)
      throw err
    }
  }

  function clearError() {
    error.value = null
  }

  return {
    // State
    loading,
    error,
    stats,
    
    // Actions
    clearAllAssets,
    clearAllScanLocations,
    wipeEntireDatabase,
    fetchDatabaseStats,
    clearError
  }
}) 