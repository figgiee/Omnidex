<template>
  <div class="draggable-divider" @mousedown.prevent="onMouseDown"></div>
</template>

<script setup lang="ts">
const emit = defineEmits<{
  (e: 'drag', movementX: number): void
}>()

const onMouseDown = (event: MouseEvent) => {
  const onMouseMove = (moveEvent: MouseEvent) => {
    emit('drag', moveEvent.movementX)
  }

  const onMouseUp = () => {
    document.removeEventListener('mousemove', onMouseMove)
    document.removeEventListener('mouseup', onMouseUp)
  }

  document.addEventListener('mousemove', onMouseMove)
  document.addEventListener('mouseup', onMouseUp)
}
</script>

<style scoped>
.draggable-divider {
  width: 5px;
  cursor: col-resize;
  background-color: #333;
  flex-shrink: 0;
  user-select: none;
  transition: background-color 0.2s ease;
}

.draggable-divider:hover {
  background-color: #555;
}

.draggable-divider:active {
  background-color: #007bff;
}
</style> 