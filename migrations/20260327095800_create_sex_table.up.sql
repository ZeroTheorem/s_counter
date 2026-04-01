-- Add up migration script here
CREATE TABLE IF NOT EXISTS sex (
    id BIGSERIAL PRIMARY KEY,
    date VARCHAR NOT NULL,
    time VARCHAR NOT NULL,
    duration INTEGER,
    notes VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX IF NOT EXISTS created_at_idx ON sex(created_at);
