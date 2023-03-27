CREATE TABLE subcategory (
  id          TEXT     NOT NULL,
  category_id TEXT     NOT NULL,
  name        TEXT     NOT NULL,
  description TEXT,
  created_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

  PRIMARY KEY (id),

  FOREIGN KEY (category_id)
    REFERENCES category (id)
      ON DELETE CASCADE
);
