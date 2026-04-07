-- Migration: Create job_listings table
CREATE TABLE IF NOT EXISTS job_listings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    company_name TEXT NOT NULL,
    title TEXT NOT NULL,
    location TEXT,
    salary_min INTEGER,
    salary_max INTEGER,
    salary_currency TEXT DEFAULT 'EUR',
    contract_type TEXT NOT NULL,
    remote_type TEXT NOT NULL,
    stack TEXT NOT NULL DEFAULT '[]', -- JSON array
    source_site TEXT NOT NULL,
    source_url TEXT NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'saved',
    date_posted DATE,
    date_saved DATETIME DEFAULT CURRENT_TIMESTAMP,
    search_vector TEXT
);

-- Index for common queries
CREATE INDEX idx_job_listings_company ON job_listings(company_name);
CREATE INDEX idx_job_listings_date ON job_listings(date_saved);
CREATE INDEX idx_job_listings_status ON job_listings(status);
