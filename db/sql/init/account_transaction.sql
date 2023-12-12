CREATE TABLE account_transaction (
  id               TEXT     NOT NULL,
  account_id       TEXT     NOT NULL,
  name             TEXT     NOT NULL,
  description      TEXT,
  amount           REAL     NOT NULL,
  transaction_type TEXT     NOT NULL,
  institution_id   TEXT     NOT NULL,
  transaction_date DATETIME NOT NULL,
  category_id      TEXT     NOT NULL,
  subcategory_id   TEXT,
  reconciled       BOOLEAN  NOT NULL DEFAULT FALSE,
  created_at       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  edited_at        DATETIME,
  reconciled_at    DATETIME,

  PRIMARY KEY (id),

  FOREIGN KEY (account_id)
    REFERENCES account (id)
      ON DELETE CASCADE,

  FOREIGN KEY (institution_id)
    REFERENCES institution (id),

  FOREIGN KEY (category_id)
    REFERENCES category (id),

  FOREIGN KEY (subcategory_id)
    REFERENCES subcategory (id)
);
