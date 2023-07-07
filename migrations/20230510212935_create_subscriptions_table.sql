-- Add migration script here
CREATE TABLE subscriptions(
    id uuid PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscrined_at timestamptz NOT NULL
);