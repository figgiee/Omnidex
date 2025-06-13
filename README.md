Omnidex is a desktop application designed to help Unreal Engine developers organize, browse, and edit their local asset libraries. It scans your folders and automatically fetches data from the Orbital Market to give you a comprehensive overview of your collection.

## Key Features

-   **Fast Local Indexing:** Quickly scan directories containing your Unreal Engine assets.
-   **Marketplace Enrichment:** Automatically pulls metadata, thumbnails, and details from the Orbital Market.
-   **Powerful Search & Filtering:** Instantly find any asset across your library.
-   **Flexible UI:** A three-pane layout with draggable dividers to customize your workspace.
-   **Offline First:** Once an asset is matched, its data is cached locally for instant access.

## Getting Started

1.  **Open Settings:** Click the gear icon (⚙️) in the top-right corner of the application.
2.  **Add a Folder:** In the "Scan Locations" section, click **Add Folder to Scan** and choose the main directory where you store your assets.
3.  **Scan Your Assets:** Click the **Rescan** button next to your newly added location. Omnidex will begin indexing your library.

## ⚠️ Important Notes & Tips

Please read these notes for the best experience.

### On Recursive Scanning

For top-level asset directories (e.g., `D:/MyUnrealAssets`), it is **highly recommended to disable recursive scanning**.

-   **Why?** A single Unreal Engine asset can contain thousands of files. Scanning recursively from a high-level folder can cause the application to make an unnecessary number of requests and checks, significantly slowing down the process. The scanner is designed to identify asset *folders*, not every single file inside them.
-   **How?** You can disable this by toggling the "Recursive" switch off for each location in the `Settings > Scan Locations` menu.

### How to Scan Your Library

The primary way to scan or re-scan your library is from the settings menu:

-   Go to **Settings > Scan Locations**.
-   Click the **Rescan** button next to the location you wish to update.
-   Click **Scan All Locations** to refresh your entire library.

### Forcing a UI Refresh

If the UI ever seems out of sync or you encounter a visual bug, you can force a hard reload of the application by pressing:

```
Ctrl + Shift + R
```

## Tech Stack

-   **Backend:** Rust, Tauri
-   **Frontend:** Vue.js, Pinia, TypeScript
-   **Database:** SQLite

## License

This project is licensed under the MIT License.
