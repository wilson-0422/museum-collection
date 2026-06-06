use rusqlite::{params, Connection};
use crate::models::artifact::Artifact;
use crate::models::exhibition::Exhibition;

pub fn list(conn: &Connection) -> Result<Vec<Exhibition>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, name, venue, start_date, end_date, curator, description, status, created_at FROM exhibitions ORDER BY id DESC"
    )?;
    let rows = stmt.query_map([], |row| Ok(Exhibition {
        id: row.get(0)?,
        name: row.get(1)?,
        venue: row.get(2)?,
        start_date: row.get(3)?,
        end_date: row.get(4)?,
        curator: row.get(5)?,
        description: row.get(6)?,
        status: row.get(7)?,
        created_at: row.get(8)?,
    }))?;
    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }
    Ok(result)
}

pub fn find_by_id(conn: &Connection, id: i64) -> Result<Exhibition, rusqlite::Error> {
    conn.query_row(
        "SELECT id, name, venue, start_date, end_date, curator, description, status, created_at FROM exhibitions WHERE id = ?1",
        params![id],
        |row| Ok(Exhibition {
            id: row.get(0)?,
            name: row.get(1)?,
            venue: row.get(2)?,
            start_date: row.get(3)?,
            end_date: row.get(4)?,
            curator: row.get(5)?,
            description: row.get(6)?,
            status: row.get(7)?,
            created_at: row.get(8)?,
        }),
    )
}

pub fn create(conn: &Connection, name: &str, venue: &str, start_date: &str, end_date: &str, curator: &str, description: &str, status: &str) -> Result<i64, rusqlite::Error> {
    conn.execute(
        "INSERT INTO exhibitions (name, venue, start_date, end_date, curator, description, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![name, venue, start_date, end_date, curator, description, status],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn add_artifact(conn: &Connection, exhibition_id: i64, artifact_id: i64) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT OR IGNORE INTO exhibition_artifacts (exhibition_id, artifact_id) VALUES (?1, ?2)",
        params![exhibition_id, artifact_id],
    )?;
    Ok(())
}

pub fn find_artifacts_by_exhibition(conn: &Connection, exhibition_id: i64) -> Result<Vec<Artifact>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT a.id, a.name, a.category, a.era, a.material, a.dimensions, a.origin, a.description, a.status, a.entry_date, a.created_at FROM artifacts a JOIN exhibition_artifacts ea ON a.id = ea.artifact_id WHERE ea.exhibition_id = ?1 ORDER BY a.id"
    )?;
    let rows = stmt.query_map(params![exhibition_id], |row| Ok(Artifact {
        id: row.get(0)?,
        name: row.get(1)?,
        category: row.get(2)?,
        era: row.get(3)?,
        material: row.get(4)?,
        dimensions: row.get(5)?,
        origin: row.get(6)?,
        description: row.get(7)?,
        status: row.get(8)?,
        entry_date: row.get(9)?,
        created_at: row.get(10)?,
    }))?;
    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }
    Ok(result)
}
