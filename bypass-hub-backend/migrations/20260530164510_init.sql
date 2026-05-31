CREATE TABLE IF NOT EXISTS xui_routes
(
    id    INTEGER PRIMARY KEY AUTOINCREMENT,
    value TEXT NOT NULL UNIQUE,
    type  TEXT NOT NULL CHECK (type IN ('domain', 'ip'))
);

CREATE TABLE IF NOT EXISTS xui_groups
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE IF NOT EXISTS xui_routes_groups
(
    route_id INTEGER REFERENCES xui_routes (id) ON DELETE CASCADE,
    group_id INTEGER REFERENCES xui_groups (id) ON DELETE CASCADE,
    PRIMARY KEY (route_id, group_id)
);

CREATE TABLE IF NOT EXISTS mikrotik_routes
(
    id    INTEGER PRIMARY KEY AUTOINCREMENT,
    value TEXT NOT NULL UNIQUE,
    type  TEXT NOT NULL CHECK (type IN ('domain', 'ip'))
);

CREATE TABLE IF NOT EXISTS mikrotik_groups
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE IF NOT EXISTS mikrotik_routes_groups
(
    route_id INTEGER REFERENCES mikrotik_routes (id) ON DELETE CASCADE,
    group_id INTEGER REFERENCES mikrotik_groups (id) ON DELETE CASCADE,
    PRIMARY KEY (route_id, group_id)
);