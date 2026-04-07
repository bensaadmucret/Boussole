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

#[tauri::command]
pub async fn update_job_listing(id: i64, data: CreateJobListingInput) -> Result<JobListing, String> {
    let pool = crate::get_db_pool();
    
    let stack_json = serde_json::to_string(&data.stack).map_err(|e| e.to_string())?;
    
    let listing = sqlx::query_as::<_, JobListing>(
        r#"
        UPDATE job_listings SET
            company_name = ?,
            title = ?,
            location = ?,
            salary_min = ?,
            salary_max = ?,
            contract_type = ?,
            remote_type = ?,
            stack = ?,
            source_site = ?,
            source_url = ?,
            description = ?,
            date_posted = ?
        WHERE id = ?
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
    .bind(id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(listing)
}

#[tauri::command]
pub async fn delete_job_listing(id: i64) -> Result<(), String> {
    let pool = crate::get_db_pool();
    
    sqlx::query("DELETE FROM job_listings WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn check_duplicate_company(company_name: String) -> Result<bool, String> {
    let pool = crate::get_db_pool();
    
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM job_listings WHERE LOWER(company_name) = LOWER(?)")
        .bind(&company_name)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(count > 0)
}

#[tauri::command]
pub async fn get_existing_company_listings(company_name: String) -> Result<Vec<JobListing>, String> {
    let pool = crate::get_db_pool();
    
    let listings = sqlx::query_as::<_, JobListing>(
        "SELECT * FROM job_listings WHERE LOWER(company_name) = LOWER(?) ORDER BY date_saved DESC")
        .bind(&company_name)
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(listings)
}
