use std::process;

use chrono::NaiveDate;
use egui::{Context, Id, Pos2, Vec2};

use crate::{
    db::{self, Game},
    filesystem,
    game_saves::GameSaves,
    widgets::{Column, TableBuilder},
    DB_NAME,
};

#[derive(Clone, Debug, Default)]
struct NewGameState {
    new_game: Game,
    release_date_input: NaiveDate,
    platform_input: String,
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
    add_game_window_open: bool,
}

impl SharkGui {
    pub fn new() -> Self {
        let db = db::Db::new(DB_NAME).expect("Failed to create database connection");
        let fs = filesystem::Filesystem::new();

        db.create_tables().expect("Failed to create tables");

        let games = db.get_all_games().expect("Failed to get games");

        Self {
            items: games,
            selected_item: None,
            db: Box::new(db),
            fs: Box::new(fs),
            add_game_window_open: false,
        }
    }

    fn load_windows(&mut self, ui: &mut egui::Ui) {
        self.load_add_game_window(ui);
    }

    fn load_add_game_window(&mut self, ui: &mut egui::Ui) {
        let game_save = GameSaves::new(self.db.as_ref(), self.fs.as_ref());

        let default_pos = ui.available_rect_before_wrap().center();

        egui::Window::new("Add game")
            .default_size(Vec2::new(400.0, 400.0))
            .default_pos(Pos2::new(default_pos.x - 200.0, default_pos.y - 200.0))
            .open(&mut self.add_game_window_open)
            .show(ui.ctx(), |ui| {
                ui.set_min_width(200.0);
                ui.label("Fill in to add game");
                ui.vertical(|ui| {
                    let mut new_game_state = NewGameState::load(ui.ctx()).unwrap_or_default();
                    new_game_state.new_game.id = -1;

                    ui.label("Title");
                    ui.text_edit_singleline(&mut new_game_state.new_game.title);

                    ui.label("Publisher");
                    ui.text_edit_singleline(&mut new_game_state.new_game.publisher);
                    ui.label("Platform");
                    ui.text_edit_singleline(&mut new_game_state.platform_input);

                    ui.label("Release Date (YYYYMMDD)");
                    ui.add(egui_extras::DatePickerButton::new(
                        &mut new_game_state.release_date_input,
                    ));

                    ui.vertical(|ui| {
                        ui.label("Location");
                        if ui.button("Open file…").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                new_game_state.location_input =
                                    path.display().to_string();
                            }
                        }

                        ui.text_edit_singleline(&mut new_game_state.location_input);
                    });

                    ui.text_edit_singleline(&mut new_game_state.location_input);

                    if ui.button("Finish").clicked() {
                        new_game_state.new_game.release_date = new_game_state
                            .release_date_input
                            .and_hms_opt(0, 0, 0)
                            .unwrap_or_default()
                            .timestamp();

                        game_save.add_game_save(
                            new_game_state.new_game.clone(),
                            new_game_state.location_input.clone(),
                            new_game_state.platform_input.clone(),
                        );
                        ui.close_menu();
                    }
                    if ui.button("Cancel").clicked() {}

                    new_game_state.store(ui.ctx());
                })
            });
    }

    fn file_top_menu(ui: &mut egui::Ui) {
        if ui.button("Exit").clicked() {
            process::exit(0);
        }
    }

    fn game_top_menu(&mut self, ui: &mut egui::Ui) {
        let add_button_response = ui.add(egui::Button::new("Add Game"));
        if add_button_response.clicked() {
            self.add_game_window_open = true;
            ui.close_menu();
        }
    }

    fn table_ui(&mut self, ui: &mut egui::Ui) {
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
            .body(|mut body| {
                for (_row_index, game) in self.items.iter().enumerate() {
                    let row_height = 18.00;
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(game.id.to_string().clone());
                        });

                        row.col(|ui| {
                            ui.label(game.publisher.to_string().clone());
                        });

                        row.col(|ui| {
                            ui.label(game.title.to_string().clone());
                        });
                    });
                }
            });
    }
}

impl eframe::App for SharkGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.load_windows(ui);

            ui.horizontal(|ui| {
                ui.visuals_mut().button_frame = false;
                ui.menu_button("File", Self::file_top_menu);
                ui.menu_button("Game", |ui| {
                    self.game_top_menu(ui);
                });
            });

            ui.separator();

            egui::ScrollArea::horizontal().show(ui, |ui| {
                self.table_ui(ui);
            });
        });
    }
}
