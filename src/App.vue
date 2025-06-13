<template>
  <div id="app" class="app">
    <!-- Header -->
    <header class="app-header">
      <div class="header-content">
        <div class="logo">
          <h1>Omnidex</h1>
        </div>
        
        <div class="search-container">
          <GlobalSearchInput @search="handleSearch" />
        </div>

        <div class="header-actions">
          <AppButton @click="openSettings" icon="settings" variant="icon" title="Settings">
            ⚙️
          </AppButton>
        </div>
      </div>
    </header>

    <!-- Scan Progress Indicator -->
    <ScanStatusIndicator />

    <!-- Main Content -->
    <main class="app-main">
      <router-view :key="route.fullPath" />
    </main>

    <!-- Footer -->
    <AppFooter />

    <!-- Modals -->
    <ModalManager />
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, watch } from 'vue'
import { useRoute } from 'vue-router'
import { useToast } from 'vue-toastification'
import { useAssetStore } from './stores/assetStore'
import { useSettingsStore } from './stores/settingsStore'
import GlobalSearchInput from './components/GlobalSearchInput.vue';
import AppButton from './components/AppButton.vue';
import AppFooter from './components/AppFooter.vue';
import SettingsModal from './components/SettingsModal.vue';
import ModalManager from './components/ModalManager.vue';
import ScanStatusIndicator from './components/ScanStatusIndicator.vue';
import { useSearchStore } from './stores/searchStore';
import { useModalStore } from './stores/modalStore';
import { useStatusBarStore } from './stores/statusBarStore';
import { useSourceListStore } from './stores/sourceListStore';
import { useAssetGridStore } from './stores/assetGridStore';
import { listen } from '@tauri-apps/api/event';

const route = useRoute()
const toast = useToast()
const searchStore = useSearchStore();
const modalStore = useModalStore();
const statusBar = useStatusBarStore();
const sourceListStore = useSourceListStore();
const assetGridStore = useAssetGridStore();

// biome-ignore lint/correctness/noUnusedVariables: Used in template
const version = '1.0.0'

const handleSearch = (query: string) => {
  searchStore.performSearch(query);
};

const openSettings = () => {
  modalStore.openModal(SettingsModal);
};

// Debug route changes
watch(() => route.fullPath, (newPath, oldPath) => {
  // console.log(`🔄 Route changed from ${oldPath} to ${newPath}`)
}, { immediate: true })

// Toast event listener
const handleToastEvent = (event: CustomEvent) => {
  const { message, type, options } = event.detail;
  const toastOptions = {
    position: 'top-right' as const,
    timeout: 4000,
    closeOnClick: true,
    pauseOnHover: true,
    draggable: true,
    hideProgressBar: false,
    ...options
  };

  switch (type) {
    case 'success':
      toast.success(message, toastOptions);
      break;
    case 'error':
      toast.error(message, toastOptions);
      break;
    case 'info':
      toast.info(message, toastOptions);
      break;
    case 'warning':
      toast.warning(message, toastOptions);
      break;
    default:
      toast(message, toastOptions);
  }
};

onMounted(async () => {
  try {
    // Add toast event listener
    window.addEventListener('app-toast', handleToastEvent as EventListener);

    // Initialize status bar store listener
    statusBar.initialize();

    // Initialize asset store
    await useAssetStore().initializeStore()
    
    // Settings store initializes automatically when created
    useSettingsStore()

    // Listen for scan completion to refresh UI
    await listen('scan_complete', () => {
      sourceListStore.fetchCategoryCounts();
      // Assuming 'All' is a valid category to refresh the main grid.
      // We might need to get the current category from the route or a store.
      assetGridStore.fetchAssets('All', true);
    });
    
    // Listen for database wipe to reload the app
    await listen('database_wiped', () => {
      window.location.reload();
    });
    
  } catch (error) {
    console.error('Error initializing stores:', error)
  }
})

onUnmounted(() => {
  // Clean up toast event listener
  window.removeEventListener('app-toast', handleToastEvent as EventListener);
})
</script>

<style>
/* Global Styles */
html,
body,
#app {
  height: 100%;
  margin: 0;
  padding: 0;
  background-color: var(--bg-primary); 
  color: var(--text-primary); 
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
}

/* Define CSS Variables */
:root {
  --app-header-height: 70px;
  --app-footer-height: 60px;
  --sidebar-width: 300px;
  
  /* Color Palette */
  --bg-primary: #1A1A1A;
  --bg-surface: #252525;
  --text-primary: #FFFFFF;
  --text-secondary: #A0A0A0;
  --text-subtle: #6B6B6B;
  --accent: #007BFF;

  /* Sizing & Spacing */
  --spacing-unit: 1rem;
  --spacing-xs: calc(var(--spacing-unit) * 0.25);
  --spacing-sm: calc(var(--spacing-unit) * 0.5);
  --spacing-md: var(--spacing-unit);
  --spacing-lg: calc(var(--spacing-unit) * 1.5);
  --spacing-xl: calc(var(--spacing-unit) * 2);

  /* Border Radius */
  --border-radius-small: 0.25rem;
  --border-radius-medium: 0.5rem;
  --border-radius-large: 0.75rem;

  /* Shadows */
  --shadow-soft: 0 2px 8px rgba(0, 0, 0, 0.1);
  --shadow-medium: 0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -1px rgba(0,0,0,0.06);
  --shadow-strong: 0 10px 15px -3px rgba(0,0,0,0.1), 0 4px 6px -2px rgba(0,0,0,0.05);
}
</style>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

.app-header {
  background: var(--bg-surface);
  border-bottom: 1px solid #333;
  padding: 1rem 0;
}

.header-content {
  max-width: 100%;
  margin: 0 auto;
  padding: 0 2rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 2rem;
}

.logo {
  flex-shrink: 0;
  width: 200px; /* Or the width of your SourceList panel */
}

.logo h1 {
  margin: 0;
  font-size: 1.8rem;
  font-weight: 700;
  color: var(--text-primary);
}

.search-container {
  flex-grow: 1;
  display: flex;
  justify-content: center;
}

.header-actions {
  flex-shrink: 0;
  width: 200px; /* Match logo width for symmetry */
  display: flex;
  justify-content: flex-end;
}

.logo .subtitle {
  display: none;
}

.app-main {
  flex: 1;
  max-width: 100%;
  margin: 0;
  padding: 0;
  width: 100%;
  box-sizing: border-box;
  display: flex;
}
</style> 