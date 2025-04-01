use eframe::egui;
use std::time::Duration;
use crate::audio::player::{AudioPlayer, PlaybackState};
pub struct AudioPlayerApp {
    player: Option<AudioPlayer>,
    playing: bool,
    current_time: Duration,
    total_duration: Duration,
    volume: f32,
}

impl Default for AudioPlayerApp {
    fn default() -> Self {
        Self {
            player: AudioPlayer::new().ok(),
            playing: false,
            current_time: Duration::from_secs(0),
            total_duration: Duration::from_secs(0),
            volume: 1.0,
        }
    }
}

impl eframe::App for AudioPlayerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // File selection button
                if ui.button("Open File").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Audio", &["mp3", "wav"])
                        .pick_file() {
                            if let Some(player) = &mut self.player {
                                let _ = player.load_file(path.to_str().unwrap());
                            }
                    }
                }

                // Play/Pause button
                if let Some(player) = &mut self.player {
                    if player.get_state() == PlaybackState::Playing {
                        if ui.button("⏸").clicked() {
                            let _ = player.pause();
                            self.playing = false;
                        }
                    } else {
                        if ui.button("▶").clicked() {
                            let _ = player.play();
                            self.playing = true;
                        }
                    }

                    // Stop button
                    if ui.button("⏹").clicked() {
                        let _ = player.stop();
                        self.playing = false;
                    }

                    // Volume slider
                    ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0)
                        .text("Volume")
                        .show_value(false));
                    let _ = player.set_volume(self.volume);
                }
            });

            // Progress bar (placeholder for now)
            ui.add(egui::ProgressBar::new(0.0)
                .show_percentage()
                .animate(self.playing));
        });

        // Request continuous updates when playing
        if self.playing {
            ctx.request_repaint();
        }
    }
}

