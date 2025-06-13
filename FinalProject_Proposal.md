## **FINAL PROJECT PROPOSAL: Omnidex**

I propose to build a utility desktop application, named "Omnidex" designed to streamline the management of my large Unreal Engine asset collection. The app will have a fast Rust backend and a user-friendly interface built with [Vue.js](https://vuejs.org/), using Tauri to create a lightweight and efficient program with native webviews.

The application will allow me to:

1. **Index Local Assets:** Rapidly scan specified local directories recursively, and storing it in a local SQLite database for filtering/sorting.  
2. **Scrape Marketplace Data:** Query the FAB Marketplace online (or a similar reference like orbital-market.com) to retrieve detailed metadata for marketplace assets. Try and find out how the API works for FAB / Orbital Market.
3. **Match and Present:** Implement a robust matching algorithm to compare my indexed local assets with the scraped marketplace data, identifying potential matches, new versions, or updates. The application will present these matches (or unmatched assets) in a user-friendly GUI.  
4. **Export/Import:** Allow users to export and import their asset catalog in common formats like JSON or CSV.

The final project will be a runnable desktop application demonstrated through a live walkthrough, showcasing its scanning, matching, and browsing capabilities. The complete source code, including both Rust backend and Vue.js frontend, will be published on my GitHub repository.

### **PROJECT STEPS**

I have previously created this app with Python and I am interested in seeing how Rust will handle it. Personally I feel like apps that serve as a tool for something quick like this, should be light-weight and not as complex. I have seen how performant apps/frameworks/libraries built with Rust can be so I'm looking forward to this. 

**Phase 1: Rust Backend \- Local Asset Indexing Core**

**Goal:** Rebuild the fundamental local asset scanning logic in Rust. This includes recursive directory traversal, basic file/folder metadata extraction, and initial asset data structuring in Rust structs.

* **Learning/Figure Out:** Rust's ownership and borrowing system, asynchronous programming (async/await with tokio), file system operations (std::fs, walkdir crate), and data serialization/deserialization (serde). (The folder names of assets are not clean) . 
* **Deliverable:** A command-line Rust program that can scan a directory and output parsed asset data to the console or a file.

**Phase 2: Rust Backend \- Database Integration** 

**Goal:** Integrate the Rust backend with a local SQLite database. Implement the AssetRepository methods (add, update, get, delete assets, scan history) to persist data.

* **Learning/Figure Out:** SQL database interactions in Rust, error handling for database operations.  
* **Deliverable:** A Rust backend that can store and retrieve asset and scan history data persistently.

**Phase 3: Tauri & Frontend Integration**

* **Goal:** Set up Tauri to connect the existing Vue.js frontend with the new Rust backend. This involves exposing Rust functions as Tauri commands (\#\[tauri::command\]) and adjusting the Vue.js frontend to use window.\_\_TAURI\_\_.invoke() for backend communication. Try and implement real-time progress updates for scanning. (Async operations make this really important to include).
* **Learning/Figure Out:** Tauri's setup, Rust-to-JavaScript communication patterns, and adapting existing Vue.js components to the new IPC.  
* **Deliverable:** A runnable Tauri desktop application where the existing Vue.js UI can trigger the Rust-based asset scanner and receive progress updates, with data syncing and saving to the SQLite database.

**Phase 4: Rust Backend \- Marketplace Web Scraping (Approx. 2-3 weeks, potentially longer)**

* **Goal:** Implement the core web scraping functionality in Rust to retrieve marketplace asset data. This will involve making HTTP requests, parsing HTML, and potentially interacting with dynamically loaded content. Need to figure this out without getting IP banned/blacklisted.  
* **Learning/Figure Out:**  
  * **Static/API Scraping:** HTTP requests, scraper for HTML parsing..  
  * **Dynamic Content:** If the target marketplace relies heavily on JavaScript for content, I will need to investigate and learn Rust's equivalent to Puppeteer or explore other strategies to bypass JavaScript rendering (e.g., identifying underlying APIs). This is the most technically challenging part.  
  * **Rate Limiting & Error Handling:** Robust handling of network errors, rate limits, and anti-bot measures. If possible I could spread out the scraping, I would only realistically need to do 1 large scrape operation, then in the future it would be small incremental scrapes that should not be a problem.  
* **Deliverable:** A Rust module capable of searching the FAB marketplace and extracting structured MarketplaceAsset data.

**Phase 5: Rust Backend \- Matching Algorithm**

* **Goal:** Develop the fuzzy matching algorithm in Rust. This component will compare local asset names and metadata with marketplace data, calculating a confidence score and updating the matchStatus in the database.  
* **Learning/Figure Out:** Rust crates for string similarity, efficient data comparison techniques, and potentially parallel processing of matches.  
* **Deliverable:** A Rust service that can identify and update asset matches in the database.

**Phase 6: UI Enhancements & Polishing**

* **Goal:** Populate the AssetBrowserView.vue and UnmatchedAssetsView.vue with real data from the database. Implement filtering, sorting, and search functionalities in the frontend, backed by database queries in Rust. Refine the UI for a smooth user experience.  
* **Learning/Figure Out:** Vue.js data display patterns, efficient filtering/sorting implementation, and general UI/UX best practices within the existing design system.  
* **Deliverable:** A fully functional UI displaying local and matched assets, with basic filtering and sorting.

### **TOOLS AND METHODOLOGIES**

* **Primary Technologies:**  
  * **Backend:** Rust  
  * **Desktop Framework:** Tauri (utilizing native webviews)  
  * **Frontend:** Vue.js (Composition API, Vue Router, Pinia)  
* **Key Rust Crates:**  
  * rusqlite or sqlx: For SQLite database interactions.  
  * serde, serde\_json, csv: For data serialization/deserialization (JSON, CSV).  
  * walkdir or jwalk: For efficient recursive file system traversal.  
  * rayon: For parallelizing CPU-bound tasks (e.g., folder size calculation, potentially matching).  
  * reqwest: For asynchronous HTTP requests (basic web scraping/API calls).  
  * scraper: For parsing HTML and using CSS selectors (for static HTML scraping).  
  * headless\_chrome: (Conditional) If dynamic JavaScript rendering is a significant barrier for marketplace scraping.  
  * log, env\_logger: For logging.  
* **Methodologies:**  
  * **Incremental Development:** Build core functionality step-by-step.  
  * **Modular Design:** Separate concerns into distinct Rust modules and Vue.js components.  
  * **Performance First:** Prioritize Rust's native performance capabilities.  
  * **Secure IPC:** Adhere to Tauri's best practices for secure communication between frontend and backend.  
* **Resources for Help:**  
  * **Rust:** The Official Rust Book, Rust by Example, Rust documentation for specific crates, crates.io for discovering libraries.  
  * **Tauri:** Official Tauri Documentation, Tauri GitHub Discussions, Tauri Discord server (highly active community).  
  * **Vue.js:** Official Vue.js Documentation, Vue.js community forums.  
  * **Web Scraping:** scrapism.lav.io  Online tutorials and articles for Rust web scraping.  
  * **General:** Stack Overflow.

**PROCESS DOCUMENTATION**

I'll be documenting my process thoroughly using the following methods:

* **Git Commit Messages:** Regular, descriptive commit messages outlining changes, features implemented, bug fixes, and my reasoning behind major decisions.  
* **DEVLOG.md:** A dedicated markdown file in the root of my repository will serve as a detailed development journal. This will include:  
  * Detailing progress made against the proposed steps.  
  * Challenges encountered.  
  * Approaches taken to solve these problems  
  * UI thought process   
* **In-Code Comments:** Comments within the Rust source code and Vue.js components to explain complex logic, algorithms, and design choices.

### **WHY?**

This project is mainly focused on streamlining the creative process for complex software like Unreal Engine.. As a developer and artist, handling a large and ever-growing collection of Unreal Engine assets can become complicated and time-consuming. This tool is designed to automatically help find, organize, and keep track of metadata for assets, so users can spend less time on organizing and more time on creating new things.

While there are websites like Orbital-Market that exist or software, this project could eventually expand into other 3D / Game Development related software, Blender has a CLI that lets you generate thumbnails orthographically which can be used in a GUI like Omnidex to help artists find what they're looking for. The current native implementation for search/filtering on the FAB marketplace is poor, and is not reliable as a means of finding assets.

### **CONSTRAINT**

To keep the scope manageable while focusing on the core challenge:

* **Minimal Marketplace Data Scraping (Initially):** For the first iteration, I will limit the scraped marketplace data to a minimal, essential set of fields: asset name, marketplace ID, direct URL, and primary version/compatibility information. This allows me to focus on the robust web scraping and matching logic without getting bogged down in extracting every single detail (e.g., all screenshots, extensive description formatting, review comments). Additional data points can be added incrementally if time allows after the core functionality is solid.  
* **Prioritize Native Webview:** I will initially prioritize web scraping methods that leverage the native webview capabilities and more lightweight HTTP requests (reqwest \+ scraper). Heavy-duty headless browser automation with headless\_chrome will only be adopted if absolutely necessary for the target marketplace's dynamic content.

