fn validate_flashcard(question: &str, answer: &str) -> Result<(), String> {
    if question.is_empty() || answer.is_empty() {
        return Err("Question and answer cannot be empty.".to_string());
    }
    Ok(())
}

fn format_question(question: &str) -> String {
    format!("Q: {}", question)
}

fn format_answer(answer: &str) -> String {
    format!("A: {}", answer)
}

fn validate_study_set_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Study set name cannot be empty.".to_string());
    }
    Ok(())
}

fn validate_quiz_question(question: &str) -> Result<(), String> {
    if question.is_empty() {
        return Err("Quiz question cannot be empty.".to_string());
    }
    Ok(())
}