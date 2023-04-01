CREATE TABLE account (
  id            TEXT     NOT NULL,
  account_type  TEXT     NOT NULL,
  name          TEXT     NOT NULL,
  description   TEXT,
  created_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  edited_at     DATETIME,
  reconciled_at DATETIME,

  PRIMARY KEY (id)
);
