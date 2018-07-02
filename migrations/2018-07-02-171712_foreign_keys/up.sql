-- Add foreign key constraints to tables.
-- SQLite does not support ALTER TABLE ADD CONSTRAINT,
-- so the tables must redone.

-- Add foreign key to deck
CREATE TABLE deck_new (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  alias TEXT NOT NULL UNIQUE,
  commander TEXT NOT NULL,
  player_id INTEGER NOT NULL,
  CONSTRAINT deck_player_fk FOREIGN KEY(player_id) REFERENCES player(id)
);
INSERT INTO deck_new SELECT * FROM deck;
DROP TABLE deck;
ALTER TABLE deck_new RENAME TO deck;

-- Add foreign key to participant
CREATE TABLE participant_new (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  game_id INTEGER NOT NULL,
  deck_id INTEGER NOT NULL,
  win BOOLEAN NOT NULL DEFAULT 0,
  CONSTRAINT participant_game_fk FOREIGN KEY(game_id) REFERENCES game(id),
  CONSTRAINT participant_deck_fk FOREIGN KEY(deck_id) REFERENCES deck(id)
);
INSERT INTO participant_new SELECT * FROM participant;
DROP TABLE participant;
ALTER TABLE participant_new RENAME TO participant;

-- Add foreign key to ranking
CREATE TABLE ranking_new (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  deck_id INTEGER NOT NULL,
  elo DOUBLE NOT NULL,
  CONSTRAINT ranking_deck_fk FOREIGN KEY(deck_id) REFERENCES deck(id)
);
INSERT INTO ranking_new SELECT * FROM ranking;
DROP TABLE ranking;
ALTER TABLE ranking_new RENAME TO ranking;
