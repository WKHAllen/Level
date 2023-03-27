CREATE TABLE reminder (
  id               TEXT     NOT NULL,
  account_id       TEXT     NOT NULL,
  note             TEXT,
  timeframe_id     TEXT     NOT NULL,
  timeframe_offset DATETIME NOT NULL,
  created_at       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

  PRIMARY KEY (id),

  FOREIGN KEY (account_id)
    REFERENCES account (id)
      ON DELETE CASCADE,

  FOREIGN KEY (timeframe_id)
    REFERENCES timeframe (id)
);
