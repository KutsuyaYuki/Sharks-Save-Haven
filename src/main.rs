use std::io::{self, Write};

use crate::db::Db;

mod db;
const DB_NAME: &str = "local_games.db";

fn main() {
    let db : db::Db;
    db = db::Db::new(DB_NAME).expect("Failed to create database connection");
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
            "1" => add_game_save(&db),
            "2" => retrieve_game_save(),
            "3" => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn add_game_save(db : &db::Db) {
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
    db.insert_save(game_id, location_id, "", platform_id).expect("Failed to insert save");

}

fn retrieve_game_save() {
    // Get the game title from the user
    print!("Enter the game title: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read line");

    // TODO: Retrieve the game save data from the database and display it to the user
    println!("TODO: Retrieve game save data");
}
