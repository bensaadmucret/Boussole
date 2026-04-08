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

    let prompt = format!(
        r#"Tu es un expert en recrutement et en rédaction de lettres de motivation.
Écris une lettre de motivation en français pour le poste de "{}" chez "{}".

Description du poste :
{}

CV du candidat :
{}

Instructions :
{}
- Met en avant les compétences et expériences du CV qui correspondent le mieux à l'offre.
- Ne mentionne pas de compétences que le candidat n'a pas dans son CV.
- La lettre doit comporter 3 ou 4 paragraphes maximum (environ 300 mots).
- Commence par "Madame, Monsieur," ou une formule d'appel appropriée si le contact est connu.
- Termine par une formule de politesse classique et laisse la signature à la fin.
- Ne mets pas de bloc d'en-tête (nom, adresse, date), génère uniquement le corps de la lettre.
"#,
        data.job_title,
        data.company_name,
        data.job_description,
        data.cv_content,
        tone_instruction
    );

    let client = reqwest::Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        config.api_key
    );

    let request_body = GeminiRequest {
        contents: vec![GeminiContent {
            parts: vec![GeminiPart { text: prompt }],
        }],
    };

    let response = client
        .post(&url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Erreur réseau lors de l'appel à Gemini : {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Erreur de l'API Gemini ({}): {}", status, error_text));
    }

    let gemini_response: GeminiResponse = response
        .json()
        .await
        .map_err(|e| format!("Erreur de parsing de la réponse : {}", e))?;

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
