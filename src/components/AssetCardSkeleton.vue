<template>
  <div class="asset-card-skeleton">
    <div class="thumbnail-skeleton">
      <SkeletonLoader height="120px" />
    </div>
    <div class="info-skeleton">
      <SkeletonLoader height="1.2rem" width="80%" style="margin-bottom: 0.75rem;" />
      <SkeletonLoader height="0.9rem" width="60%" style="margin-bottom: 0.5rem;" />
      <SkeletonLoader height="0.8rem" width="40%" />
    </div>
  </div>
</template>

<script setup lang="ts">
import SkeletonLoader from '@/components/SkeletonLoader.vue';
</script>

<style scoped>
.asset-card-skeleton {
  display: flex;
  flex-direction: column;
  border-radius: var(--border-radius-medium);
  background-color: var(--bg-surface);
  padding: 1rem;
  border: 1px solid #333;
}

.thumbnail-skeleton {
  border-radius: var(--border-radius-small);
  margin-bottom: 1rem;
  overflow: hidden;
}

.info-skeleton {
  display: flex;
  flex-direction: column;
}
</style> 