-- Migration: Create documents table for multi-profile CVs
CREATE TABLE IF NOT EXISTS documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL, -- 'cv', 'cover_letter', 'other'
    profile_name TEXT, -- e.g., 'Lead Dev', 'Chef de projet IT', 'DevOps'
    name TEXT NOT NULL,
    content BLOB,
    version INTEGER DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Indexes
CREATE INDEX idx_documents_type ON documents(type);
CREATE INDEX idx_documents_profile ON documents(profile_name);
