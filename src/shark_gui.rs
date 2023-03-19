use crate::{filesystem, db::{self, Game}, DB_NAME, widgets::{TableBuilder, Column}, game_saves::GameSaves};

pub struct SharkGui {
    items: Vec<Game>,
    selected_item: Option<usize>,
    db: Box<db::Db>,
    fs: Box<filesystem::Filesystem>,
}

impl SharkGui {
    pub fn new() -> Self {
        let db =db::Db::new(DB_NAME).expect("Failed to create database connection");
        let fs =filesystem::Filesystem::new();

        db.create_tables().expect("Failed to create tables");

        let games = db.get_all_games().expect("Failed to get games");

        let fs = Box::new(fs);
        let db = Box::new(db);
        Self {
            items: games,
            selected_item: None,
            db,
            fs,
        }
    }

    fn table_ui(&mut self, ui: &mut egui::Ui) {
        // Create a table builder and add columns
        let table = TableBuilder::new(ui)
        .striped(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::initial(100.0).range(40.0..=300.0).resizable(true))
        .column(
            Column::initial(100.0)
                .at_least(40.0)
                .resizable(true)
                .clip(true),
        )
        .column(Column::remainder())
        .min_scrolled_height(0.0)
        .selected_row(&mut self.selected_item);
    
    table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("Game ID");
            });
            header.col(|ui| {
                ui.strong("Publisher");
            });
            header.col(|ui| {
                ui.strong("Title");
            });
        })
        .body(|mut body| match &self.items {
            game => {
                for row_index in 0..game.len() {
                    let row_height = 18.00;
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(
                                game[row_index].id.to_string().clone(),
                            );
                        });
    
                        row.col(|ui| {
                            ui.label(
                                game[row_index]
                                    .publisher
                                    .to_string()
                                    .clone(),
                            );
                        });
    
                        row.col(|ui| {
                            ui.label(
                                game[row_index].title.to_string().clone(),
                            );
                        });
                    });
                }
            }
        });
    }
}

pub fn show_message_box(ctx: &egui::Context, title: &str, message: &str) {
    egui::Window::new(title)
.default_width(200.0)
.show(&ctx, |ui| {
    ui.label(message);
    if ui.button("OK").clicked() {
        // handle OK button clicked event
    }
});
}

impl eframe::App for SharkGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Shark's Safe Haven");

            // Leave room for the source code link after the table demo:
            use egui_extras::{Size, StripBuilder};
            StripBuilder::new(ui)
                .size(Size::remainder().at_least(100.0)) // for the table
                .size(Size::exact(40.0)) // for the source code link
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            self.table_ui(ui);
                        });
                    });
                    strip.cell(|ui| {
                        ui.separator();
                        ui.horizontal(|ui| {
                            let game_save = GameSaves::new(self.db.as_ref(), self.fs.as_ref());
                            let response = ui.button("Add Game");
                            if response.clicked() {
                                game_save.add_game_save();
                            }
                            let response = ui.button("Restore Game");
                            if response.clicked() {
                                game_save.restore_game_save();
                            }
                            let response = ui.button("Edit Game");
                            if response.clicked() {
                                game_save.edit_game_save();
                            }
                            let response = ui.button("Remove Game");
                            if response.clicked() {
                                show_message_box(ctx, "Remove Game", "Are you sure you want to remove this game?");
                                game_save.remove_game_save();
                            }
                        });

                            ui.label(self.selected_item.map_or("None".to_string(), |i| format!("Selected: {}", i)));
                    });
                });
        });
    }
}
