-- Add up migration script here
CREATE TABLE posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    url TEXT NOT NULL,
    caption TEXT,
    lat REAL CHECK ( lat IS NULL OR (lat >= -90 AND lat <=90) ),
    lng REAL CHECK ( lng IS NULL OR (lng >= -180 AND lng <= 180) ),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE
);