<template>
  <div class="star-rating" :aria-label="`Rating: ${rating} out of ${maxStars}`">
    <div
      v-for="star in maxStars"
      :key="star"
      class="star-container"
      :style="{ width: `${starSize}px`, height: `${starSize}px` }"
      @mousemove="handleMouseMove(star, $event)"
      @mouseleave="handleMouseLeave"
      @click="setRating(star)"
    >
      <!-- This is the 'background' or empty star -->
      <svg class="star-background" :width="starSize" :height="starSize" viewBox="0 0 24 24">
        <path :d="starPath" />
      </svg>
      <!-- This is the 'foreground' or filled star, clipped to show partials -->
      <svg class="star-foreground" :width="starSize" :height="starSize" viewBox="0 0 24 24" :style="{ clipPath: starClipPath(star) }">
        <path :d="starPath" />
      </svg>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';

const props = defineProps({
  rating: {
    type: Number,
    required: true,
  },
  maxStars: {
    type: Number,
    default: 5,
  },
  starSize: {
    type: Number,
    default: 24,
  },
  readOnly: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['update:rating']);

// A standard SVG path for a star
const starPath = "M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z";

const hoverRating = ref(0);

const displayRating = computed(() => {
  // If we are hovering, show the hover rating, otherwise show the prop rating
  return hoverRating.value > 0 ? hoverRating.value : props.rating;
});

// The magic for partial stars!
const starClipPath = (starIndex) => {
  const rating = displayRating.value;
  if (rating >= starIndex) {
    return 'inset(0 0 0 0)'; // Full star
  }
  if (rating > starIndex - 1 && rating < starIndex) {
    const percentage = (rating - (starIndex - 1)) * 100;
    return `inset(0 ${100 - percentage}% 0 0)`; // Partial star
  }
  return 'inset(0 100% 0 0)'; // Empty star (fully clipped)
};

// --- Interactivity Methods ---
const handleMouseMove = (starIndex, event) => {
  if (props.readOnly) return;
  // Calculate hover position within the star for half-star precision
  const boundingBox = event.currentTarget.getBoundingClientRect();
  const hoverPosition = (event.clientX - boundingBox.left) / boundingBox.width;
  hoverRating.value = starIndex - 1 + hoverPosition; // Gives a more granular hover effect
};

const handleMouseLeave = () => {
  if (props.readOnly) return;
  hoverRating.value = 0;
};

const setRating = (starIndex) => {
  if (props.readOnly) return;
  // Use the more precise hoverRating to set the value
  const finalRating = Math.ceil(hoverRating.value * 2) / 2; // Snap to nearest 0.5
  emit('update:rating', finalRating);
};
</script>

<style scoped>
.star-rating {
  display: inline-flex;
  align-items: center;
  gap: 4px; /* Space between stars */
}

.star-container {
  position: relative;
  cursor: pointer;
  display: inline-block;
  color: #c5c5c5; /* Color of the empty star */
}

.star-rating[aria-disabled="true"] .star-container,
.star-container:has(input[readonly]) {
    cursor: default;
}

.star-background {
  position: absolute;
  top: 0;
  left: 0;
  fill: currentColor;
}

.star-foreground {
  position: absolute;
  top: 0;
  left: 0;
  fill: #ffc700; /* Color of the filled star */
  transition: clip-path 0.1s ease-in-out;
}
</style> 