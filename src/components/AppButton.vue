<template>
  <button :class="['app-button', `variant-${variant}`]" :disabled="disabled">
    <slot></slot>
  </button>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  variant?: 'primary' | 'secondary' | 'icon' | 'danger' | 'ghost' | 'link';
  disabled?: boolean;
}>(), {
  variant: 'primary',
  disabled: false,
});
</script>

<style scoped>
.app-button {
  padding: 0.5rem 1rem;
  border-radius: var(--border-radius-medium);
  border: 1px solid transparent;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  user-select: none;
}

.app-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Primary Variant */
.variant-primary {
  background-color: var(--accent);
  color: var(--text-primary);
}

.variant-primary:not(:disabled):hover {
  filter: brightness(1.2);
}

/* Secondary Variant */
.variant-secondary {
  background-color: var(--bg-surface);
  color: var(--text-primary);
  border-color: var(--text-subtle);
}

.variant-secondary:not(:disabled):hover {
  background-color: #3e3e3e;
  border-color: var(--text-secondary);
}

/* Icon Variant */
.variant-icon {
  background-color: transparent;
  border-color: transparent;
  color: var(--text-secondary);
  padding: 0.5rem;
  border-radius: 50%;
}

.variant-icon:not(:disabled):hover {
  background-color: var(--bg-surface);
  color: var(--text-primary);
}

/* Danger Variant */
.variant-danger {
  background-color: #dc3545;
  color: white;
  border-color: #dc3545;
}

.variant-danger:not(:disabled):hover {
  background-color: #c82333;
  border-color: #bd2130;
}

/* Ghost Variant */
.variant-ghost {
  background-color: transparent;
  border-color: transparent;
  color: var(--text-secondary);
}

.variant-ghost:not(:disabled):hover {
  background-color: var(--bg-surface);
  color: var(--text-primary);
}

/* Link Variant */
.variant-link {
  background-color: transparent;
  border-color: transparent;
  color: var(--accent);
  padding: 0;
  text-decoration: none;
}

.variant-link:not(:disabled):hover {
  text-decoration: underline;
}
</style> 