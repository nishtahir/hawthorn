CREATE TABLE token (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  player_id INTEGER NOT NULL,
  content TEXT NOT NULL,
  CONSTRAINT participant_player_fk FOREIGN KEY(player_id) REFERENCES player(id)
);