ALTER TABLE games ADD FOREIGN KEY(winner) REFERENCES players(id);
ALTER TABLE games ADD FOREIGN KEY(loser) REFERENCES players(id);
