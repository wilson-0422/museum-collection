use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exhibition {
    pub id: i64,
    pub name: String,
    pub venue: String,
    pub start_date: String,
    pub end_date: String,
    pub curator: String,
    pub description: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExhibitionArtifact {
    pub exhibition_id: i64,
    pub artifact_id: i64,
}
