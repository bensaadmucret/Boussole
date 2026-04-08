use crate::models::ai::{GeminiConfig, GenerateLetterInput, GeneratedLetter, GeminiRequest, GeminiContent, GeminiPart, GeminiResponse};
use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::crypto;

const GEMINI_CONFIG_KEY: &str = "gemini_api_config";

async fn read_json_value<T>(key: &str) -> Result<T, String>
where
    T: DeserializeOwned + Default,
{
    let pool = crate::get_db_pool();

    let raw_value: Option<String> = sqlx::query_scalar("SELECT value FROM app_settings WHERE key = ?")
        .bind(key)
        .fetch_optional(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    match raw_value {
        Some(value) => {
            let decrypted = crypto::decrypt(&value)?;
            serde_json::from_str(&decrypted).map_err(|e| e.to_string())
        }
        None => Ok(T::default()),
    }
}

async fn write_json_value<T>(key: &str, value: &T) -> Result<(), String>
where
    T: Serialize,
{
    let pool = crate::get_db_pool();
    let json = serde_json::to_string(value).map_err(|e| e.to_string())?;
    let encrypted_value = crypto::encrypt(&json)?;

    sqlx::query(
        r#"
        INSERT INTO app_settings (key, value, updated_at)
        VALUES (?, ?, datetime('now'))
        ON CONFLICT(key) DO UPDATE SET
            value = excluded.value,
            updated_at = datetime('now')
        "#,
    )
    .bind(key)
    .bind(encrypted_value)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_gemini_config() -> Result<GeminiConfig, String> {
    read_json_value(GEMINI_CONFIG_KEY).await
}

#[tauri::command]
pub async fn save_gemini_config(config: GeminiConfig) -> Result<(), String> {
    write_json_value(GEMINI_CONFIG_KEY, &config).await
}

#[tauri::command]
pub async fn generate_cover_letter(data: GenerateLetterInput) -> Result<GeneratedLetter, String> {
    let config: GeminiConfig = get_gemini_config().await?;
    
    if config.api_key.trim().is_empty() {
        return Err("La clé API Gemini n'est pas configurée. Veuillez la renseigner dans les paramètres de l'Assistant IA.".to_string());
    }

    let tone_instruction = match data.tone.as_deref() {
        Some("chaleureux") => "- Le ton doit être professionnel mais chaleureux et enthousiaste.",
        Some("creatif") => "- Le ton doit être audacieux, créatif et marquer les esprits.",
        Some("direct") => "- Le ton doit être direct, concis et droit au but.",
        _ => "- Le ton doit être formel et professionnel.",
    };

    // Build context sections based on available data
    let company_section = if data.company_name.trim().is_empty() {
        "Lettre de motivation spontanée.".to_string()
    } else {
        format!("Entreprise visée : {}", data.company_name)
    };
    
    let cv_section = if data.cv_content.trim().is_empty() {
        "Pas de CV fourni - rédige une lettre générique qui met en avant les qualités professionnelles générales.".to_string()
    } else {
        format!("CV du candidat :\n{}", data.cv_content)
    };

    let prompt = format!(
        r#"Tu es un expert en recrutement et en rédaction de lettres de motivation.
Écris une lettre de motivation en français pour le poste de "{}".

{}

Description du poste :
{}

{}

Instructions :
{}
- La lettre doit comporter 3 ou 4 paragraphes maximum (environ 300 mots).
- Commence par "Madame, Monsieur," ou une formule d'appel appropriée.
- Termine par une formule de politesse classique et laisse la signature à la fin.
- Ne mets pas de bloc d'en-tête (nom, adresse, date), génère uniquement le corps de la lettre.
"#,
        data.job_title,
        company_section,
        data.job_description,
        cv_section,
        tone_instruction
    );

    let client = reqwest::Client::new();
    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent";

    let request_body = GeminiRequest {
        contents: vec![GeminiContent {
            parts: vec![GeminiPart { text: prompt }],
        }],
    };

    let response = client
        .post(url)
        .header("x-goog-api-key", &config.api_key)  // API key in header, not URL
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Erreur réseau lors de l'appel à Gemini : {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Erreur de l'API Gemini ({}): {}", status, error_text));
    }

    // Get raw text first for better error handling
    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Erreur lors de la lecture de la réponse : {}", e))?;
    
    // Try to parse JSON
    let gemini_response: GeminiResponse = match serde_json::from_str(&response_text) {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("[Gemini Debug] Raw response (first 500 chars): {}", &response_text[..response_text.len().min(500)]);
            return Err(format!("Erreur de parsing JSON ({}): La réponse de l'API n'est pas valide. Détails: {}", 
                if response_text.len() > 1000 { "très longue" } else { &response_text },
                e));
        }
    };

    let content = gemini_response
        .candidates
        .first()
        .and_then(|c| c.content.parts.first())
        .map(|p| p.text.clone())
        .ok_or_else(|| "La réponse de Gemini était vide ou mal formatée.".to_string())?;

    Ok(GeneratedLetter {
        content,
        tokens_used: 0, // Gemini ne renvoie pas toujours le décompte facilement, on met 0 pour l'instant
    })
}
