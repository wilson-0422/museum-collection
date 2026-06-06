use rusqlite::Connection;

pub fn init_db() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("museum.db")?;
    conn.execute_batch("
        PRAGMA journal_mode=WAL;
        PRAGMA foreign_keys=ON;

        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            display_name TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'staff',
            created_at TEXT NOT NULL DEFAULT (datetime('now','localtime'))
        );

        CREATE TABLE IF NOT EXISTS artifacts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            era TEXT NOT NULL,
            material TEXT NOT NULL,
            dimensions TEXT NOT NULL DEFAULT '',
            origin TEXT NOT NULL DEFAULT '',
            description TEXT NOT NULL DEFAULT '',
            status TEXT NOT NULL DEFAULT '在库',
            entry_date TEXT NOT NULL DEFAULT (date('now','localtime')),
            created_at TEXT NOT NULL DEFAULT (datetime('now','localtime'))
        );

        CREATE TABLE IF NOT EXISTS conservations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            artifact_id INTEGER NOT NULL,
            method TEXT NOT NULL,
            performer TEXT NOT NULL,
            start_date TEXT NOT NULL,
            end_date TEXT NOT NULL DEFAULT '',
            notes TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL DEFAULT (datetime('now','localtime')),
            FOREIGN KEY (artifact_id) REFERENCES artifacts(id)
        );

        CREATE TABLE IF NOT EXISTS exhibitions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            venue TEXT NOT NULL,
            start_date TEXT NOT NULL,
            end_date TEXT NOT NULL DEFAULT '',
            curator TEXT NOT NULL DEFAULT '',
            description TEXT NOT NULL DEFAULT '',
            status TEXT NOT NULL DEFAULT '筹备中',
            created_at TEXT NOT NULL DEFAULT (datetime('now','localtime'))
        );

        CREATE TABLE IF NOT EXISTS exhibition_artifacts (
            exhibition_id INTEGER NOT NULL,
            artifact_id INTEGER NOT NULL,
            PRIMARY KEY (exhibition_id, artifact_id),
            FOREIGN KEY (exhibition_id) REFERENCES exhibitions(id),
            FOREIGN KEY (artifact_id) REFERENCES artifacts(id)
        );

        CREATE TABLE IF NOT EXISTS restorations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            artifact_id INTEGER NOT NULL,
            restorer TEXT NOT NULL,
            method TEXT NOT NULL,
            start_date TEXT NOT NULL,
            end_date TEXT NOT NULL DEFAULT '',
            cost REAL NOT NULL DEFAULT 0.0,
            description TEXT NOT NULL DEFAULT '',
            status TEXT NOT NULL DEFAULT '进行中',
            created_at TEXT NOT NULL DEFAULT (datetime('now','localtime')),
            FOREIGN KEY (artifact_id) REFERENCES artifacts(id)
        );

        CREATE TABLE IF NOT EXISTS reservations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            visitor_name TEXT NOT NULL,
            phone TEXT NOT NULL,
            visit_date TEXT NOT NULL,
            visitor_count INTEGER NOT NULL DEFAULT 1,
            exhibition_id INTEGER,
            status TEXT NOT NULL DEFAULT '待确认',
            created_at TEXT NOT NULL DEFAULT (datetime('now','localtime')),
            FOREIGN KEY (exhibition_id) REFERENCES exhibitions(id)
        );
    ")?;
    Ok(conn)
}
