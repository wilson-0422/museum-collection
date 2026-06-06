use rusqlite::{params, Connection};
use crate::models::conservation::Conservation;

pub fn list(conn: &Connection) -> Result<Vec<Conservation>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT c.id, c.artifact_id, a.name, c.method, c.performer, c.start_date, c.end_date, c.notes, c.created_at FROM conservations c JOIN artifacts a ON c.artifact_id = a.id ORDER BY c.id DESC"
    )?;
    let rows = stmt.query_map([], |row| Ok(Conservation {
        id: row.get(0)?,
        artifact_id: row.get(1)?,
        artifact_name: row.get(2)?,
        method: row.get(3)?,
        performer: row.get(4)?,
        start_date: row.get(5)?,
        end_date: row.get(6)?,
        notes: row.get(7)?,
        created_at: row.get(8)?,
    }))?;
    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }
    Ok(result)
}

pub fn find_by_id(conn: &Connection, id: i64) -> Result<Conservation, rusqlite::Error> {
    conn.query_row(
        "SELECT c.id, c.artifact_id, a.name, c.method, c.performer, c.start_date, c.end_date, c.notes, c.created_at FROM conservations c JOIN artifacts a ON c.artifact_id = a.id WHERE c.id = ?1",
        params![id],
        |row| Ok(Conservation {
            id: row.get(0)?,
            artifact_id: row.get(1)?,
            artifact_name: row.get(2)?,
            method: row.get(3)?,
            performer: row.get(4)?,
            start_date: row.get(5)?,
            end_date: row.get(6)?,
            notes: row.get(7)?,
            created_at: row.get(8)?,
        }),
    )
}

pub fn list_by_artifact(conn: &Connection, artifact_id: i64) -> Result<Vec<Conservation>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT c.id, c.artifact_id, a.name, c.method, c.performer, c.start_date, c.end_date, c.notes, c.created_at FROM conservations c JOIN artifacts a ON c.artifact_id = a.id WHERE c.artifact_id = ?1 ORDER BY c.id DESC"
    )?;
    let rows = stmt.query_map(params![artifact_id], |row| Ok(Conservation {
        id: row.get(0)?,
        artifact_id: row.get(1)?,
        artifact_name: row.get(2)?,
        method: row.get(3)?,
        performer: row.get(4)?,
        start_date: row.get(5)?,
        end_date: row.get(6)?,
        notes: row.get(7)?,
        created_at: row.get(8)?,
    }))?;
    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }
    Ok(result)
}

pub fn create(conn: &Connection, artifact_id: i64, method: &str, performer: &str, start_date: &str, end_date: &str, notes: &str) -> Result<i64, rusqlite::Error> {
    conn.execute(
        "INSERT INTO conservations (artifact_id, method, performer, start_date, end_date, notes) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![artifact_id, method, performer, start_date, end_date, notes],
    )?;
    Ok(conn.last_insert_rowid())
}
