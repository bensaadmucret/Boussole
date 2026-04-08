use crate::crypto;
use crate::models::calendar::{
    CalendarEventSource, GoogleCalendarAccount, GoogleCalendarSource, GoogleOAuthConfig,
    UnifiedCalendarEvent, UnifiedCalendarSettings,
};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::{Hash, Hasher};
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use url::Url;
use reqwest::{Client, ClientBuilder};

const GOOGLE_CLIENT_ID_KEY: &str = "google_oauth_config";
const ACCOUNTS_KEY: &str = "google_calendar_accounts";
const TOKENS_KEY: &str = "google_refresh_tokens";
const SETTINGS_KEY: &str = "unified_calendar_settings";
const EVENTS_KEY: &str = "unified_calendar_events";

/// Keys that should be encrypted before storage
const ENCRYPTED_KEYS: &[&str] = &[GOOGLE_CLIENT_ID_KEY, TOKENS_KEY];

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
            // Decrypt if this key should be encrypted
            let decrypted = if ENCRYPTED_KEYS.contains(&key) {
                crypto::decrypt(&value)?
            } else {
                value
            };
            serde_json::from_str(&decrypted).map_err(|e| e.to_string())
        }
        None => Ok(T::default()),
    }
}

#[tauri::command]
pub async fn get_google_oauth_config() -> Result<GoogleOAuthConfig, String> {
    load_google_oauth_config().await
}

#[tauri::command]
pub async fn save_google_oauth_config(config: GoogleOAuthConfig) -> Result<(), String> {
    store_google_oauth_config(&config).await
}

#[tauri::command]
pub async fn connect_google_calendar_account() -> Result<GoogleCalendarAccount, String> {
    connect_google_account_internal().await
}

#[tauri::command]
pub async fn delete_google_calendar_account(email: String) -> Result<(), String> {
    let mut tokens: std::collections::HashMap<String, String> =
        read_json_value(TOKENS_KEY).await.unwrap_or_default();
    tokens.remove(&email);
    let _ = write_json_value(TOKENS_KEY, &tokens).await;

    let mut accounts = get_google_calendar_accounts().await?;
    accounts.retain(|account| account.email != email);
    save_google_accounts(&accounts).await
}

#[tauri::command]
pub async fn sync_unified_calendar_events() -> Result<Vec<UnifiedCalendarEvent>, String> {
    let config = load_google_oauth_config().await?;
    let client_id = if config.client_id.trim().is_empty() {
        env::var("GOOGLE_CLIENT_ID").map_err(|_| "Ajoute ton Google Client ID dans Paramètres ou via GOOGLE_CLIENT_ID".to_string())?
    } else {
        config.client_id.clone()
    };
    let client_secret = config.client_secret.as_deref().filter(|s| !s.trim().is_empty()).map(|s| s.to_string());

    let accounts = get_google_calendar_accounts().await?;
    let settings = get_unified_calendar_settings().await?;

    if !settings.enabled {
        save_unified_calendar_events(Vec::new()).await?;
        return Ok(Vec::new());
    }

    let mut all_events = Vec::new();
    for account in accounts.iter() {
        match refresh_events_for_account(&client_id, client_secret.as_deref(), account, &settings).await {
            Ok(mut events) => all_events.append(&mut events),
            Err(e) => eprintln!("[Sync] Erreur pour {}: {}", account.email, e),
        }
    }

    all_events.sort_by(|a, b| a.start.cmp(&b.start));
    save_unified_calendar_events(all_events.clone()).await?;

    Ok(all_events)
}

async fn write_json_value<T>(key: &str, value: &T) -> Result<(), String>
where
    T: Serialize,
{
    let pool = crate::get_db_pool();
    let json = serde_json::to_string(value).map_err(|e| e.to_string())?;

    // Encrypt if this key should be encrypted
    let final_value = if ENCRYPTED_KEYS.contains(&key) {
        crypto::encrypt(&json)?
    } else {
        json
    };

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
    .bind(final_value)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct GoogleUserInfo {
    email: String,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    id_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleIdTokenClaims {
    email: Option<String>,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleCalendarListResponse {
    items: Vec<GoogleCalendarListItem>,
}

#[derive(Debug, Deserialize)]
struct GoogleCalendarListItem {
    id: String,
    summary: Option<String>,
    #[serde(rename = "backgroundColor")]
    background_color: Option<String>,
    selected: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct GoogleEventsResponse {
    #[serde(default)]
    items: Vec<GoogleEventItem>,
}

#[derive(Debug, Deserialize)]
struct GoogleEventItem {
    id: String,
    summary: Option<String>,
    location: Option<String>,
    start: GoogleEventDateTime,
    end: GoogleEventDateTime,
}

#[derive(Debug, Deserialize)]
struct GoogleEventDateTime {
    #[serde(rename = "dateTime")]
    date_time: Option<String>,
    date: Option<String>,
}

impl GoogleEventDateTime {
    fn as_iso_string(&self) -> Option<String> {
        self.date_time
            .clone()
            .or_else(|| self.date.clone().map(|value| format!("{value}T00:00:00")))
    }
}

fn google_color_from_email(email: &str) -> String {
    const PALETTE: [&str; 6] = ["#84cc16", "#65a30d", "#a3e635", "#d9f99d", "#4d7c0f", "#86efac"];

    let mut hasher = DefaultHasher::new();
    email.hash(&mut hasher);
    let index = (hasher.finish() as usize) % PALETTE.len();

    PALETTE[index].to_string()
}

fn generate_pkce_pair() -> (String, String) {
    let verifier: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(96)
        .map(char::from)
        .collect();

    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    (verifier, challenge)
}

fn build_google_auth_url(
    client_id: &str,
    redirect_url: &str,
    state: &str,
    code_challenge: &str,
) -> Result<Url, String> {
    let mut url = Url::parse("https://accounts.google.com/o/oauth2/v2/auth").map_err(|e| e.to_string())?;
    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", redirect_url)
        .append_pair("scope", "openid email profile https://www.googleapis.com/auth/calendar.readonly")
        .append_pair("code_challenge", code_challenge)
        .append_pair("code_challenge_method", "S256")
        .append_pair("access_type", "offline")
        .append_pair("prompt", "consent")
        .append_pair("include_granted_scopes", "true")
        .append_pair("state", state);
    Ok(url)
}

async fn exchange_code_for_tokens(
    http_client: &Client,
    client_id: &str,
    client_secret: Option<&str>,
    redirect_url: &str,
    code: &str,
    code_verifier: &str,
) -> Result<GoogleTokenResponse, String> {
    let mut params = vec![
        ("grant_type", "authorization_code"),
        ("client_id", client_id),
        ("code", code),
        ("code_verifier", code_verifier),
        ("redirect_uri", redirect_url),
    ];
    let secret_owned;
    if let Some(secret) = client_secret {
        secret_owned = secret.to_string();
        params.push(("client_secret", secret_owned.as_str()));
    }
    let response = http_client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_else(|_| "(impossible de lire le body)".to_string());
        return Err(format!("Échec OAuth Google: {} — {}", status, body));
    }

    response.json::<GoogleTokenResponse>().await.map_err(|e| e.to_string())
}

fn decode_google_id_token(id_token: &str) -> Result<GoogleIdTokenClaims, String> {
    let mut parts = id_token.split('.');
    let _header = parts.next().ok_or_else(|| "Google ID token invalide".to_string())?;
    let payload = parts.next().ok_or_else(|| "Google ID token invalide".to_string())?;

    let decoded = URL_SAFE_NO_PAD
        .decode(payload.as_bytes())
        .map_err(|e| e.to_string())?;

    serde_json::from_slice(&decoded).map_err(|e| e.to_string())
}

async fn refresh_access_token(
    http_client: &Client,
    client_id: &str,
    client_secret: Option<&str>,
    refresh_token: &str,
) -> Result<String, String> {
    let mut params = vec![
        ("grant_type", "refresh_token"),
        ("client_id", client_id),
        ("refresh_token", refresh_token),
    ];
    let secret_owned;
    if let Some(secret) = client_secret {
        secret_owned = secret.to_string();
        params.push(("client_secret", secret_owned.as_str()));
    }
    let response = http_client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Échec du rafraîchissement du token Google: {} — {}", status, body));
    }

    let payload = response.json::<GoogleTokenResponse>().await.map_err(|e| e.to_string())?;
    Ok(payload.access_token)
}

async fn save_refresh_token(email: &str, token: &str) -> Result<(), String> {
    let mut tokens: std::collections::HashMap<String, String> =
        read_json_value(TOKENS_KEY).await.unwrap_or_default();
    tokens.insert(email.to_string(), token.to_string());
    write_json_value(TOKENS_KEY, &tokens).await
}

async fn load_refresh_token(email: &str) -> Result<String, String> {
    let tokens: std::collections::HashMap<String, String> =
        read_json_value(TOKENS_KEY).await.unwrap_or_default();
    tokens.get(email).cloned().ok_or_else(|| format!("Refresh token introuvable pour {}", email))
}

async fn load_access_token(client_id: &str, client_secret: Option<&str>, email: &str) -> Result<String, String> {
    let refresh_token = load_refresh_token(email).await?;

    let http_client = ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| e.to_string())?;

    refresh_access_token(&http_client, client_id, client_secret, &refresh_token).await
}

async fn save_google_accounts(accounts: &[GoogleCalendarAccount]) -> Result<(), String> {
    write_json_value(ACCOUNTS_KEY, &accounts).await
}

async fn load_google_oauth_config() -> Result<GoogleOAuthConfig, String> {
    read_json_value(GOOGLE_CLIENT_ID_KEY).await
}

async fn store_google_oauth_config(config: &GoogleOAuthConfig) -> Result<(), String> {
    write_json_value(GOOGLE_CLIENT_ID_KEY, config).await
}

async fn fetch_user_info(http_client: &Client, access_token: &str) -> Result<GoogleUserInfo, String> {
    let response = http_client
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json::<GoogleUserInfo>().await.map_err(|e| e.to_string())
}

async fn fetch_calendar_sources(http_client: &Client, access_token: &str) -> Result<Vec<GoogleCalendarSource>, String> {
    let response = http_client
        .get("https://www.googleapis.com/calendar/v3/users/me/calendarList")
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let payload = response
        .json::<GoogleCalendarListResponse>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(payload
        .items
        .into_iter()
        .map(|item| GoogleCalendarSource {
            id: item.id,
            name: item.summary.unwrap_or_else(|| "Calendrier".to_string()),
            color: item.background_color.unwrap_or_else(|| "#84cc16".to_string()),
            active: item.selected.unwrap_or(true),
        })
        .collect())
}

async fn fetch_calendar_events_for_source(
    http_client: &Client,
    access_token: &str,
    account: &GoogleCalendarAccount,
    calendar: &GoogleCalendarSource,
    time_min: &str,
    time_max: &str,
) -> Result<Vec<UnifiedCalendarEvent>, String> {
    let url = Url::parse_with_params(
        &format!(
            "https://www.googleapis.com/calendar/v3/calendars/{}/events",
            url::form_urlencoded::byte_serialize(calendar.id.as_bytes()).collect::<String>()
        ),
        &[
            ("timeMin", time_min),
            ("timeMax", time_max),
            ("singleEvents", "true"),
            ("orderBy", "startTime"),
            ("maxResults", "100"),
        ],
    )
    .map_err(|e| e.to_string())?;

    let response = http_client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Erreur Calendar API [{}] {}: {} — {}", account.email, calendar.name, status, body));
    }

    let payload = response
        .json::<GoogleEventsResponse>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(payload
        .items
        .into_iter()
        .filter_map(|item| {
            let start = item.start.as_iso_string()?;
            let end = item.end.as_iso_string()?;

            Some(UnifiedCalendarEvent {
                id: format!("{}:{}:{}", account.email, calendar.id, item.id),
                title: item.summary.unwrap_or_else(|| "Événement Google Calendar".to_string()),
                start,
                end,
                location: item.location,
                source: CalendarEventSource {
                    account_email: account.email.clone(),
                    account_name: account.display_name.clone(),
                    calendar_id: calendar.id.clone(),
                    calendar_name: calendar.name.clone(),
                    color: calendar.color.clone(),
                },
            })
        })
        .collect())
}

async fn respond_http(mut socket: tokio::net::TcpStream, body: &str) -> Result<(), String> {
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );

    socket.write_all(response.as_bytes()).await.map_err(|e| e.to_string())?;
    socket.shutdown().await.map_err(|e| e.to_string())?;
    Ok(())
}

async fn wait_for_oauth_redirect(listener: &TcpListener) -> Result<String, String> {
    let (mut socket, _) = listener.accept().await.map_err(|e| e.to_string())?;
    let mut buffer = [0_u8; 4096];
    let bytes_read = socket.read(&mut buffer).await.map_err(|e| e.to_string())?;
    let request = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
    let first_line = request.lines().next().ok_or_else(|| "Requête OAuth invalide".to_string())?;
    let mut parts = first_line.split_whitespace();
    let _method = parts.next().ok_or_else(|| "Requête OAuth invalide".to_string())?;
    let target = parts.next().ok_or_else(|| "Requête OAuth invalide".to_string())?;
    let parsed = Url::parse(&format!("http://127.0.0.1{}", target)).map_err(|e| e.to_string())?;
    let query = parsed.query().unwrap_or_default().to_string();

    let success_page = r#"<html><body style="font-family:system-ui;padding:24px"><h1>Connexion réussie</h1><p>Vous pouvez fermer cette fenêtre et revenir à Boussole.</p></body></html>"#;
    let error_page = r#"<html><body style="font-family:system-ui;padding:24px"><h1>Connexion OAuth interrompue</h1><p>Vous pouvez fermer cette fenêtre et revenir à Boussole.</p></body></html>"#;

    let body = if query.contains("error=") { error_page } else { success_page };
    let _ = respond_http(socket, body).await;

    Ok(query)
}

async fn connect_google_account_internal() -> Result<GoogleCalendarAccount, String> {
    let stored_config = load_google_oauth_config().await?;
    let client_id = if stored_config.client_id.trim().is_empty() {
        env::var("GOOGLE_CLIENT_ID").map_err(|_| "Ajoute ton Google Client ID dans Paramètres ou via GOOGLE_CLIENT_ID".to_string())?
    } else {
        stored_config.client_id.clone()
    };
    let client_secret = stored_config.client_secret.as_deref().filter(|s| !s.trim().is_empty());

    let listener = TcpListener::bind("127.0.0.1:0").await.map_err(|e| e.to_string())?;
    let port = listener.local_addr().map_err(|e| e.to_string())?.port();
    let redirect_url = format!("http://127.0.0.1:{port}/oauth/google/callback");

    let (pkce_verifier, pkce_challenge) = generate_pkce_pair();
    let csrf_state: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let auth_url = build_google_auth_url(&client_id, &redirect_url, &csrf_state, &pkce_challenge)?;

    webbrowser::open(auth_url.as_str()).map_err(|e| e.to_string())?;

    let query = wait_for_oauth_redirect(&listener).await?;
    eprintln!("[OAuth Debug] Got redirect query");
    
    let parsed = Url::parse(&format!("http://127.0.0.1:{port}/?{query}" )).map_err(|e| e.to_string())?;
    let pairs: std::collections::HashMap<_, _> = parsed.query_pairs().into_owned().collect();

    if let Some(error) = pairs.get("error") {
        return Err(format!("OAuth Google interrompu: {error}"));
    }

    let returned_state = pairs.get("state").ok_or_else(|| "État OAuth manquant".to_string())?;
    if returned_state != &csrf_state {
        return Err("État OAuth invalide".to_string());
    }

    let code = pairs.get("code").ok_or_else(|| "Code OAuth manquant".to_string())?.to_string();
    eprintln!("[OAuth Debug] Got auth code");

    let http_client = ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| e.to_string())?;

    let token_response = match exchange_code_for_tokens(&http_client, &client_id, client_secret, &redirect_url, &code, &pkce_verifier).await {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("[OAuth Debug] Token exchange FAILED: {}", e);
            return Err(e);
        }
    };
    eprintln!("[OAuth Debug] Token exchange successful");

    let refresh_token = token_response
        .refresh_token
        .ok_or_else(|| "Google n'a pas renvoyé de refresh token. Réessaie la connexion.".to_string())?;
    eprintln!("[OAuth Debug] Got refresh token");

    let access_token = token_response.access_token;
    let id_token_claims = token_response
        .id_token
        .as_deref()
        .and_then(|id_token| decode_google_id_token(id_token).ok());

    let user_info = fetch_user_info(&http_client, &access_token).await.ok();
    let email = id_token_claims
        .as_ref()
        .and_then(|claims| claims.email.clone())
        .or_else(|| user_info.as_ref().map(|info| info.email.clone()))
        .ok_or_else(|| "Impossible de récupérer l'adresse email Google".to_string())?;
    eprintln!("[OAuth Debug] Got email: {}", email);

    let display_name = id_token_claims
        .as_ref()
        .and_then(|claims| claims.name.clone())
        .or_else(|| user_info.as_ref().and_then(|info| info.name.clone()))
        .unwrap_or_else(|| email.clone());

    let mut account = GoogleCalendarAccount {
        id: email.clone(),
        email: email.clone(),
        display_name,
        avatar_color: google_color_from_email(&email),
        connected_at: Utc::now().to_rfc3339(),
        calendars: Vec::new(),
    };

    let mut accounts = get_google_calendar_accounts().await?;
    accounts.retain(|existing| existing.email != account.email);
    accounts.push(account.clone());
    save_google_accounts(&accounts).await?;
    eprintln!("[OAuth Debug] Saved account to SQLite: {}", email);

    if let Err(error) = save_refresh_token(&email, &refresh_token).await {
        eprintln!("[OAuth Debug] Failed to store refresh token for {}: {}", email, error);
    } else {
        eprintln!("[OAuth Debug] Stored refresh token in SQLite");
    }

    if let Ok(calendars) = fetch_calendar_sources(&http_client, &access_token).await {
        eprintln!("[OAuth Debug] Fetched {} calendars", calendars.len());
        account.calendars = calendars;

        let mut accounts = get_google_calendar_accounts().await?;
        accounts.retain(|existing| existing.email != account.email);
        accounts.push(account.clone());
        save_google_accounts(&accounts).await?;
        eprintln!("[OAuth Debug] Updated account with calendars");
    } else {
        eprintln!("[OAuth Debug] Failed to fetch calendars, but account is saved");
    }

    eprintln!("[OAuth Debug] Returning account: {:?}", account.email);
    Ok(account)
}

async fn refresh_events_for_account(
    client_id: &str,
    client_secret: Option<&str>,
    account: &GoogleCalendarAccount,
    settings: &UnifiedCalendarSettings,
) -> Result<Vec<UnifiedCalendarEvent>, String> {
    let access_token = load_access_token(client_id, client_secret, &account.email).await?;
    let active_calendars: Vec<_> = account.calendars.iter().filter(|c| c.active).collect();
    eprintln!("[Sync] {} → {} calendriers actifs sur {}", account.email, active_calendars.len(), account.calendars.len());
    let http_client = ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| e.to_string())?;

    let now = Utc::now();
    let time_min = (now - Duration::days(settings.sync_window_days as i64)).to_rfc3339();
    let time_max = (now + Duration::days(settings.sync_window_days as i64)).to_rfc3339();

    let mut events = Vec::new();
    for calendar in account.calendars.iter().filter(|calendar| calendar.active) {
        let mut source_events = fetch_calendar_events_for_source(
            &http_client,
            &access_token,
            account,
            calendar,
            &time_min,
            &time_max,
        )
        .await?;
        events.append(&mut source_events);
    }

    Ok(events)
}

impl Default for GoogleCalendarAccount {
    fn default() -> Self {
        Self {
            id: String::new(),
            email: String::new(),
            display_name: String::new(),
            avatar_color: String::from("#84cc16"),
            connected_at: String::new(),
            calendars: Vec::new(),
        }
    }
}

impl Default for UnifiedCalendarEvent {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            start: String::new(),
            end: String::new(),
            location: None,
            source: crate::models::calendar::CalendarEventSource {
                account_email: String::new(),
                account_name: String::new(),
                calendar_id: String::new(),
                calendar_name: String::new(),
                color: String::from("#84cc16"),
            },
        }
    }
}

#[tauri::command]
pub async fn get_google_calendar_accounts() -> Result<Vec<GoogleCalendarAccount>, String> {
    read_json_value(ACCOUNTS_KEY).await
}

#[tauri::command]
pub async fn save_google_calendar_accounts(accounts: Vec<GoogleCalendarAccount>) -> Result<(), String> {
    write_json_value(ACCOUNTS_KEY, &accounts).await
}

#[tauri::command]
pub async fn get_unified_calendar_settings() -> Result<UnifiedCalendarSettings, String> {
    read_json_value(SETTINGS_KEY).await
}

#[tauri::command]
pub async fn save_unified_calendar_settings(settings: UnifiedCalendarSettings) -> Result<(), String> {
    write_json_value(SETTINGS_KEY, &settings).await
}

#[tauri::command]
pub async fn get_unified_calendar_events() -> Result<Vec<UnifiedCalendarEvent>, String> {
    read_json_value(EVENTS_KEY).await
}

#[tauri::command]
pub async fn save_unified_calendar_events(events: Vec<UnifiedCalendarEvent>) -> Result<(), String> {
    write_json_value(EVENTS_KEY, &events).await
}
