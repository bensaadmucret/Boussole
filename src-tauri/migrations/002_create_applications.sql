-- Migration: Create applications table
CREATE TABLE IF NOT EXISTS applications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    job_listing_id INTEGER REFERENCES job_listings(id) ON DELETE SET NULL,
    company_name TEXT NOT NULL,
    position TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'applied',
    applied_date DATE NOT NULL,
    response_date DATE,
    notes TEXT,
    contact_email TEXT,
    contact_name TEXT,
    cv_version_id INTEGER,
    cover_letter_id INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Indexes
CREATE INDEX idx_applications_company ON applications(company_name);
CREATE INDEX idx_applications_status ON applications(status);
CREATE INDEX idx_applications_date ON applications(applied_date);
