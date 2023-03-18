use rusqlite::{params, Connection, Result};

pub fn create_db() -> Result<()> {
    // Open a new SQLite database file
    let conn = Connection::open("games.db")?;

    // Create the Game table
    conn.execute(
        "CREATE TABLE Game (
            id INTEGER PRIMARY KEY,
            title TEXT,
            publisher TEXT,
            release_date DATE
        )",
        params![],
    )?;

    // Create the Platform table
    conn.execute(
        "CREATE TABLE Platform (
            id INTEGER PRIMARY KEY,
            platform_name TEXT
        )",
        params![],
    )?;

    // Create the Location table
    conn.execute(
        "CREATE TABLE Location (
            id INTEGER PRIMARY KEY,
            location_path TEXT,
            description TEXT
        )",
        params![],
    )?;

    // Create the Save table
    conn.execute(
        "CREATE TABLE Save (
            id INTEGER PRIMARY KEY,
            game_id INTEGER,
            location_id INTEGER,
            metadata TEXT,
            FOREIGN KEY (game_id) REFERENCES Game(id),
            FOREIGN KEY (location_id) REFERENCES Location(id)
        )",
        params![],
    )?;

    Ok(())
}
