<template>
  <div class="scan-progress-component">
    <!-- Clean Header with proper hierarchy -->
    <div class="progress-header">
      <div class="header-main">
        <div class="scan-status">
          <div class="status-icon">
            <div v-if="isScanning" class="scanning-spinner"></div>
            <div v-else class="complete-icon">✓</div>
          </div>
          <div class="status-text">
            <h2>{{ isScanning ? 'Scanning in Progress' : 'Scan Complete' }}</h2>
            <p 
              v-if="scanProgress && scanProgress.current_path" 
              class="current-file"
              :title="scanProgress.current_path"
            >
              {{ currentFileName }}
            </p>
          </div>
        </div>
      </div>
      
      <div class="header-actions">
        <button 
          @click="showDetailedLogs = !showDetailedLogs"
          class="action-btn secondary"
        >
          {{ showDetailedLogs ? 'Hide' : 'Show' }} Details
        </button>
        <button 
          v-if="isScanning" 
          @click="cancelScan"
          class="action-btn danger"
        >
          Cancel Scan
        </button>
      </div>
    </div>

    <div v-if="scanProgress" class="progress-content">
      <!-- Main Progress Section with better visual hierarchy -->
      <div class="progress-main">
        <div class="progress-info-row">
          <span class="progress-label">Overall Progress</span>
          <span class="progress-percentage">{{ progressPercentage }}%</span>
        </div>
        
        <div class="progress-bar-container">
          <div 
            class="progress-bar" 
            :style="progressBarStyle"
          >
            <div class="progress-glow"></div>
          </div>
        </div>
        
        <div class="progress-details">
          <span class="items-count">
            {{ scanProgress.processed_items.toLocaleString() }} of {{ scanProgress.total_items.toLocaleString() }} items
          </span>
          <span v-if="estimatedTimeRemaining" class="time-remaining">
            {{ estimatedTimeRemaining }} remaining
          </span>
        </div>
      </div>

      <!-- Error Alert and Success Message are now first -->
      <div v-if="scanProgress.error && !wasCancelled" class="error-alert">
        <div class="alert-icon">⚠️</div>
        <div class="alert-content">
          <h4>Scan Error</h4>
          <p>{{ scanProgress.error }}</p>
        </div>
        <button @click="showErrors = !showErrors" class="alert-toggle">
          {{ showErrors ? 'Hide Details' : 'Show Details' }}
        </button>
      </div>

      <div v-if="scanProgress.completed_successfully" class="success-message">
        <div class="success-icon">✅</div>
        <div class="success-content">
          <h3>Scan Complete!</h3>
          <p class="success-summary">
            Successfully processed {{ scanProgress.processed_items.toLocaleString() }} items in {{ elapsedTime }}
          </p>
          <p v-if="scanSummary.errors > 0" class="success-warnings">
            {{ scanSummary.errors }} errors encountered during scan
          </p>
        </div>
      </div>

      <!-- Detailed Logs Section is now more prominent -->
      <div v-if="showDetailedLogs" class="logs-section">
        <div class="logs-header">
          <h4>Activity Log</h4>
          <div class="log-controls">
            <button 
              @click="autoScroll = !autoScroll" 
              class="control-btn"
              :class="{ active: autoScroll }"
            >
              Auto-scroll
            </button>
            <button @click="clearLogs" class="control-btn">
              Clear
            </button>
          </div>
        </div>
        
        <div class="logs-container" ref="logsContainer">
          <div v-if="scanLogs.length === 0" class="logs-empty">
            <div class="empty-icon">📄</div>
            <p>No activity logs yet. Logs will appear as scanning progresses.</p>
          </div>
          
          <div v-for="(log, index) in scanLogs" :key="index" class="log-entry" :class="getLogEntryClass(log)">
            <div class="log-time">{{ formatTimestamp(log.timestamp) }}</div>
            <div class="log-icon">{{ getStatusIcon(log.status) }}</div>
            <div class="log-content">
              <div class="log-message">{{ log.message }}</div>
              <div v-if="log.details" class="log-details">{{ log.details }}</div>
            </div>
          </div>
        </div>
      </div>

      <!-- Statistics Cards are now at the bottom -->
      <div class="stats-section">
        <div class="stat-card">
          <div class="stat-value">{{ scanProgress.processed_items.toLocaleString() }}</div>
          <div class="stat-label">Items Processed</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">{{ scanProgress.total_items.toLocaleString() }}</div>
          <div class="stat-label">Total Items</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">{{ assetsFound.toLocaleString() }}</div>
          <div class="stat-label">Assets Found</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">{{ elapsedTime }}</div>
          <div class="stat-label">Elapsed Time</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, onMounted, onUnmounted, ref, watch, nextTick } from 'vue'
import { useSettingsStore } from '@/stores/settingsStore'
import { useScanLocationStore } from '@/stores/scanLocationStore'
import type { LogEntry, ScanProgress } from '@/types';

const props = defineProps({
  isScanning: {
    type: Boolean,
    required: true
  },
  scanProgress: {
    type: Object as () => ScanProgress,
    required: true
  }
})

interface ScanLogEntry {
  timestamp: Date
  type: 'asset' | 'error' | 'info' | 'success' | 'warning'
  message: string
  details?: string
  status: 'processing' | 'success' | 'error' | 'warning' | 'info'
}

// State
const showErrors = ref(false)
const showDetailedLogs = ref(false)
const autoScroll = ref(true)
const scanStartTime = ref<Date | null>(null)
const currentTime = ref(new Date())
const timer = ref<number | null>(null)
const logsContainer = ref<HTMLElement | null>(null)
const scanLogs = ref<ScanLogEntry[]>([])
const lastProcessedIndex = ref(0)
const wasCancelled = ref(false)

const settingsStore = useSettingsStore()
const scanStore = useScanLocationStore()

// Computed properties
const progressPercentage = computed(() => {
  if (!props.scanProgress || props.scanProgress.total_items === 0) return 0
  return Math.round((props.scanProgress.processed_items / props.scanProgress.total_items) * 100)
})

const progressBarStyle = computed(() => ({
  width: `${progressPercentage.value}%`,
  background: wasCancelled.value 
    ? 'linear-gradient(90deg, #f59e0b, #fbbf24)' 
    : 'linear-gradient(90deg, #3b82f6, #4ade80)',
}))

const assetsFound = computed(() => {
  if (!props.scanProgress) return 0
  return Math.round(props.scanProgress.processed_items * 0.3) // Rough estimate
})

const currentFileName = computed(() => {
  if (!props.scanProgress?.current_path) return ''
  
  const path = props.scanProgress.current_path
  const maxLength = 80
  
  if (path.length <= maxLength) return path
  
  // For very long paths, show beginning and end
  const start = path.substring(0, 30)
  const end = path.substring(path.length - 40)
  return `${start}...${end}`
})

const elapsedTime = computed(() => {
  if (!scanStartTime.value) return '0s'
  const elapsed = Math.floor((currentTime.value.getTime() - scanStartTime.value.getTime()) / 1000)
  return formatDuration(elapsed)
})

const estimatedTimeRemaining = computed(() => {
  if (!props.scanProgress || !scanStartTime.value || props.scanProgress.processed_items === 0) {
    return null
  }
  
  const elapsed = (currentTime.value.getTime() - scanStartTime.value.getTime()) / 1000
  const rate = props.scanProgress.processed_items / elapsed
  const remaining = (props.scanProgress.total_items - props.scanProgress.processed_items) / rate
  
  return formatDuration(Math.round(remaining))
})

const scanSummary = computed(() => {
  return {
    total: scanLogs.value.length,
    success: scanLogs.value.filter(log => log.status === 'success').length,
    errors: scanLogs.value.filter(log => log.status === 'error').length,
    warnings: scanLogs.value.filter(log => log.status === 'warning').length,
  }
})

// Methods
function formatDuration(seconds: number): string {
  if (seconds < 60) return `${seconds}s`
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m ${seconds % 60}s`
  return `${Math.floor(seconds / 3600)}h ${Math.floor((seconds % 3600) / 60)}m`
}

function formatTimestamp(date: Date): string {
  return date.toLocaleTimeString('en-US', { 
    hour12: false, 
    hour: '2-digit', 
    minute: '2-digit', 
    second: '2-digit' 
  })
}

function getLogEntryClass(log: ScanLogEntry): string {
  return `log-${log.status}`
}

function getStatusIcon(status: string): string {
  switch (status) {
    case 'success': return '✅'
    case 'error': return '❌'
    case 'warning': return '⚠️'
    case 'processing': return '🔄'
    default: return 'ℹ️'
  }
}

function addLogEntry(type: ScanLogEntry['type'], message: string, details?: string, status: ScanLogEntry['status'] = 'info') {
  const entry = {
    timestamp: new Date(),
    type,
    message,
    details,
    status
  }
  
  scanLogs.value.push(entry)

  
  // Keep only the last 100 log entries to prevent memory issues
  if (scanLogs.value.length > 100) {
    scanLogs.value = scanLogs.value.slice(-100)
  }
  
  if (autoScroll.value) {
    nextTick(() => {
      scrollToBottom()
    })
  }
}

function scrollToBottom() {
  if (logsContainer.value) {
    logsContainer.value.scrollTop = logsContainer.value.scrollHeight
  }
}

function clearLogs() {
  scanLogs.value = []
}



async function cancelScan() {
  try {
    const cancelledCount = await invoke<number>('cancel_all_scans')

    addLogEntry('info', `Scan cancellation requested - ${cancelledCount} scans cancelled`, undefined, 'warning')
    wasCancelled.value = true
  } catch (error) {
    console.error('Failed to cancel scan:', error)
    addLogEntry('error', 'Failed to cancel scan', String(error), 'error')
  }
}

// Parse backend logs and extract meaningful information
function parseBackendLogs() {
  
  addLogEntry('info', 'Log viewer initialized', 'Waiting for scan activity...', 'info')
  
  watch(() => scanStore.scanProgress, (newProgress) => {
    if (newProgress) {
      // Add scan progress information to our logs
      addLogEntry('info', `Progress: ${newProgress.processed_items}/${newProgress.total_items}`, newProgress.current_path, 'processing')
      if (autoScroll.value) {
        nextTick(() => {
          scrollToBottom()
        })
      }
    }
  })
}

function extractAssetName(logLine: string): string {
  const match = logLine.match(/Asset (.+?) found, attempting/)
  return match ? match[1] : 'Unknown Asset'
}

function extractAssetFromFetching(logLine: string): string {
  const match = logLine.match(/Fetching Orbital data for asset: (.+)$/)
  return match ? match[1] : 'Unknown Asset'
}

function extractAssetFromFailedLog(logLine: string): string {
  const match = logLine.match(/Failed to refresh metadata for asset (.+?):/)
  return match ? match[1] : 'Unknown Asset'
}

function extractAssetFromWarning(logLine: string): string {
  const match = logLine.match(/falling back to web search for: (.+)$/)
  return match ? match[1] : 'Unknown Asset'
}

function extractUrl(logLine: string): string {
  const match = logLine.match(/https:\/\/[^\s]+/)
  return match ? match[0] : ''
}

function extractPath(logLine: string): string {
  const match = logLine.match(/Starting folder scan of location: (.+)$/)
  return match ? match[1] : 'Unknown path'
}

function extractFolderCount(logLine: string): string {
  const match = logLine.match(/Found (\d+) folders/)
  return match ? match[1] : '0'
}

function extractWarningMessage(logLine: string): string {
  const match = logLine.match(/\[WARN\].*?\] (.+)$/)
  return match ? match[1] : logLine
}

function extractErrorDetails(logLine: string): string {
  if (logLine.includes('404 Not Found')) return '404 - Asset not found on Orbital Market'
  if (logLine.includes('Client error')) return 'Network error while fetching data'
  if (logLine.includes('Failed to fetch URL')) return 'Unable to connect to Orbital Market'
  return 'Unknown error occurred'
}

function extractScanStats(logLine: string): string {
  const timeMatch = logLine.match(/in ([0-9.]+s)/)
  const processedMatch = logLine.match(/Processed (\d+)\/(\d+)/)
  const foundMatch = logLine.match(/Found (\d+) assets/)
  
  let stats = ''
  if (timeMatch) stats += `Duration: ${timeMatch[1]}`
  if (processedMatch) stats += `, Processed: ${processedMatch[1]}/${processedMatch[2]} folders`
  if (foundMatch) stats += `, Found: ${foundMatch[1]} assets`
  
  return stats
}

// Watchers
watch(() => props.isScanning, (newValue) => {
  if (newValue && !scanStartTime.value) {
    scanStartTime.value = new Date()
    addLogEntry('info', 'Scan started', undefined, 'processing')
    clearLogs() // Clear previous logs when starting a new scan
  } else if (!newValue && scanStartTime.value) {
    addLogEntry('success', 'Scan finished', undefined, 'success')
  }
})

// Lifecycle
onMounted(async () => {
  // Update current time every second for elapsed time calculation
  timer.value = window.setInterval(() => {
    currentTime.value = new Date()
  }, 1000)
  
  // Initialize log parsing
  parseBackendLogs()
})

onUnmounted(() => {
  if (timer.value) {
    clearInterval(timer.value)
  }
})
</script>

<style scoped>
/* Modern, clean design with proper spacing and hierarchy */
.scan-progress-component {
  background: var(--bg-surface);
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: var(--text-primary);
  max-width: 100%;
  max-height: 85vh;
  margin: 0 auto;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

/* Header with better visual hierarchy */
.progress-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 2rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  background: linear-gradient(135deg, var(--bg-surface) 0%, rgba(255, 255, 255, 0.02) 100%);
  flex-shrink: 0;
}

.header-main {
  flex: 1;
}

.scan-status {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
  flex: 1;
  min-width: 0;
}

.status-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 12px;
  background: rgba(59, 130, 246, 0.1);
  border: 1px solid rgba(59, 130, 246, 0.2);
}

.scanning-spinner {
  width: 24px;
  height: 24px;
  border: 3px solid rgba(59, 130, 246, 0.3);
  border-top-color: #3b82f6;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

.complete-icon {
  font-size: 24px;
  color: #10b981;
}

.status-text {
  flex: 1;
  min-width: 0;
}

.status-text h2 {
  margin: 0 0 0.25rem 0;
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1.2;
}

.current-file {
  margin: 0.5rem 0 0 0;
  color: var(--text-secondary);
  font-size: 0.85rem;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  background: rgba(0, 0, 0, 0.2);
  padding: 0.75rem;
  border-radius: 6px;
  max-width: 100%;
  word-break: break-all;
  line-height: 1.4;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.header-actions {
  display: flex;
  gap: 0.75rem;
  align-items: center;
  flex-shrink: 0;
}

.action-btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 8px;
  font-weight: 600;
  font-size: 0.875rem;
  cursor: pointer;
  transition: all 0.2s ease;
  white-space: nowrap;
}

.action-btn.secondary {
  background: rgba(255, 255, 255, 0.1);
  color: var(--text-primary);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.action-btn.secondary:hover {
  background: rgba(255, 255, 255, 0.15);
  transform: translateY(-1px);
}

.action-btn.danger {
  background: #dc2626;
  color: white;
}

.action-btn.danger:hover {
  background: #b91c1c;
  transform: translateY(-1px);
}

/* Main content with generous spacing and scrolling */
.progress-content {
  padding: 2rem;
  display: flex;
  flex-direction: column;
  gap: 2rem;
  overflow-y: auto;
  flex: 1;
  min-height: 0;
}

/* Enhanced progress section */
.progress-main {
  background: rgba(255, 255, 255, 0.02);
  border-radius: 12px;
  padding: 2rem;
  border: 1px solid rgba(255, 255, 255, 0.05);
}

.progress-info-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.progress-label {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--text-primary);
}

.progress-percentage {
  font-size: 1.5rem;
  font-weight: 700;
  color: #4ade80;
}

.progress-bar-container {
  width: 100%;
  height: 12px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  overflow: hidden;
  margin-bottom: 1rem;
  position: relative;
}

.progress-bar {
  height: 100%;
  border-radius: 6px;
  transition: width 0.6s cubic-bezier(0.4, 0, 0.2, 1), background 0.5s ease;
  position: relative;
}

.progress-glow {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
  animation: progress-glow 2s ease-in-out infinite;
}

.progress-details {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 0.9rem;
}

.items-count {
  color: var(--text-secondary);
  font-weight: 500;
}

.time-remaining {
  color: #4ade80;
  font-weight: 600;
}

/* Statistics cards with better spacing */
.stats-section {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 1.5rem;
}

.stat-card {
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  padding: 1.5rem;
  text-align: center;
  transition: all 0.2s ease;
}

.stat-card:hover {
  background: rgba(255, 255, 255, 0.05);
  transform: translateY(-2px);
}

.stat-value {
  font-size: 2rem;
  font-weight: 800;
  color: #4ade80;
  margin-bottom: 0.5rem;
  line-height: 1;
}

.stat-label {
  color: var(--text-secondary);
  font-size: 0.875rem;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

/* Alert messages with proper hierarchy */
.error-alert {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.2);
  border-radius: 12px;
  padding: 1.5rem;
}

.alert-icon {
  font-size: 1.5rem;
  flex-shrink: 0;
}

.alert-content {
  flex: 1;
}

.alert-content h4 {
  margin: 0 0 0.5rem 0;
  color: #f87171;
  font-size: 1.125rem;
  font-weight: 600;
}

.alert-content p {
  margin: 0;
  color: #fecaca;
  line-height: 1.5;
}

.alert-toggle {
  padding: 0.5rem 1rem;
  background: rgba(239, 68, 68, 0.2);
  border: 1px solid rgba(239, 68, 68, 0.3);
  border-radius: 6px;
  color: #fecaca;
  font-size: 0.875rem;
  cursor: pointer;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.alert-toggle:hover {
  background: rgba(239, 68, 68, 0.3);
}

/* Success message */
.success-message {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.2);
  border-radius: 12px;
  padding: 2rem;
}

.success-icon {
  font-size: 3rem;
  flex-shrink: 0;
}

.success-content h3 {
  margin: 0 0 0.5rem 0;
  color: #10b981;
  font-size: 1.5rem;
  font-weight: 700;
}

.success-summary {
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
  font-size: 1rem;
  line-height: 1.5;
}

.success-warnings {
  margin: 0;
  color: #f59e0b;
  font-size: 0.875rem;
}

/* Clean logs section */
.logs-section {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  overflow: hidden;
}

.logs-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  background: rgba(255, 255, 255, 0.02);
}

.logs-header h4 {
  margin: 0;
  color: var(--text-primary);
  font-size: 1.125rem;
  font-weight: 600;
}

.log-controls {
  display: flex;
  gap: 0.5rem;
}

.control-btn {
  padding: 0.5rem 1rem;
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 6px;
  color: var(--text-secondary);
  font-size: 0.875rem;
  cursor: pointer;
  transition: all 0.2s ease;
}

.control-btn:hover,
.control-btn.active {
  background: rgba(59, 130, 246, 0.2);
  border-color: rgba(59, 130, 246, 0.3);
  color: #60a5fa;
}

.logs-container {
  max-height: calc(50vh - 120px);
  min-height: 150px;
  overflow-y: auto;
  padding: 1rem;
}

.logs-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem 1rem;
  text-align: center;
}

.empty-icon {
  font-size: 2.5rem;
  margin-bottom: 1rem;
  opacity: 0.5;
}

.logs-empty p {
  margin: 0;
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.log-entry {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
  padding: 1rem;
  background: rgba(255, 255, 255, 0.02);
  border-radius: 8px;
  margin-bottom: 0.5rem;
  border-left: 3px solid transparent;
  transition: all 0.2s ease;
}

.log-entry:hover {
  background: rgba(255, 255, 255, 0.05);
}

.log-time {
  color: var(--text-subtle);
  font-size: 0.8rem;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  flex-shrink: 0;
  width: 80px;
}

.log-icon {
  font-size: 1rem;
  flex-shrink: 0;
  width: 20px;
  text-align: center;
}

.log-content {
  flex: 1;
  min-width: 0;
}

.log-message {
  color: var(--text-primary);
  font-size: 0.9rem;
  line-height: 1.4;
  margin-bottom: 0.25rem;
}

.log-details {
  color: var(--text-secondary);
  font-size: 0.8rem;
  line-height: 1.3;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  word-break: break-all;
}

/* Log entry status styling */
.log-success {
  border-left-color: #10b981;
}

.log-error {
  border-left-color: #ef4444;
}

.log-warning {
  border-left-color: #f59e0b;
}

.log-processing,
.log-info {
  border-left-color: #3b82f6;
}

/* Animations */
@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

@keyframes progress-glow {
  0% {
    transform: translateX(-100%);
  }
  50% {
    transform: translateX(0%);
  }
  100% {
    transform: translateX(100%);
  }
}

/* Scrollbar styling */
.progress-content::-webkit-scrollbar,
.logs-container::-webkit-scrollbar {
  width: 8px;
}

.progress-content::-webkit-scrollbar-track,
.logs-container::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 4px;
}

.progress-content::-webkit-scrollbar-thumb,
.logs-container::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.2);
  border-radius: 4px;
}

.progress-content::-webkit-scrollbar-thumb:hover,
.logs-container::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.3);
}

/* Ensure smooth scrolling */
.progress-content {
  scroll-behavior: smooth;
}

/* Responsive design */
@media (max-width: 1024px) {
  .progress-header {
    flex-direction: column;
    align-items: stretch;
    gap: 1.5rem;
  }
  
  .scan-status {
    align-items: center;
  }
  
  .header-actions {
    justify-content: center;
    flex-wrap: wrap;
  }
}

@media (max-width: 768px) {
  .scan-progress-component {
    max-height: 90vh;
  }
  
  .progress-header {
    padding: 1.5rem;
    gap: 1rem;
  }
  
  .scan-status {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.75rem;
  }
  
  .status-text h2 {
    font-size: 1.25rem;
  }
  
  .header-actions {
    justify-content: flex-start;
  }
  
  .progress-content {
    padding: 1.5rem;
  }
  
  .progress-main {
    padding: 1.5rem;
  }
  
  .stats-section {
    grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
    gap: 1rem;
  }
  
  .stat-card {
    padding: 1rem;
  }
  
  .stat-value {
    font-size: 1.5rem;
  }
}

/* For very tall content, ensure the logs section doesn't grow too much */
.logs-section {
  max-height: 50vh;
  min-height: 200px;
}
</style> 