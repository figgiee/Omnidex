<template>
  <div class="global-search-container">
    <input
      type="text"
      class="global-search-input"
      placeholder="Search all assets..."
      v-model="query"
      @input="onInput"
    />
    <div v-if="query" class="clear-button" @click="clearSearch">×</div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useDebounceFn } from '@vueuse/core';

const query = ref('');

const emit = defineEmits<{
  (e: 'search', query: string): void;
}>();

const debouncedSearch = useDebounceFn((searchQuery: string) => {
  emit('search', searchQuery);
}, 300);

const onInput = () => {
  debouncedSearch(query.value);
};

const clearSearch = () => {
  query.value = '';
  emit('search', '');
};
</script>

<style scoped>
.global-search-container {
  position: relative;
  width: 100%;
  max-width: 600px;
}

.global-search-input {
  width: 100%;
  padding: 0.5rem 2.5rem 0.5rem 1rem;
  border-radius: var(--border-radius-medium);
  border: 1px solid var(--text-subtle);
  background-color: var(--bg-surface);
  color: var(--text-primary);
  font-size: 1rem;
  transition: all 0.2s ease;
}

.global-search-input:focus {
  outline: none;
  border-color: var(--accent);
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.3);
}

.clear-button {
  position: absolute;
  top: 50%;
  right: 0.75rem;
  transform: translateY(-50%);
  cursor: pointer;
  color: var(--text-secondary);
  font-size: 1.25rem;
}
</style> 