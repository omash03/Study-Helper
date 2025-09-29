use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Quiz {
    title: String,
    questions: Vec<Question>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Question {
    prompt: String,
    options: Vec<String>,
    answer: String,
    question_type: QuestionType,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum QuestionType {
    MultipleChoice,
    FillInTheBlank,
}

/// Public editable representation of a question for the UI.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct QuestionData {
    pub prompt: String,
    pub options: Vec<String>,
    pub answer: String,
    pub question_type: QuestionType,
}

impl Quiz {
    pub fn new(title: String) -> Self {
        Quiz {
            title,
            questions: Vec::new(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn add_question(&mut self, prompt: String, options: Vec<String>, answer: String, question_type: QuestionType) {
        let question = Question {
            prompt,
            options,
            answer,
            question_type,
        };
        self.questions.push(question);
    }

    /// Number of questions in this quiz
    pub fn question_count(&self) -> usize {
        self.questions.len()
    }

    /// Get a copy of the question data for UI editing.
    pub fn get_question_data(&self, index: usize) -> Option<QuestionData> {
        self.questions.get(index).map(|q| QuestionData {
            prompt: q.prompt.clone(),
            options: q.options.clone(),
            answer: q.answer.clone(),
            question_type: q.question_type.clone(),
        })
    }

    /// Update a question from a QuestionData value; returns true if updated.
    pub fn update_question(&mut self, index: usize, data: QuestionData) -> bool {
        if let Some(q) = self.questions.get_mut(index) {
            q.prompt = data.prompt;
            q.options = data.options;
            q.answer = data.answer;
            q.question_type = data.question_type;
            true
        } else {
            false
        }
    }

    /// Remove a question and return its QuestionData if present.
    pub fn remove_question(&mut self, index: usize) -> Option<QuestionData> {
        if index < self.questions.len() {
            let q = self.questions.remove(index);
            Some(QuestionData { prompt: q.prompt, options: q.options, answer: q.answer, question_type: q.question_type })
        } else {
            None
        }
    }

    pub fn check_answer(&self, question_index: usize, user_answer: &str) -> bool {
        if let Some(question) = self.questions.get(question_index) {
            match question.question_type {
                QuestionType::MultipleChoice => question.answer == user_answer,
                QuestionType::FillInTheBlank => question.answer.trim().eq_ignore_ascii_case(user_answer.trim()),
            }
        } else {
            false
        }
    }

    pub fn questions(&self) -> &Vec<Question> {
        &self.questions
    }

    /// Add placeholder questions to the quiz for UI-driven creation.
    /// This avoids exposing the internal Question type to UI code.
    pub fn add_placeholder_questions(&mut self, mc: usize, tf: usize, sa: usize, mb: usize) {
        for i in 0..mc {
            let prompt = format!("Multiple choice placeholder #{}", i + 1);
            let options = vec!["Option A".to_string(), "Option B".to_string(), "Option C".to_string(), "Option D".to_string()];
            let answer = "Option A".to_string();
            self.add_question(prompt, options, answer, QuestionType::MultipleChoice);
        }
        for i in 0..tf {
            let prompt = format!("True/False placeholder #{}", i + 1);
            let options = vec!["True".to_string(), "False".to_string()];
            let answer = "True".to_string();
            self.add_question(prompt, options, answer, QuestionType::MultipleChoice);
        }
        for i in 0..sa {
            let prompt = format!("Short answer placeholder #{}", i + 1);
            let options: Vec<String> = Vec::new();
            let answer = "".to_string();
            self.add_question(prompt, options, answer, QuestionType::FillInTheBlank);
        }
        for i in 0..mb {
            let prompt = format!("Multiple blank placeholder #{}", i + 1);
            let options: Vec<String> = Vec::new();
            let answer = "".to_string();
            self.add_question(prompt, options, answer, QuestionType::FillInTheBlank);
        }
    }
}