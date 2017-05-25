PRAGMA foreign_keys = ON;

CREATE TABLE games (
	id INTEGER PRIMARY KEY,
	player1 TEXT NOT NULL,
	player2 TEXT NOT NULL,
	winner TEXT NOT NULL,
	start_time DATETIME NOT NULL,
	duration INTEGER NOT NULL
)
