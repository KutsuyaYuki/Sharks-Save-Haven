use rusqlite::{params, Connection, Result};

pub struct Db {
    conn: Connection,
}

#[derive(Clone, Debug, Default)]
pub struct Game {
    pub id: i32,
    pub title: String,
    pub publisher: String,
    pub release_date: i32,
    pub platform: String,
}

pub struct Platform {
    pub id: i32,
    pub platform_name: String,
}

pub struct Location {
    pub id: i32,
    pub location_path: String,
    pub description: String,
}
pub struct Save {
    pub id: i32,
    pub game_id: i32,
    pub location_id: i32,
    pub metadata: Option<String>,
    pub platform_id: i32,
}

impl Db {
    /// Opens a new connection to a SQLite database file.
    ///
    /// # Arguments
    ///
    /// * `filename` - The name of the SQLite database file to connect to.
    ///
    /// # Errors
    ///
    /// This function will return an error if the database file cannot be opened.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crate::db::Db;
    ///
    /// let db = Db::new("mydatabase.db").expect("Failed to create database connection");
    /// ```
    pub fn new(filename: &str) -> Result<Self> {
        let conn = Connection::open(filename)?;
        Ok(Self { conn })
    }

    /// Create the necessary database tables if they do not already exist.
    ///
    /// # Errors
    ///
    /// This function will return an error if there is an issue executing the SQL statements to create the tables.
    pub fn create_tables(&self) -> Result<()> {
        // Check if the tables in the database exist If they don't, create them
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Game (
                id INTEGER PRIMARY KEY,
                title TEXT,
                publisher TEXT,
                release_date DATE
            )",
            params![],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Platform (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                platform_name TEXT NOT NULL UNIQUE
            )",
            params![],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Location (
                id INTEGER PRIMARY KEY,
                location_path TEXT,
                description TEXT
            )",
            params![],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Save (
                id INTEGER PRIMARY KEY,
                game_id INTEGER,
                location_id INTEGER,
                metadata TEXT,
                platform_id INTEGER,
                FOREIGN KEY (game_id) REFERENCES Game(id),
                FOREIGN KEY (location_id) REFERENCES Location(id),
                FOREIGN KEY (platform_id) REFERENCES Platform(id)
            )",
            params![],
        )?;

        Ok(())
    }

    /// Inserts a new game into the database with the given title, publisher, and release date.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the game to be inserted.
    /// * `publisher` - The publisher of the game to be inserted.
    /// * `release_date` - The release date of the game to be inserted.
    ///
    /// # Errors
    ///
    /// This function will return an error if there was a problem inserting the game into the database.
    ///
    /// # Returns
    ///
    /// Returns the ID of the newly inserted game on success.
        pub fn insert_game(&self, title: &str, publisher: &str, release_date: &str) -> Result<i32> {
        self.conn.execute(
            "INSERT INTO Game (title, publisher, release_date) VALUES (?1, ?2, ?3)",
            params![title, publisher, release_date],
        )?;
        // return the last inserted row id
        let id = self.conn.last_insert_rowid() as i32;
        Ok(id)
    }

    /// Inserts a new platform into the database with the given platform name.
    ///
    /// # Arguments
    ///
    /// * `platform_name` - The name of the platform to be inserted.
    ///
    /// # Errors
    ///
    /// This function will return an error if there was a problem inserting the platform into the database.
    ///
    /// # Returns
    ///
    /// Returns the ID of the newly inserted or updated platform on success. If a row with the same platform name already exists in the database, the function will return the ID of that row instead.
    pub fn insert_platform(&self, platform_name: &str) -> Result<i32> {
        // Check if a row with the same platform_name already exists
        let mut stmt = self.conn.prepare("SELECT id FROM Platform WHERE platform_name = ?1")?;
        let mut rows = stmt.query(params![platform_name])?;
        if let Ok(Some(row)) = rows.next() {
            // If a row with the same platform_name exists, return its ID
            let id = row.get(0)?;
            return Ok(id);
        }
    
        // Use INSERT OR REPLACE to update or insert the row
        self.conn.execute(
            "INSERT OR REPLACE INTO Platform (id, platform_name) VALUES ((SELECT id FROM Platform WHERE platform_name = ?1), ?2)",
            params![platform_name, platform_name],
        )?;
    
        // Return the ID of the row that was inserted or updated
        let id = self.conn.last_insert_rowid() as i32;
        Ok(id)
    }
    
    /// Inserts a new location into the database with the given location path and description.
    ///
    /// # Arguments
    ///
    /// * `location_path` - The path of the location to be inserted.
    /// * `description` - A description of the location to be inserted.
    ///
    /// # Errors
    ///
    /// This function will return an error if there was a problem inserting the location into the database.
    ///
    /// # Returns
    ///
    /// Returns the ID of the newly inserted location on success.
    pub fn insert_location(&self, location_path: &str, description: &str) -> Result<i32> {
        self.conn.execute(
            "INSERT INTO Location (location_path, description) VALUES (?1, ?2)",
            params![location_path, description],
        )?;
        // return the last inserted row id
        let id = self.conn.last_insert_rowid() as i32;
        Ok(id)
    }

    /// Inserts a new save into the database with the given game ID, location ID, metadata, and platform ID.
    ///
    /// # Arguments
    ///
    /// * `game_id` - The ID of the game that the save belongs to.
    /// * `location_id` - The ID of the location where the save is stored.
    /// * `metadata` - Any additional metadata associated with the save.
    /// * `platform_id` - The ID of the platform that the save is for.
    ///
    /// # Errors
    ///
    /// This function will return an error if there was a problem inserting the save into the database.
    ///
    /// # Returns
    ///
    /// Returns the ID of the newly inserted save on success.
    pub fn insert_save(&self, game_id: i32, location_id: i32, metadata: &str, platform_id: i32) -> Result<i32> {
        self.conn.execute(
            "INSERT INTO Save (game_id, location_id, metadata, platform_id) VALUES (?1, ?2, ?3, ?4)",
            params![game_id, location_id, metadata, platform_id],
        )?;
        // return the last inserted row id
        let id = self.conn.last_insert_rowid() as i32;
        Ok(id)
    }

    /// Updates the details of a game in the database.
    ///
    /// # Arguments
    ///
    /// * `game_id` - An integer representing the ID of the game to be updated.
    /// * `title` - A string slice representing the updated title of the game.
    /// * `publisher` - A string slice representing the updated publisher of the game.
    /// * `release_date` - A string slice representing the updated release date of the game.
    ///
    /// # Errors
    ///
    /// This function will return an error if the update operation fails.
    pub fn update_game(&self, game_id: i32, title: &str, publisher: &str, release_date: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE Game SET title = ?1, publisher = ?2, release_date = ?3 WHERE id = ?4",
            params![title, publisher, release_date, game_id],
        )?;
        Ok(())
    }

        /// Retrieves the game data from the database with the specified ID.
        ///
        /// # Arguments
        ///
        /// * `game_id` - The ID of the game to retrieve from the database.
        ///
        /// # Returns
        ///
        /// Returns a `Result` containing the retrieved `Game` struct, or an error if the game
        /// cannot be found in the database.
        ///
        /// # Errors
        ///
        /// This function will return an error if the game cannot be found in the database, or if there
        /// is an issue with the SQLite query.
        pub fn get_game(&self, game_id: i32) -> Result<Game> {
        let mut stmt = self.conn.prepare("SELECT * FROM Game WHERE id = ?1")?;
        let game_iter = stmt.query_map(params![game_id], |row| {
            Ok(Game {
                id: row.get(0)?,
                title: row.get(1).unwrap_or_default(),
                publisher: row.get(2).unwrap_or_default(),
                release_date: row.get(3).unwrap_or_default(),
                platform: row.get(4).unwrap_or_default(),
            })
        })?;

        for game in game_iter {
            return Ok(game?);
        }

        Ok(Game {
            id: -1,
            title: String::from(""),
            publisher: String::from(""),
            release_date: i32::from(0),
            platform: String::from(""),
        })
    }

    /// Retrieves a game from the database by its title.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the game to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `Game` struct representing the retrieved game if successful, or an error if the operation failed.
    ///
    /// # Example
    ///
    /// ```
    /// let game_title = "The Legend of Zelda: Breath of the Wild";
    /// let game = db.get_game_by_title(game_title)?;
    ///
    /// println!("Game Title: {}", game.title);
    /// println!("Publisher: {}", game.publisher);
    /// println!("Release Date: {}", game.release_date);
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the database operation fails for any reason.
    pub fn get_game_by_title(&self, title: &str) -> Result<Game> {
        let mut stmt = self.conn.prepare("SELECT * FROM Game WHERE title = ?1")?;
        let game_iter = stmt.query_map(params![title], |row| {
            Ok(Game {
                id: row.get(0)?,
                title: row.get(1).unwrap_or_default(),
                publisher: row.get(2).unwrap_or_default(),
                release_date: row.get(3).unwrap_or_default(),
                platform: row.get(4).unwrap_or_default(),
            })
        })?;

        for game in game_iter {
            return Ok(game?);
        }

        Ok(Game {
            id: -1,
            title: String::from(""),
            publisher: String::from(""),
            release_date: i32::from(0),
            platform: String::from(""),
        })
    }

        /// Retrieves a vector of all games in the database that have a title starting with the specified string.
        ///
        /// # Arguments
        ///
        /// * `title` - A string slice representing the beginning of the title of the desired games.
        ///
        /// # Returns
        ///
        /// * A `Result` containing a vector of `Game` structs if successful, otherwise an error.
        ///
        /// # Examples
        ///
        /// ```
        /// let db = Db::new("local_games.db").unwrap();
        /// let games = db.get_games_by_title("The");
        /// println!("{:?}", games);
        /// ```
        pub fn get_games_by_title(&self, title: &str) -> Result<Vec<Game>>{
        // Shows all the games starting with the title
        let mut stmt = self.conn.prepare("SELECT * FROM Game WHERE title LIKE ?1")?;
        let game_iter = stmt.query_map(params![format!("{}%", title)], |row| {
            Ok(Game {
                id: row.get(0)?,
                title: row.get(1).unwrap_or_default(),
                publisher: row.get(2).unwrap_or_default(),
                release_date: row.get(3).unwrap_or_default(),
                platform: row.get(4).unwrap_or_default(),
            })
        })?;
        let games = game_iter.collect::<Result<Vec<Game>>>()?;
        Ok(games)
    }

    /// Retrieves a platform from the database with the specified platform ID.
    ///
    /// # Arguments
    ///
    /// * `platform_id` - An `i32` that represents the ID of the platform to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `Platform` struct representing the platform retrieved from the database.
    /// If the platform is not found in the database, the function returns a `Platform` struct with an ID of -1
    /// and an empty `platform_name` field.
    ///
    /// # Errors
    ///
    /// This function will return an error if there is a problem executing the SQL statement to retrieve the platform.
    pub fn get_platform(&self, platform_id: i32) -> Result<Platform> {
        let mut stmt = self.conn.prepare("SELECT platform_name FROM Platform WHERE id = ?1")?;
        let platform_iter = stmt.query_map(params![platform_id], |row| {
            Ok(Platform {
                id: platform_id,
                platform_name: row.get(0).unwrap_or_default(),
            })
        })?;

        for platform in platform_iter {
            return Ok(platform?);
        }

        Ok(Platform {
            id: -1,
            platform_name: String::from(""),
        })
    }

    /// Retrieves a location record from the database with the given location ID.
    ///
    /// # Arguments
    ///
    /// * `location_id` - An integer representing the ID of the location to retrieve.
    ///
    /// # Returns
    ///
    /// A `Location` struct containing the location information, or a `Location` struct with default values if the location
    /// with the given ID is not found.
    ///
    /// # Errors
    ///
    /// This function will return an error if the database connection fails or if the SQL query fails.
    pub fn get_location(&self, location_id: i32) -> Result<Location> {
        let mut stmt = self.conn.prepare("SELECT location_path, description FROM Location WHERE id = ?1")?;
        let location_iter = stmt.query_map(params![location_id], |row| {
            Ok(Location {
                id: location_id,
                location_path: row.get(0).unwrap_or_default(),
                description: row.get(1).unwrap_or_default(),
            })
        })?;

        for location in location_iter {
            return Ok(location?);
        }

        Ok(Location {
            id: -1,
            location_path: String::from(""),
            description: String::from(""),
        })
    }

    pub fn get_save(&self, save_id: i32) -> Result<String> {
        let mut stmt = self.conn.prepare("SELECT metadata FROM Save WHERE id = ?1")?;
        let save_iter = stmt.query_map(params![save_id], |row| {
            Ok(row.get(0)?)
        })?;

        for save in save_iter {
            return Ok(save?);
        }

        Ok(String::from("No save found"))
    }

    /// Returns a vector of `Save` objects for a given `game_id`.
    ///
    /// # Arguments
    ///
    /// * `game_id` - The ID of the game to retrieve saves for.
    ///
    /// # Errors
    ///
    /// This function will return an error if the query fails.
    pub fn get_saves_by_game_id(&self, game_id: i32) -> Result<Vec<Save>> {
        let mut stmt = self.conn.prepare("SELECT * FROM Save WHERE game_id = ?1")?;
        let save_iter = stmt.query_map(params![game_id], |row| {
            Ok(Save {
                id: row.get(0)?,
                game_id: row.get(1)?,
                location_id: row.get(2)?,
                metadata: row.get(3).unwrap_or_default(),
                platform_id: row.get(4)?,
            })
        })?;

        let mut saves = Vec::new();
        for save in save_iter {
            saves.push(save?);
        }

        Ok(saves)
    }
    
    pub fn get_all_games(&self) -> Result<Vec<Game>> {
        let mut stmt = self.conn.prepare("SELECT * FROM Game")?;
        let rows = stmt.query_map([], |row| {
            Ok(Game {
                id: row.get(0)?,
                title: row.get(1)?,
                publisher: row.get(2)?,
                release_date: row.get(3)?,
                platform: row.get(4)?,
            })
        })?;

        let mut games = Vec::new();
        for game in rows {
            games.push(game?);
        }

        Ok(games)
    }

    pub fn get_all_platforms(&self) -> Result<Vec<Platform>> {
        let mut stmt = self.conn.prepare("SELECT * FROM Platform")?;
        let rows = stmt.query_map([], |row| {
            Ok(Platform {
                id: row.get(0)?,
                platform_name: row.get(1)?,
            })
        })?;

        let mut platforms = Vec::new();
        for platform in rows {
            platforms.push(platform?);
        }

        Ok(platforms)
    }

    pub fn get_all_locations(&self) -> Result<Vec<Location>> {
        let mut stmt = self.conn.prepare("SELECT location_path FROM Location")?;
        let rows = stmt.query_map([], |row| {
            Ok(Location {
                id: row.get(0)?,
                location_path: row.get(1)?,
                description: row.get(2)?,
            })
        })?;

        let mut locations = Vec::new();
        for location in rows {
            locations.push(location?);
        }

        Ok(locations)
    }

    pub fn get_all_saves(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT metadata FROM Save")?;
        let rows = stmt.query_map([], |row| row.get(0))?;

        let mut saves = Vec::new();
        for save in rows {
            saves.push(save?);
        }

        Ok(saves)
    }

    pub fn get_all_saves_by_id(&self, game_id: i32) -> Result<Vec<Save>> {
        let mut stmt = self.conn.prepare("SELECT * FROM Save WHERE game_id = ?1")?;
        let rows = stmt.query_map(params![game_id], |row| {
            Ok(Save {
                id: row.get(0)?,
                game_id: row.get(1)?,
                location_id: row.get(2)?,
                metadata: row.get(3).unwrap_or_default(),
                platform_id: row.get(4)?,
            })
        })?;

        let mut saves = Vec::new();
        for save in rows {
            saves.push(save?);
        }

        Ok(saves)
    }

    pub fn get_all_saves_for_platform(&self, platform_id: i32) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT metadata FROM Save WHERE platform_id = ?1")?;
        let rows = stmt.query_map([], |row| row.get(0))?;

        let mut saves = Vec::new();
        for save in rows {
            saves.push(save?);
        }

        Ok(saves)
    }

    pub fn get_all_saves_for_location(&self, location_id: i32) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT metadata FROM Save WHERE location_id = ?1")?;
        let rows = stmt.query_map([], |row| row.get(0))?;

        let mut saves = Vec::new();
        for save in rows {
            saves.push(save?);
        }

        Ok(saves)
    }

    pub fn get_all_saves_for_game_and_platform(&self, game_id: i32, platform_id: i32) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT metadata FROM Save WHERE game_id = ?1 AND platform_id = ?2")?;
        let rows = stmt.query_map([], |row| row.get(0))?;

        let mut saves = Vec::new();
        for save in rows {
            saves.push(save?);
        }

        Ok(saves)
    }

    pub fn get_all_saves_for_game_and_location(&self, game_id: i32, location_id: i32) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT metadata FROM Save WHERE game_id = ?1 AND location_id = ?2")?;
        let rows = stmt.query_map([], |row| row.get(0))?;

        let mut saves = Vec::new();
        for save in rows {
            saves.push(save?);
        }

        Ok(saves)
    }

    pub fn get_all_saves_for_platform_and_location(&self, platform_id: i32, location_id: i32) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT metadata FROM Save WHERE platform_id = ?1 AND location_id = ?2")?;
        let rows = stmt.query_map([], |row| row.get(0))?;

        let mut saves = Vec::new();
        for save in rows {
            saves.push(save?);
        }

        Ok(saves)
    }

    pub fn get_all_saves_for_game_and_platform_and_location(&self, game_id: i32, platform_id: i32, location_id: i32) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT metadata FROM Save WHERE game_id = ?1 AND platform_id = ?2 AND location_id = ?3")?;
        let rows = stmt.query_map([], |row| row.get(0))?;

        let mut saves = Vec::new();
        for save in rows {
            saves.push(save?);
        }

        Ok(saves)
    }

    pub fn delete_game(&self, game_id: i32) -> Result<()> {
        let mut stmt = self.conn.prepare("DELETE FROM Game WHERE id = ?1")?;
        stmt.execute(params![game_id])?;

        Ok(())
    }

    pub fn delete_save(&self, save_id: i32) -> Result<()> {
        let mut stmt = self.conn.prepare("DELETE FROM Save WHERE id = ?1")?;
        stmt.execute(params![save_id])?;

        Ok(())
    }

    pub fn delete_location(&self, location_id: i32) -> Result<()> {
        let mut stmt = self.conn.prepare("DELETE FROM Location WHERE id = ?1")?;
        stmt.execute(params![location_id])?;

        Ok(())
    }
}

