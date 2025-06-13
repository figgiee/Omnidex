<template>
  <div class="scan-locations-view">
    <div class="scan-locations-header">
      <h1>Scan Locations</h1>
      <p>Manage directories to scan for assets</p>
    </div>

    <!-- Add New Scan Location -->
    <div class="card mb-3">
      <div class="card-header">
        <h2 class="card-title">Add New Scan Location</h2>
      </div>
      <div class="card-body">
        <form @submit.prevent="handleAddLocation" class="add-location-form">
          <div class="form-row">
            <div class="form-group flex-1">
              <label class="form-label">Location Name</label>
              <input 
                v-model="newLocation.name"
                type="text" 
                class="form-input" 
                placeholder="e.g., My UE Assets"
                required
              />
            </div>
            <div class="form-group flex-1">
              <label class="form-label">Directory Path</label>
              <div class="path-input-group">
                <input 
                  v-model="newLocation.path"
                  type="text" 
                  class="form-input" 
                  placeholder="Type or paste directory path, or use Browse button..."
                  required
                />
                <button 
                  type="button" 
                  @click="selectDirectory"
                  class="btn btn-secondary"
                  :disabled="loading"
                >
                  Browse
                </button>
              </div>
            </div>
          </div>
          
          <div class="form-row">
            <div class="form-group">
              <div class="form-checkbox">
                <input 
                  v-model="newLocation.scanRecursive"
                  type="checkbox" 
                  id="recursive"
                />
                <label for="recursive">Scan subdirectories recursively</label>
              </div>
            </div>
            <div class="form-group flex-1">
              <label class="form-label">File Extensions (optional)</label>
              <input 
                v-model="newLocation.fileExtensions"
                type="text" 
                class="form-input" 
                placeholder="e.g., uasset,umap,fbx (leave empty for all)"
              />
            </div>
          </div>

          <div class="form-group">
            <label class="form-label">Description (optional)</label>
            <input 
              v-model="newLocation.description"
              type="text" 
              class="form-input" 
              placeholder="Brief description of this location"
            />
          </div>

          <div class="form-actions">
            <button 
              type="submit" 
              class="btn btn-primary"
              :disabled="loading || !newLocation.name || !newLocation.path"
            >
              <span v-if="loading">Loading...</span>
              <span v-else>Add Location</span>
            </button>
            <button 
              type="button" 
              @click="resetForm"
              class="btn btn-secondary"
            >
              Clear
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Existing Scan Locations -->
    <div class="card">
      <div class="card-header">
        <h2 class="card-title">Scan Locations</h2>
                  <button 
            @click="refreshLocations"
            class="btn btn-secondary"
            :disabled="loading"
          >
            Refresh
          </button>
      </div>
      <div class="card-body">
        <div v-if="loading && scanLocations.length === 0" class="loading-state">
          <div class="spinner"></div>
          <p>Loading scan locations...</p>
        </div>

        <div v-else-if="scanLocations.length === 0" class="empty-state">
          <div class="empty-icon"></div>
          <h3>No scan locations configured</h3>
          <p>Add your first scan location above to get started indexing your assets.</p>
        </div>

        <div v-else class="locations-list">
          <div 
            v-for="location in scanLocations" 
            :key="location.id"
            class="location-item"
          >
            <div class="location-info">
              <div class="location-header">
                <h4>{{ location.name }}</h4>
                <div class="location-status">
                  <span 
                    class="status-badge"
                    :class="{ active: location.is_active }"
                  >
                    {{ location.is_active ? 'Active' : 'Inactive' }}
                  </span>
                </div>
              </div>
              
              <div class="location-details">
                <div class="detail-item">
                  <span class="detail-label">Path:</span>
                  <span class="detail-value">{{ location.path }}</span>
                </div>
                <div class="detail-item">
                  <span class="detail-label">Recursive:</span>
                  <span class="detail-value">{{ location.scan_recursive ? 'Yes' : 'No' }}</span>
                </div>
                <div v-if="location.file_extensions" class="detail-item">
                  <span class="detail-label">Extensions:</span>
                  <span class="detail-value">{{ location.file_extensions }}</span>
                </div>
                <div v-if="location.last_scanned" class="detail-item">
                  <span class="detail-label">Last Scan:</span>
                  <span class="detail-value">{{ formatDate(location.last_scanned) }}</span>
                </div>
                <div v-if="location.description" class="detail-item">
                  <span class="detail-label">Description:</span>
                  <span class="detail-value">{{ location.description }}</span>
                </div>
              </div>
            </div>

            <div class="location-actions">
              <button 
                @click="startScan(location.id!)"
                class="btn btn-success"
                :disabled="isScanning || !location.is_active"
              >
                {{ isScanning ? 'Scanning...' : 'Start Scan' }}
              </button>
              <button 
                v-if="location.scan_recursive"
                @click="fixRecursiveScanning(location.id!)"
                class="btn btn-warning"
                :disabled="loading || isScanning"
                title="This scan location is set to recursive. Click to disable recursive scanning for top-level assets only."
              >
                Fix Recursive
              </button>
              <button 
                @click="clearLocationAssets(location.id!)"
                class="btn btn-danger"
                :disabled="loading || isScanning"
                title="Clear all assets from this scan location and re-scan"
              >
                Clear & Rescan
              </button>
              <button 
                @click="toggleLocationStatus(location.id!)"
                class="btn btn-secondary"
                :disabled="loading"
              >
                {{ location.is_active ? 'Disable' : 'Enable' }}
              </button>
              <button 
                @click="deleteLocation(location.id!)"
                class="btn btn-danger"
                :disabled="loading || isScanning"
              >
                Delete
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Scan Progress -->
    <div v-if="scanProgress" class="card mt-3">
      <div class="card-header">
        <h2 class="card-title">Scan Progress</h2>
      </div>
      <div class="card-body">
        <div class="progress-content">
          <div class="progress">
            <div 
              class="progress-bar" 
              :style="{ width: `${(scanProgress.processed_items / scanProgress.total_items) * 100}%` }"
            ></div>
          </div>
          <div class="progress-info">
            <p>{{ scanProgress.processed_items }} / {{ scanProgress.total_items }} items processed</p>
            <small v-if="scanProgress.current_path">{{ scanProgress.current_path }}</small>
            <div v-if="scanProgress.error" class="progress-errors">
              <p class="error-count">Error: {{ scanProgress.error }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Error Display -->
    <div v-if="error" class="card mt-3 error-card">
      <div class="card-header">
        <h2 class="card-title">Error</h2>
      </div>
      <div class="card-body">
        <p>{{ error }}</p>
        <button @click="clearError" class="btn btn-secondary">Dismiss</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, onUnmounted } from 'vue'
import { useScanLocationStore } from '@/stores/scanLocationStore'
import { useSettingsStore } from '@/stores/settingsStore'
import { open } from '@tauri-apps/plugin-dialog'
import { storeToRefs } from 'pinia'

const scanLocationStore = useScanLocationStore()
const settingsStore = useSettingsStore()
const { scanLocations, loading, error, scanProgress, isScanning } = storeToRefs(scanLocationStore)

interface NewLocation {
  name: string;
  path: string;
  scanRecursive: boolean;
  fileExtensions?: string;
  description?: string;
}

const newLocation = ref<NewLocation>({
  name: '',
  path: '',
  scanRecursive: true,
  fileExtensions: '',
  description: ''
})

// Computed properties
const formatDate = (dateString: string | undefined) => {
  if (!dateString) return 'N/A'
  const date = new Date(dateString)
  return `${date.toLocaleDateString()} ${date.toLocaleTimeString()}`
}

// Methods
const selectDirectory = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
    });
    if (typeof selected === 'string') {
      newLocation.value.path = selected
      if (!newLocation.value.name) {
        newLocation.value.name = selected.split(/[\\/]/).pop() || 'New Location'
      }
    }
  } catch (err) {
    console.error('Directory selection failed:', err)
  }
}

const resetForm = () => {
  newLocation.value = {
    name: '',
    path: '',
    scanRecursive: true,
    fileExtensions: '',
    description: ''
  }
}

const handleAddLocation = async () => {
  if (loading.value || !newLocation.value.name || !newLocation.value.path) return;
  try {
    await scanLocationStore.createScanLocation(
      newLocation.value.name,
      newLocation.value.path,
      newLocation.value.scanRecursive,
      newLocation.value.fileExtensions,
      newLocation.value.description
    )
    resetForm()
  } catch (err) {
    // Error is handled in the store
  }
}

const startScan = async (locationId: number) => {
  await scanLocationStore.startScan(locationId);
};

const clearLocationAssets = async (locationId: number) => {
  if (confirm('Are you sure you want to clear all assets from this location and re-scan? This cannot be undone.')) {
    await scanLocationStore.clearAssetsFromScanLocation(locationId);
    await scanLocationStore.startScan(locationId);
  }
};

const deleteLocation = async (locationId: number) => {
  if (confirm('Are you sure you want to delete this scan location? This cannot be undone.')) {
    await scanLocationStore.deleteScanLocation(locationId);
  }
};

const toggleLocationStatus = async (locationId: number) => {
  const location = scanLocations.value.find((loc) => loc.id === locationId);
  if (location) {
    await scanLocationStore.updateScanLocation(
      locationId,
      location.name,
      location.path,
      !location.is_active,
      location.scan_recursive || false,
      location.file_extensions,
      location.description
    );
  }
};

const fixRecursiveScanning = async (locationId: number) => {
  const location = scanLocations.value.find((loc) => loc.id === locationId);
  if (location?.scan_recursive) {
    await scanLocationStore.updateScanRecursive(locationId, false);
  }
};

const refreshLocations = () => {
  scanLocationStore.fetchScanLocations();
};

const clearError = () => {
  scanLocationStore.clearError();
};

// Lifecycle
onMounted(() => {
  scanLocationStore.fetchScanLocations()
  scanLocationStore.initializeScanListener()
})

onUnmounted(() => {
  scanLocationStore.destroyScanListener()
})
</script>

<style scoped>
.scan-locations-view {
  max-width: 1200px;
  margin: 0 auto;
}

.scan-locations-header {
  text-align: center;
  margin-bottom: 2rem;
}

.scan-locations-header h1 {
  color: white;
  margin-bottom: 0.5rem;
}

.scan-locations-header p {
  color: rgba(255, 255, 255, 0.8);
  font-size: 1.1rem;
}

/* Form Styles */
.add-location-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.form-row {
  display: flex;
  gap: 1rem;
  align-items: end;
}

.flex-1 {
  flex: 1;
}

.path-input-group {
  display: flex;
  gap: 0.5rem;
}

.path-input-group .form-input {
  flex: 1;
}

.form-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-start;
}

/* Card Header with Actions */
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

/* Loading and Empty States */
.loading-state,
.empty-state {
  text-align: center;
  padding: 3rem 1rem;
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
}

.empty-state {
  color: rgba(255, 255, 255, 0.8);
}

.empty-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
}

.empty-state h3 {
  color: white;
  margin-bottom: 0.5rem;
}

/* Locations List */
.locations-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.location-item {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 1.5rem;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 0.75rem;
  border: 1px solid rgba(255, 255, 255, 0.1);
  transition: all 0.3s ease;
}

.location-item:hover {
  background: rgba(255, 255, 255, 0.08);
  border-color: rgba(255, 255, 255, 0.2);
}

.location-info {
  flex: 1;
  margin-right: 1rem;
}

.location-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.location-header h4 {
  margin: 0;
  color: white;
  font-size: 1.1rem;
  font-weight: 600;
}

.location-status {
  display: flex;
  align-items: center;
}

.status-badge {
  padding: 0.25rem 0.75rem;
  border-radius: 1rem;
  font-size: 0.8rem;
  font-weight: 500;
  background: rgba(255, 255, 255, 0.1);
  color: rgba(255, 255, 255, 0.7);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.status-badge.active {
  background: rgba(81, 207, 102, 0.2);
  color: #51cf66;
  border-color: rgba(81, 207, 102, 0.3);
}

.location-details {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.detail-item {
  display: flex;
  gap: 0.5rem;
}

.detail-label {
  font-weight: 500;
  color: rgba(255, 255, 255, 0.8);
  min-width: 80px;
}

.detail-value {
  color: rgba(255, 255, 255, 0.9);
  word-break: break-all;
}

.location-actions {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  min-width: 120px;
}

.location-actions .btn {
  justify-content: center;
  font-size: 0.9rem;
  padding: 0.5rem 1rem;
}

.btn-warning {
  background: rgba(255, 193, 7, 0.2);
  color: #ffc107;
  border-color: rgba(255, 193, 7, 0.3);
}

.btn-warning:hover:not(:disabled) {
  background: rgba(255, 193, 7, 0.3);
  border-color: rgba(255, 193, 7, 0.5);
}

/* Progress Content */
.progress-content {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.progress-info p {
  margin: 0;
  color: white;
  font-weight: 500;
}

.progress-info small {
  color: rgba(255, 255, 255, 0.7);
  font-size: 0.8rem;
  word-break: break-all;
}

.progress-errors {
  margin-top: 0.5rem;
}

.error-count {
  color: #ff6b6b;
  font-weight: 500;
  margin: 0;
}

/* Error Card */
.error-card {
  border-color: rgba(255, 107, 107, 0.3);
  background: rgba(255, 107, 107, 0.1);
}

.error-card .card-title {
  color: #ff6b6b;
}

.error-card .card-body p {
  color: rgba(255, 255, 255, 0.9);
  margin-bottom: 1rem;
}

/* Responsive Design */
@media (max-width: 768px) {
  .form-row {
    flex-direction: column;
    align-items: stretch;
  }
  
  .location-item {
    flex-direction: column;
    gap: 1rem;
  }
  
  .location-info {
    margin-right: 0;
  }
  
  .location-actions {
    flex-direction: row;
    min-width: auto;
  }
  
  .detail-item {
    flex-direction: column;
    gap: 0.25rem;
  }
  
  .detail-label {
    min-width: auto;
    font-size: 0.9rem;
  }
}
</style> 