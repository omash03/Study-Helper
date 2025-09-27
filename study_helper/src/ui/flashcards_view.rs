use eframe::{egui, epi};
use crate::models::flashcard::Flashcard;
use crate::models::study_set::StudySet;

pub struct FlashcardsView {
    current_flashcard: Option<Flashcard>,
    flashcards: Vec<Flashcard>,
    index: usize,
}

impl FlashcardsView {
    pub fn new(study_set: &StudySet) -> Self {
        Self {
            current_flashcard: None,
            flashcards: study_set.flashcards.clone(),
            index: 0,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.flashcards.is_empty() {
                ui.label("No flashcards available.");
                return;
            }

            if self.index < self.flashcards.len() {
                self.current_flashcard = Some(self.flashcards[self.index].clone());
                let flashcard = self.current_flashcard.as_ref().unwrap();

                ui.label(&flashcard.question);
                ui.horizontal(|ui| {
                    if ui.button("Show Answer").clicked() {
                        ui.label(&flashcard.answer);
                    }
                    if ui.button("Next").clicked() {
                        self.index = (self.index + 1) % self.flashcards.len();
                    }
                });
            } else {
                ui.label("You've gone through all the flashcards!");
            }
        });
    }
}