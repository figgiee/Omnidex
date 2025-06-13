<template>
  <div 
    class="slideshow-modal-backdrop" 
    @click.self="close"
    role="dialog"
    aria-modal="true"
    aria-labelledby="slideshow-title"
    aria-describedby="slideshow-description"
  >
    <div class="slideshow-modal-content">
      <!-- Header with counter and close button -->
      <div class="slideshow-header">
        <div class="header-spacer"></div>
        <div class="image-counter" id="slideshow-title">
          {{ displayCounter }}
        </div>
        <button 
          class="close-button" 
          @click="close"
          aria-label="Close slideshow"
          ref="closeButtonRef"
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="close-icon">
            <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
          </svg>
        </button>
      </div>
      
      <div class="slideshow-container">
        <swiper-container
          slides-per-view="1"
          space-between="0"
          navigation="true"
          :pagination="{clickable: true}"
          :keyboard="{enabled: true}"
          @slidechange="onSlideChange"
          class="slideshow-swiper"
          ref="swiperContainer"
        >
          <swiper-slide 
            v-for="(media, index) in mediaItems" 
            :key="index"
            class="slideshow-slide"
          >
            <div class="slide-content">
              <!-- Loading skeleton -->
              <div 
                v-if="loadingStates[index]" 
                class="loading-skeleton"
                aria-hidden="true"
              >
                <div class="skeleton-shimmer"></div>
              </div>
              
              <iframe
                v-if="getMediaType(media) === 'youtube'"
                :src="getMediaUrl(media)"
                title="YouTube video player"
                class="slideshow-video"
                :class="{ 'loaded': !loadingStates[index] }"
                allow="accelerometer; encrypted-media; gyroscope; picture-in-picture"
                allowfullscreen
                @load="onMediaLoad(index)"
                :aria-describedby="index === currentSlideIndex ? 'slideshow-description' : undefined"
              />
              <img 
                v-else
                :src="getMediaUrl(media)" 
                alt="Asset gallery image" 
                class="slideshow-image"
                :class="{ 'loaded': !loadingStates[index] }"
                @load="onMediaLoad(index)"
                @error="onMediaError(index)"
                :aria-describedby="index === currentSlideIndex ? 'slideshow-description' : undefined"
              />
            </div>
          </swiper-slide>
        </swiper-container>
      </div>
      
      <!-- Screen reader description -->
      <div id="slideshow-description" class="sr-only">
        Slideshow view. Use arrow keys or navigation buttons to browse images. Press Escape to close.
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits, onMounted, onUnmounted, ref, reactive, nextTick, computed } from 'vue';
import { register } from 'swiper/element/bundle';
import type { Swiper as SwiperInstance } from 'swiper';
import 'swiper/css';
import 'swiper/css/navigation';
import 'swiper/css/pagination';

interface SwiperContainer extends HTMLElement {
  swiper?: SwiperInstance;
}

const props = defineProps<{
  mediaItems: string[];
  startIndex: number;
}>();

const emit = defineEmits(['close']);

const closeButtonRef = ref<HTMLButtonElement | null>(null);
const currentSlideIndex = ref(props.startIndex);
const loadingStates = reactive<Record<number, boolean>>({});
const swiperContainer = ref<SwiperContainer | null>(null);

// Initialize loading states
props.mediaItems.forEach((_, index) => {
  loadingStates[index] = true;
});

// Computed property for the display counter (converts 0-based to 1-based)
const displayCounter = computed(() => {
  const displayNumber = currentSlideIndex.value + 1;
  const total = props.mediaItems.length;
  const counter = `${displayNumber} of ${total}`;
  return counter;
});

const close = () => {
  emit('close');
};

const onSlideChange = (event: CustomEvent) => {
  const swiper = event.detail[0] as SwiperInstance;
  const newIndex = swiper.activeIndex;
  currentSlideIndex.value = newIndex;
};

const onMediaLoad = (index: number) => {
  loadingStates[index] = false;
};

const onMediaError = (index: number) => {
  loadingStates[index] = false;
  console.error(`Failed to load media at index ${index}`);
};

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    close();
  }
  // Swiper's keyboard module handles arrow keys automatically
};

onMounted(() => {
  // Register Swiper Element if not already registered
  register();
  
  document.addEventListener('keydown', handleKeydown);
  document.body.style.overflow = 'hidden';
  

  
  // Focus the close button initially for keyboard accessibility and set slide
  nextTick(() => {
    closeButtonRef.value?.focus();
    
    // Set up the slide after Swiper is ready
    const setSlideTo = (index: number) => {
      if (swiperContainer.value) {
        const swiper = swiperContainer.value.swiper;
        if (swiper && swiper.slides && swiper.slides.length > 0) {

          swiper.slideTo(index, 0); // 0 = no animation
          currentSlideIndex.value = index;
          return true;
        }
      }
      return false;
    };
    
    // Try to set the slide immediately
    if (!setSlideTo(props.startIndex)) {
      // If it fails, wait for the swiper to be ready
      const checkReady = () => {
        if (setSlideTo(props.startIndex)) {

          // Also add a direct event listener as backup
          if (swiperContainer.value?.swiper) {
            const swiper = swiperContainer.value.swiper;
            swiper.on('slideChange', () => {
              const newIndex = swiper.activeIndex;
              currentSlideIndex.value = newIndex;
            });
          }
          return;
        }
        
        // Keep checking until ready (max 1 second)
        setTimeout(checkReady, 50);
      };
      
      setTimeout(checkReady, 50);
    } else {
      // Even if immediate setting worked, add the event listener
      if (swiperContainer.value?.swiper) {
        const swiper = swiperContainer.value.swiper;
        swiper.on('slideChange', () => {
          const newIndex = swiper.activeIndex;
          currentSlideIndex.value = newIndex;
        });
      }
    }
  });
});

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown);
  document.body.style.overflow = '';
});

const getMediaType = (media: string): 'youtube' | 'image' => {
  const url = getMediaUrl(media);
  
  // Check if it's a YouTube URL
  if (url.includes('youtube.com/embed/') || url.includes('youtu.be/')) {
    return 'youtube';
  }
  
  return 'image';
};

const getMediaUrl = (media: string): string => {
  return media;
};
</script>

<style scoped>
.slideshow-modal-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(5px);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
  animation: backdropFadeIn 0.3s ease-out;
}

@keyframes backdropFadeIn {
  from {
    opacity: 0;
    backdrop-filter: blur(0px);
  }
  to {
    opacity: 1;
    backdrop-filter: blur(5px);
  }
}

.slideshow-modal-content {
  position: relative;
  width: 70%;
  height: 70%;
  max-width: 1200px;
  background-color: var(--color-background-2);
  border-radius: var(--border-radius-lg);
  box-shadow: 0 10px 30px rgba(0,0,0,0.2);
  overflow: hidden;
  animation: modalSlideIn 0.3s ease-out;
}

@keyframes modalSlideIn {
  from {
    transform: scale(0.9) translateY(-20px);
    opacity: 0;
  }
  to {
    transform: scale(1) translateY(0);
    opacity: 1;
  }
}

.slideshow-header {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1rem;
  background: linear-gradient(to bottom, rgba(0,0,0,0.7), transparent);
  z-index: 1010;
}

.header-spacer {
  flex: 1;
}

.image-counter {
  flex: 1;
  text-align: center;
  color: white;
  font-size: 0.9rem;
  font-weight: 500;
  text-shadow: 0 1px 3px rgba(0,0,0,0.8);
}

.close-button {
  display: flex;
  align-self: flex-start;
  font-size: 1.2rem;
  font-weight: normal;
  color: white;
  background: rgba(0, 0, 0, 0.6);
  border: none;
  border-radius: 6px;
  width: 32px;
  height: 32px;
  cursor: pointer;
  line-height: 1;
  transition: all 0.2s ease;
  align-items: center;
  justify-content: center;
  backdrop-filter: blur(10px);
  flex-shrink: 0;
}

.close-button:hover {
  background: rgba(0, 0, 0, 0.8);
  transform: scale(1.05);
}

.close-button:focus {
  outline: 2px solid var(--color-primary, #007acc);
  outline-offset: 2px;
}

.close-icon {
  width: 16px;
  height: 16px;
  fill: currentColor;
}

.slideshow-container {
  width: 100%;
  height: 100%;
}

.slideshow-swiper {
  width: 100%;
  height: 100%;
}

.slideshow-slide {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100%;
}

.slide-content {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100%;
  padding: 3rem 2rem 2rem;
  position: relative;
}

.loading-skeleton {
  position: absolute;
  top: 3rem;
  left: 2rem;
  right: 2rem;
  bottom: 2rem;
  background: var(--color-background-3, #2a2a2a);
  border-radius: var(--border-radius-md);
  overflow: hidden;
}

.skeleton-shimmer {
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.1),
    transparent
  );
  animation: shimmer 2s infinite;
}

@keyframes shimmer {
  0% { transform: translateX(-100%); }
  100% { transform: translateX(100%); }
}

.slideshow-image,
.slideshow-video {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  border-radius: var(--border-radius-md);
  opacity: 0;
  transition: opacity 0.3s ease;
}

.slideshow-image.loaded,
.slideshow-video.loaded {
  opacity: 1;
}

.slideshow-video {
  width: 100%;
  height: 100%;
  aspect-ratio: 16/9;
}

.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

/* Swiper Navigation Buttons */
:deep(.swiper-button-prev),
:deep(.swiper-button-next) {
  color: white;
  background: rgba(0, 0, 0, 0.5);
  border-radius: 50%;
  width: 44px;
  height: 44px;
  margin-top: -22px;
  transition: all 0.2s ease;
  backdrop-filter: blur(10px);
}

:deep(.swiper-button-prev):hover,
:deep(.swiper-button-next):hover {
  background: rgba(0, 0, 0, 0.8);
  transform: scale(1.05);
}

:deep(.swiper-button-prev::after),
:deep(.swiper-button-next::after) {
  font-size: 18px;
  font-weight: bold;
}

/* Swiper Pagination */
:deep(.swiper-pagination) {
  bottom: 20px !important;
}

:deep(.swiper-pagination-bullet) {
  background: rgba(255, 255, 255, 0.6);
  opacity: 1;
  transition: all 0.2s ease;
}

:deep(.swiper-pagination-bullet-active) {
  background: var(--color-primary, #007acc);
  transform: scale(1.2);
}
</style> 