use rusqlite::{params, Connection};
use crate::models::artifact::Artifact;

pub fn list(conn: &Connection) -> Result<Vec<Artifact>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, name, category, era, material, dimensions, origin, description, status, entry_date, created_at FROM artifacts ORDER BY id DESC"
    )?;
    let rows = stmt.query_map([], |row| Ok(Artifact {
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

pub fn find_by_id(conn: &Connection, id: i64) -> Result<Artifact, rusqlite::Error> {
    conn.query_row(
        "SELECT id, name, category, era, material, dimensions, origin, description, status, entry_date, created_at FROM artifacts WHERE id = ?1",
        params![id],
        |row| Ok(Artifact {
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
        }),
    )
}

pub fn create(conn: &Connection, a: &Artifact) -> Result<i64, rusqlite::Error> {
    conn.execute(
        "INSERT INTO artifacts (name, category, era, material, dimensions, origin, description, status, entry_date) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![a.name, a.category, a.era, a.material, a.dimensions, a.origin, a.description, a.status, a.entry_date],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update(conn: &Connection, a: &Artifact) -> Result<(), rusqlite::Error> {
    conn.execute(
        "UPDATE artifacts SET name=?1, category=?2, era=?3, material=?4, dimensions=?5, origin=?6, description=?7, status=?8, entry_date=?9 WHERE id=?10",
        params![a.name, a.category, a.era, a.material, a.dimensions, a.origin, a.description, a.status, a.entry_date, a.id],
    )?;
    Ok(())
}

pub fn delete(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
    conn.execute("DELETE FROM artifacts WHERE id=?1", params![id])?;
    Ok(())
}
