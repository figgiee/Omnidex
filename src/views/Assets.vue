<template>
  <div class="asset-view-container">
    <div class="pane left-pane" :style="{ width: `${leftPaneWidth}px` }">
      <SourceList />
    </div>
    <DraggableDivider @drag="onDragLeft" />
    <div class="pane middle-pane">
      <AssetGrid />
      <ActionBar />
    </div>
    <DraggableDivider @drag="onDragRight" />
    <div class="pane right-pane" :style="{ width: `${rightPaneWidth}px` }">
      <InspectorPanel />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import DraggableDivider from '@/components/DraggableDivider.vue';
import SourceList from '@/components/SourceList.vue';
import AssetGrid from '@/components/AssetGrid.vue';
import InspectorPanel from '@/components/InspectorPanel.vue';
import ActionBar from '@/components/ActionBar.vue';
import { useSourceListStore } from '@/stores/sourceListStore';

const sourceListStore = useSourceListStore();

const leftPaneWidth = ref(250);
const lastLeftPaneWidth = ref(leftPaneWidth.value);
const rightPaneWidth = ref(350);

watch(
  () => sourceListStore.isPanelOpen,
  (isOpen) => {
    if (isOpen) {
      leftPaneWidth.value = lastLeftPaneWidth.value > 60 ? lastLeftPaneWidth.value : 250;
    } else {
      lastLeftPaneWidth.value = leftPaneWidth.value;
      leftPaneWidth.value = 60;
    }
  }
);

const onDragLeft = (movementX: number) => {
  if (sourceListStore.isPanelOpen) {
    leftPaneWidth.value += movementX;
  }
};

const onDragRight = (movementX: number) => {
  rightPaneWidth.value -= movementX;
};
</script>

<style scoped>
.asset-view-container {
  display: flex;
  height: 100%;
  width: 100%;
  overflow: hidden;
}

.pane {
  height: 100%;
  overflow: auto;
  padding: 1rem;
  color: white;
}

.left-pane {
  background-color: #1e1e1e;
  flex-shrink: 0;
  transition: width 0.3s ease;
}

.middle-pane {
  background-color: var(--bg-primary);
  flex-grow: 1;
  padding: 0;
  position: relative;
}

.right-pane {
  background-color: #1e1e1e;
  flex-shrink: 0;
}
</style> 