import type { GalleryItem, GalleryImageMedia, GalleryVideoMedia } from '@/types'

/**
 * Extract gallery media items from HTML content
 * Looks for both images and YouTube embeds
 */
export function extractGalleryMedia(htmlContent: string): GalleryItem[] {
  const parser = new DOMParser()
  const doc = parser.parseFromString(htmlContent, 'text/html')
  const mediaItems: GalleryItem[] = []

  // Extract images from various sources
  const images = doc.querySelectorAll('img')
  images.forEach(img => {
    const src = img.src || img.getAttribute('data-src')
    if (src && isValidImageUrl(src)) {
      const imageMedia: GalleryImageMedia = {
        type: 'image',
        src: src,
        alt: img.alt || `Image at ${src}`
      }
      mediaItems.push(imageMedia)
    }
  })

  // Extract YouTube embeds
  const iframes = doc.querySelectorAll('iframe')
  iframes.forEach(iframe => {
    const src = iframe.src
    if (src && isYouTubeEmbed(src)) {
      const videoMedia: GalleryVideoMedia = {
        type: 'video',
        src: src,
        post: iframe.title || `Video at ${src}`
      }
      mediaItems.push(videoMedia)
    }
  })

  return mediaItems
}

/**
 * Check if a URL is a valid image URL
 */
export function isValidImageUrl(url: string): boolean {
  return /\.(jpg|jpeg|png|webp|avif|gif|svg)$/.test(url)
}

/**
 * Detect the source of an image URL
 */
function detectImageSource(url: string): 'fab' | 'epicgames' | 'orbital' {
  if (url.includes('media.fab.com')) {
    return 'fab'
  } else if (url.includes('epicgames.com')) {
    return 'epicgames'
  } else {
    return 'orbital'
  }
}

/**
 * Check if a URL is a YouTube embed
 */
function isYouTubeEmbed(url: string): boolean {
  return url.includes('youtube.com/embed/') || url.includes('youtu.be/')
}

/**
 * Extract YouTube video ID from embed URL
 */
export function extractYouTubeVideoId(url: string): string | null {
  const embedMatch = url.match(/youtube\.com\/embed\/([^?&]+)/)
  const shortMatch = url.match(/youtu\.be\/([^?&]+)/)
  return embedMatch ? embedMatch[1] : shortMatch ? shortMatch[1] : null
}

/**
 * Convert a YouTube video ID to embed URL
 */
export function youTubeIdToEmbedUrl(videoId: string): string {
  return `https://www.youtube.com/embed/${videoId}`
}

/**
 * Normalize media items - ensure proper format for display
 */
export function normalizeGalleryItems(items: any[]): GalleryItem[] {
  return items.map(item => {
    // If it's already a valid GalleryItem, keep as is
    if (typeof item === 'object' && item !== null && item.type && item.src) {
      return item as GalleryItem
    }
    
    // If it's an object with url property, convert to correct format
    if (typeof item === 'object' && item !== null && item.url) {
      if (isYouTubeEmbed(item.url)) {
        return {
          type: 'video',
          src: item.url,
          post: item.title || `Video at ${item.url}`
        } as GalleryVideoMedia
      } else {
        return {
          type: 'image',
          src: item.url,
          alt: item.alt_text || item.alt || `Image at ${item.url}`
        } as GalleryImageMedia
      }
    }
    
    return null
  }).filter(Boolean) as GalleryItem[]
}

export function extractMediaFromUrl(src: string): GalleryItem {
  // Handle image URLs
  if (isValidImageUrl(src)) {
    return {
      type: 'image',
      src: src,
      alt: `Image at ${src}`
    } as GalleryImageMedia;
  }

  // Handle YouTube URLs
  if (src.includes('youtube.com') || src.includes('youtu.be')) {
    const videoId = extractYouTubeVideoId(src);
    if (videoId) {
      return {
        type: 'video',
        src: `https://www.youtube.com/embed/${videoId}`,
        post: `Video: ${videoId}`
      } as GalleryVideoMedia;
    }
  }

  // Default to image if no other type is detected
  return {
    type: 'image',
    src: src,
    alt: `Image at ${src}`
  } as GalleryImageMedia;
}

export function extractMediaFromFab(items: any[]): GalleryItem[] {
  if (!Array.isArray(items)) {
    return [];
  }

  const extractedItems: GalleryItem[] = [];

  for (const item of items) {
    const url = item.url || item.src || '';
    if (!url) continue;

    if (url.includes('youtube.com') || url.includes('youtu.be')) {
      const videoId = extractYouTubeVideoId(url);
      if (videoId) {
        extractedItems.push({
          type: 'video',
          src: `https://www.youtube.com/embed/${videoId}`,
          post: `Video: ${videoId}`
        } as GalleryVideoMedia);
      }
    } else {
      extractedItems.push({
        type: 'image',
        src: url,
        alt: item.alt_text || 'Gallery image'
      } as GalleryImageMedia);
    }
  }
  return extractedItems;
}