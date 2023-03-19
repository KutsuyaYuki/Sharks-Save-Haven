use std::fs::{self};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use db::Game;
use eframe::egui;
use widgets::{Column, TableBuilder};

mod db;
mod filesystem;
mod widgets;
const DB_NAME: &str = "local_games.db";

struct MyApp {
    items: Vec<Game>,
    selected_item: Option<usize>,
    db: Box<db::Db>,
    fs: Box<filesystem::Filesystem>,
}

impl MyApp {
    fn new() -> Self {
        let db =db::Db::new(DB_NAME).expect("Failed to create database connection");
        let fs =filesystem::Filesystem::new();

        db.create_tables().expect("Failed to create tables");

        let games = db.get_all_games().expect("Failed to get games");

        Self {
            items: games,
            selected_item: None,
            db: Box::new(db),
            fs: Box::new(fs),
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

impl eframe::App for MyApp {
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
                            let response = ui.button("Add Game");
                            if response.clicked() {
                                add_game_save(self.db.as_ref(), self.fs.as_ref());
                            }

                            ui.label(self.selected_item.map_or("None".to_string(), |i| format!("Selected: {}", i)));
                    });
                });

        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let my_app = MyApp::new();

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

fn delete_game_save(db: &db::Db, fs: &filesystem::Filesystem) {
    // Get the game title from the user
    print!("Enter the game title: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    // Retrieve games from the database and display them to the user
    let games = db
        .get_games_by_title(title.trim())
        .expect("Failed to get games");
    if games.is_empty() {
        println!("No games found with that title");
    } else {
        println!("Select a game to delete:");
        for game in games {
            println!("{} - {}", game.id, game.title);
        }

        // Get the user's choice
        print!("> ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        // Convert the user's choice to an integer
        let game_id = choice.trim().parse::<i32>().expect("Invalid input");

        // Check if the game exists in the database
        let existing_game = db.get_game(game_id).expect("Failed to get game");

        // Delete the game's save files by using get_all_saves
        let saves = db
            .get_all_saves_by_id(game_id)
            .expect("Failed to get saves");
        for save in saves {
            let backup_file_location = PathBuf::from(&format!(
                "backups/{}/{}/{}/",
                save.game_id, save.platform_id, save.id
            ));

            if backup_file_location.exists() {
                fs::remove_dir_all(&backup_file_location).expect("Failed to delete game save");
                println!("Game save for '{}' deleted", existing_game.title);
            } else {
                println!("No save files found for '{}'", existing_game.title);
            }

            // Delete the save from the database
            db.delete_save(save.id).expect("Failed to delete save");
            db.delete_location(save.location_id)
                .expect("Failed to delete location");
        }

        // Delete the game from the database
        db.delete_game(game_id).expect("Failed to delete game");

        println!("'{}' deleted", existing_game.title);
    }
}

fn update_game_save(db: &db::Db, fs: &filesystem::Filesystem) {
    // Get the game title from the user
    print!("Enter the game title: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    // Retrieve games from the database and display them to the user
    let games = db
        .get_games_by_title(title.trim())
        .expect("Failed to get games");
    if games.is_empty() {
        println!("No games found with that title");
    } else {
        println!("Select a game to update:");
        for game in games {
            println!("{} - {}", game.id, game.title);
        }

        // Get the user's choice
        print!("> ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        // Convert the user's choice to an integer
        let game_id = choice.trim().parse::<i32>().expect("Invalid input");

        // Check if the game exists in the database
        let existing_game = db.get_game(game_id).expect("Failed to get game");

        // Display the game information to the user
        println!("Game information:");
        println!("Title: {}", existing_game.title);
        println!("Publisher: {}", existing_game.publisher);
        println!("Release date: {}", existing_game.release_date);

        // Get the new game information from the user
        print!("Enter new title (leave empty to keep existing title): ");
        io::stdout().flush().unwrap();
        let mut new_title = String::new();
        io::stdin()
            .read_line(&mut new_title)
            .expect("Failed to read line");

        print!("Enter new publisher (leave empty to keep existing publisher): ");
        io::stdout().flush().unwrap();
        let mut new_publisher = String::new();
        io::stdin()
            .read_line(&mut new_publisher)
            .expect("Failed to read line");

        print!("Enter new release date (leave empty to keep existing release date): ");
        io::stdout().flush().unwrap();
        let mut new_release_date = String::new();
        io::stdin()
            .read_line(&mut new_release_date)
            .expect("Failed to read line");

        // Update the game information in the database
        let new_title = new_title.trim().to_string();
        let new_publisher = new_publisher.trim().to_string();
        let new_release_date = new_release_date.trim().to_string();

        if !new_title.is_empty() || !new_publisher.is_empty() || !new_release_date.is_empty() {
            let title = if new_title.is_empty() {
                existing_game.title
            } else {
                new_title
            };
            let publisher = if new_publisher.is_empty() {
                existing_game.publisher
            } else {
                new_publisher
            };
            let release_date = if new_release_date.is_empty() {
                existing_game.release_date.to_string()
            } else {
                new_release_date
            };

            db.update_game(game_id, &title, &publisher, &release_date)
                .expect("Failed to update game");
            println!("Game information updated");
        } else {
            println!("No changes made to game information");
        }
    }
}

/// Adds a new game save to the database and creates a backup of the save files in the backup folder.
///
/// # Arguments
///
/// * `db` - A reference to a `db::Db` instance.
/// * `fs` - A reference to a `filesystem::Filesystem` instance.
///
/// # Errors
///
/// This function will return an error if any of the following operations fail:
///
/// * Failed to read user input
/// * Failed to insert game information into the database
/// * Failed to insert platform information into the database
/// * Failed to insert location information into the database
/// * Failed to insert save information into the database
/// * Failed to copy save files to backup folder
fn add_game_save(db: &db::Db, fs: &filesystem::Filesystem) {
    // Get the game title from the user
    print!("Enter the game title: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    // Get the publisher from the user
    print!("Enter the publisher: ");
    io::stdout().flush().unwrap();
    let mut publisher = String::new();
    io::stdin()
        .read_line(&mut publisher)
        .expect("Failed to read line");

    // Get the release date from the user
    print!("Enter the release date: ");
    io::stdout().flush().unwrap();
    let mut release_date = String::new();
    io::stdin()
        .read_line(&mut release_date)
        .expect("Failed to read line");

    // Get the platform from the user
    print!("Enter the platform: ");
    io::stdout().flush().unwrap();
    let mut platform = String::new();
    io::stdin()
        .read_line(&mut platform)
        .expect("Failed to read line");

    // Get the save file location from the user
    print!("Enter the save file location: ");
    io::stdout().flush().unwrap();
    let mut location = String::new();
    io::stdin()
        .read_line(&mut location)
        .expect("Failed to read line");

    let game_id = db
        .insert_game(title.trim(), publisher.trim(), release_date.trim())
        .expect("Failed to insert game");
    let platform_id = db
        .insert_platform(platform.trim())
        .expect("Failed to insert platform");
    let location_id = db
        .insert_location(&location.trim(), "")
        .expect("Failed to insert location");
    let save_id = db
        .insert_save(game_id, location_id, "", platform_id)
        .expect("Failed to insert save");

    // Copy the save files to the backup folder
    let save_file_location = PathBuf::from(&location.trim());
    let backup_file_location =
        PathBuf::from(&format!("backups/{}/{}/{}/", game_id, platform_id, save_id));

    fs.copy_files(&save_file_location, &backup_file_location)
        .expect("Failed to copy files");
}

/// Prompts the user to restore a game save.
///
/// Restores the game save data from the database and displays it to the user. Asks the user if they want to restore the game save and if so, copies the save files from the backup folder to the original save file location. If the user wants to restore only select files, prompts the user to confirm each file copy operation.
///
/// # Arguments
///
/// * `db` - A reference to a `Db` instance to restore data from the local_games database.
/// * `fs` - A reference to a `Filesystem` instance to handle file I/O operations.
///
/// # Examples
///
/// ```
/// let db = db::Db::new("local_games.db").unwrap();
/// let fs = filesystem::Filesystem::new();
/// restore_game_save(&db, &fs);
/// ```
///
/// # Errors
///
/// This function will return an error if the backup file copy operation fails due to a file I/O error.
fn restore_game_save(db: &db::Db, fs: &filesystem::Filesystem) {
    // Get the game title from the user
    print!("Enter the game title: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    // Restore games from the database and display them to the user
    let games = db
        .get_games_by_title(title.trim())
        .expect("Failed to get games");
    if games.is_empty() {
        println!("No games found with that title");
    } else {
        println!("Select a game to restore:");
        for game in games {
            println!("{} - {}", game.id, game.title);
        }

        // Get the user's choice
        print!("> ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        // Convert the user's choice to an integer
        let game_id = choice.trim().parse::<i32>().expect("Invalid input");

        // Check if the game exists in the database
        let game = db.get_game(game_id).expect("Failed to get game");

        // Display the game information to the user
        println!("Game information:");
        println!("Title: {}", game.title);
        println!("Publisher: {}", game.publisher);
        println!("Release date: {}", game.release_date);

        let saves = db
            .get_saves_by_game_id(game.id)
            .expect("Failed to retrieve save from database");

        // Display the game save data to the user
        for save in saves.iter() {
            let location = db
                .get_location(save.location_id)
                .expect("Failed to retrieve location from database");
            let platform = db
                .get_platform(save.platform_id)
                .expect("Failed to retrieve platform from database");

            // Display the game save data
            println!("Game title: {}", game.title);
            println!("Publisher: {}", game.publisher);
            println!("Release date: {}", game.release_date);
            println!("Platform: {}", platform.platform_name);
            println!("Save file location: {}", location.location_path);

            // Ask the user if they want to restore the game save
            print!("Do you want to restore this game save? (Y/n/a): ");
            io::stdout().flush().unwrap();
            let mut restore = String::new();
            io::stdin()
                .read_line(&mut restore)
                .expect("Failed to read line");

            // If the user wants to restore the game save, copy the save file to the correct location
            // default option y
            if restore.trim() == "Y" || restore.trim() == "" {
                println!("Restoring game save...");

                // Copy the save files from the backup folder to the save file location one by one and ask per file
                let backup_file_location = PathBuf::from(&format!(
                    "backups/{}/{}/{}/",
                    game.id, save.platform_id, save.id
                ));

                for entry in fs::read_dir(&backup_file_location).expect("Failed to read directory")
                {
                    let entry = entry.expect("Failed to read directory entry");
                    let file_name = entry.file_name();
                    let file_path = entry.path();

                    // Ask the user whether to copy the file or not
                    print!("Copy file {:?}? (Y/n): ", file_name);
                    io::stdout().flush().unwrap();
                    let mut answer = String::new();
                    io::stdin()
                        .read_line(&mut answer)
                        .expect("Failed to read answer");

                    if answer.trim().to_lowercase() == "y" || restore.trim() == "" {
                        // Copy the file to the save file location
                        let dest_file = Path::new(&location.location_path).join(file_name);
                        fs::copy(&file_path, &dest_file).expect("Failed to copy file");
                    }
                }
            }
            // if the user wants to restore all game saves, copy the save file to the correct location
            else if restore.trim() == "a" {
                println!("Restoring all game saves...");

                // Copy the save files from the backup folder to the save file location
                let save_file_location = PathBuf::from(&location.location_path);
                let backup_file_location = PathBuf::from(&format!(
                    "backups/{}/{}/{}/",
                    game.id, save.platform_id, save.id
                ));

                fs.copy_files(&backup_file_location, &save_file_location)
                    .expect("Failed to copy files");
            }
        }
    }
}
