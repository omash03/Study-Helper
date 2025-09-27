// src/gui.rs
use eframe::{egui, App, Frame};

pub struct StudyHelperApp {
    current_view: AppView,
}

enum AppView {
    Flashcards,
    StudySets,
    Quiz,
}

impl Default for StudyHelperApp {
    fn default() -> Self {
        Self {
            current_view: AppView::Flashcards,
        }
    }
}

impl App for StudyHelperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Welcome to Study Helper!");

            if ui.button("Flashcards").clicked() {
                self.current_view = AppView::Flashcards;
            }
            if ui.button("Study Sets").clicked() {
                self.current_view = AppView::StudySets;
            }
            if ui.button("Quiz").clicked() {
                self.current_view = AppView::Quiz;
            }

            match self.current_view {
                AppView::Flashcards => self.flashcards_view(ui),
                AppView::StudySets => self.study_sets_view(ui),
                AppView::Quiz => self.quiz_view(ui),
            }
        });
    }
}

impl StudyHelperApp {
    fn flashcards_view(&self, ui: &mut egui::Ui) {
        ui.label("Flashcards View");
        // Add flashcard display and interaction logic here
    }

    fn study_sets_view(&self, ui: &mut egui::Ui) {
        ui.label("Study Sets View");
        // Add study set management logic here
    }

    fn quiz_view(&self, ui: &mut egui::Ui) {
        ui.label("Quiz View");
        // Add quiz display and interaction logic here
    }
}