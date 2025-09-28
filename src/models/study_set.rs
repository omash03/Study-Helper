use super::flashcard::Flashcard;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct StudySet {
    name: String,
    flashcards: Vec<Flashcard>,
}

impl StudySet {
    pub fn new(name: String) -> Self {
        StudySet {
            name,
            flashcards: Vec::new(),
        }
    }

    pub fn add_flashcard(&mut self, flashcard: Flashcard) {
        self.flashcards.push(flashcard);
    }

    pub fn remove_flashcard(&mut self, index: usize) -> Option<Flashcard> {
        if index < self.flashcards.len() {
            Some(self.flashcards.remove(index))
        } else {
            None
        }
    }

    pub fn get_flashcard(&self, index: usize) -> Option<&Flashcard> {
        self.flashcards.get(index)
    }

    pub fn get_all_flashcards(&self) -> &Vec<Flashcard> {
        &self.flashcards
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}