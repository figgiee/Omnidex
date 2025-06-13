<template>
  <div class="tag-pill">
    <span class="tag-text">{{ text }}</span>
    <button v-if="isEditable" class="remove-button" @click="$emit('remove')">
      ×
    </button>
  </div>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  text: string;
  isEditable?: boolean;
}>(), {
  isEditable: false,
});

defineEmits(['remove']);
</script>

<style scoped>
.tag-pill {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.25rem 0.75rem;
  border-radius: var(--border-radius-large);
  background-color: var(--bg-surface);
  color: var(--text-secondary);
  font-size: 0.875rem;
  font-weight: 500;
  border: 1px solid var(--text-subtle);
}

.tag-text {
  white-space: nowrap;
}

.remove-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 1rem;
  height: 1rem;
  border-radius: 50%;
  background-color: transparent;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0;
  font-size: 1rem;
  line-height: 1rem;
}

.remove-button:hover {
  background-color: #4f4f4f;
  color: var(--text-primary);
}
</style> 