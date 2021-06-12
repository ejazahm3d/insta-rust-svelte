-- Add up migration script here
CREATE TABLE likes
(
    id         UUID PRIMARY KEY         DEFAULT gen_random_uuid(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    user_id    UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    post_id    UUID REFERENCES posts (id) ON DELETE CASCADE,
    comment_id UUID REFERENCES comments (id) ON DELETE CASCADE,
    CHECK (
                COALESCE((CASE WHEN post_id IS NULL THEN 0 ELSE 1 END), 0)
                +
                COALESCE((CASE WHEN comment_id IS NULL THEN 0 ELSE 1 END), 0)
            = 1
        ),
    UNIQUE (user_id, post_id, comment_id)
);