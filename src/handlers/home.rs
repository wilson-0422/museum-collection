use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
};
use axum_extra::extract::cookie::CookieJar;
use std::sync::Arc;
use crate::AppState;
use crate::middleware::auth::get_current_user;
use crate::services::{artifact_service, exhibition_service, conservation_service, restoration_service, reservation_service};

pub async fn index(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    let db = state.db.lock().unwrap();
    let artifacts = artifact_service::list(&db).unwrap_or_default();
    let exhibitions = exhibition_service::list(&db).unwrap_or_default();
    let recent_artifacts: Vec<_> = artifacts.into_iter().take(6).collect();
    let active_exhibitions: Vec<_> = exhibitions.into_iter().filter(|e| e.status == "进行中").take(3).collect();
    drop(db);
    let mut ctx = tera::Context::new();
    ctx.insert("recent_artifacts", &recent_artifacts);
    ctx.insert("active_exhibitions", &active_exhibitions);
    if let Some(u) = &user {
        ctx.insert("user", u);
    }
    let html = state.tera.render("index.html", &ctx).unwrap();
    Html(html)
}

pub async fn dashboard(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let db = state.db.lock().unwrap();
    let artifact_count: i64 = db.query_row("SELECT COUNT(*) FROM artifacts", [], |r| r.get(0)).unwrap_or(0);
    let exhibition_count: i64 = db.query_row("SELECT COUNT(*) FROM exhibitions", [], |r| r.get(0)).unwrap_or(0);
    let reservation_count: i64 = db.query_row("SELECT COUNT(*) FROM reservations", [], |r| r.get(0)).unwrap_or(0);
    let conservation_count: i64 = db.query_row("SELECT COUNT(*) FROM conservations", [], |r| r.get(0)).unwrap_or(0);
    let restoration_count: i64 = db.query_row("SELECT COUNT(*) FROM restorations", [], |r| r.get(0)).unwrap_or(0);
    let on_display: i64 = db.query_row("SELECT COUNT(*) FROM artifacts WHERE status='在展'", [], |r| r.get(0)).unwrap_or(0);
    let in_repair: i64 = db.query_row("SELECT COUNT(*) FROM artifacts WHERE status='修复中'", [], |r| r.get(0)).unwrap_or(0);
    let recent_artifacts = artifact_service::list(&db).unwrap_or_default();
    let recent_artifacts: Vec<_> = recent_artifacts.into_iter().take(5).collect();
    let recent_reservations = reservation_service::list(&db).unwrap_or_default();
    let recent_reservations: Vec<_> = recent_reservations.into_iter().take(5).collect();
    drop(db);
    let mut ctx = tera::Context::new();
    ctx.insert("user", &user);
    ctx.insert("artifact_count", &artifact_count);
    ctx.insert("exhibition_count", &exhibition_count);
    ctx.insert("reservation_count", &reservation_count);
    ctx.insert("conservation_count", &conservation_count);
    ctx.insert("restoration_count", &restoration_count);
    ctx.insert("on_display", &on_display);
    ctx.insert("in_repair", &in_repair);
    ctx.insert("recent_artifacts", &recent_artifacts);
    ctx.insert("recent_reservations", &recent_reservations);
    let html = state.tera.render("dashboard/overview.html", &ctx).unwrap();
    Html(html).into_response()
}
