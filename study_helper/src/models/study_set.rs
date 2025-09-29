use super::flashcard::Flashcard;
use super::quiz::Quiz;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct StudySet {
    name: String,
    flashcards: Vec<Flashcard>,
    quizzes: Vec<Quiz>,
}

impl StudySet {
    pub fn new(name: String) -> Self {
        StudySet {
            name,
            flashcards: Vec::new(),
            quizzes: Vec::new(),
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

    /// Quiz related helpers
    pub fn add_quiz(&mut self, quiz: Quiz) {
        self.quizzes.push(quiz);
    }

    pub fn remove_quiz(&mut self, index: usize) -> Option<Quiz> {
        if index < self.quizzes.len() {
            Some(self.quizzes.remove(index))
        } else {
            None
        }
    }

    pub fn get_all_quizzes(&self) -> &Vec<Quiz> {
        &self.quizzes
    }

    /// Mutable access to quizzes for in-place editing.
    pub fn get_all_quizzes_mut(&mut self) -> &mut Vec<Quiz> {
        &mut self.quizzes
    }

    pub fn get_quiz_mut(&mut self, index: usize) -> Option<&mut Quiz> {
        self.quizzes.get_mut(index)
    }

    pub fn quiz_titles(&self) -> Vec<String> {
        self.quizzes.iter().map(|q| q.title().to_string()).collect()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}