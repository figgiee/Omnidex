<template>
  <div class="tagging-modal">
    <h3>Add Tags</h3>
    <p>Add tags to the selected items. Separate tags with commas.</p>
    <input type="text" placeholder="e.g. character, sci-fi" class="tag-input" />
    <div class="modal-actions">
      <button @click="closeModal" class="btn-secondary">Cancel</button>
      <button @click="applyTags" class="btn-primary">Apply</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useModalStore } from '@/stores/modalStore';

const modalStore = useModalStore();

const closeModal = () => {
  modalStore.closeModal();
};

const applyTags = () => {
  // Placeholder for tagging logic
  
  closeModal();
};
</script>

<style scoped>
.tagging-modal {
  padding: 1.5rem;
  background-color: var(--background-secondary);
  border-radius: 8px;
}

.tag-input {
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