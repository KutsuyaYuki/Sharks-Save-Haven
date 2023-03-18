use rusqlite::{params, Connection, Result};

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn new(filename: &str) -> Result<Self> {
        let conn = Connection::open(filename)?;
        Ok(Self { conn })
    }

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

    /// Insert a game into the database
    pub fn insert_game(&self, title: &str, publisher: &str, release_date: &str) -> Result<i32> {
        self.conn.execute(
            "INSERT INTO Game (title, publisher, release_date) VALUES (?1, ?2, ?3)",
            params![title, publisher, release_date],
        )?;
        // return the last inserted row id
        let id = self.conn.last_insert_rowid() as i32;
        Ok(id)
    }
    

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
    
    pub fn insert_location(&self, location_path: &str, description: &str) -> Result<i32> {
        self.conn.execute(
            "INSERT INTO Location (location_path, description) VALUES (?1, ?2)",
            params![location_path, description],
        )?;
        // return the last inserted row id
        let id = self.conn.last_insert_rowid() as i32;
        Ok(id)
    }

    pub fn insert_save(&self, game_id: i32, location_id: i32, metadata: &str, platform_id: i32) -> Result<i32> {
        self.conn.execute(
            "INSERT INTO Save (game_id, location_id, metadata, platform_id) VALUES (?1, ?2, ?3, ?4)",
            params![game_id, location_id, metadata, platform_id],
        )?;
        // return the last inserted row id
        let id = self.conn.last_insert_rowid() as i32;
        Ok(id)
    }

    pub fn get_game(&self, game_id: i32) -> Result<String> {
        let mut stmt = self.conn.prepare("SELECT title FROM Game WHERE id = ?1")?;
        let game_iter = stmt.query_map(params![game_id], |row| {
            Ok(row.get(0)?)
        })?;

        for game in game_iter {
            return Ok(game?);
        }

        Ok(String::from("No game found"))
    }

    pub fn get_platform(&self, platform_id: i32) -> Result<String> {
        let mut stmt = self.conn.prepare("SELECT platform_name FROM Platform WHERE id = ?1")?;
        let platform_iter = stmt.query_map(params![platform_id], |row| {
            Ok(row.get(0)?)
        })?;

        for platform in platform_iter {
            return Ok(platform?);
        }

        Ok(String::from("No platform found"))
    }

    pub fn get_location(&self, location_id: i32) -> Result<String> {
        let mut stmt = self.conn.prepare("SELECT location_path FROM Location WHERE id = ?1")?;
        let location_iter = stmt.query_map(params![location_id], |row| {
            Ok(row.get(0)?)
        })?;

        for location in location_iter {
            return Ok(location?);
        }

        Ok(String::from("No location found"))
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
    
    pub fn get_all_games(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT title FROM Game")?;
        let rows = stmt.query_map([], |row| row.get(0))?;

        let mut games = Vec::new();
        for game in rows {
            games.push(game?);
        }

        Ok(games)
    }

    pub fn get_all_platforms(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT platform_name FROM Platform")?;
        let rows = stmt.query_map([], |row| row.get(0))?;

        let mut platforms = Vec::new();
        for platform in rows {
            platforms.push(platform?);
        }

        Ok(platforms)
    }

    pub fn get_all_locations(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT location_path FROM Location")?;
        let rows = stmt.query_map([], |row| row.get(0))?;

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

    pub fn get_all_saves_for_game(&self, game_id: i32) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT metadata FROM Save WHERE game_id = ?1")?;
        let rows = stmt.query_map([], |row| row.get(0))?;

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
}

