# Copilot / AI contributor instructions — Study Helper

This file documents the current Study Helper Rust desktop app as of the latest edits. It explains the project layout, important files, UI behavior, storage contract, and practical guidance for making safe changes.

High-level summary
- Language & UI: Rust application using eframe/egui for the desktop GUI. Entry point: `src/main.rs` which launches `gui::StudyHelperApp::default()` via `eframe::run_native`.
- What the app does: in-memory study sets and flashcards with JSON import/export and class-folder based storage. The GUI provides a two-pane Study Sets view and a Flashcards view for reviewing cards.

Repository layout (key files)
- `src/main.rs` — app entry; constructs and runs `StudyHelperApp`.
- `src/gui.rs` — central GUI and application state. Contains the main panel, view switching, and the implementations of `flashcards_view`, `study_sets_view`, and `quiz_view`.
- `src/models/` — data models:
  - `flashcard.rs` — `Flashcard` (now derives `Clone`, `Serialize`, `Deserialize`).
  - `study_set.rs` — `StudySet` (derives `Clone`, `Serialize`, `Deserialize`).
  - `quiz.rs` — `Quiz` (minimal quiz model currently present).
- `src/storage/json_store.rs` — file-based JSON helpers: save/load a set, save into a class folder, load all sets from a class folder, import/export files, and list class folders.
- `src/ui/` — small view helpers (legacy/experimental): `sets_view.rs`, `quiz_view.rs` etc.
- `data/` — sample data like `sample_set.json` used by tests/examples.

Dependencies (in `Cargo.toml`)
- `eframe` (egui) — GUI framework (version in repo: 0.32.3).
- `serde` + `serde_json` — serialization for persisting StudySet / Flashcard.
- `sanitize-filename` — sanitize user-supplied set names when creating filenames.
- `rfd` — native file/folder pickers used for Browse dialogs.

Storage contract and helpers
- File layout: study sets are stored as JSON files under `<base_dir>/<class_name>/<sanitized_set_name>.json`.
- Important functions in `src/storage/json_store.rs`:
  - `save_study_set_to_file(study_set: &StudySet, file_path: &Path) -> io::Result<()>`
  - `load_study_set_from_file(file_path: &Path) -> io::Result<StudySet>`
  - `save_set_into_class_folder(base_dir: &Path, class_name: &str, set_name: &str, study_set: &StudySet) -> io::Result<PathBuf>`
  - `load_sets_from_class_folder(base_dir: &Path, class_name: &str) -> io::Result<Vec<StudySet>>`
  - `import_set_file_to_class(base_dir: &Path, class_name: &str, src_file: &Path) -> io::Result<PathBuf>`
  - `export_set_file(src_file: &Path, dst_file: &Path) -> io::Result<()>`
  - `list_class_folders(base_dir: &Path) -> io::Result<Vec<String>>`

GUI behavior and conventions
- Overall layout: The Study Sets view uses a two-column layout: left column (~1/3) for class/set selection and right column (~2/3) for editing/adding flashcards. A vertical separator separates them.
- Study Sets view specifics:
  - Class dropdown (left pane) lists class folders discovered under the chosen base path. A "Refresh classes" small button appears below the class ComboBox.
  - Set dropdown (left pane) lists sets loaded for the selected class; selecting a set populates the right pane for editing.
  - Create New Set is implemented as a pop-up window (an `egui::Window`) to avoid taking persistent layout space. The popup contains storage fields (base path, class name), Browse buttons (using `rfd::FileDialog`), Load class, Import/Export controls, and Create/Cancel buttons.
  - Save Set in the right pane persists the currently-selected set to the configured base/class folder via `save_set_into_class_folder`.
- Flashcards view specifics:
  - Shows the selected study set (chosen from a ComboBox). If a set is selected, the selected flashcard is displayed inside a bordered card with the question/answer centered.
  - Prev/Flip/Next buttons are placed below the card (outside the border) and control the current card index and flip state.
  - Hints are available via a collapsing panel below the controls.
- Alignment and pixel issues: recent edits rounded widths/heights and used explicit allocations to avoid egui's "Unaligned" debug marker (fractional pixel positions are avoided where possible).

UI state & data patterns
- `StudyHelperApp` holds all UI and domain state (study_sets: Vec<StudySet>, selected_set: Option<usize>, available_classes: Vec<String>, selected_class: Option<usize>, storage_base_path: String, storage_class_name: String, and many temporary fields like `new_question`, `new_hints`, etc.).
- Selection is index-based (Option<usize>); always check bounds before indexing (existing code does this).

Useful implementation notes for contributors
- Use `serde::{Serialize, Deserialize}` on model structs that need to be persisted. `Flashcard` and `StudySet` already derive them.
- For file/folder picking use `rfd::FileDialog` (the UI already uses `.pick_folder()` and `.pick_file()` for Browse actions).
- When saving filenames use `sanitize_filename::sanitize(set_name)` (already used in storage helpers).
- The storage functions are intentionally simple (copying/importing files and reading/writing JSON). If you need transactional behavior or locking, add it in `src/storage/json_store.rs`.

Build / run / test
- Build: `cargo build` from the repository root.
- Run: `cargo run` (PowerShell: run from the project folder). GUI runs as a native desktop window.
- Tests: `cargo test`. There is a storage test file `tests/storage_tests.rs` that expects `data/sample_set.json` to exist.

Known small issues / notes
- The codebase currently emits a few harmless compiler warnings (unused imports and a `mut` that can be removed). These do not affect runtime and can be cleaned up in a small follow-up patch.
- The GUI code has several manual layout allocations (egui `allocate_*` calls). If you refactor layout code, prefer using egui's `columns`/`Grid`/`Layout` helpers where possible to avoid pixel-alignment problems.

Where to make safe, incremental contributions
- Add features inside `src/gui.rs` (new buttons, save/load flows) — keep changes focused to a few functions.
- Add small model helpers/tests in `src/models/*` and `tests/`.
- Improve storage error messages or edge-case handling in `src/storage/json_store.rs`.

Checklist before opening a PR
- Build locally: `cargo build` (must succeed).
- Run tests: `cargo test`.
- Run the app with `cargo run` and smoke-test your UI changes.
- Keep changes small and explain the UI/behavior changes in the PR description.

If anything above is unclear or you want me to make a small cleanup (remove the unused `mut` and unused imports, or switch the two-pane layout to use egui columns), tell me which item and I'll make a focused patch.
