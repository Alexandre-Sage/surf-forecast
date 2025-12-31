-- Add migration script here

CREATE TABLE IF NOT EXISTS api_keys (
    id TEXT PRIMARY KEY NOT NULL,
    key TEXT NOT NULL UNIQUE,
    daily_usage INTEGER NOT NULL DEFAULT 0,
    date TEXT NOT NULL
);
