mod audio;
mod dsp;
mod gui;

use gui::window::AudioPlayerApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 240.0])
            .with_min_inner_size([300.0, 200.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Audio Effects Player",
        native_options,
        Box::new(|_cc| Box::new(AudioPlayerApp::default()))
    )
}
