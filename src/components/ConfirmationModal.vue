<template>
  <div class="confirmation-modal">
    <h3>{{ title }}</h3>
    <p>{{ message }}</p>
    <div class="modal-actions">
      <AppButton @click="$emit('cancel')" variant="secondary">Cancel</AppButton>
      <AppButton @click="$emit('confirm')" :variant="confirmButtonVariant">{{ confirmButtonText }}</AppButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import AppButton from '@/components/AppButton.vue';

withDefaults(defineProps<{
  title: string;
  message: string;
  confirmButtonText?: string;
  confirmButtonVariant?: 'primary' | 'secondary' | 'danger';
}>(), {
  confirmButtonText: 'Confirm',
  confirmButtonVariant: 'primary',
});

defineEmits(['cancel', 'confirm']);
</script>

<style scoped>
.confirmation-modal {
  padding: 1.5rem;
  background-color: var(--bg-surface);
  color: var(--text-primary);
  border-radius: 8px;
  text-align: center;
  width: 400px;
  max-width: 90vw;
}

.confirmation-modal h3 {
  margin-top: 0;
  font-size: 1.25rem;
}

.confirmation-modal p {
  margin: 1rem 0;
}

.modal-actions {
  display: flex;
  justify-content: center;
  gap: 1rem;
  margin-top: 1.5rem;
}
</style> 