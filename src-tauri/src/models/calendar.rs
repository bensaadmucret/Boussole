use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCalendarSource {
    pub id: String,
    pub name: String,
    pub color: String,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCalendarAccount {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub avatar_color: String,
    pub connected_at: String,
    pub calendars: Vec<GoogleCalendarSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnifiedCalendarSettings {
    pub enabled: bool,
    pub show_source_labels: bool,
    pub color_events_by_account: bool,
    pub sync_window_days: i32,
    pub refresh_interval_minutes: i32,
}

impl Default for UnifiedCalendarSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            show_source_labels: true,
            color_events_by_account: true,
            sync_window_days: 30,
            refresh_interval_minutes: 15,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleOAuthConfig {
    pub client_id: String,
    #[serde(default)]
    pub client_secret: Option<String>,
}

impl Default for GoogleOAuthConfig {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            client_secret: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEventSource {
    pub account_email: String,
    pub account_name: String,
    pub calendar_id: String,
    pub calendar_name: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnifiedCalendarEvent {
    pub id: String,
    pub title: String,
    pub start: String,
    pub end: String,
    pub location: Option<String>,
    pub source: CalendarEventSource,
}
