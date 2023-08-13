-- Add up migration script here
CREATE TABLE IF NOT EXISTS todos (
    id serial PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

