// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::OnceCell;
use sqlx::{sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions}, Pool, Sqlite};
use std::sync::Arc;
use std::str::FromStr;

mod commands;
mod models;
mod crypto;

use commands::{job_listings, applications, documents, ai, browser};
use commands::calendar;

// Global database pool
static DB_POOL: OnceCell<Arc<Pool<Sqlite>>> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database
    let options = SqliteConnectOptions::from_str("sqlite://boussole.db")?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    
    let pool = Arc::new(pool);
    DB_POOL.set(pool.clone()).unwrap();

    // Initialize encryption
    if let Err(e) = crypto::init_encryption() {
        eprintln!("Warning: Failed to initialize encryption: {}", e);
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            job_listings::create_job_listing,
            job_listings::get_job_listings,
            job_listings::update_job_listing,
            job_listings::delete_job_listing,
            job_listings::check_duplicate_company,
            job_listings::get_existing_company_listings,
            applications::create_application,
            applications::get_applications,
            applications::update_application_status,
            documents::create_document,
            documents::get_documents,
            documents::get_document_by_id,
            documents::delete_document,
            documents::get_documents_by_profile,
            documents::get_cv_profiles,
            calendar::get_google_oauth_config,
            calendar::save_google_oauth_config,
            calendar::connect_google_calendar_account,
            calendar::delete_google_calendar_account,
            calendar::get_google_calendar_accounts,
            calendar::save_google_calendar_accounts,
            calendar::get_unified_calendar_settings,
            calendar::save_unified_calendar_settings,
            calendar::get_unified_calendar_events,
            calendar::save_unified_calendar_events,
            calendar::sync_unified_calendar_events,
            ai::generate_cover_letter,
            ai::get_gemini_config,
            ai::save_gemini_config,
            browser::open_external_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

pub fn get_db_pool() -> Arc<Pool<Sqlite>> {
    DB_POOL.get().unwrap().clone()
}
