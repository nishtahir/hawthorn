 ALTER TABLE deck RENAME TO temp_deck;
 
CREATE TABLE deck (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  alias TEXT NOT NULL UNIQUE,
  commander TEXT NOT NULL,
  player_id INTEGER NOT NULL,
  CONSTRAINT deck_player_fk FOREIGN KEY(player_id) REFERENCES player(id)
);
 
INSERT INTO deck SELECT id, alias, commander, player_id FROM temp_deck;

DROP TABLE temp_deck;