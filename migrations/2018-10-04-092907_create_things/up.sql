CREATE TABLE things (
  id         INTEGER   NOT NULL PRIMARY KEY AUTOINCREMENT,
  name       VARCHAR   NOT NULL,
  type       VARCHAR   NOT NULL,
  token      VARCHAR   NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX thing_token_idx      ON things (token);
CREATE INDEX thing_type_idx       ON things (type); 
CREATE INDEX thing_created_at_idx ON things (created_at);

PRAGMA foreign_keys=off;

ALTER TABLE iot_datas RENAME TO _iot_datas_old;

CREATE TABLE iot_datas (
  id         INTEGER   NOT NULL PRIMARY KEY AUTOINCREMENT,
  thing_id   INTEGER   NOT NULL,
  value      VARCHAR   NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (thing_id) REFERENCES things(thing_id)
);

-- Migrate exising data
INSERT INTO iot_datas (thing_id, value, created_at)
  SELECT 1, value, created_at
  FROM _iot_datas_old;

DROP TABLE _iot_datas_old;

PRAGMA foreign_keys=on;