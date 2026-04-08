use webbrowser;

#[tauri::command]
pub async fn open_external_url(url: String) -> Result<(), String> {
    webbrowser::open(&url).map_err(|e| e.to_string())
}
