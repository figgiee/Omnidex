<template>
  <div class="select-wrapper">
    <select 
      class="app-select" 
      :value="modelValue"
      @change="handleChange"
      v-bind="$attrs"
    >
      <slot></slot>
    </select>
    <div class="select-arrow">▼</div>
  </div>
</template>

<script setup lang="ts">
defineOptions({
  inheritAttrs: false,
});

interface Props {
  modelValue?: string | number;
}

interface Emits {
  (e: 'update:modelValue', value: string): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const handleChange = (event: Event) => {
  const target = event.target as HTMLSelectElement;
  emit('update:modelValue', target.value);
};
</script>

<style scoped>
.select-wrapper {
  position: relative;
  display: inline-block;
}

.app-select {
  -webkit-appearance: none;
  -moz-appearance: none;
  appearance: none;
  padding: 0.5rem 2rem 0.5rem 1rem;
  border-radius: var(--border-radius-medium);
  border: 1px solid var(--text-subtle);
  background-color: var(--bg-surface);
  color: var(--text-primary);
  font-weight: 500;
  cursor: pointer;
  width: 100%;
}

.app-select:focus {
  outline: none;
  border-color: var(--accent);
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.3);
}

.select-arrow {
  position: absolute;
  top: 50%;
  right: 0.75rem;
  transform: translateY(-50%);
  pointer-events: none;
  color: var(--text-secondary);
  font-size: 0.75rem;
}
</style> 