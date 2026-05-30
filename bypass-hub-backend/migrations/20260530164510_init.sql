CREATE TABLE IF NOT EXISTS routes
(
    id    INTEGER PRIMARY KEY AUTOINCREMENT,
    value TEXT NOT NULL UNIQUE, -- домен или ip
    type  TEXT NOT NULL CHECK (type IN ('domain', 'ip'))
);

CREATE TABLE IF NOT EXISTS "group"
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE IF NOT EXISTS routes_groups
(
    route_id INTEGER REFERENCES routes (id) ON DELETE CASCADE,
    group_id INTEGER REFERENCES "group" (id) ON DELETE CASCADE,
    PRIMARY KEY (route_id, group_id)
);

