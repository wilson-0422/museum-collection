use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Restoration {
    pub id: i64,
    pub artifact_id: i64,
    pub artifact_name: String,
    pub restorer: String,
    pub method: String,
    pub start_date: String,
    pub end_date: String,
    pub cost: f64,
    pub description: String,
    pub status: String,
    pub created_at: String,
}
