import { defineStore } from 'pinia';
import { listen } from '@tauri-apps/api/event';

interface ScanProgressPayload {
  current: number;
  total: number;
}

export const useStatusBarStore = defineStore('statusBar', {
  state: () => ({
    message: 'Ready',
    progress: 0,
    total: 0,
    isVisible: false,
    isComplete: false,
  }),
  actions: {
    async initialize() {
      await listen('scan_progress', (event) => {
        const payload = event.payload as ScanProgressPayload;
        this.message = `Scanning...`;
        this.progress = payload.current;
        this.total = payload.total;
        this.isVisible = true;
        this.isComplete = false;
      });

      await listen('scan_complete', (event) => {
        const { assets_found } = event.payload as { assets_found: number };
        this.message = `Scan complete. ${assets_found} new assets found.`;
        this.isComplete = true;
        setTimeout(() => {
          this.reset();
        }, 5000); // Hide after 5 seconds
      });
    },
    reset() {
      this.message = 'Ready';
      this.progress = 0;
      this.total = 0;
      this.isVisible = false;
      this.isComplete = false;
    },
  },
});
