use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: i64,
    pub doc_type: String,
    pub profile_name: Option<String>,
    pub name: String,
    pub content: Option<Vec<u8>>,
    pub version: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDocumentInput {
    pub doc_type: String,
    pub profile_name: Option<String>,
    pub name: String,
    pub content: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct UpdateDocumentInput {
    pub name: Option<String>,
    pub content: Option<Vec<u8>>,
    pub profile_name: Option<String>,
}
