<template>
  <div class="manual-match-modal">
    <h3>Manual Matching</h3>
    <p>Enter the marketplace URL for this asset to match it manually.</p>
    <input v-model="url" type="text" placeholder="Enter URL..." class="url-input" />
    <div class="modal-actions">
      <button @click="closeModal" class="btn-secondary" :disabled="isMatching">Cancel</button>
      <button @click="performMatch" class="btn-primary" :disabled="isMatching">
        {{ isMatching ? 'Matching...' : 'Match' }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useModalStore } from '@/stores/modalStore';
import { useInspectorStore } from '@/stores/inspectorStore';
import { invoke } from '@tauri-apps/api/core';
import { useToast } from 'vue-toastification';
import { useAssetGridStore } from '@/stores/assetGridStore';

const props = defineProps<{
  assetId: number;
}>();

const modalStore = useModalStore();
const inspectorStore = useInspectorStore();
const toast = useToast();

const url = ref('');
const isMatching = ref(false);

const closeModal = () => {
  modalStore.closeModal();
};

const performMatch = async () => {
  if (!url.value) {
    toast.error('Please enter a URL');
    return;
  }

  isMatching.value = true;
  try {
    await invoke('match_asset_manually', { assetId: props.assetId, url: url.value });
    toast.success('Asset matched successfully!');
    
    // Refresh the inspector to show the new data
    inspectorStore.fetchAssetDetails(props.assetId);
    
    // Refresh the asset grid to show the updated matched state
    const assetGridStore = useAssetGridStore();
    assetGridStore.fetchAssets(assetGridStore.currentCategory, true);

    closeModal();
  } catch (error) {
    console.error('Manual match failed:', error);
    toast.error(`Match failed: ${error}`);
  } finally {
    isMatching.value = false;
  }
};
</script>

<style scoped>
.manual-match-modal {
  padding: 1.5rem;
  background-color: var(--background-secondary);
  border-radius: 8px;
}

.url-input {
  width: 100%;
  padding: 0.5rem;
  margin-block: 1rem;
  background-color: var(--background-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  color: var(--text-primary);
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
  margin-top: 1rem;
}
</style> 