CREATE TABLE conversations (
  id TEXT PRIMARY KEY NOT NULL,
  room_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  content VARCHAR NOT NULL,
  created_at TEXT NOT NULL
);

CREATE TABLE rooms (
  id TEXT PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL,
  last_message TEXT NOT NULL,
  participant_ids TEXT NOT NULL,
  created_at TEXT NOT NULL
);

CREATE TABLE users (
  id TEXT PRIMARY KEY NOT NULL,
  username VARCHAR NOT NULL,
  created_at TEXT NOT NULL,
  unique(username)
);