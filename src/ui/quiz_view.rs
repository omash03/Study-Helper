// src/ui/quiz_view.rs
use eframe::{egui, epi};
use crate::models::quiz::{Quiz, Question};

pub struct QuizView {
    quiz: Quiz,
    current_question_index: usize,
    user_answers: Vec<Option<String>>,
}

impl QuizView {
    pub fn new(quiz: Quiz) -> Self {
        let question_count = quiz.questions.len();
        Self {
            quiz,
            current_question_index: 0,
            user_answers: vec![None; question_count],
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Quiz");

            if self.current_question_index < self.quiz.questions.len() {
                self.display_question(ui);
            } else {
                ui.label("Quiz Completed!");
                if ui.button("Restart Quiz").clicked() {
                    self.restart_quiz();
                }
            }
        });
    }

    fn display_question(&mut self, ui: &mut egui::Ui) {
        let question = &self.quiz.questions[self.current_question_index];
        ui.label(&question.text);

        match &question {
            Question::MultipleChoice { options, .. } => {
                for (i, option) in options.iter().enumerate() {
                    let answer = ui.radio_value(
                        &mut self.user_answers[self.current_question_index],
                        Some(option.clone()),
                        option,
                    );
                }
            }
            Question::FillInTheBlank { .. } => {
                ui.text_edit_singleline(&mut self.user_answers[self.current_question_index]);
            }
        }

        if ui.button("Next").clicked() {
            self.current_question_index += 1;
        }
    }

    fn restart_quiz(&mut self) {
        self.current_question_index = 0;
        self.user_answers.fill(None);
    }
}