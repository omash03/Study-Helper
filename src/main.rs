use eframe::App;
mod app;
mod models;
mod gui;
mod storage;

/// Application entry point. Initializes logging and starts the eframe GUI.
fn main() {
    // Configure logging from RUST_LOG environment variable (default to info)
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting Study Helper");

    eframe::run_native(
        "Study Helper",
        eframe::NativeOptions::default(),
        Box::new(|_cc: &eframe::CreationContext<'_>| {
            Ok(Box::new(gui::StudyHelperApp::default()) as Box<dyn App>)
        }),
    ).expect("Failed to run native app");
}