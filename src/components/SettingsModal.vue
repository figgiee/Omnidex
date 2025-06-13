<template>
  <div class="settings-modal">
    <div class="modal-header">
      <h2>Application Settings</h2>
      <button @click="$emit('close')" class="close-button">&times;</button>
    </div>
    <div class="settings-content">
      <div class="settings-section">
        <h3>Scan Locations</h3>
        <div class="scan-all-container">
          <AppButton @click="scanAllLocations" :is-loading="scanStore.isScanning" :disabled="scanStore.isScanning">
            Scan All Locations
          </AppButton>
        </div>
        <div v-if="scanStore.isScanning || scanStore.scanProgress" class="scan-progress-container">
            <ScanProgress
              v-if="(scanStore.isScanning || scanStore.scanProgress) && scanStore.scanProgress"
              :is-scanning="scanStore.isScanning"
              :scan-progress="scanStore.scanProgress"
            />
            <AppButton v-if="!scanStore.isScanning && scanStore.scanProgress" @click="clearScanProgress" class="close-scan-progress">
                Close
            </AppButton>
        </div>
        <ul class="location-list">
          <li v-for="location in scanStore.scanLocations" :key="location.id">
            <span class="location-path" :title="location.path">{{ location.name }}</span>
            <div class="location-actions">
              <div class="recursive-toggle">
                <label :for="'recursive-' + location.id">Recursive</label>
                <ToggleSwitch
                  :id="'recursive-' + location.id"
                  :modelValue="location.scan_recursive || false"
                  @update:modelValue="handleRecursiveToggle(location)"
                />
              </div>
              <AppButton @click="rescanLocation(location.id)" :is-loading="scanStore.isScanning" :disabled="scanStore.isScanning" variant="secondary" size="small" title="Rescan this location">Rescan</AppButton>
              <AppButton @click="removeLocation(location.id)" :disabled="scanStore.isScanning" variant="danger" size="small" title="Remove this location">Remove</AppButton>
            </div>
          </li>
        </ul>
        <AppButton @click="addFolder" class="add-folder-button">Add Folder to Scan</AppButton>
      </div>

      <div class="settings-section">
        <h3>Data Management</h3>
        <p>Run offline processes to update your asset library.</p>
        <AppButton @click="showReprocessConfirm = true" :is-loading="isReprocessing" :disabled="isReprocessing">
          Reprocess All Orbital Data
        </AppButton>
         <p class="section-description">
          Forces the application to re-parse all locally cached marketplace data. This is useful for migrating data to new formats after an application update without re-downloading anything.
        </p>
      </div>

      <div class="settings-section">
        <h3>Recategorize Assets</h3>
        <div class="button-group">
          <AppButton @click="rescanCategories" :disabled="isProcessing">
            {{ isProcessing ? 'Processing...' : 'Recategorize Assets' }}
          </AppButton>
        </div>
        <p class="description">
          Manually re-assigns categories to all existing assets based on their metadata files. 
          Use this if categories seem incorrect after an update.
        </p>
      </div>

      <div class="settings-section danger-zone">
        <h3>Danger Zone</h3>
        <p>These actions are irreversible. Please be certain.</p>
        <AppButton @click="showWipeConfirmation = true" variant="danger">
          Clear All Data
        </AppButton>
      </div>
    </div>
    <Teleport to="body">
      <div v-if="showWipeConfirmation" class="modal-overlay">
        <ConfirmationModal
          title="Confirm Database Wipe"
          message="This will delete all assets and is irreversible. Are you sure?"
          confirm-button-text="Yes, Wipe Everything"
          confirm-button-variant="danger"
          @cancel="showWipeConfirmation = false"
          @confirm="wipeDatabase"
        />
      </div>
      <div v-if="showReprocessConfirm" class="modal-overlay">
        <ConfirmationModal
          title="Confirm Data Reprocessing"
          message="This will re-parse all local data for every asset. It may take some time and can't be undone. Continue?"
          confirm-button-text="Yes, Reprocess"
          @cancel="showReprocessConfirm = false"
          @confirm="reprocessData"
        />
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useScanLocationStore } from '@/stores/scanLocationStore';
import AppButton from '@/components/AppButton.vue';
import ToggleSwitch from '@/components/ToggleSwitch.vue';
import ConfirmationModal from '@/components/ConfirmationModal.vue';
import ScanProgress from '@/components/ScanProgress.vue';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { sendNotification } from '@tauri-apps/plugin-notification';
import { useToast } from 'vue-toastification';
import { useSourceListStore } from '@/stores/sourceListStore';

const scanStore = useScanLocationStore();
const emit = defineEmits(['close']);
const showWipeConfirmation = ref(false);
const isReprocessing = ref(false);
const showReprocessConfirm = ref(false);
const toast = useToast();
const sourceListStore = useSourceListStore();
const isProcessing = ref(false);

onMounted(() => {
  scanStore.fetchScanLocations();
});

const reprocessData = async () => {
  isReprocessing.value = true;
  showReprocessConfirm.value = false;
  try {
    const result = await invoke('reprocess_cached_orbital_data');
    await sendNotification({
        title: 'Processing Complete',
        body: result as string,
    });
  } catch (error) {
    console.error('Failed to reprocess orbital data:', error);
    await sendNotification({
        title: 'Processing Failed',
        body: 'Could not reprocess Orbital data. Check logs for details.',
    });
  } finally {
    isReprocessing.value = false;
  }
};

const wipeDatabase = async () => {
  try {
    const result = await invoke('wipe_entire_database');
    showWipeConfirmation.value = false;
    emit('close');
    // The backend will emit an event to trigger a reload.
  } catch (error) {
    console.error('Failed to wipe database:', error);
    showWipeConfirmation.value = false;
  }
};

const scanAllLocations = () => {
  scanStore.scanAll();
};

const rescanLocation = (id: number) => {
  if (id) {
    scanStore.startScan(id);
  }
};

const clearScanProgress = () => {
    scanStore.clearScanProgress();
}

const removeLocation = (id: number) => {
  if (id) {
    scanStore.deleteScanLocation(id);
  }
};

const handleRecursiveToggle = (location: any) => {
  scanStore.updateScanRecursive(location.id, !location.scan_recursive);
};

const addFolder = async () => {
  const result = await open({
    directory: true,
    multiple: false,
    title: 'Select Asset Folder',
  });

  if (typeof result === 'string') {
    const name = result.split(/[\\/]/).pop() || 'New Location';
    await scanStore.createScanLocation(name, result, true);
  }
};

const rescanCategories = async () => {
  isProcessing.value = true;
  try {
    const updatedCount: number = await invoke('post_process_asset_categories');
    toast.success(`Successfully updated ${updatedCount} asset categories. The view will now refresh.`);
    
    // Refresh the category counts in the source list
    await sourceListStore.fetchCategoryCounts();

  } catch (error) {
    console.error('Failed to rescan asset categories:', error);
    toast.error(`Error recategorizing assets: ${error}`);
  } finally {
    isProcessing.value = false;
  }
};
</script>

<style scoped>
.settings-modal {
  width: 600px;
  max-width: 90vw;
  background-color: var(--bg-surface);
  color: var(--text-primary);
  display: flex;
  flex-direction: column;
}

.modal-header {
  padding: 1rem 1.5rem;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.modal-header h2 {
  margin: 0;
  font-size: 1.25rem;
}

.close-button {
  background: none;
  border: none;
  color: var(--text-secondary);
  font-size: 1.5rem;
  cursor: pointer;
}

.settings-content {
  padding: 1.5rem;
  overflow-y: auto;
}

.settings-section h3 {
  margin: 0 0 1rem 0;
  color: var(--text-primary);
  font-size: 1.1rem;
  font-weight: 600;
}

.settings-section p {
  color: var(--text-secondary);
  margin-top: 0;
  margin-bottom: 1rem;
  font-size: 0.9rem;
  max-width: 100%; /* Ensure p tags don't cause overflow */
}

.settings-section .section-description {
    font-size: 0.8rem;
    color: var(--text-tertiary);
    margin-top: 0.5rem;
}

.scan-all-container {
  margin-bottom: 1.5rem;
  padding-bottom: 1.5rem;
  border-bottom: 1px solid var(--border-color);
}

.location-list {
  list-style: none;
  padding: 0;
  margin: 0 0 1.5rem 0;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.location-list li {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1rem;
  background-color: var(--bg-primary);
  border-radius: var(--border-radius-medium);
}

.location-path {
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 200px;
}

.location-actions {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.recursive-toggle {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.scan-progress-container {
    margin-bottom: 1.5rem;
}

.add-folder-button {
  width: 100%;
}

.danger-zone {
  border-top: 1px solid var(--border-color-danger);
  margin-top: 1.5rem;
  padding-top: 1.5rem;
}

.danger-zone h3 {
  color: var(--color-danger);
}

.danger-zone p {
  color: var(--text-secondary);
  margin-bottom: 1rem;
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.7);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.button-group {
  display: flex;
  gap: 1rem;
}

.description {
  font-size: 0.9rem;
  color: var(--text-secondary);
  margin-top: 0.25rem;
}
</style> 