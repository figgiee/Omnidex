<template>
  <transition name="fade">
    <div v-if="modalStore.modal.component" class="modal-backdrop" @click.self="close">
      <transition name="slide-up">
        <div v-if="modalStore.modal.component" class="modal-container">
          <component :is="modalStore.modal.component" v-bind="modalStore.modal.props" @close="close" />
        </div>
      </transition>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { useModalStore } from '@/stores/modalStore';

const modalStore = useModalStore();

const close = () => {
  modalStore.closeModal();
};
</script>

<style scoped>
.modal-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.6);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-container {
  background-color: var(--bg-primary);
  border-radius: var(--border-radius-large);
  box-shadow: 0 10px 25px -5px rgba(0,0,0,0.1), 0 10px 10px -5px rgba(0,0,0,0.04);
  max-width: 90vw;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.slide-up-enter-active,
.slide-leave-active {
  transition: transform 0.3s ease;
}

.slide-up-enter-from,
.slide-leave-to {
  transform: translateY(20px);
}
</style> 