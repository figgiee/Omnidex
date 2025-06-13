import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

export interface OrbitalAsset {
  id: string
  product_slug: string
  title: string
  description?: string
  technical_details?: string
  seller: {
    id: string
    name: string
    display_name: string
  }
  categories: Array<{
    id: string
    name: string
    path: string
  }>
  compatible_apps: string[]
  gallery_images: Array<{
    url: string
    image_type?: string
    width?: number
    height?: number
  }>
  rating?: {
    average_rating: number
    total_ratings: number
  }
  price?: {
    amount: number
    currency: string
    discount_percentage?: number
  }
  release_date?: string
  last_modified?: string
  tags: string[]
}

export interface AssetMatch {
  local_asset_id: number
  orbital_asset?: OrbitalAsset
  match_confidence: number
  match_type: 'Exact' | 'HighConfidence' | 'MediumConfidence' | 'LowConfidence' | 'NoMatch'
  match_reasons: string[]
}

export const useOrbitalStore = defineStore('orbital', {
  state: () => ({
    isApiAccessible: false,
    isAuthenticated: false,
    isLoading: false,
    error: null as string | null,
    currentAsset: null as OrbitalAsset | null,
    assetMatches: [] as AssetMatch[],
    authInstructions: '',
  }),

  actions: {
    async testApiAccess() {
      this.isLoading = true
      this.error = null
      
      try {
        const result = await invoke<boolean>('test_orbital_api_access')
        this.isApiAccessible = result
        return result
      } catch (error) {
        this.error = error as string
        this.isApiAccessible = false
        return false
      } finally {
        this.isLoading = false
      }
    },

    async fetchAssetDetails(productSlug: string) {
      this.isLoading = true
      this.error = null
      
      try {
        const asset = await invoke<OrbitalAsset>('fetch_orbital_asset_details', {
          productSlug
        })
        this.currentAsset = asset
        return asset
      } catch (error) {
        this.error = error as string
        throw error
      } finally {
        this.isLoading = false
      }
    },

    async matchLocalAssets() {
      this.isLoading = true
      this.error = null
      
      try {
        const matches = await invoke<AssetMatch[]>('match_local_assets_with_orbital')
        this.assetMatches = matches
        return matches
      } catch (error) {
        this.error = error as string
        throw error
      } finally {
        this.isLoading = false
      }
    },

    async setAuthToken(accessToken: string, tokenType: string = 'Bearer', expiresIn: number = 3600) {
      this.isLoading = true
      this.error = null
      
      try {
        await invoke('set_orbital_auth_token', {
          accessToken,
          tokenType,
          expiresIn
        })
        this.isAuthenticated = true
      } catch (error) {
        this.error = error as string
        throw error
      } finally {
        this.isLoading = false
      }
    },

    async getAuthInstructions() {
      try {
        const instructions = await invoke<string>('get_orbital_auth_instructions')
        this.authInstructions = instructions
        return instructions
      } catch (error) {
        this.error = error as string
        throw error
      }
    },

    clearError() {
      this.error = null
    },

    clearCurrentAsset() {
      this.currentAsset = null
    },

    async testFolderCleaning(folderName: string) {
      try {
        const result = await invoke<[string, string[]]>('test_orbital_folder_cleaning', {
          folderName
        })
        return result
      } catch (error) {
        this.error = error as string
        throw error
      }
    },

    async searchAssets(query: string, limit: number = 20) {
      this.isLoading = true
      this.error = null
      
      try {
        const results = await invoke<OrbitalAsset[]>('search_orbital_assets', {
          query,
          limit
        })
        return results
      } catch (error) {
        this.error = error as string
        throw error
      } finally {
        this.isLoading = false
      }
    }
  }
}) 