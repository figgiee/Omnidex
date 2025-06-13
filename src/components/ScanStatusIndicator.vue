<template>
  <Transition name="slide-down">
    <div v-if="scanStore.isScanning && scanStore.scanProgress" class="scan-status-indicator">
      <div class="progress-container">
        <div class="spinner"></div>
        <div class="progress-info">
          <span class="progress-text">
            Scanning... {{ scanStore.scanProgress.processed_items.toLocaleString() }} / {{ scanStore.scanProgress.total_items.toLocaleString() }} items
          </span>
          <div class="progress-bar">
            <div 
              class="progress-fill" 
              :style="{ width: `${progressPercentage}%` }"
            ></div>
          </div>
          <span class="progress-percentage">{{ progressPercentage }}%</span>
        </div>
        <button @click="cancelScan" class="cancel-button" title="Cancel Scan">
          ✕
        </button>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { useScanLocationStore } from '@/stores/scanLocationStore';
import { onMounted, onUnmounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const scanStore = useScanLocationStore();

const progressPercentage = computed(() => {
  if (!scanStore.scanProgress || scanStore.scanProgress.total_items === 0) return 0;
  return Math.round((scanStore.scanProgress.processed_items / scanStore.scanProgress.total_items) * 100);
});

const cancelScan = async () => {
  try {
    const cancelledCount = await invoke<number>('cancel_all_scans');

  } catch (error) {
    console.error('Failed to cancel scans:', error);
  }
};

onMounted(() => {
  scanStore.initializeScanListener();
});

onUnmounted(() => {
  scanStore.destroyScanListener();
});
</script>

<style scoped>
.scan-status-indicator {
  background: var(--bg-surface);
  border-bottom: 1px solid #333;
  padding: 0.75rem 2rem;
  width: 100%;
}

.progress-container {
  display: flex;
  align-items: center;
  gap: 1rem;
  max-width: 1200px;
  margin: 0 auto;
}

.spinner {
  width: 20px;
  height: 20px;
  border: 2px solid var(--text-subtle);
  border-top-color: var(--accent);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  flex-shrink: 0;
}

.progress-info {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 1rem;
}

.progress-text {
  color: var(--text-primary);
  font-weight: 500;
  min-width: 200px;
}

.progress-bar {
  flex: 1;
  height: 6px;
  background: #333;
  border-radius: 3px;
  overflow: hidden;
  min-width: 200px;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--accent), #28a745);
  border-radius: 3px;
  transition: width 0.3s ease;
}

.progress-percentage {
  color: var(--text-secondary);
  font-weight: 500;
  min-width: 40px;
  text-align: right;
}

.cancel-button {
  background: transparent;
  border: 1px solid #666;
  color: var(--text-secondary);
  border-radius: 4px;
  padding: 0.25rem 0.5rem;
  cursor: pointer;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.cancel-button:hover {
  background: #ff4444;
  border-color: #ff4444;
  color: white;
}

/* Transition animations */
.slide-down-enter-active,
.slide-down-leave-active {
  transition: all 0.3s ease;
}

.slide-down-enter-from {
  transform: translateY(-100%);
  opacity: 0;
}

.slide-down-leave-to {
  transform: translateY(-100%);
  opacity: 0;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style> 