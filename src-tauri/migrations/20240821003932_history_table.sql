CREATE TABLE history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  file_id UUID NOT NULL,
  name TEXT NOT NULL,
  assistant_id INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

  FOREIGN KEY (assistant_id) REFERENCES assistant(id)
);
