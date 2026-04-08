use tauri::Manager;

#[tauri::command]
pub async fn save_report_pdf(
    app_handle: tauri::AppHandle,
    data: Vec<u8>,
    filename: String,
) -> Result<String, String> {
    let downloads_dir = app_handle.path().download_dir()
        .map_err(|e| format!("Impossible d'accéder au dossier Téléchargements : {}", e))?;

    let file_path = downloads_dir.join(&filename);

    std::fs::write(&file_path, &data)
        .map_err(|e| format!("Échec de l'écriture du PDF : {}", e))?;

    Ok(file_path.to_string_lossy().into_owned())
}
