// src/gui.rs
use eframe::{egui, App, Frame};
use egui::{TextStyle, FontId, RichText, Vec2};
use crate::models::{Flashcard as AppFlashcard, StudySet as AppStudySet, Quiz as AppQuiz};
use crate::storage::{load_config, save_config, list_class_folders};
use rfd::FileDialog;

pub struct StudyHelperApp {
    current_view: AppView,
    // in-memory study sets (not yet persisted)
    study_sets: Vec<AppStudySet>,

    // UI state for creating/selecting sets
    selected_set: Option<usize>,
    new_set_name: String,
    show_create_set_popup: bool,
    // class dropdown state
    available_classes: Vec<String>,
    selected_class: Option<usize>,
    // cache of last-known storage settings to avoid repeated reloads on every frame
    last_storage_base_path: String,
    last_storage_class_name: String,
    // storage/state for import/export
    storage_base_path: String,
    storage_class_name: String,
    import_file_path: String,
    export_dest_path: String,
    status_message: String,

    // UI state for creating a flashcard
    new_question: String,
    new_answer: String,
    new_hint_input: String,
    new_hints: Vec<String>,
    // flashcard viewing state
    current_card_index: usize,
    card_flipped: bool,
    show_hint: bool,
    // quiz UI state
    selected_quiz: Option<usize>,
    new_quiz_name: String,
    show_create_quiz_popup: bool,
    // whether the focused quiz window is open (separate distraction-free window)
    show_quiz_window: bool,
    // counts for placeholder question creation
    new_quiz_mc_count: usize,
    new_quiz_tf_count: usize,
    new_quiz_sa_count: usize,
    new_quiz_mb_count: usize,
}

enum AppView {
    Flashcards,
    StudySets,
    Quiz,
}

impl Default for StudyHelperApp {
    fn default() -> Self {
        // Start with defaults, then try to load persisted configuration and sets.
        let mut storage_base_path = String::new();
        let mut storage_class_name = String::new();
        let mut available_classes: Vec<String> = Vec::new();
        let mut selected_class: Option<usize> = None;
        let mut study_sets: Vec<AppStudySet> = Vec::new();
        let mut selected_set: Option<usize> = None;
        let status_message = String::new();

        if let Ok(cfg) = load_config() {
            storage_base_path = cfg.storage_base_path.clone();
            storage_class_name = cfg.storage_class_name.clone();
            if !storage_base_path.trim().is_empty() {
                if let Ok(list) = list_class_folders(std::path::Path::new(&storage_base_path)) {
                    available_classes = list;
                        if !available_classes.is_empty() {
                        // try to find index for configured class name
                        if !storage_class_name.trim().is_empty() {
                            if let Some(pos) = available_classes.iter().position(|s| s == &storage_class_name) {
                                selected_class = Some(pos);
                                if let Some(class_name) = available_classes.get(pos) {
                                    if let Ok(sets) = crate::storage::load_sets_from_class_folder(std::path::Path::new(&storage_base_path), class_name) {
                                        study_sets = sets;
                                        if !study_sets.is_empty() { selected_set = Some(0); }
                                    }
                                }
                            } else {
                                // fallback to first class
                                selected_class = Some(0);
                            }
                        } else {
                            selected_class = Some(0);
                        }
                    }
                }
            }
        }
        // Prepare cached copies of the storage path values before moving them into the struct
        let last_storage_base_path = storage_base_path.clone();
        let last_storage_class_name = storage_class_name.clone();

        Self {
            current_view: AppView::Flashcards,
            study_sets,
            selected_set,
            new_set_name: String::new(),
            new_question: String::new(),
            new_answer: String::new(),
            new_hint_input: String::new(),
            new_hints: Vec::new(),
            current_card_index: 0,
            card_flipped: false,
            show_hint: false,
            selected_quiz: None,
            new_quiz_name: String::new(),
            show_create_quiz_popup: false,
            show_quiz_window: false,
            new_quiz_mc_count: 0,
            new_quiz_tf_count: 0,
            new_quiz_sa_count: 0,
            new_quiz_mb_count: 0,
            storage_base_path,
            storage_class_name,
            import_file_path: String::new(),
            export_dest_path: String::new(),
            status_message,
            show_create_set_popup: false,
            available_classes,
            selected_class,
            last_storage_base_path,
            last_storage_class_name,
        }
    }
}

impl App for StudyHelperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Validate storage paths and in-memory indexes against the filesystem
        self.ensure_storage_consistency();
        // Compute a simple scale factor based on available width so UI scales on large displays.
        let available = ctx.available_rect().size();
        let base_width = 900.0_f32; // tweakable "design" width
        let mut scale = (available.x / base_width).max(0.7).min(3.0);
        if !scale.is_finite() || scale <= 0.0 {
            scale = 1.0;
        }

        // Update style to scale text sizes and spacing.
        // Some egui versions expose only immutable access, so clone, modify and set back.
        let mut style = (*ctx.style()).clone();
        style.text_styles.insert(TextStyle::Heading, FontId::proportional(28.0 * scale));
        style.text_styles.insert(TextStyle::Body, FontId::proportional(16.0 * scale));
        style.text_styles.insert(TextStyle::Button, FontId::proportional(18.0 * scale));
        style.spacing.button_padding = Vec2::new((10.0 * scale).round(), (6.0 * scale).round());
        style.spacing.item_spacing = Vec2::new((8.0 * scale).round(), (8.0 * scale).round());
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space((8.0 * scale).round());
                ui.label(RichText::new("Welcome to Study Helper!").heading());
                ui.add_space((8.0 * scale).round());

                let btn_size = Vec2::new((140.0 * scale).round(), (48.0 * scale).round());

                ui.horizontal(|ui| {
                    if ui.add_sized(btn_size, egui::Button::new(RichText::new("Flashcards").size(18.0 * scale))).clicked() {
                        self.current_view = AppView::Flashcards;
                    }
                    if ui.add_sized(btn_size, egui::Button::new(RichText::new("Study Sets").size(18.0 * scale))).clicked() {
                        self.current_view = AppView::StudySets;
                    }
                    if ui.add_sized(btn_size, egui::Button::new(RichText::new("Quiz").size(18.0 * scale))).clicked() {
                        self.current_view = AppView::Quiz;
                    }
                });

                ui.separator();
                ui.add_space((8.0 * scale).round());

                match self.current_view {
                    AppView::Flashcards => self.flashcards_view(ui, scale),
                    AppView::StudySets => self.study_sets_view(ui, scale),
                    AppView::Quiz => self.quiz_view(ui, scale),
                }
            });
        });
    }
}

impl StudyHelperApp {
    /// Ensure storage paths and indexes are consistent with the filesystem state.
    /// If the configured base folder or class folder no longer exist, this will
    /// clear or repair in-memory lists to avoid out-of-bounds indexing and crashes.
    fn ensure_storage_consistency(&mut self) {
        // If no base path configured, nothing to do.
        if self.storage_base_path.trim().is_empty() {
            self.available_classes.clear();
            self.selected_class = None;
            self.study_sets.clear();
            self.selected_set = None;
            return;
        }

        let base = std::path::Path::new(&self.storage_base_path);
        if !base.exists() {
            // Base folder disappeared since last configured
            self.available_classes.clear();
            self.selected_class = None;
            self.study_sets.clear();
            self.selected_set = None;
            self.status_message = "Configured base folder no longer exists".to_string();
            return;
        }

        // Only refresh the list of class folders when the base path actually changes.
        if self.storage_base_path != self.last_storage_base_path {
            match crate::storage::list_class_folders(base) {
                Ok(list) => {
                    self.available_classes = list;
                    // update cache
                    self.last_storage_base_path = self.storage_base_path.clone();
                }
                Err(e) => {
                    self.available_classes.clear();
                    self.selected_class = None;
                    self.study_sets.clear();
                    self.selected_set = None;
                    self.status_message = format!("Error listing classes: {}", e);
                    return;
                }
            }
        }

        if self.available_classes.is_empty() {
            self.selected_class = None;
            self.study_sets.clear();
            self.selected_set = None;
            return;
        }

        // Try to resolve the configured class name to an index if possible.
        if !self.storage_class_name.trim().is_empty() {
            if let Some(pos) = self.available_classes.iter().position(|s| s == &self.storage_class_name) {
                self.selected_class = Some(pos);
            } else {
                // Configured class no longer present; choose first class and clear sets.
                self.selected_class = Some(0);
                self.storage_class_name = self.available_classes[0].clone();
                self.study_sets.clear();
                self.selected_set = None;
                self.status_message = "Configured class not found; switched to first available class".to_string();
            }
        } else if let Some(idx) = self.selected_class {
            // ensure index is still within range
            if idx >= self.available_classes.len() {
                self.selected_class = Some(0);
            }
        } else {
            // No selected class recorded; pick first available
            self.selected_class = Some(0);
        }

        // If a class is selected, try loading sets (but handle errors gracefully)
        // Only load sets from disk when the selected class changes or when the configured
        // class name has changed since the last successful load. This avoids reloading
        // on every UI frame (which was generating repeated log messages).
        if let Some(idx) = self.selected_class {
            if idx < self.available_classes.len() {
                let class_name = &self.available_classes[idx];
                if self.storage_class_name != self.last_storage_class_name || self.study_sets.is_empty() {
                    match crate::storage::load_sets_from_class_folder(base, class_name) {
                        Ok(sets) => {
                            self.study_sets = sets;
                            // update cache so we don't reload repeatedly
                            self.last_storage_class_name = self.storage_class_name.clone();
                            if self.study_sets.is_empty() {
                                self.selected_set = None;
                            } else {
                                // clamp selected set index
                                if let Some(sidx) = self.selected_set {
                                    if sidx >= self.study_sets.len() { self.selected_set = Some(0); }
                                } else {
                                    self.selected_set = Some(0);
                                }
                            }
                        }
                        Err(e) => {
                            self.study_sets.clear();
                            self.selected_set = None;
                            self.status_message = format!("Error loading sets: {}", e);
                        }
                    }
                }
            }
        }
    }

    fn flashcards_view(&mut self, ui: &mut egui::Ui, scale: f32) {
        ui.label(RichText::new("Flashcards View").heading());
    ui.add_space((6.0 * scale).round());

        // Select a study set to review
        ui.horizontal(|ui| {
            ui.label("Study set:");
                if self.study_sets.is_empty() {
                ui.label(RichText::new("(no sets yet)").italics());
            } else {
                let labels: Vec<String> = self.study_sets.iter().map(|s| s.name().to_string()).collect();
                let mut selected = self.selected_set.unwrap_or(0);
                // clamp selected index to valid range
                if !labels.is_empty() {
                    if selected >= labels.len() { selected = labels.len() - 1; }
                } else {
                    selected = 0;
                }
                egui::ComboBox::from_id_salt("study_set_select")
                    .selected_text(if labels.is_empty() { "(none)" } else { &labels[selected] })
                    .show_ui(ui, |ui| {
                        for (i, label) in labels.iter().enumerate() {
                            ui.selectable_value(&mut selected, i, label);
                        }
                    });
                // detect selection change
                if self.selected_set != Some(selected) {
                    self.selected_set = Some(selected);
                    self.current_card_index = 0;
                    self.card_flipped = false;
                    self.show_hint = false;
                }
            }
        });

    ui.add_space((8.0 * scale).round());

        // Show selected flashcard as flippable card
        if let Some(idx) = self.selected_set {
            if idx < self.study_sets.len() {
                let set = &self.study_sets[idx];
                let cards = set.get_all_flashcards();
                if cards.is_empty() {
                    ui.label("This set has no flashcards.");
                    return;
                }

                // clamp current index
                if self.current_card_index >= cards.len() {
                    self.current_card_index = 0;
                }

                let card = &cards[self.current_card_index];

                // Card display box: allocate an exact, pixel-aligned rect and draw a border.
                let card_w = ui.available_width().round();
                let card_h = ((80.0 * scale).max(60.0)).round();
                let (card_rect, _resp) = ui.allocate_exact_size(Vec2::new(card_w.round(), card_h.round()), egui::Sense::hover());
                // draw a subtle border around the card using four line segments
                let stroke = egui::Stroke::new(1.0, egui::Color32::from_gray(100));
                let tl = card_rect.left_top();
                let tr = card_rect.right_top();
                let bl = card_rect.left_bottom();
                let br = card_rect.right_bottom();
                ui.painter().line_segment([tl, tr], stroke);
                ui.painter().line_segment([tr, br], stroke);
                ui.painter().line_segment([br, bl], stroke);
                ui.painter().line_segment([bl, tl], stroke);

                // Center the question/answer text inside the rect using the painter to avoid child UI alignment issues
                let text = if self.card_flipped { card.answer() } else { card.question() };
                let font_id = FontId::proportional((24.0 * scale).round());
                ui.painter().text(card_rect.center(), egui::Align2::CENTER_CENTER, text, font_id, ui.visuals().text_color());

                // Buttons outside the card (below it)
                ui.add_space((6.0 * scale).round());
                ui.horizontal(|ui_h| {
                    if ui_h.button("Prev").clicked() {
                        if self.current_card_index == 0 {
                            self.current_card_index = cards.len() - 1;
                        } else {
                            self.current_card_index -= 1;
                        }
                        self.card_flipped = false;
                        self.show_hint = false;
                    }
                    if ui_h.button(if self.card_flipped { "Show Question" } else { "Flip" }).clicked() {
                        self.card_flipped = !self.card_flipped;
                    }
                    if ui_h.button("Next").clicked() {
                        self.current_card_index = (self.current_card_index + 1) % cards.len();
                        self.card_flipped = false;
                        self.show_hint = false;
                    }
                });

                ui.add_space((6.0 * scale).round());
                if ui.button(if self.show_hint { "Hide Hint" } else { "Show Hint" }).clicked() {
                    self.show_hint = !self.show_hint;
                }
                if self.show_hint {
                    ui.collapsing("Hints", |ui| {
                        if card.get_hints().is_empty() {
                            ui.label("No hints available.");
                        } else {
                            for h in card.get_hints() {
                                ui.label(h);
                            }
                        }
                    });
                }
            }
        } else {
            ui.label("Select a study set in the Study Sets view or create one there.");
        }
    }

    fn study_sets_view(&mut self, ui: &mut egui::Ui, scale: f32) {
        ui.label(RichText::new("Study Sets View").heading());
    ui.add_space((6.0 * scale).round());
    // Split the view into two vertical sections (left ~1/3 width, right ~2/3)
    let avail = ui.available_size();
    let total_width = avail.x.round();
    let avail_height = avail.y.round();
    let left_width = ((total_width * 0.33).max(200.0 * scale)).round();
    let right_width = ((total_width - left_width - 8.0 * scale).max(200.0 * scale)).round();

    // A small deferred removal slot so we can mutate `self.study_sets` after
    // the UI closures (avoids borrow conflicts between left/right panes).
    let mut to_remove_card: Option<(usize, usize)> = None; // (set_idx, card_idx)

    ui.horizontal(|ui| {
            // LEFT: study set selection, delete button, and flashcard selector (about 1/3 width)
            let left_height = avail_height;
            ui.allocate_ui_with_layout(egui::Vec2::new(left_width.round(), left_height.round()), egui::Layout::top_down(egui::Align::Min), |ui_left| {
                ui_left.label("Class:");
                // Class dropdown populated from available_classes; allow refresh
                let classes = self.available_classes.clone();
                let mut sel_class = self.selected_class.unwrap_or(0);
                // clamp sel_class to available classes length
                if !classes.is_empty() {
                    if sel_class >= classes.len() { sel_class = classes.len() - 1; }
                } else {
                    sel_class = 0;
                }
                // Class ComboBox (single-line)
                egui::ComboBox::from_id_salt("class_select")
                    .selected_text(if classes.is_empty() { "(none)" } else { &classes[sel_class] })
                    .show_ui(ui_left, |ui| {
                        for (i, label) in classes.iter().enumerate() {
                            ui.selectable_value(&mut sel_class, i, label);
                        }
                    });

                // Put the Refresh button below the ComboBox to reduce horizontal clutter.
                ui_left.add_space((4.0 * scale).round());
                if ui_left.small_button("Refresh classes").clicked() {
                    if !self.storage_base_path.trim().is_empty() {
                        if let Ok(list) = crate::storage::list_class_folders(std::path::Path::new(&self.storage_base_path)) {
                            self.available_classes = list;
                            if !self.available_classes.is_empty() {
                                self.selected_class = Some(0);
                            }
                        }
                        // re-check consistency after refreshing
                        self.ensure_storage_consistency();
                    }
                }

                // apply selection if changed
                if self.selected_class != Some(sel_class) {
                    self.selected_class = Some(sel_class);
                    // load sets for this class
                    if !self.storage_base_path.trim().is_empty() {
                        if !self.available_classes.is_empty() && sel_class < self.available_classes.len() {
                            if let Ok(sets) = crate::storage::load_sets_from_class_folder(std::path::Path::new(&self.storage_base_path), &self.available_classes[sel_class]) {
                                self.study_sets = sets;
                                if self.study_sets.is_empty() {
                                    self.selected_set = None;
                                } else {
                                    self.selected_set = Some(0);
                                }
                            }
                            // persist the chosen class name
                            self.storage_class_name = self.available_classes[sel_class].clone();
                            let _ = save_config(&crate::storage::Config { storage_base_path: self.storage_base_path.clone(), storage_class_name: self.storage_class_name.clone() });
                        } else {
                            // no available classes: clear sets and selection
                            self.study_sets.clear();
                            self.selected_set = None;
                        }
                    }
                }

                ui_left.add_space((6.0 * scale).round());
                ui_left.label("Set:");
                if self.study_sets.is_empty() {
                    ui_left.label(RichText::new("(no sets yet)").italics());
                } else {
                    let labels: Vec<String> = self.study_sets.iter().map(|s| s.name().to_string()).collect();
                    let mut selected = self.selected_set.unwrap_or(0);
                    ui_left.horizontal(|ui_h| {
                        egui::ComboBox::from_id_salt("study_set_select")
                            .selected_text(if labels.is_empty() { "(none)" } else { &labels[selected] })
                            .show_ui(ui_h, |ui| {
                                for (i, label) in labels.iter().enumerate() {
                                    ui.selectable_value(&mut selected, i, label);
                                }
                            });
                        // When the user changes the selection in the ComboBox, apply it to the app state.
                        if labels.len() > 0 {
                            if self.selected_set != Some(selected) {
                                // ensure selected index is valid
                                let sel = if selected >= labels.len() { labels.len() - 1 } else { selected };
                                self.selected_set = Some(sel);
                                self.current_card_index = 0;
                                self.card_flipped = false;
                                self.show_hint = false;
                            }
                        } else {
                            self.selected_set = None;
                        }
                    }); // end study_set_select horizontal

                    // Flashcard selector for the selected set
                            if let Some(idx) = self.selected_set {
                        if idx < self.study_sets.len() {
                            let set = &self.study_sets[idx];
                            let cards = set.get_all_flashcards();
                            let card_labels: Vec<String> = cards.iter().enumerate().map(|(i, c)| format!("{}: {}", i + 1, c.question())).collect();
                            if cards.is_empty() {
                                ui_left.label(RichText::new("(no flashcards)").italics());
                            } else {
                                let mut sel_card = self.current_card_index.min(cards.len() - 1);
                                egui::ComboBox::from_id_salt("flashcard_select")
                                    .selected_text(&card_labels[sel_card])
                                    .show_ui(ui_left, |ui| {
                                        for (i, label) in card_labels.iter().enumerate() {
                                            ui.selectable_value(&mut sel_card, i, label);
                                        }
                                    });

                                // Apply selection change
                                if sel_card != self.current_card_index {
                                    self.current_card_index = sel_card;
                                    self.card_flipped = false;
                                    self.show_hint = false;
                                }

                                // Edit button: pre-fill the add/edit form on the right
                                ui_left.horizontal(|ui_h| {
                                    if ui_h.small_button("Edit").clicked() {
                                        if self.current_card_index < cards.len() {
                                            let c = &cards[self.current_card_index];
                                            self.new_question = c.question().to_string();
                                            self.new_answer = c.answer().to_string();
                                            self.new_hints = c.get_hints().clone();
                                            // switch focus to right side by ensuring selection remains
                                            // (no explicit focus API here)
                                        }
                                    }

                                    // Delete the currently selected flashcard. We defer the
                                    // actual mutation until after the UI closures to avoid
                                    // borrowing `self.study_sets` mutably while it is already
                                    // immutably borrowed for rendering.
                                    if ui_h.small_button("Delete").clicked() {
                                        if self.current_card_index < cards.len() {
                                            to_remove_card = Some((idx, self.current_card_index));
                                        }
                                    }
                                });
                            }
                        }
                    }
                }
            });


        // Perform deferred removal (if any). Do this after UI code to avoid borrow conflicts.
        if let Some((set_idx, card_idx)) = to_remove_card {
            if set_idx < self.study_sets.len() {
                // remove the card and update indices/state
                if let Some(removed) = self.study_sets[set_idx].remove_flashcard(card_idx) {
                    // clamp current card index for the currently selected set
                    if Some(set_idx) == self.selected_set {
                        let remaining = self.study_sets[set_idx].get_all_flashcards().len();
                        if remaining == 0 {
                            self.current_card_index = 0;
                        } else if self.current_card_index >= remaining {
                            self.current_card_index = remaining - 1;
                        }
                    }

                    self.status_message = format!("Removed flashcard: '{}'", removed.question());
                    log::info!("Removed flashcard '{}' from set '{}'", removed.question(), self.study_sets[set_idx].name());

                    // Persist immediately if storage is configured so the change is not lost
                    if !self.storage_base_path.trim().is_empty() && !self.storage_class_name.trim().is_empty() {
                        let base = std::path::Path::new(&self.storage_base_path);
                        match crate::storage::save_set_into_class_folder(base, &self.storage_class_name, self.study_sets[set_idx].name(), &self.study_sets[set_idx]) {
                            Ok(p) => self.status_message = format!("Removed and saved: {}", p.display()),
                            Err(e) => self.status_message = format!("Removed but save failed: {}", e),
                        }
                    }
                }
            }
        }
            // vertical separator between left and right panes
            ui.add(egui::Separator::default().vertical());
            ui.add_space((8.0 * scale).round());

            // RIGHT: everything else (new set creation + add/edit flashcard form)
            let right_height = avail_height.round();
            ui.allocate_ui_with_layout(egui::Vec2::new(right_width.round(), right_height), egui::Layout::top_down(egui::Align::Min), |ui_right| {
                // Create new set via pop-up so it doesn't consume workspace
                if ui_right.button("Create New Set").clicked() {
                    self.show_create_set_popup = true;
                    self.new_set_name.clear();
                }

                ui_right.add_space((8.0 * scale).round());
                // If a set is selected, show add-card (or edit pre-filled) form on the right
                if let Some(idx) = self.selected_set {
                    if idx < self.study_sets.len() {
                        let set = &mut self.study_sets[idx];

                        ui_right.label(RichText::new(format!("Set: {} ({} cards)", set.name(), set.get_all_flashcards().len())).heading());
                        ui_right.add_space((6.0 * scale).round());

                        ui_right.label(RichText::new("Add / Edit flashcard").heading());
                        ui_right.add_space((4.0 * scale).round());

                        ui_right.label("Question:");
                        ui_right.text_edit_singleline(&mut self.new_question);
                        ui_right.label("Answer:");
                        ui_right.text_edit_singleline(&mut self.new_answer);

                        ui_right.horizontal(|ui_h| {
                            ui_h.label("Hint:");
                            ui_h.text_edit_singleline(&mut self.new_hint_input);
                            if ui_h.button("Add Hint").clicked() {
                                let hint = self.new_hint_input.trim().to_string();
                                if !hint.is_empty() {
                                    self.new_hints.push(hint);
                                }
                                self.new_hint_input.clear();
                            }
                        });

                        if !self.new_hints.is_empty() {
                            ui_right.label("Hints:");
                            let mut to_remove: Option<usize> = None;
                            for (i, h) in self.new_hints.iter().enumerate() {
                                ui_right.horizontal(|ui_h| {
                                    ui_h.label(format!("{}.", i + 1));
                                    ui_h.label(h);
                                    if ui_h.small_button("x").clicked() {
                                        to_remove = Some(i);
                                    }
                                });
                            }
                            if let Some(i) = to_remove {
                                if i < self.new_hints.len() {
                                    self.new_hints.remove(i);
                                }
                            }
                        }

                        ui_right.add_space((6.0 * scale).round());
                        ui_right.horizontal(|ui_h| {
                            let can_add = !self.new_question.trim().is_empty() && !self.new_answer.trim().is_empty();
                            if ui_h.add_enabled(can_add, egui::Button::new("Add Flashcard")).clicked() {
                                // Build the card and push it into the selected set
                                let q = self.new_question.trim().to_string();
                                let a = self.new_answer.trim().to_string();
                                let hints = self.new_hints.clone();
                                let card = AppFlashcard::new(q.clone(), a.clone(), hints.clone());
                                set.add_flashcard(card);
                                // select the newly added card so the user sees it immediately
                                self.current_card_index = set.get_all_flashcards().len().saturating_sub(1);
                                self.card_flipped = false;
                                self.show_hint = false;
                                // Try to persist the updated set immediately so subsequent
                                // calls to `ensure_storage_consistency` won't clobber changes.
                                if !self.storage_base_path.trim().is_empty() && !self.storage_class_name.trim().is_empty() {
                                    let base = std::path::Path::new(&self.storage_base_path);
                                    match crate::storage::save_set_into_class_folder(base, &self.storage_class_name, set.name(), set) {
                                        Ok(p) => {
                                            self.status_message = format!("Added and saved flashcard: '{}' -> {}", q, p.display());
                                            log::info!("Saved set '{}' after adding flashcard to {}", set.name(), p.display());
                                        }
                                        Err(e) => {
                                            self.status_message = format!("Added but save failed: {}", e);
                                            log::warn!("Failed to save set '{}' after add: {}", set.name(), e);
                                        }
                                    }
                                } else {
                                    // stronger status message and structured logging for debugging
                                    self.status_message = format!("Added flashcard: '{}'", q);
                                    log::debug!("Add Flashcard clicked - question='{}', answer='{}', hints={:?}", q, a, hints);
                                }

                                // clear inputs
                                self.new_question.clear();
                                self.new_answer.clear();
                                self.new_hints.clear();
                                self.new_hint_input.clear();
                            }

                            // If the Add button is disabled, show a small hint to the user
                            if !can_add {
                                ui_h.label(egui::RichText::new("Question and Answer required").small().color(egui::Color32::from_gray(160)));
                            }

                            // Save the currently edited set to disk (user-chosen base/class)
                            if ui_h.button("Save Set").clicked() {
                                if !self.storage_base_path.trim().is_empty() && !self.storage_class_name.trim().is_empty() {
                                    let base = std::path::Path::new(&self.storage_base_path);
                                    match crate::storage::save_set_into_class_folder(base, &self.storage_class_name, set.name(), set) {
                                        Ok(p) => self.status_message = format!("Saved: {}", p.display()),
                                        Err(e) => self.status_message = format!("Save error: {}", e),
                                    }
                                } else {
                                    self.status_message = "Set storage not configured. Open 'Create New Set' and set Base folder and Class folder.".to_string();
                                }
                            }
                        });
                    }
                }
                else {
                    // No set selected: allocate a small pixel-aligned placeholder below the Create button to avoid unaligned markers
                    let ph_w = ui_right.available_width().round();
                    let ph_h = (4.0 * scale).round();
                    ui_right.allocate_ui_with_layout(egui::Vec2::new(ph_w.round(), ph_h.round()), egui::Layout::top_down(egui::Align::Min), |_ui_ph| {
                        // intentionally empty placeholder
                    });
                }
            });
        });

        // Popup window for creating a new study set (includes storage fields)
        if self.show_create_set_popup {
            egui::Window::new("Create Study Set").collapsible(false).resizable(false).show(ui.ctx(), |ui_win| {
                ui_win.label("Set name:");
                ui_win.text_edit_singleline(&mut self.new_set_name);
                ui_win.add_space((6.0 * scale).round());

                ui_win.label(RichText::new("Storage (where sets will be stored)").heading());
                ui_win.horizontal(|ui_h| {
                    ui_h.label("Base folder:");
                    ui_h.text_edit_singleline(&mut self.storage_base_path);
                    if ui_h.small_button("Browse").clicked() {
                        if let Some(dir) = FileDialog::new().set_title("Select base folder").pick_folder() {
                            if let Some(s) = dir.to_str() { 
                                self.storage_base_path = s.to_string();
                                // persist base path immediately
                                let _ = save_config(&crate::storage::Config { storage_base_path: self.storage_base_path.clone(), storage_class_name: self.storage_class_name.clone() });
                            }
                        }
                    }
                });

                ui_win.horizontal(|ui_h| {
                    ui_h.label("Class folder (name):");
                    ui_h.text_edit_singleline(&mut self.storage_class_name);
                    if ui_h.small_button("Browse").clicked() {
                        if let Some(dir) = FileDialog::new().set_title("Select class folder").pick_folder() {
                            if let Some(s) = dir.file_name().and_then(|n| n.to_str()) {
                                self.storage_class_name = s.to_string();
                                // persist class name
                                let _ = save_config(&crate::storage::Config { storage_base_path: self.storage_base_path.clone(), storage_class_name: self.storage_class_name.clone() });
                            }
                        }
                    }
                    // Load class button to populate study_sets from selected folder
                    if ui_h.button("Load class").clicked() {
                        if self.storage_base_path.trim().is_empty() || self.storage_class_name.trim().is_empty() {
                            self.status_message = "Please set both Base folder and Class folder".to_string();
                        } else {
                            let base = std::path::Path::new(&self.storage_base_path);
                            match crate::storage::load_sets_from_class_folder(base, &self.storage_class_name) {
                                Ok(sets) => {
                                    self.study_sets = sets;
                                    if self.study_sets.is_empty() {
                                        self.selected_set = None;
                                        self.status_message = "No sets found in class folder".to_string();
                                    } else {
                                        self.selected_set = Some(0);
                                        self.current_card_index = 0;
                                        self.card_flipped = false;
                                        self.show_hint = false;
                                        self.status_message = format!("Loaded {} sets", self.study_sets.len());
                                    }
                                    // save the chosen base/class
                                    let _ = save_config(&crate::storage::Config { storage_base_path: self.storage_base_path.clone(), storage_class_name: self.storage_class_name.clone() });
                                    // now ensure everything is consistent with the filesystem
                                    self.ensure_storage_consistency();
                                }
                                Err(e) => {
                                    self.status_message = format!("Load error: {}", e);
                                }
                            }
                        }
                    }
                });

                ui_win.add_space((6.0 * scale).round());
                ui_win.label(RichText::new("Import / Export").heading());

                ui_win.horizontal(|ui_h| {
                    ui_h.label("Import file:");
                    ui_h.text_edit_singleline(&mut self.import_file_path);
                    if ui_h.small_button("Browse").clicked() {
                        if let Some(f) = FileDialog::new().add_filter("JSON", &["json"]).pick_file() {
                            if let Some(s) = f.to_str() { self.import_file_path = s.to_string(); }
                        }
                    }
                    if ui_h.button("Import into class").clicked() {
                        if !self.storage_base_path.trim().is_empty() && !self.storage_class_name.trim().is_empty() && !self.import_file_path.trim().is_empty() {
                            let base = std::path::Path::new(&self.storage_base_path);
                            let src = std::path::Path::new(&self.import_file_path);
                            match crate::storage::import_set_file_to_class(base, &self.storage_class_name, src) {
                                Ok(p) => self.status_message = format!("Imported: {}", p.display()),
                                Err(e) => self.status_message = format!("Import error: {}", e),
                            }
                        }
                    }
                });

                ui_win.horizontal(|ui_h| {
                    ui_h.label("Export dest:");
                    ui_h.text_edit_singleline(&mut self.export_dest_path);
                    if ui_h.small_button("Browse").clicked() {
                        if let Some(f) = FileDialog::new().set_title("Select export destination").save_file() {
                            if let Some(s) = f.to_str() { self.export_dest_path = s.to_string(); }
                        }
                    }
                    if ui_h.button("Export selected").clicked() {
                        if let Some(idx) = self.selected_set {
                            if idx < self.study_sets.len() && !self.export_dest_path.trim().is_empty() {
                                let src = std::path::Path::new(&self.storage_base_path).join(&self.storage_class_name).join(format!("{}.json", sanitize_filename::sanitize(self.study_sets[idx].name())));
                                let dst = std::path::Path::new(&self.export_dest_path);
                                match crate::storage::export_set_file(&src, &dst) {
                                    Ok(()) => self.status_message = format!("Exported to {}", dst.display()),
                                    Err(e) => self.status_message = format!("Export error: {}", e),
                                }
                            }
                        }
                    }
                });

                if !self.status_message.is_empty() {
                    ui_win.label(&self.status_message);
                }

                ui_win.add_space((6.0 * scale).round());
                ui_win.horizontal(|ui_h| {
                    if ui_h.button("Create").clicked() {
                        if !self.new_set_name.trim().is_empty() {
                            let s = AppStudySet::new(self.new_set_name.trim().to_string());
                            self.study_sets.push(s);
                            self.selected_set = Some(self.study_sets.len() - 1);
                            // Optionally save immediately if storage provided
                            if !self.storage_base_path.trim().is_empty() && !self.storage_class_name.trim().is_empty() {
                                let base = std::path::Path::new(&self.storage_base_path);
                                let _ = crate::storage::save_set_into_class_folder(base, &self.storage_class_name, self.new_set_name.trim(), &self.study_sets[self.study_sets.len()-1]);
                            }
                            self.new_set_name.clear();
                            self.show_create_set_popup = false;
                        }
                    }
                    if ui_h.button("Cancel").clicked() {
                        self.show_create_set_popup = false;
                    }
                });
            });
        }
        
    }

    fn quiz_view(&mut self, ui: &mut egui::Ui, scale: f32) {
        // We'll mirror the Study Sets layout: left pane for class/set/quiz selection, right pane for actions
        ui.label(RichText::new("Quiz View").heading());
        ui.add_space((6.0 * scale).round());

        let avail = ui.available_size();
        let total_width = avail.x.round();
        let avail_height = avail.y.round();
        let left_width = (total_width * 0.33).max(200.0 * scale).round();
        let right_width = (total_width - left_width - 8.0 * scale).max(200.0 * scale).round();

        ui.horizontal(|ui| {
            // LEFT
            ui.allocate_ui_with_layout(egui::Vec2::new(left_width, avail_height), egui::Layout::top_down(egui::Align::Min), |ui_left| {
                ui_left.label("Class:");
                if self.available_classes.is_empty() {
                    ui_left.label(RichText::new("(no classes)").italics());
                } else {
                    let mut sel = self.selected_class.unwrap_or(0);
                    if sel >= self.available_classes.len() { sel = 0; }
                    egui::ComboBox::from_id_salt("quiz_class_select").selected_text(&self.available_classes[sel]).show_ui(ui_left, |ui| {
                        for (i, c) in self.available_classes.iter().enumerate() {
                            ui.selectable_value(&mut sel, i, c);
                        }
                    });
                }

                ui_left.add_space((6.0 * scale).round());
                ui_left.label("Set:");
                if self.study_sets.is_empty() {
                    ui_left.label(RichText::new("(no sets)").italics());
                } else {
                    let labels: Vec<String> = self.study_sets.iter().map(|s| s.name().to_string()).collect();
                    let mut selected = self.selected_set.unwrap_or(0);
                    if selected >= labels.len() { selected = 0; }
                    egui::ComboBox::from_id_salt("quiz_set_select").selected_text(&labels[selected]).show_ui(ui_left, |ui| {
                        for (i, l) in labels.iter().enumerate() {
                            ui.selectable_value(&mut selected, i, l);
                        }
                    });

                    // Apply selection change
                    if self.selected_set != Some(selected) {
                        // no mutability of self in this view fn; real updates happen elsewhere
                    }

                    // Quiz dropdown + controls moved here (Delete / Save)
                    ui_left.add_space((6.0 * scale).round());
                    ui_left.label("Quiz:");
                    if let Some(idx) = self.selected_set {
                        if idx < self.study_sets.len() {
                            let set = &self.study_sets[idx];
                            let titles = set.quiz_titles();
                            if titles.is_empty() {
                                ui_left.label(RichText::new("(no quizzes)").italics());
                            } else {
                                let mut qsel = self.selected_quiz.unwrap_or(0);
                                if qsel >= titles.len() { qsel = 0; }
                                // Start Quiz button placed above the quiz dropdown
                                ui_left.add_space((4.0 * scale).round());
                                if ui_left.button("Start Quiz").clicked() {
                                    self.show_quiz_window = true;
                                    self.status_message = "Starting quiz...".to_string();
                                }
                                egui::ComboBox::from_id_salt("quiz_select").selected_text(&titles[qsel]).show_ui(ui_left, |ui| {
                                    for (i, t) in titles.iter().enumerate() {
                                        ui.selectable_value(&mut qsel, i, t);
                                    }
                                });
                                // Place the Create button below the dropdown so it's close to the Quiz controls
                                ui_left.add_space((4.0 * scale).round());
                                if ui_left.small_button("Create New").clicked() {
                                    self.show_create_quiz_popup = true;
                                    self.new_quiz_name.clear();
                                }
                                // apply selection change immediately
                                if self.selected_quiz != Some(qsel) {
                                    self.selected_quiz = Some(qsel);
                                    self.current_card_index = 0;
                                    self.card_flipped = false;
                                    self.show_hint = false;
                                }

                                // Controls: Delete Quiz (top) and Save (below)
                                ui_left.add_space((6.0 * scale).round());
                                ui_left.vertical(|ui_v| {
                                    if ui_v.small_button("Delete Quiz").clicked() {
                                        if let Some(qi) = self.selected_quiz {
                                            if let Some(removed) = self.study_sets[idx].remove_quiz(qi) {
                                                self.status_message = format!("Deleted quiz '{}'", removed.title());
                                                // persist set if storage configured
                                                if !self.storage_base_path.trim().is_empty() && !self.storage_class_name.trim().is_empty() {
                                                    let base = std::path::Path::new(&self.storage_base_path);
                                                    let set_ref = &self.study_sets[idx];
                                                    match crate::storage::save_set_into_class_folder(base, &self.storage_class_name, set_ref.name(), set_ref) {
                                                        Ok(p) => self.status_message = format!("Deleted and saved: {}", p.display()),
                                                        Err(e) => self.status_message = format!("Deleted but save failed: {}", e),
                                                    }
                                                }
                                                // adjust selection
                                                let remaining = self.study_sets[idx].get_all_quizzes().len();
                                                if remaining == 0 { self.selected_quiz = None; }
                                                else if qi >= remaining { self.selected_quiz = Some(0); }
                                            }
                                        }
                                    }
                                    ui_v.add_space((4.0 * scale).round());
                                    if ui_v.button("Save").clicked() {
                                        if !self.storage_base_path.trim().is_empty() && !self.storage_class_name.trim().is_empty() {
                                            let base = std::path::Path::new(&self.storage_base_path);
                                            let set_ref = &self.study_sets[idx];
                                            match crate::storage::save_set_into_class_folder(base, &self.storage_class_name, set_ref.name(), set_ref) {
                                                Ok(p) => self.status_message = format!("Saved: {}", p.display()),
                                                Err(e) => self.status_message = format!("Save error: {}", e),
                                            }
                                        } else {
                                            self.status_message = "Set storage not configured. Open 'Create New Set' and set Base folder and Class folder.".to_string();
                                        }
                                    }
                                });
                            }
                        }
                    }
                }
            });

            ui.add(egui::Separator::default().vertical());
            ui.add_space((8.0 * scale).round());

            // RIGHT: actions
            ui.allocate_ui_with_layout(egui::Vec2::new(right_width, avail_height), egui::Layout::top_down(egui::Align::Min), |ui_right| {
                // Create button moved to left pane above the Quiz dropdown
                ui_right.add_space((6.0 * scale).round());
                // If a set is selected, show the inline quiz editor (questions) in the right pane.
                if let Some(set_idx) = self.selected_set {
                    if set_idx < self.study_sets.len() {
                        ui_right.add_space((4.0 * scale).round());
                        ui_right.separator();
                        ui_right.label("Questions:");
                        if let Some(qi) = self.selected_quiz {
                            if qi < self.study_sets[set_idx].get_all_quizzes().len() {
                                // number of questions (immutable read)
                                let qcount = self.study_sets[set_idx].get_all_quizzes()[qi].question_count();

                                ui_right.horizontal(|ui_h| {
                                    // compute explicit widths from the available inner width so
                                    // the editor column size is predictable and text fields
                                    // can be sized directly from it.
                                    let inner_total = ui_h.available_width();
                                    // make the list column a bit narrower and give the editor more room
                                    let list_w = (inner_total * 0.15).max(80.0).round();
                                    let edit_w = (inner_total - list_w - (6.0 * scale)).max(200.0).round();

                                    // QUESTION LIST (fixed width)
                                    ui_h.allocate_ui_with_layout(egui::Vec2::new(list_w, avail_height), egui::Layout::top_down(egui::Align::Min), |ui_list| {
                                        for i in 0..qcount {
                                            if ui_list.selectable_label(self.current_card_index == i, format!("{}", i + 1)).clicked() {
                                                self.current_card_index = i;
                                            }
                                        }
                                        if qcount == 0 { ui_list.label("(no questions)"); }
                                        ui_list.add_space((6.0 * scale).round());
                                        ui_list.horizontal(|ui_lh| {
                                            if ui_lh.small_button("Add").clicked() {
                                                if let Some(quiz_mut) = self.study_sets[set_idx].get_all_quizzes_mut().get_mut(qi) {
                                                    quiz_mut.add_question("New question".to_string(), Vec::new(), "".to_string(), crate::models::QuestionType::FillInTheBlank);
                                                }
                                            }
                                            if ui_lh.small_button("Delete").clicked() {
                                                let idx = self.current_card_index;
                                                if let Some(quiz_mut) = self.study_sets[set_idx].get_all_quizzes_mut().get_mut(qi) {
                                                    if idx < quiz_mut.question_count() {
                                                        quiz_mut.remove_question(idx);
                                                        if self.current_card_index > 0 { self.current_card_index -= 1; }
                                                    }
                                                }
                                            }
                                        });
                                    });

                                    ui_h.add(egui::Separator::default().vertical());

                                    // EDITOR (use the precomputed edit_w for sizing the text fields)
                                    ui_h.allocate_ui_with_layout(egui::Vec2::new(edit_w, avail_height), egui::Layout::top_down(egui::Align::Min), |ui_edit| {
                                        let sel_q = self.current_card_index;
                                        if sel_q < qcount {
                                            if let Some(qdata) = self.study_sets[set_idx].get_all_quizzes()[qi].get_question_data(sel_q) {
                                                let mut qdata = qdata; // owned copy for editing
                                                ui_edit.label("Prompt:");
                                                let text_w = (edit_w * 0.95).round();
                                                ui_edit.add(egui::TextEdit::multiline(&mut qdata.prompt).desired_rows(2).desired_width(text_w));
                                                ui_edit.label("Answer:");
                                                ui_edit.add(egui::TextEdit::multiline(&mut qdata.answer).desired_rows(2).desired_width(text_w));
                                                ui_edit.label("Options (comma-separated, for MC):");
                                                let mut opts_joined = qdata.options.join(", ");
                                                ui_edit.add(egui::TextEdit::multiline(&mut opts_joined).desired_rows(2).desired_width(text_w));
                                                ui_edit.label("Type:");
                                                let mut qtype = qdata.question_type.clone();
                                                ui_edit.horizontal(|ui_ht| {
                                                    if ui_ht.selectable_label(qtype == crate::models::QuestionType::MultipleChoice, "Multiple Choice").clicked() {
                                                        qtype = crate::models::QuestionType::MultipleChoice;
                                                    }
                                                    if ui_ht.selectable_label(qtype == crate::models::QuestionType::FillInTheBlank, "Fill In The Blank").clicked() {
                                                        qtype = crate::models::QuestionType::FillInTheBlank;
                                                    }
                                                });

                                                ui_edit.add_space((6.0 * scale).round());
                                                ui_edit.horizontal(|ui_apply| {
                                                    if ui_apply.button("Apply").clicked() {
                                                        if let Some(quiz_mut) = self.study_sets[set_idx].get_all_quizzes_mut().get_mut(qi) {
                                                            let new_opts: Vec<String> = opts_joined.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                                                            let new_data = crate::models::QuestionData { prompt: qdata.prompt, options: new_opts, answer: qdata.answer, question_type: qtype };
                                                            quiz_mut.update_question(sel_q, new_data);
                                                        }
                                                    }
                                                    if ui_apply.button("Save").clicked() {
                                                        if !self.storage_base_path.trim().is_empty() && !self.storage_class_name.trim().is_empty() {
                                                            let base = std::path::Path::new(&self.storage_base_path);
                                                            let set_ref = &self.study_sets[set_idx];
                                                            match crate::storage::save_set_into_class_folder(base, &self.storage_class_name, set_ref.name(), set_ref) {
                                                                Ok(p) => self.status_message = format!("Saved: {}", p.display()),
                                                                Err(e) => self.status_message = format!("Save error: {}", e),
                                                            }
                                                        } else {
                                                            self.status_message = "Set storage not configured.".to_string();
                                                        }
                                                    }
                                                });
                                            }
                                        } else {
                                            ui_edit.label("Select a question to edit or add a new one.");
                                        }
                                    });
                                });
                            }
                        }
                    }
                }
            });
        });

        // Popup for creating a quiz
        if self.show_create_quiz_popup {
            egui::Window::new("Create Quiz").collapsible(false).resizable(false).show(ui.ctx(), |ui_win| {
                ui_win.label("Quiz title:");
                ui_win.text_edit_singleline(&mut self.new_quiz_name);
                ui_win.add_space((6.0 * scale).round());

                ui_win.label("Create placeholder questions:");
                ui_win.horizontal(|ui_h| {
                    ui_h.label("Multiple choice:");
                    ui_h.add(egui::DragValue::new(&mut self.new_quiz_mc_count).range(0..=50));
                });
                ui_win.horizontal(|ui_h| {
                    ui_h.label("True/False:");
                    ui_h.add(egui::DragValue::new(&mut self.new_quiz_tf_count).range(0..=50));
                });
                ui_win.horizontal(|ui_h| {
                    ui_h.label("Short answer:");
                    ui_h.add(egui::DragValue::new(&mut self.new_quiz_sa_count).range(0..=50));
                });
                ui_win.horizontal(|ui_h| {
                    ui_h.label("Multiple blank:");
                    ui_h.add(egui::DragValue::new(&mut self.new_quiz_mb_count).range(0..=50));
                });

                ui_win.add_space((6.0 * scale).round());
                ui_win.horizontal(|ui_h| {
                    if ui_h.button("Create").clicked() {
                        if !self.new_quiz_name.trim().is_empty() {
                            let mut q = AppQuiz::new(self.new_quiz_name.trim().to_string());
                            q.add_placeholder_questions(self.new_quiz_mc_count, self.new_quiz_tf_count, self.new_quiz_sa_count, self.new_quiz_mb_count);
                            if let Some(idx) = self.selected_set {
                                if idx < self.study_sets.len() {
                                    self.study_sets[idx].add_quiz(q);
                                    // persist if configured
                                    if !self.storage_base_path.trim().is_empty() && !self.storage_class_name.trim().is_empty() {
                                        let base = std::path::Path::new(&self.storage_base_path);
                                        let set = &self.study_sets[idx];
                                        match crate::storage::save_set_into_class_folder(base, &self.storage_class_name, set.name(), set) {
                                            Ok(p) => self.status_message = format!("Created and saved quiz in {}", p.display()),
                                            Err(e) => self.status_message = format!("Created but save failed: {}", e),
                                        }
                                    } else {
                                        self.status_message = format!("Created quiz '{}'", self.new_quiz_name.trim());
                                    }
                                    self.selected_quiz = Some(self.study_sets[idx].get_all_quizzes().len() - 1);
                                }
                            }
                            self.show_create_quiz_popup = false;
                            self.new_quiz_name.clear();
                            // reset counts
                            self.new_quiz_mc_count = 0;
                            self.new_quiz_tf_count = 0;
                            self.new_quiz_sa_count = 0;
                            self.new_quiz_mb_count = 0;
                        }
                    }
                    if ui_h.button("Cancel").clicked() {
                        self.show_create_quiz_popup = false;
                    }
                });
            });
        }
        // Distraction-free quiz window (rendered when Start Quiz is clicked)
        if self.show_quiz_window {
            // Get full screen rect and size the window to match it
            let screen_rect = ui.ctx().input(|i| i.screen_rect);
            let win_w = screen_rect.width().max(1.0);
            let win_h = screen_rect.height().max(1.0);

            // Use a Window forced to the foreground so it overlays everything and looks like a modal
            egui::Window::new("Quiz Session")
                .order(egui::Order::Foreground)
                .collapsible(false)
                .resizable(false)
                .movable(false)
                .title_bar(false)
                .default_pos(egui::pos2(screen_rect.left(), screen_rect.top()))
                .fixed_size(egui::vec2(win_w, win_h))
                .show(ui.ctx(), |ui_win| {
                    // Paint a dark background for the whole window to fully obscure underlying UI
                    let win_rect = ui.ctx().available_rect();
                    let panel_bg = egui::Color32::from_rgb(18, 18, 20);
                    ui_win.painter().rect_filled(win_rect, 0.0, panel_bg);

                    // Content centered and in white for dark mode
                    ui_win.vertical_centered(|ui_c| {
                        ui_c.add_space(8.0);
                        ui_c.label(RichText::new("Quiz Session").heading().color(egui::Color32::WHITE));
                        ui_c.add_space(12.0);
                        ui_c.label(RichText::new("(Distraction-free mode — quiz content will appear here.)").color(egui::Color32::WHITE));
                        ui_c.add_space(18.0);
                        if ui_c.add_sized(egui::Vec2::new(96.0, 36.0), egui::Button::new(RichText::new("Exit").color(egui::Color32::WHITE))).clicked() {
                            self.show_quiz_window = false;
                        }
                    });
                });
        }
    }
}