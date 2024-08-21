CREATE TABLE configs (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  temperature DECIMAL(3, 2) NOT NULL DEFAULT 0.8 CHECK (temperature >= 0 AND temperature <= 2),
  num_ctx INTEGER NOT NULL DEFAULT CHECK (num_ctx > 0),
  frequency_penalty DECIMAL(3, 2) NOT NULL DEFAULT 1.1 CHECK (frequency_penalty >= 0 AND frequency_penalty <= 2),
  presence_penalty DECIMAL(3, 2) NOT NULL DEFAULT 0.9 CHECK (presence_penalty >= 0 AND presence_penalty <= 2),
);
