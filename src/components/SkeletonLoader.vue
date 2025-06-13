<template>
  <div
    class="skeleton-loader"
    :style="{
      width: width,
      height: height,
      borderRadius: borderRadius
    }"
  ></div>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  width?: string;
  height?: string;
  borderRadius?: string;
}>(), {
  width: '100%',
  height: '1rem',
  borderRadius: '0.25rem',
});
</script>

<style scoped>
.skeleton-loader {
  background-color: var(--bg-surface);
  position: relative;
  overflow: hidden;
}

.skeleton-loader::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.1),
    transparent
  );
  animation: shimmer 1.5s infinite linear;
}

@keyframes shimmer {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}
</style> 