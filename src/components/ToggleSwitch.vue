<template>
  <div class="toggle-container">
    <label class="toggle-switch" :for="id">
      <input
        :id="id"
        type="checkbox"
        :checked="modelValue"
        @change="$emit('update:modelValue', ($event.target as HTMLInputElement).checked)"
        :disabled="disabled"
      />
      <span class="slider"></span>
    </label>
    <div class="toggle-info" v-if="label || description">
      <label :for="id" class="toggle-label" v-if="label">
        {{ label }}
      </label>
      <p class="toggle-description" v-if="description">
        {{ description }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  id: string
  modelValue: boolean
  label?: string
  description?: string
  disabled?: boolean
}

defineProps<Props>()

defineEmits<{
  'update:modelValue': [value: boolean]
}>()
</script>

<style scoped>
.toggle-container {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
}

.toggle-switch {
  position: relative;
  display: inline-block;
  width: 50px;
  height: 28px;
  flex-shrink: 0;
  margin-top: 2px;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.2);
  transition: all 0.3s ease;
  border-radius: 28px;
  border: 1px solid rgba(255, 255, 255, 0.3);
}

.slider:before {
  position: absolute;
  content: "";
  height: 20px;
  width: 20px;
  left: 3px;
  bottom: 3px;
  background: white;
  transition: all 0.3s ease;
  border-radius: 50%;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

input:checked + .slider {
  background: linear-gradient(45deg, #667eea, #764ba2);
  border-color: rgba(255, 255, 255, 0.5);
}

input:focus + .slider {
  box-shadow: 0 0 0 3px rgba(255, 255, 255, 0.2);
}

input:checked + .slider:before {
  transform: translateX(22px);
}

input:disabled + .slider {
  opacity: 0.5;
  cursor: not-allowed;
}

input:disabled + .slider:before {
  opacity: 0.7;
}

.toggle-info {
  flex: 1;
}

.toggle-label {
  display: block;
  font-weight: 500;
  font-size: 1rem;
  color: var(--color-text-primary);
  margin-bottom: 0.25rem;
  cursor: pointer;
  transition: color 0.2s ease;
}

.toggle-label:hover {
  color: var(--color-text-headings);
}

.toggle-description {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  margin: 0;
  line-height: 1.4;
}
</style> 