-- Remove foreign key constraints from tables.
-- SQLite does not support ALTER TABLE REMOVE CONSTRAINT,
-- so the tables must redone.

-- Remove foreign key from deck
CREATE TABLE deck_new (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  alias TEXT NOT NULL UNIQUE,
  commander TEXT NOT NULL,
  player_id INTEGER NOT NULL
);
INSERT INTO deck_new SELECT * FROM deck;
DROP TABLE deck;
ALTER TABLE deck_new RENAME TO deck;

-- Remove foreign key from participant
CREATE TABLE participant_new (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  game_id INTEGER NOT NULL,
  deck_id INTEGER NOT NULL,
  win BOOLEAN NOT NULL DEFAULT 0
);
INSERT INTO participant_new SELECT * FROM participant;
DROP TABLE participant;
ALTER TABLE participant_new RENAME TO participant;

-- Remove foreign key from ranking
CREATE TABLE ranking_new (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  deck_id INTEGER NOT NULL,
  elo DOUBLE NOT NULL
);
INSERT INTO ranking_new SELECT * FROM ranking;
DROP TABLE ranking;
ALTER TABLE ranking_new RENAME TO ranking;
