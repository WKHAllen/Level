CREATE TABLE account_transaction_tag (
  account_transaction_id TEXT     NOT NULL,
  tag_id                 TEXT     NOT NULL,
  created_at             DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

  FOREIGN KEY (account_transaction_id)
    REFERENCES account_transaction (id)
      ON DELETE CASCADE,

  FOREIGN KEY (tag_id)
    REFERENCES tag (id)
      ON DELETE CASCADE
);
