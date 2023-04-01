CREATE TABLE budget (
  account_id       TEXT     NOT NULL,
  note             TEXT,
  total_limit      REAL     NOT NULL,
  timeframe        TEXT     NOT NULL,
  timeframe_offset DATETIME NOT NULL,
  created_at       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

  FOREIGN KEY (account_id)
    REFERENCES account (id)
      ON DELETE CASCADE
);
