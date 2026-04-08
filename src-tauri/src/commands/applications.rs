use crate::models::application::{CreateApplicationInput, Application};

const VALID_STATUSES: &[&str] = &["applied", "phone", "interview", "final", "offer", "rejected", "withdrawn"];

fn validate_status(status: &str) -> Result<(), String> {
    if VALID_STATUSES.contains(&status) {
        Ok(())
    } else {
        Err(format!("Statut invalide : '{}'. Valeurs autorisées : {:?}", status, VALID_STATUSES))
    }
}

#[tauri::command]
pub async fn create_application(data: CreateApplicationInput) -> Result<Application, String> {
    validate_status(&data.status)?;
    let pool = crate::get_db_pool();
    
    let application = sqlx::query_as::<_, Application>(
        r#"
        INSERT INTO applications (
            job_listing_id, company_name, position, status, 
            applied_date, notes, contact_email, contact_name
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING *
        "#
    )
    .bind(data.job_listing_id)
    .bind(&data.company_name)
    .bind(&data.position)
    .bind(&data.status)
    .bind(data.applied_date)
    .bind(&data.notes)
    .bind(&data.contact_email)
    .bind(&data.contact_name)
    .fetch_one(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(application)
}

#[tauri::command]
pub async fn get_applications() -> Result<Vec<Application>, String> {
    let pool = crate::get_db_pool();
    
    let applications = sqlx::query_as::<_, Application>(
        "SELECT * FROM applications ORDER BY applied_date DESC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(applications)
}

#[tauri::command]
pub async fn update_application_status(id: i64, status: String) -> Result<Application, String> {
    validate_status(&status)?;
    let pool = crate::get_db_pool();
    
    let application = sqlx::query_as::<_, Application>(
        r#"
        UPDATE applications 
        SET status = ?,
            response_date = CASE 
                WHEN ? IN ('rejected', 'offer', 'withdrawn') THEN datetime('now')
                ELSE response_date 
            END
        WHERE id = ?
        RETURNING *
        "#
    )
    .bind(&status)
    .bind(&status)
    .bind(id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(application)
}
