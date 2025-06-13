# Omnidex Development Log

> Development journal for Omnidex - Local Asset Organizer for Unreal Engine

## Project Overview

**Omnidex** is a desktop application built with Tauri (Rust + Vue.js) designed to help Unreal Engine developers organize and manage their local asset collections. The application features local asset indexing, smart search capabilities, and integration with the Orbital Market.

### Tech Stack
- **Backend**: Rust with Tauri 2.x
- **Frontend**: Vue.js 3 + TypeScript + Pinia
- **Database**: SQLite with SQLx
- **Build System**: Vite + pnpm
- **Styling**: SCSS with glassmorphism design

---

## Development Entries

### May 24, 2025 - Project Initialization
Initialized Tauri project with Vue.js + TypeScript template. Designed core database schema with three main tables: `scan_locations` (directories to track), `assets` (asset metadata), and `app_settings` (configuration). Set up SQLx with initial migration files.

### May 27, 2025 - Frontend Foundation
Set up Vue Router and Pinia state management. Created initial components: `ScanLocations` list and main `AssetGrid` view. Established basic UI structure needed to test backend functionality and display data.

### May 30, 2025 - File Scanning System
Implemented core asset scanner using `jwalk` for parallel directory traversal. Added background thread execution with `tauri::async_runtime::spawn` and cancellation logic using `Arc<AtomicBool>` for graceful scan termination. Scanner discovers assets and inserts them into database.

### June 1, 2025 - Frontend-Backend Integration
Wired frontend components to backend commands (`get_scan_locations`, `get_assets`) for basic data display. Fixed critical integration issues: type mismatch in `location_id` passing and non-reactive asset grid updates during scans. Refactored event listener logic in Pinia store for proper real-time updates.

### June 3, 2025 - Core MVP Complete
Achieved functional MVP: users can add scan locations, start scans, and view assets populating in real-time. Basic proof-of-concept complete with end-to-end scanning and display workflow working reliably.

### June 4, 2025 - Orbital Market Integration
Started Orbital Market scraping system. Implemented URL construction from folder names, HTML fetching with `reqwest`, and page parsing with `scraper` crate. Initial approach extracts asset names, descriptions, and thumbnail URLs from product pages.

### June 5, 2025 - Scraper Refinement
Enhanced scraper accuracy through improved folder-name-to-URL mapping. Added regex patterns and string manipulation for better URL construction. Implemented robust CSS selectors and error handling to prevent parsing failures. Achieved ~70% success rate on test assets with marketplace data enrichment.

### June 9, 2025 - UI/UX Overhaul
Complete interface redesign with three-pane layout (Source List, Asset Grid, Inspector) and draggable dividers. Migrated to dark theme (`#1A1A1A` base) with `Inter` font. Built standardized component library: `AppButton`, `AppSelect`, `TagPill`, `SkeletonLoader`. Added `/style-guide` route for component documentation. Rebuilt main views with new components and created dedicated Pinia stores for each pane.

### June 10, 2025 - Interface Polish & Unification
Unified application into single asset browser view with persistent header featuring centered global search and settings button. Implemented `AppFooter` with live database statistics via `get_database_stats` command. Created "Recent Assets" smart folder and contextual `ActionBar` for selected assets. Developed reusable `ModalDialog` component and centralized `SettingsModal`.

### June 11, 2025 - Tauri v2 Migration
Fixed Vite build error from Tauri v2 API changes. Updated import paths from `@tauri-apps/api/tauri` to `@tauri-apps/api/core` in `statusBarStore.ts`. Verified no other legacy import paths across codebase.

### June 12, 2025 - Advanced Features & Batch Operations
**Inspector Panel**: Complete rebuild with high-resolution previews, marketplace data (ratings, creator info), scrollable descriptions, and distinct "My Tags" vs "Scraped Tags" sections. Added unmatched asset states and skeleton loading.

**Batch Operations**: Implemented multi-select in `AssetGrid` with `Ctrl+Click` and `Shift+Click`. Added functional `ActionBar` with batch commands: `toggle_favorite_status`, `add_tags_to_assets`, and `delete_assets` with confirmation dialogs.

**Manual Matching Workflow**: Fixed manual matching system where users can provide Orbital Market URLs for unmatched assets. Resolved data mapping issues between Rust camelCase serialization and TypeScript expectations.

**Scan Management**: Integrated complete scan workflow in Settings Modal with Add/Remove/Rescan for individual directories and "Scan All Locations" button. Added `ScanStatusIndicator` in footer with real-time progress display during scans.

v1.0 is Ready!