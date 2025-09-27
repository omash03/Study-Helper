// src/app.rs
use eframe::{egui, App};

pub struct StudyHelperApp {
    flashcards: Vec<Flashcard>,
    study_sets: Vec<StudySet>,
    quizzes: Vec<Quiz>,
}

impl StudyHelperApp {
    pub fn new() -> Self {
        Self {
            flashcards: Vec::new(),
            study_sets: Vec::new(),
            quizzes: Vec::new(),
        }
    }

    pub fn add_flashcard(&mut self, flashcard: Flashcard) {
        self.flashcards.push(flashcard);
    }

    pub fn remove_flashcard(&mut self, index: usize) {
        if index < self.flashcards.len() {
            self.flashcards.remove(index);
        }
    }

    pub fn create_study_set(&mut self, study_set: StudySet) {
        self.study_sets.push(study_set);
    }

    pub fn create_quiz(&mut self, quiz: Quiz) {
        self.quizzes.push(quiz);
    }

    pub fn save_study_sets(&self) {
        // Logic to save study sets to storage
    }

    pub fn load_study_sets(&mut self) {
        // Logic to load study sets from storage
    }
}

// Define the Flashcard, StudySet, and Quiz structs here or import them from models
pub struct Flashcard {
    pub question: String,
    pub answer: String,
    pub hints: Vec<String>,
}

pub struct StudySet {
    pub name: String,
    pub flashcards: Vec<Flashcard>,
}

pub struct Quiz {
    pub questions: Vec<QuizQuestion>,
}

pub struct QuizQuestion {
    pub question: String,
    pub options: Vec<String>,
    pub answer: String,
}