CREATE TABLE report_template (
  id          TEXT     NOT NULL,
  name        TEXT     NOT NULL,
  description TEXT,
  data        TEXT     NOT NULL,
  created_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

  PRIMARY KEY (id)
);
