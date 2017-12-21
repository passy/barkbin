CREATE TABLE barks (
  id INTEGER NOT NULL PRIMARY KEY,
  slug VARCHAR NOT NULL,
  filename VARCHAR NOT NULL,
  body TEXT NOT NULL,
  datetime TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX barks_slug ON barks(slug);
CREATE INDEX barks_datetime ON barks(datetime);
