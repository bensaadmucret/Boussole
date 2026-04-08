// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::OnceCell;
use sqlx::{sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions}, Pool, Sqlite};
use std::sync::Arc;
use std::str::FromStr;
use tauri::Manager;

mod commands;
mod models;
mod crypto;

use commands::{job_listings, applications, documents, ai, browser};
use commands::calendar;

// Global database pool
static DB_POOL: OnceCell<Arc<Pool<Sqlite>>> = OnceCell::new();

async fn init_database(app_handle: &tauri::AppHandle) -> Result<(), String> {
    // Use app_data_dir() - explicit and reliable in both dev and production
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = app_data_dir.join("boussole.db");
    println!("[DB] Using database path: {:?}", db_path);

    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data dir: {}", e))?;

    // Safe path conversion without unwrap
    let db_str = db_path.to_str()
        .ok_or_else(|| "Database path contains invalid (non-UTF8) characters".to_string())?;
    let db_url = format!("sqlite://{}", db_str);

    let options = SqliteConnectOptions::from_str(&db_url)
        .map_err(|e| e.to_string())?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| format!("Migration failed: {}", e))?;

    DB_POOL.set(Arc::new(pool))
        .map_err(|_| "Database already initialized".to_string())?;

    // Key stored in app_data_dir/.key (avoids macOS Keychain issues with unsigned apps)
    crypto::init_encryption(&app_data_dir)
        .map_err(|e| format!("Failed to initialize encryption (fatal): {}", e))?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            let (tx, rx) = std::sync::mpsc::channel::<Result<(), String>>();

            // Spawn a dedicated thread with its own Tokio runtime to init DB.
            // This avoids any conflict with Tauri's internal async runtime and
            // ensures DB is fully ready before any command can be invoked.
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new()
                    .expect("Failed to create Tokio runtime for DB init");
                let result = rt.block_on(init_database(&app_handle));
                let _ = tx.send(result);
            });

            // Block setup until DB is fully initialized
            rx.recv()
                .map_err(|e| e.to_string())?
                .map_err(|e| Box::<dyn std::error::Error>::from(e))?;

            println!("Database initialized successfully");
            Ok(())
        })
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
}

pub fn get_db_pool() -> Arc<Pool<Sqlite>> {
    DB_POOL.get()
        .expect("Database pool not initialized — this is a bug")
        .clone()
}
