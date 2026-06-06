use rusqlite::{params, Connection};
use crate::models::restoration::Restoration;

pub fn list(conn: &Connection) -> Result<Vec<Restoration>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT r.id, r.artifact_id, a.name, r.restorer, r.method, r.start_date, r.end_date, r.cost, r.description, r.status, r.created_at FROM restorations r JOIN artifacts a ON r.artifact_id = a.id ORDER BY r.id DESC"
    )?;
    let rows = stmt.query_map([], |row| Ok(Restoration {
        id: row.get(0)?,
        artifact_id: row.get(1)?,
        artifact_name: row.get(2)?,
        restorer: row.get(3)?,
        method: row.get(4)?,
        start_date: row.get(5)?,
        end_date: row.get(6)?,
        cost: row.get(7)?,
        description: row.get(8)?,
        status: row.get(9)?,
        created_at: row.get(10)?,
    }))?;
    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }
    Ok(result)
}

pub fn find_by_id(conn: &Connection, id: i64) -> Result<Restoration, rusqlite::Error> {
    conn.query_row(
        "SELECT r.id, r.artifact_id, a.name, r.restorer, r.method, r.start_date, r.end_date, r.cost, r.description, r.status, r.created_at FROM restorations r JOIN artifacts a ON r.artifact_id = a.id WHERE r.id = ?1",
        params![id],
        |row| Ok(Restoration {
            id: row.get(0)?,
            artifact_id: row.get(1)?,
            artifact_name: row.get(2)?,
            restorer: row.get(3)?,
            method: row.get(4)?,
            start_date: row.get(5)?,
            end_date: row.get(6)?,
            cost: row.get(7)?,
            description: row.get(8)?,
            status: row.get(9)?,
            created_at: row.get(10)?,
        }),
    )
}

pub fn list_by_artifact(conn: &Connection, artifact_id: i64) -> Result<Vec<Restoration>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT r.id, r.artifact_id, a.name, r.restorer, r.method, r.start_date, r.end_date, r.cost, r.description, r.status, r.created_at FROM restorations r JOIN artifacts a ON r.artifact_id = a.id WHERE r.artifact_id = ?1 ORDER BY r.id DESC"
    )?;
    let rows = stmt.query_map(params![artifact_id], |row| Ok(Restoration {
        id: row.get(0)?,
        artifact_id: row.get(1)?,
        artifact_name: row.get(2)?,
        restorer: row.get(3)?,
        method: row.get(4)?,
        start_date: row.get(5)?,
        end_date: row.get(6)?,
        cost: row.get(7)?,
        description: row.get(8)?,
        status: row.get(9)?,
        created_at: row.get(10)?,
    }))?;
    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }
    Ok(result)
}

pub fn create(conn: &Connection, artifact_id: i64, restorer: &str, method: &str, start_date: &str, end_date: &str, cost: f64, description: &str, status: &str) -> Result<i64, rusqlite::Error> {
    conn.execute(
        "INSERT INTO restorations (artifact_id, restorer, method, start_date, end_date, cost, description, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![artifact_id, restorer, method, start_date, end_date, cost, description, status],
    )?;
    Ok(conn.last_insert_rowid())
}
