-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL, -- Changed from username
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'user', -- Changed from is_admin, added default
    created_at DATETIME NOT NULL, -- ISO8601 string
    updated_at DATETIME NOT NULL  -- ISO8601 string
);

-- Create receipts table
CREATE TABLE IF NOT EXISTS receipts (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    title TEXT NOT NULL, -- Added title
    amount REAL NOT NULL,
    date DATETIME NOT NULL, -- Store as YYYY-MM-DD or ISO8601 string
    description TEXT, -- Changed to allow NULL
    category TEXT,    -- Added category, allows NULL
    file_url TEXT,    -- Added file_url, allows NULL
    created_at DATETIME NOT NULL, -- ISO8601 string
    updated_at DATETIME NOT NULL, -- ISO8601 string
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_users_email ON users (email);
CREATE INDEX IF NOT EXISTS idx_receipts_user_id ON receipts (user_id);
CREATE INDEX IF NOT EXISTS idx_receipts_date ON receipts (date);

