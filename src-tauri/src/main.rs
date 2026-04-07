// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::OnceCell;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::sync::Arc;
use tauri::Manager;

mod commands;
mod models;

use commands::{job_listings, applications};

// Global database pool
static DB_POOL: OnceCell<Arc<Pool<Sqlite>>> = OnceCell::new();

pub struct DbState(Arc<Pool<Sqlite>>);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:boussole.db")
        .await?;
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    
    let pool = Arc::new(pool);
    DB_POOL.set(pool.clone()).unwrap();

    tauri::Builder::default()
        .manage(DbState(pool))
        .invoke_handler(tauri::generate_handler![
            job_listings::create_job_listing,
            job_listings::get_job_listings,
            applications::create_application,
            applications::get_applications,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

pub fn get_db_pool() -> Arc<Pool<Sqlite>> {
    DB_POOL.get().unwrap().clone()
}
