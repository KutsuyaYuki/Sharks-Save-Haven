CREATE TABLE Game (
  id INTEGER PRIMARY KEY,
  title TEXT,
  publisher TEXT,
  release_date DATE
);

CREATE TABLE Save (
  id INTEGER PRIMARY KEY,
  game_id INTEGER,
  location_id INTEGER,
  metadata TEXT,
  FOREIGN KEY (game_id) REFERENCES Game(id),
  FOREIGN KEY (location_id) REFERENCES Location(id)
);

CREATE TABLE Platform (
  id INTEGER PRIMARY KEY,
  platform_name TEXT
);

CREATE TABLE Location (
  id INTEGER PRIMARY KEY,
  location_path TEXT,
  description TEXT
);
