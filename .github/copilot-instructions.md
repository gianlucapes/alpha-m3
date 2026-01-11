# Copilot Instructions for alpha-m3

## Project Overview
- **alpha-m3** is a Tauri desktop app combining a Rust backend (physics engine, simulation logic) and a React/TypeScript frontend (UI, visualization).
- The project is modular: Rust code in `src-tauri/` (with `engine/`, `interface/`, `math/`), frontend in `src/`.
- Data flows from Rust to JS via Tauri commands/events (see `invoke`, `listen` in `App.tsx` and Rust `#[tauri::command]`).

## Key Directories & Files
- `src-tauri/` — Rust backend
  - `engine/` — simulation domain logic (physics, solver)
  - `interface/` — Tauri command handlers (e.g., `controller.rs`)
  - `math/` — math utilities (e.g., `vector.rs`)
  - `main.rs` — Tauri app entry, sets up state and handlers
  - `lib.rs` — additional Tauri commands
- `src/` — React/TypeScript frontend
  - `App.tsx` — main UI, canvas rendering, Tauri integration
  - `main.tsx` — React entrypoint
  - `assets/` — static assets
- `vite.config.ts` — Vite config, tailored for Tauri (fixed port, ignores `src-tauri`)
- `Cargo.toml` — Rust dependencies and build config

## Developer Workflows
- **Build & Run (Dev):**
  - `npm install` (frontend deps)
  - `cargo build` (Rust backend)
  - `npm run tauri dev` (launches full app with hot reload)
- **Production Build:**
  - `npm run tauri build`
- **Rust only:**
  - `cargo run` (backend logic, no UI)
- **Frontend only:**
  - `npm run dev` (UI only, no Rust integration)

## Patterns & Conventions
- **Rust <-> JS Communication:**
  - Use `invoke` (JS) to call Rust commands (annotated with `#[tauri::command]`).
  - Use `listen` (JS) to receive events from Rust (e.g., simulation updates).
- **State Management:**
  - React state for UI, simulation state from Rust via events.
- **Styling:**
  - CSS in `App.css`, custom styles for simulation canvas and controls.
- **Modularity:**
  - Rust: keep domain logic in `engine/`, interface logic in `interface/`.
  - JS: keep UI logic in `App.tsx`, types/interfaces at top of file.

## Integration Points
- **Tauri:**
  - All cross-language calls go through Tauri's `invoke`/`listen`.
  - Rust commands must be registered in `main.rs`/`lib.rs`.
- **Vite:**
  - Configured for Tauri (see `vite.config.ts`), uses strict port and disables watching `src-tauri`.

## Examples
- See `App.tsx` for how to start/stop simulation (`invoke('start_simulation')`, `invoke('stop_simulation')`), and how to listen for updates (`listen('update-physics', ...)`).
- See `src-tauri/src/vector.rs` for idiomatic Rust operator overloading and struct documentation.

## Tips
- When adding new Rust commands, annotate with `#[tauri::command]` and register in `main.rs`.
- When adding new frontend features, prefer functional React components and keep Tauri integration in one place.
- For cross-component changes, update both Rust and JS types/interfaces as needed.
