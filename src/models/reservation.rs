use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
    pub id: i64,
    pub visitor_name: String,
    pub phone: String,
    pub visit_date: String,
    pub visitor_count: i64,
    pub exhibition_id: Option<i64>,
    pub exhibition_name: String,
    pub status: String,
    pub created_at: String,
}
