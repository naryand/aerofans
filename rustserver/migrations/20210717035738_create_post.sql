-- create_posts.sql
CREATE TABLE IF NOT EXISTS posts (
    id BIGSERIAL PRIMARY KEY,
    text TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'utc')
);

-- seed db with some test data for local dev
INSERT INTO posts
(text)
VALUES
('Test post 1'),
('Test post 2'),
('Test post 3');
