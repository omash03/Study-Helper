use eframe::{egui, epi};
use crate::models::study_set::StudySet;

pub struct SetsView {
    pub study_sets: Vec<StudySet>,
}

impl SetsView {
    pub fn new() -> Self {
        Self {
            study_sets: Vec::new(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Study Sets");

            if ui.button("Create New Study Set").clicked() {
                // Logic to create a new study set
            }

            ui.separator();

            for study_set in &self.study_sets {
                ui.horizontal(|ui| {
                    ui.label(&study_set.name);
                    if ui.button("Edit").clicked() {
                        // Logic to edit the study set
                    }
                    if ui.button("Delete").clicked() {
                        // Logic to delete the study set
                    }
                });
            }
        });
    }
}