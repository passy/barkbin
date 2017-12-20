CREATE TABLE barks (
  id INTEGER PRIMARY KEY,
  uuid BLOB NOT NULL,
  filename VARCHAR NOT NULL,
  body TEXT NOT NULL
);

CREATE UNIQUE INDEX barks_uuid ON barks(uuid);