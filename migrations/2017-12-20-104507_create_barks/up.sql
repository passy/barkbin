CREATE TABLE barks (
  id INTEGER NOT NULL PRIMARY KEY,
  uuid BLOB NOT NULL,
  filename VARCHAR NOT NULL,
  body TEXT NOT NULL,
  datetime TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX barks_uuid ON barks(uuid);
CREATE INDEX barks_datetime ON barks(datetime);
