use crate::models::document::{CreateDocumentInput, Document};

#[tauri::command]
pub async fn create_document(data: CreateDocumentInput) -> Result<Document, String> {
    let pool = crate::get_db_pool();
    
    let doc = sqlx::query_as::<_, Document>(
        r#"
        INSERT INTO documents (
            doc_type, profile_name, name, content, version, created_at, updated_at
        ) VALUES (?, ?, ?, ?, 1, datetime('now'), datetime('now'))
        RETURNING *
        "#
    )
    .bind(&data.doc_type)
    .bind(&data.profile_name)
    .bind(&data.name)
    .bind(&data.content)
    .fetch_one(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(doc)
}

#[tauri::command]
pub async fn get_documents(doc_type: Option<String>) -> Result<Vec<Document>, String> {
    let pool = crate::get_db_pool();
    
    let query = if let Some(dt) = doc_type {
        sqlx::query_as::<_, Document>(
            "SELECT * FROM documents WHERE doc_type = ? ORDER BY updated_at DESC"
        )
        .bind(dt)
    } else {
        sqlx::query_as::<_, Document>(
            "SELECT * FROM documents ORDER BY updated_at DESC"
        )
    };
    
    let docs = query
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(docs)
}

#[tauri::command]
pub async fn get_document_by_id(id: i64) -> Result<Document, String> {
    let pool = crate::get_db_pool();
    
    let doc = sqlx::query_as::<_, Document>(
        "SELECT * FROM documents WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(doc)
}

#[tauri::command]
pub async fn delete_document(id: i64) -> Result<(), String> {
    let pool = crate::get_db_pool();
    
    sqlx::query("DELETE FROM documents WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_documents_by_profile(profile_name: String) -> Result<Vec<Document>, String> {
    let pool = crate::get_db_pool();
    
    let docs = sqlx::query_as::<_, Document>(
        "SELECT * FROM documents WHERE profile_name = ? ORDER BY updated_at DESC"
    )
    .bind(&profile_name)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(docs)
}

#[tauri::command]
pub async fn get_cv_profiles() -> Result<Vec<String>, String> {
    let pool = crate::get_db_pool();
    
    let profiles: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT profile_name FROM documents WHERE doc_type = 'cv' AND profile_name IS NOT NULL ORDER BY profile_name"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(profiles)
}
