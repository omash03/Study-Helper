# Study Helper

This project is a study application built using Rust and the eframe framework. It allows users to create and manage flashcards, study sets, and quizzes. The application provides a user-friendly interface for studying and testing knowledge.

## Features

- **Flashcards**: Create, view, and manage flashcards with questions, answers, and hints.
- **Study Sets**: Organize flashcards into study sets for focused learning.
- **Quizzes**: Generate quizzes with multiple-choice and fill-in-the-blank questions to test knowledge.
- **Data Storage**: Save and load study sets and flashcards using JSON files.

## Project Structure

```
study_helper
├── .gitignore
├── Cargo.toml
├── README.md
├── data
│   └── sample_set.json
├── src
│   ├── main.rs
│   ├── app.rs
│   ├── gui.rs
│   ├── utils.rs
│   ├── models
│   │   ├── mod.rs
│   │   ├── flashcard.rs
│   │   ├── study_set.rs
│   │   └── quiz.rs
│   ├── storage
│   │   ├── mod.rs
│   │   └── json_store.rs
│   └── ui
│       ├── mod.rs
│       ├── flashcards_view.rs
│       ├── sets_view.rs
│       └── quiz_view.rs
└── tests
    └── storage_tests.rs
```

## Setup Instructions

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd study_helper
   ```

2. **Install Rust**: Ensure you have Rust installed on your machine. You can install it from [rustup.rs](https://rustup.rs/).

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run the application**:
   ```bash
   cargo run
   ```

## Usage Guidelines

- Upon launching the application, you will be presented with the main interface.
- Use the navigation options to create and manage flashcards, study sets, and quizzes.
- Follow the prompts to interact with the application and utilize its features.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.