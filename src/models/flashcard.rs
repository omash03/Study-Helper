struct Flashcard {
    question: String,
    answer: String,
    hints: Vec<String>,
}

impl Flashcard {
    pub fn new(question: String, answer: String, hints: Vec<String>) -> Self {
        Flashcard { question, answer, hints }
    }

    pub fn check_answer(&self, user_answer: &str) -> bool {
        self.answer.eq_ignore_ascii_case(user_answer)
    }

    pub fn get_hints(&self) -> &Vec<String> {
        &self.hints
    }
}