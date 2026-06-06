use rusqlite::{params, Connection};
use crate::models::reservation::Reservation;

pub fn list(conn: &Connection) -> Result<Vec<Reservation>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT r.id, r.visitor_name, r.phone, r.visit_date, r.visitor_count, r.exhibition_id, COALESCE(e.name, ''), r.status, r.created_at FROM reservations r LEFT JOIN exhibitions e ON r.exhibition_id = e.id ORDER BY r.id DESC"
    )?;
    let rows = stmt.query_map([], |row| Ok(Reservation {
        id: row.get(0)?,
        visitor_name: row.get(1)?,
        phone: row.get(2)?,
        visit_date: row.get(3)?,
        visitor_count: row.get(4)?,
        exhibition_id: row.get(5)?,
        exhibition_name: row.get(6)?,
        status: row.get(7)?,
        created_at: row.get(8)?,
    }))?;
    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }
    Ok(result)
}

pub fn create(conn: &Connection, visitor_name: &str, phone: &str, visit_date: &str, visitor_count: i64, exhibition_id: Option<i64>, status: &str) -> Result<i64, rusqlite::Error> {
    conn.execute(
        "INSERT INTO reservations (visitor_name, phone, visit_date, visitor_count, exhibition_id, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![visitor_name, phone, visit_date, visitor_count, exhibition_id, status],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update_status(conn: &Connection, id: i64, status: &str) -> Result<(), rusqlite::Error> {
    conn.execute(
        "UPDATE reservations SET status=?1 WHERE id=?2",
        params![status, id],
    )?;
    Ok(())
}
