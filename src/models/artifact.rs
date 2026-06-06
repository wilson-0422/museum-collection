use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub era: String,
    pub material: String,
    pub dimensions: String,
    pub origin: String,
    pub description: String,
    pub status: String,
    pub entry_date: String,
    pub created_at: String,
}
