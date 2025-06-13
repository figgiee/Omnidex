<template>
  <transition name="modal-fade">
    <div v-if="isOpen" class="modal-overlay" @click.self="close">
      <div class="modal-dialog">
        <header class="modal-header">
          <h2 class="modal-title">{{ title }}</h2>
          <button class="close-button" @click="close">&times;</button>
        </header>
        <section class="modal-body">
          <slot></slot>
        </section>
        <footer class="modal-footer">
          <slot name="footer">
            <AppButton @click="close">Close</AppButton>
          </slot>
        </footer>
      </div>
    </div>
  </transition>
</template>

<script setup lang="ts">
import AppButton from '@/components/AppButton.vue';

interface Props {
  isOpen: boolean;
  title: string;
}

const props = defineProps<Props>();
const emit = defineEmits(['close']);

const close = () => {
  emit('close');
};
</script>

<style scoped>
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

.modal-dialog {
  background: var(--bg-surface);
  border-radius: var(--border-radius-large);
  box-shadow: var(--shadow-strong);
  width: 90%;
  max-width: 600px;
  display: flex;
  flex-direction: column;
}

.modal-header {
  padding: 1.5rem;
  border-bottom: 1px solid #333;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.modal-title {
  margin: 0;
  font-size: 1.5rem;
  color: var(--text-primary);
}

.close-button {
  background: none;
  border: none;
  color: var(--text-secondary);
  font-size: 2rem;
  cursor: pointer;
}

.modal-body {
  padding: 1.5rem;
  overflow-y: auto;
}

.modal-footer {
  padding: 1.5rem;
  border-top: 1px solid #333;
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
}

.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
</style> 