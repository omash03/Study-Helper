// tests/storage_tests.rs
use std::fs;
use std::path::Path;
use serde_json::Value;

#[test]
fn test_load_sample_set() {
    let path = Path::new("data/sample_set.json");
    let data = fs::read_to_string(path).expect("Unable to read file");
    let json: Value = serde_json::from_str(&data).expect("JSON was not well-formatted");

    assert!(json.is_object());
    assert!(json.get("flashcards").is_some());
}

#[test]
fn test_save_and_load_study_set() {
    let sample_set = r#"
    {
        "flashcards": [
            {
                "question": "What is Rust?",
                "answer": "A systems programming language."
            }
        ]
    }"#;

    let path = Path::new("data/test_set.json");
    fs::write(path, sample_set).expect("Unable to write file");

    let data = fs::read_to_string(path).expect("Unable to read file");
    let json: Value = serde_json::from_str(&data).expect("JSON was not well-formatted");

    assert!(json.is_object());
    assert_eq!(json["flashcards"].as_array().unwrap().len(), 1);
    assert_eq!(json["flashcards"][0]["question"], "What is Rust?");
    assert_eq!(json["flashcards"][0]["answer"], "A systems programming language.");

    // Clean up test file
    fs::remove_file(path).expect("Unable to delete test file");
}