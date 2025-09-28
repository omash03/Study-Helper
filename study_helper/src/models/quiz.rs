pub struct Quiz {
    title: String,
    questions: Vec<Question>,
}

struct Question {
    prompt: String,
    options: Vec<String>,
    answer: String,
    question_type: QuestionType,
}

enum QuestionType {
    MultipleChoice,
    FillInTheBlank,
}

impl Quiz {
    pub fn new(title: String) -> Self {
        Quiz {
            title,
            questions: Vec::new(),
        }
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
}