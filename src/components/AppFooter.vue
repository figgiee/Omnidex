<template>
  <footer class="app-footer">
    <transition name="slide-fade">
      <div v-if="statusBar.isVisible" class="status-bar-container" :class="{ 'complete': statusBar.isComplete }">
        <div class="status-message">{{ statusBar.message }}</div>
        <div v-if="!statusBar.isComplete && statusBar.total > 0" class="progress-bar">
          <div class="progress-bar-inner" :style="{ width: `${(statusBar.progress / statusBar.total) * 100}%` }"></div>
        </div>
        <div v-if="!statusBar.isComplete && statusBar.total > 0" class="status-progress-text">
          {{ statusBar.progress }} / {{ statusBar.total }}
        </div>
      </div>
    </transition>
  </footer>
</template>

<script setup lang="ts">
import { useStatusBarStore } from '@/stores/statusBarStore';

const statusBar = useStatusBarStore();
</script>

<style scoped>
.app-footer {
  height: var(--app-footer-height);
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--bg-primary);
  border-top: 1px solid var(--border-color);
  position: relative;
  overflow: hidden;
}

.status-bar-container {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 100%;
  background-color: var(--accent);
  color: white;
  display: flex;
  align-items: center;
  padding: 0 1.5rem;
  gap: 1rem;
  transition: background-color 0.3s ease;
}

.status-bar-container.complete {
  background-color: #28a745; /* Green for success */
}

.status-message {
  font-weight: 500;
}

.progress-bar {
  flex-grow: 1;
  height: 10px;
  background-color: rgba(255, 255, 255, 0.3);
  border-radius: 5px;
  overflow: hidden;
}

.progress-bar-inner {
  height: 100%;
  background-color: white;
  transition: width 0.2s linear;
}

.status-progress-text {
  font-size: 0.9rem;
  min-width: 80px;
  text-align: right;
}

/* Animation */
.slide-fade-enter-active,
.slide-fade-leave-active {
  transition: transform 0.5s cubic-bezier(0.25, 0.46, 0.45, 0.94), opacity 0.5s ease;
}

.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateY(100%);
  opacity: 0;
}
</style> 