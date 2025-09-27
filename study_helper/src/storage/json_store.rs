// src/storage/json_store.rs

use std::fs;
use std::io::{self, Write, Read};
use serde::{Serialize, Deserialize};
use crate::models::study_set::StudySet;

#[derive(Serialize, Deserialize)]
struct JsonStudySet {
    title: String,
    flashcards: Vec<crate::models::flashcard::Flashcard>,
}

pub fn save_study_set(study_set: &StudySet, file_path: &str) -> io::Result<()> {
    let json_study_set = JsonStudySet {
        title: study_set.title.clone(),
        flashcards: study_set.flashcards.clone(),
    };

    let json_data = serde_json::to_string(&json_study_set)?;
    let mut file = fs::File::create(file_path)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

pub fn load_study_set(file_path: &str) -> io::Result<StudySet> {
    let mut file = fs::File::open(file_path)?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;

    let json_study_set: JsonStudySet = serde_json::from_str(&json_data)?;
    Ok(StudySet {
        title: json_study_set.title,
        flashcards: json_study_set.flashcards,
    })
}