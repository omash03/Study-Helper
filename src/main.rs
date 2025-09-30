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
        eframe::NativeOptions {
            // Start with a reasonable default inner size so the UI elements fit by default.
            viewport: eframe::egui::ViewportBuilder::default().with_inner_size(eframe::egui::Vec2::new(1000.0, 700.0)),
            ..Default::default()
        },
        Box::new(|_cc: &eframe::CreationContext<'_>| {
            Ok(Box::new(gui::StudyHelperApp::default()) as Box<dyn App>)
        }),
    ).expect("Failed to run native app");
}