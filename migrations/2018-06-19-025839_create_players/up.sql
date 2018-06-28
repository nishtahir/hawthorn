CREATE TABLE player (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  alias TEXT NOT NULL UNIQUE
);

CREATE TABLE deck (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  alias TEXT NOT NULL UNIQUE,
  commander TEXT NOT NULL,
  player_id INTEGER NOT NULL
);

CREATE TABLE game (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  time_stamp DOUBLE NOT NULL
);

CREATE TABLE participant (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  game_id INTEGER NOT NULL,
  deck_id INTEGER NOT NULL,
  win BOOLEAN NOT NULL DEFAULT 0
);

CREATE TABLE ranking (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  deck_id INTEGER NOT NULL,
  elo DOUBLE NOT NULL
);