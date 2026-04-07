use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct JobListing {
    pub id: i64,
    pub company_name: String,
    pub title: String,
    pub location: Option<String>,
    pub salary_min: Option<i32>,
    pub salary_max: Option<i32>,
    pub salary_currency: String,
    pub contract_type: String,
    pub remote_type: String,
    pub stack: String, // JSON array
    pub source_site: String,
    pub source_url: String,
    pub description: String,
    pub status: String,
    pub date_posted: Option<NaiveDate>,
    pub date_saved: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateJobListingInput {
    pub company_name: String,
    pub title: String,
    pub location: Option<String>,
    pub salary_min: Option<i32>,
    pub salary_max: Option<i32>,
    pub contract_type: String,
    pub remote_type: String,
    pub stack: Vec<String>,
    pub source_site: String,
    pub source_url: String,
    pub description: String,
    pub date_posted: Option<NaiveDate>,
}
