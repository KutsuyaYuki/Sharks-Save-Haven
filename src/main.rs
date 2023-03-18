use std::fs::{self};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

mod db;
mod filesystem;
const DB_NAME: &str = "local_games.db";

fn main() {
    let db : db::Db;
    db = db::Db::new(DB_NAME).expect("Failed to create database connection");

    let fs : filesystem::Filesystem;
    fs = filesystem::Filesystem::new();

    db.create_tables().expect("Failed to create tables");

    // Display the main menu
    loop {
        println!("--- Game Save Backup and Restore ---");
        println!("1. Add a game save");
        println!("2. Retrieve a game save");
        println!("3. Exit");

        // Get the user's choice
        print!("> ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        // Handle the user's choice
        match choice.trim() {
            "1" => add_game_save(&db, &fs),
            "2" => restore_game_save(&db, &fs),
            "3" => break,
            _ => println!("Invalid choice"),
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
fn add_game_save(db : &db::Db, fs : &filesystem::Filesystem) {
    // Get the game title from the user
    print!("Enter the game title: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read line");

    // Get the publisher from the user
    print!("Enter the publisher: ");
    io::stdout().flush().unwrap();
    let mut publisher = String::new();
    io::stdin().read_line(&mut publisher).expect("Failed to read line");

    // Get the release date from the user
    print!("Enter the release date: ");
    io::stdout().flush().unwrap();
    let mut release_date = String::new();
    io::stdin().read_line(&mut release_date).expect("Failed to read line");

    // Get the platform from the user
    print!("Enter the platform: ");
    io::stdout().flush().unwrap();
    let mut platform = String::new();
    io::stdin().read_line(&mut platform).expect("Failed to read line");

    // Get the save file location from the user
    print!("Enter the save file location: ");
    io::stdout().flush().unwrap();
    let mut location = String::new();
    io::stdin().read_line(&mut location).expect("Failed to read line");

    let game_id = db.insert_game(title.trim(), publisher.trim(), release_date.trim()).expect("Failed to insert game");
    let platform_id = db.insert_platform(platform.trim()).expect("Failed to insert platform");
    let location_id = db.insert_location(&location.trim(), "").expect("Failed to insert location");
    let save_id = db.insert_save(game_id, location_id, "", platform_id).expect("Failed to insert save");

    // Copy the save files to the backup folder
    let save_file_location = PathBuf::from(&location.trim());
    let backup_file_location = PathBuf::from(&format!("backups/{}/{}/{}/", game_id, platform_id, save_id));

    fs.copy_files(&save_file_location, &backup_file_location).expect("Failed to copy files");

}

/// Prompts the user to restore a game save.
///
/// Retrieves the game save data from the database and displays it to the user. Asks the user if they want to restore the game save and if so, copies the save files from the backup folder to the original save file location. If the user wants to restore only select files, prompts the user to confirm each file copy operation.
///
/// # Arguments
///
/// * `db` - A reference to a `Db` instance to retrieve data from the local_games database.
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
fn restore_game_save(db : &db::Db, fs : &filesystem::Filesystem) {
    // Get the game title from the user
    print!("Enter the game title: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read line");

    // Retrieve the data from the database
    let game = db.get_game_by_title(title.trim()).expect("Failed to retrieve game from database");

    // Check if the game exists in the database
    if game.id == -1 {
        println!("Game not found");
        return ();
    }

    let saves = db.get_saves_by_game_id(game.id).expect("Failed to retrieve save from database");

    // Display the game save data to the user
    for save in saves.iter() {
        let location = db.get_location(save.location_id).expect("Failed to retrieve location from database");
        let platform = db.get_platform(save.platform_id).expect("Failed to retrieve platform from database");
        
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
        io::stdin().read_line(&mut restore).expect("Failed to read line");

        // If the user wants to restore the game save, copy the save file to the correct location
        // default option y
        if restore.trim() == "Y" || restore.trim() == "" {
            println!("Restoring game save...");

            // Copy the save files from the backup folder to the save file location one by one and ask per file
            let backup_file_location = PathBuf::from(&format!("backups/{}/{}/{}/", game.id, save.platform_id, save.id));

            for entry in fs::read_dir(&backup_file_location).expect("Failed to read directory") {
                let entry = entry.expect("Failed to read directory entry");
                let file_name = entry.file_name();
                let file_path = entry.path();
            
                // Ask the user whether to copy the file or not
                print!("Copy file {:?}? (Y/n): ", file_name);
                io::stdout().flush().unwrap();
                let mut answer = String::new();
                io::stdin().read_line(&mut answer).expect("Failed to read answer");
            
                if answer.trim().to_lowercase() == "y"  || restore.trim() == "" {
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
            let backup_file_location = PathBuf::from(&format!("backups/{}/{}/{}/", game.id, save.platform_id, save.id));

            fs.copy_files(&backup_file_location, &save_file_location).expect("Failed to copy files");
        }
    }
}
