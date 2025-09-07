-- Create users table
CREATE TABLE users (
    id TEXT PRIMARY KEY NOT NULL,
    login TEXT NOT NULL UNIQUE,
    created_at DATETIME NOT NULL,
    last_active_at DATETIME NOT NULL
);

-- Create index on login for faster lookups
CREATE INDEX idx_users_login ON users(login);

-- Create index on last_active_at for activity queries
CREATE INDEX idx_users_last_active_at ON users(last_active_at);
