PRAGMA foreign_keys = ON;

CREATE TABLE players (
	id TEXT PRIMARY KEY,
	name TEXT NOT NULL,
	active BOOLEAN NOT NULL DEFAULT 't'
)
