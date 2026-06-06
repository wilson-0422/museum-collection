use axum_extra::extract::cookie::CookieJar;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize)]
pub struct SessionInfo {
    pub user_id: i64,
    pub username: String,
    pub display_name: String,
    pub role: String,
}

pub fn get_current_user(
    sessions: &Mutex<HashMap<String, SessionInfo>>,
    jar: &CookieJar,
) -> Option<SessionInfo> {
    let cookie = jar.get("session_id")?;
    let session_id = cookie.value();
    let sessions = sessions.lock().ok()?;
    sessions.get(session_id).cloned()
}
