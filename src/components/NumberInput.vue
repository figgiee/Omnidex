<template>
  <div class="number-input-container">
    <div class="input-wrapper">
      <input
        :id="id"
        type="number"
        :value="modelValue"
        @input="handleInput"
        :min="min"
        :max="max"
        :step="step"
        :disabled="disabled"
        class="number-input"
      />
      <span v-if="suffix" class="suffix">{{ suffix }}</span>
    </div>
    <div class="input-info" v-if="label || description">
      <label :for="id" class="input-label" v-if="label">
        {{ label }}
      </label>
      <p class="input-description" v-if="description">
        {{ description }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
	id: string;
	modelValue: number;
	label?: string;
	description?: string;
	min?: number;
	max?: number;
	step?: number;
	suffix?: string;
	disabled?: boolean;
}

defineProps<Props>();

const emit = defineEmits<{
	"update:modelValue": [value: number];
}>();

// biome-ignore lint/correctness/noUnusedVariables: handleInput is used in template @input event
function handleInput(event: Event) {
	const target = event.target as HTMLInputElement;
	const value = parseInt(target.value, 10);
	if (!Number.isNaN(value)) {
		emit("update:modelValue", value);
	}
}
</script>

<style scoped>
.number-input-container {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

.number-input {
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.3);
  border-radius: var(--border-radius-medium);
  padding: 0.75rem 1rem;
  color: var(--color-text-primary);
  font-size: 1rem;
  width: 120px;
  transition: all 0.3s ease;
}

.number-input:focus {
  outline: none;
  border-color: rgba(255, 255, 255, 0.5);
  background: rgba(255, 255, 255, 0.15);
  box-shadow: 0 0 0 3px rgba(255, 255, 255, 0.1);
}

.number-input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.number-input::-webkit-outer-spin-button,
.number-input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.number-input[type=number] {
  -moz-appearance: textfield;
}

.suffix {
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  margin-left: 0.5rem;
  white-space: nowrap;
}

.input-label {
  display: block;
  font-weight: 500;
  font-size: 1rem;
  color: var(--color-text-primary);
  margin-bottom: 0.25rem;
}

.input-description {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  margin: 0;
  line-height: 1.4;
}
</style> 