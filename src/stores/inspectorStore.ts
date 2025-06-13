import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import type { Asset, AssetDetails } from '@/types';

export const useInspectorStore = defineStore('inspector', {
  state: () => ({
    details: null as AssetDetails | null,
    loading: false,
    error: null as string | null,
  }),

  actions: {
    async fetchAssetDetails(id: number) {
      if (!id) return;
      this.loading = true;
      this.error = null;
      try {
        const asset: any = await invoke('get_asset_details', { id });
        
        const is_matched = !!asset.matchedOrbitalProductSlug;

        // Parse JSON strings for orbital data
        let gallery_images: string[] = [];
        if (asset.orbitalGalleryImages) {
          try {
            gallery_images = JSON.parse(asset.orbitalGalleryImages);
    
          } catch (e) {
            console.warn('Failed to parse orbitalGalleryImages:', e);
          }
        }

        let engine_version: string[] | undefined = undefined;
        if (asset.orbitalSupportedVersions) {
          try {
            const versions = JSON.parse(asset.orbitalSupportedVersions);
            if (Array.isArray(versions) && versions.length > 0) {
              engine_version = versions;
            }

          } catch (e) {
            console.warn('Failed to parse orbitalSupportedVersions:', e);
          }
        }

        this.details = {
          id: asset.id,
          name: asset.orbitalTitle || asset.name,
          thumbnail_url: asset.orbitalThumbnailUrl || undefined,
          gallery_images,
          creator_name: asset.orbitalSellerName || undefined,
          rating: asset.orbitalRatingAverage || undefined,
          reviews_count: asset.orbitalRatingCount || undefined,
          price: asset.orbitalPrice || undefined,
          source_url: asset.orbitalSourceUrl || undefined,
          release_date: asset.orbitalReleaseDate || undefined,
          description: asset.orbitalDescription || asset.description || undefined,
          technical_description: asset.orbitalTechnicalDetails || undefined,
          tags: asset.tags ?? [],
          scraped_tags: [],
          file_path: asset.filePath,
          file_size: asset.fileSize,
          date_created: asset.createdDate,
          engine_version,
          is_matched,
          folder_name: is_matched ? undefined : asset.name,
        };



      } catch (e: any) {
        this.error = e.toString();
        this.details = null;
      } finally {
        this.loading = false;
      }
    },
    async updateAssetTags(id: number, tags: string[]) {
      if (!id) return;
      try {
        await invoke('update_asset_tags', { id, tags });
        if (this.details) {
          this.details.tags = tags;
        }
      } catch (e: any) {
        this.error = e.toString();
      }
    },
    async updateMultipleAssetTags(ids: number[], tags: string[]) {
      if (!ids || ids.length === 0) return;
      try {
        await invoke('add_tags_to_assets', { asset_ids: ids, tags });
        // Optionally, refetch details if one of the selected assets is the primary one
        if (this.details && ids.includes(this.details.id)) {
          // Add new tags to local state without removing old ones in multi-select
          this.details.tags = [...new Set([...this.details.tags, ...tags])];
        }
      } catch (e: any) {
        console.error("Failed to update tags for multiple assets", e);
        this.error = e.toString();
      }
    },
    clearDetails() {
      this.details = null;
    }
  },
}); 