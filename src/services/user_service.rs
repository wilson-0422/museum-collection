use rusqlite::{params, Connection};
use crate::models::user::User;

pub fn find_by_username(conn: &Connection, username: &str) -> Result<User, rusqlite::Error> {
    conn.query_row(
        "SELECT id, username, password_hash, display_name, role, created_at FROM users WHERE username = ?1",
        params![username],
        |row| Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password_hash: row.get(2)?,
            display_name: row.get(3)?,
            role: row.get(4)?,
            created_at: row.get(5)?,
        }),
    )
}

pub fn find_by_id(conn: &Connection, id: i64) -> Result<User, rusqlite::Error> {
    conn.query_row(
        "SELECT id, username, password_hash, display_name, role, created_at FROM users WHERE id = ?1",
        params![id],
        |row| Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password_hash: row.get(2)?,
            display_name: row.get(3)?,
            role: row.get(4)?,
            created_at: row.get(5)?,
        }),
    )
}

pub fn create(conn: &Connection, username: &str, password: &str, display_name: &str) -> Result<i64, String> {
    let hash = bcrypt::hash(password, 4).map_err(|e| format!("密码加密失败: {}", e))?;
    conn.execute(
        "INSERT INTO users (username, password_hash, display_name) VALUES (?1, ?2, ?3)",
        params![username, hash, display_name],
    ).map_err(|e| format!("创建用户失败: {}", e))?;
    Ok(conn.last_insert_rowid())
}

pub fn verify_user(conn: &Connection, username: &str, password: &str) -> Result<User, String> {
    let user = find_by_username(conn, username).map_err(|_| "用户不存在".to_string())?;
    let valid = bcrypt::verify(password, &user.password_hash).map_err(|_| "密码验证失败".to_string())?;
    if valid {
        Ok(user)
    } else {
        Err("密码错误".to_string())
    }
}
