CREATE TABLE institution (
  id          TEXT     NOT NULL,
  name        TEXT     NOT NULL,
  description TEXT,
  created_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

  PRIMARY KEY (id)
);
