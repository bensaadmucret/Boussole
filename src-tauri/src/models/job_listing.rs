use chrono::NaiveDate;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::FromRow;

fn serialize_stack_as_array<S>(stack: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let parsed: Vec<String> = serde_json::from_str(stack).unwrap_or_default();
    parsed.serialize(serializer)
}

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
    #[serde(serialize_with = "serialize_stack_as_array")]
    pub stack: String, // JSON array stored as string, serialized as array
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
