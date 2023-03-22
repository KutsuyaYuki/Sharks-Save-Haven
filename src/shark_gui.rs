use egui::{Context, Id};

use crate::{
    db::{self, Game},
    filesystem,
    game_saves::GameSaves,
    widgets::{self, Column, TableBuilder},
    DB_NAME,
};

#[derive(Clone, Debug, Default)]
struct NewGameState {
    new_game: Game,
    release_date_input: String,
    plartform_input: String,
    location_input: String,
}

impl NewGameState {
    pub fn load(ctx: &Context) -> Option<Self> {
        ctx.data_mut(|d| d.get_temp(Id::null()))
    }

    fn store(self, ctx: &Context) {
        ctx.data_mut(|d| d.insert_temp(Id::null(), self));
    }
}

pub struct SharkGui {
    items: Vec<Game>,
    selected_item: Option<usize>,
    db: Box<db::Db>,
    fs: Box<filesystem::Filesystem>,
    picked_path: Option<String>,
}

impl SharkGui {
    pub fn new() -> Self {
        let db = db::Db::new(DB_NAME).expect("Failed to create database connection");
        let fs = filesystem::Filesystem::new();

        db.create_tables().expect("Failed to create tables");

        let games = db.get_all_games().expect("Failed to get games");

        let fs = Box::new(fs);
        let db = Box::new(db);
        Self {
            items: games,
            selected_item: None,
            db,
            fs,
            picked_path: None,
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
                                ui.label(game[row_index].id.to_string().clone());
                            });

                            row.col(|ui| {
                                ui.label(game[row_index].publisher.to_string().clone());
                            });

                            row.col(|ui| {
                                ui.label(game[row_index].title.to_string().clone());
                            });
                        });
                    }
                }
            });
    }
}

impl eframe::App for SharkGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let remove_popup_id = ui.make_persistent_id("remove_game_popup");
            let add_popup_id = ui.make_persistent_id("add_game_popup");

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
                            let add_button_response = ui.button("Add Game");
                            if add_button_response.clicked() {
                                ui.memory_mut(|mem| mem.open_popup(add_popup_id));
                            }
                            let restore_button_response = ui.button("Restore Game");
                            if restore_button_response.clicked() {
                                game_save.restore_game_save();
                            }
                            let edit_button_response = ui.button("Edit Game");
                            if edit_button_response.clicked() {
                                game_save.edit_game_save();
                            }
                            let remove_button_response = ui.button("Remove Game");
                            if remove_button_response.clicked() {
                                ui.memory_mut(|mem| mem.toggle_popup(remove_popup_id));
                            }
                            let below = egui::AboveOrBelow::Above;
                            egui::popup::popup_above_or_below_widget(
                                ui,
                                remove_popup_id,
                                &remove_button_response,
                                below,
                                |ui| {
                                    ui.set_min_width(200.0); // if you want to control the size
                                    ui.label("Are you sure you want to remove this game?");
                                    ui.horizontal(|ui| {
                                        if ui.button("Yes").clicked() {
                                            if let Some(selected_index) = self.selected_item {
                                                let game_id = self.items[selected_index].id;
                                                game_save.remove_game_save(game_id);
                                            } else {
                                                println!("No game is selected.");
                                            }
                                        }
                                        if ui.button("No").clicked() {
                                            return;
                                        }
                                    })
                                },
                            );
                            widgets::popup::popup_above_or_below_widget(
                                ui,
                                add_popup_id,
                                &add_button_response,
                                below,
                                |ui| {
                                    ui.set_min_width(200.0); // if you want to control the size
                                    ui.label("Fill in to add game");
                                    ui.vertical(|ui| {
                                        let year = 2023;
                                        let month = 3;
                                        let day = 22;
                                        let release_date =
                                            format!("{:04}{:02}{:02}", year, month, day)
                                                .parse::<i32>()
                                                .unwrap();

                                        let mut new_game_state =
                                            NewGameState::load(ctx).unwrap_or_default();
                                        new_game_state.new_game.id = -1;

                                        ui.label("Title");
                                        ui.text_edit_singleline(&mut new_game_state.new_game.title);

                                        ui.label("Publisher");
                                        ui.text_edit_singleline(
                                            &mut new_game_state.new_game.publisher,
                                        );
                                        ui.label("Platform");
                                        ui.text_edit_singleline(
                                            &mut new_game_state.plartform_input,
                                        );

                                        ui.label("Release Date (YYYYMMDD)");
                                        ui.text_edit_singleline(
                                            &mut new_game_state.release_date_input,
                                        );

                                        ui.vertical(|ui| {
                                            ui.label("Location");
                                            if ui.button("Open file…").clicked() {
                                                if let Some(path) =
                                                    rfd::FileDialog::new().pick_folder()
                                                {
                                                    new_game_state.location_input =
                                                        Some(path.display().to_string())
                                                            .unwrap_or_default();
                                                }
                                            }

                                            ui.text_edit_singleline(
                                                &mut new_game_state.location_input,
                                            );
                                        });

                                        if ui.button("Finish").clicked() {
                                            if let Ok(parsed_release_date) = new_game_state.release_date_input.parse::<i32>() {
                                                new_game_state.new_game.release_date = parsed_release_date;
                                            } else {
                                                // handle invalid input
                                            }
                                            // Insert game
                                            game_save.add_game_save(
                                                new_game_state.new_game.clone(),
                                                new_game_state.location_input.clone(), new_game_state.plartform_input.clone());
                                            ui.memory_mut(|mem| mem.toggle_popup(add_popup_id));
                                        }
                                        if ui.button("Cancel").clicked() {
                                            ui.memory_mut(|mem| mem.toggle_popup(add_popup_id));
                                        }

                                        new_game_state.store(ctx);
                                    })
                                },
                            );
                        });

                        ui.label(
                            self.selected_item
                                .map_or("None".to_string(), |i| format!("Selected: {}", i)),
                        );
                    });
                });
        });
    }
}
