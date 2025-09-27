use eframe::App;
mod gui;

fn main() {
    eframe::run_native(
        "Study Helper",
        eframe::NativeOptions::default(),
        Box::new(|_cc: &eframe::CreationContext<'_>| {
            Ok(Box::new(gui::StudyHelperApp::default()) as Box<dyn App>)
        }),
    ).expect("Failed to run native app");
}