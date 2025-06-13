<template>
  <div class="delete-confirmation-modal">
    <h3>Delete Items</h3>
    <p>Are you sure you want to delete the selected items? This action cannot be undone.</p>
    <div class="modal-actions">
      <button @click="closeModal" class="btn-secondary">Cancel</button>
      <button @click="confirmDelete" class="btn-danger">Delete</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useModalStore } from '@/stores/modalStore';

const modalStore = useModalStore();

const closeModal = () => {
  modalStore.closeModal();
};

const confirmDelete = () => {
  // Placeholder for delete logic
  
  closeModal();
};
</script>

<style scoped>
.delete-confirmation-modal {
  padding: 1.5rem;
  background-color: var(--background-secondary);
  border-radius: 8px;
  text-align: center;
}

.modal-actions {
  display: flex;
  justify-content: center;
  gap: 1rem;
  margin-top: 1.5rem;
}

.btn-danger {
    background-color: var(--color-danger);
    color: white;
}
</style> 