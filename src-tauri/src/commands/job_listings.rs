use crate::models::job_listing::{CreateJobListingInput, JobListing};
use serde_json;

#[tauri::command]
pub async fn create_job_listing(data: CreateJobListingInput) -> Result<JobListing, String> {
    let pool = crate::get_db_pool();
    
    let stack_json = serde_json::to_string(&data.stack).map_err(|e| e.to_string())?;
    
    let listing = sqlx::query_as::<_, JobListing>(
        r#"
        INSERT INTO job_listings (
            company_name, title, location, salary_min, salary_max, 
            contract_type, remote_type, stack, source_site, source_url, 
            description, status, date_posted, date_saved
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'saved', ?, datetime('now'))
        RETURNING *
        "#
    )
    .bind(&data.company_name)
    .bind(&data.title)
    .bind(&data.location)
    .bind(data.salary_min)
    .bind(data.salary_max)
    .bind(&data.contract_type)
    .bind(&data.remote_type)
    .bind(&stack_json)
    .bind(&data.source_site)
    .bind(&data.source_url)
    .bind(&data.description)
    .bind(data.date_posted)
    .fetch_one(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(listing)
}

#[tauri::command]
pub async fn get_job_listings() -> Result<Vec<JobListing>, String> {
    let pool = crate::get_db_pool();
    
    let listings = sqlx::query_as::<_, JobListing>(
        "SELECT * FROM job_listings ORDER BY date_saved DESC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(listings)
}
