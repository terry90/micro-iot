CREATE TABLE iot_datas (
  id                   INTEGER   NOT NULL PRIMARY KEY AUTOINCREMENT,
  thing_name           VARCHAR   NOT NULL,
  thing_type           VARCHAR   NOT NULL,
  value                VARCHAR   NOT NULL,
  created_at           TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX thing_name_idx ON iot_datas (thing_name);
