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

CREATE TABLE IF NOT EXISTS servers
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL UNIQUE,
    address     TEXT NOT NULL,
    xui_api_key TEXT NOT NULL,
    uuid        TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS server_inbounds
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    server_id  INTEGER NOT NULL REFERENCES servers (id) ON DELETE CASCADE,
    inbound_id INTEGER NOT NULL,
    UNIQUE (server_id, inbound_id)
);

CREATE TABLE IF NOT EXISTS mikrotiks
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT    NOT NULL,
    server_id   INTEGER NOT NULL REFERENCES servers (id) ON DELETE RESTRICT,
    inbound_id  INTEGER NOT NULL REFERENCES server_inbounds (id) ON DELETE RESTRICT,
    public_key  TEXT,
    assigned_ip TEXT,
    uuid        TEXT    NOT NULL UNIQUE,
    status      TEXT    NOT NULL DEFAULT 'pending_key' CHECK (status IN ('pending_key', 'active')),
    created_at  INTEGER NOT NULL DEFAULT (unixepoch())
);
