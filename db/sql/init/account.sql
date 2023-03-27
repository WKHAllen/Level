CREATE TABLE account (
  id                 TEXT     NOT NULL,
  account_type_id    TEXT     NOT NULL,
  name               TEXT     NOT NULL,
  description        TEXT,
  created_at         DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  edited_at          DATETIME,
  last_reconciled_at DATETIME,

  PRIMARY KEY (id),

  FOREIGN KEY (account_type_id)
    REFERENCES account_type (id)
);
