use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conservation {
    pub id: i64,
    pub artifact_id: i64,
    pub artifact_name: String,
    pub method: String,
    pub performer: String,
    pub start_date: String,
    pub end_date: String,
    pub notes: String,
    pub created_at: String,
}
