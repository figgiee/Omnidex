import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export interface AppSettings {
  disableRecursiveScanning: boolean
  maxConcurrentScans: number
  autoSaveInterval: number
  enableNotifications: boolean
  darkMode: boolean
  showThumbnails: boolean
  enableFabMatching: boolean
}

const DEFAULT_SETTINGS: AppSettings = {
  disableRecursiveScanning: true,
  maxConcurrentScans: 3,
  autoSaveInterval: 30, // seconds
  enableNotifications: true,
  darkMode: false,
  showThumbnails: true,
  enableFabMatching: true
}

export const useSettingsStore = defineStore('settings', () => {
  // State
  const settings = ref<AppSettings>({ ...DEFAULT_SETTINGS })
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Actions
  function loadSettings() {
    try {
      const savedSettings = localStorage.getItem('omnidex-settings')
      if (savedSettings) {
        const parsed = JSON.parse(savedSettings)
        // Merge with defaults to ensure all settings exist
        settings.value = { ...DEFAULT_SETTINGS, ...parsed }
      }
    } catch (err) {
      console.error('Failed to load settings from localStorage:', err)
      error.value = 'Failed to load settings'
      // Reset to defaults on error
      settings.value = { ...DEFAULT_SETTINGS }
    }
  }

  function saveSettings() {
    try {
      localStorage.setItem('omnidex-settings', JSON.stringify(settings.value))

    } catch (err) {
      console.error('Failed to save settings to localStorage:', err)
      error.value = 'Failed to save settings'
    }
  }

  function updateSetting<K extends keyof AppSettings>(key: K, value: AppSettings[K]) {
    settings.value[key] = value
    saveSettings()
  }

  function resetSettings() {
    settings.value = { ...DEFAULT_SETTINGS }
    saveSettings()
  }

  function clearError() {
    error.value = null
  }

  // Watch for changes and auto-save
  watch(
    settings,
    () => {
      saveSettings()
    },
    { deep: true }
  )

  // Initialize settings on store creation
  loadSettings()

  return {
    // State
    settings,
    loading,
    error,
    
    // Actions
    loadSettings,
    saveSettings,
    updateSetting,
    resetSettings,
    clearError,
    
    // Computed getters for individual settings
    get disableRecursiveScanning() { return settings.value.disableRecursiveScanning },
    get maxConcurrentScans() { return settings.value.maxConcurrentScans },
    get autoSaveInterval() { return settings.value.autoSaveInterval },
    get enableNotifications() { return settings.value.enableNotifications },
    get darkMode() { return settings.value.darkMode },
    get showThumbnails() { return settings.value.showThumbnails },
    get enableFabMatching() { return settings.value.enableFabMatching }
  }
}) 