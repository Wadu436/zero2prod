-- Add migration script here
ALTER TABLE subscriptions
    RENAME COLUMN subscrined_at TO subscribed_at;