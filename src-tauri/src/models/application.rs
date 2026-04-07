use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub id: i64,
    pub job_listing_id: Option<i64>,
    pub company_name: String,
    pub position: String,
    pub status: String,
    pub applied_date: NaiveDate,
    pub response_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub contact_email: Option<String>,
    pub contact_name: Option<String>,
    pub cv_version_id: Option<i64>,
    pub cover_letter_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApplicationInput {
    pub job_listing_id: Option<i64>,
    pub company_name: String,
    pub position: String,
    pub status: String,
    pub applied_date: NaiveDate,
    pub notes: Option<String>,
    pub contact_email: Option<String>,
    pub contact_name: Option<String>,
}
