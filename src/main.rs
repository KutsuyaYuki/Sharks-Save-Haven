use eframe::egui;

mod game_saves;
mod shark_gui;
mod db;
mod filesystem;
mod widgets;
const DB_NAME: &str = "local_games.db";

fn main() -> Result<(), eframe::Error> {
    let my_app = shark_gui::SharkGui::new();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Shark's Safe Haven",
        options,
        Box::new(|_cc| Box::new(my_app)),
    )
}
