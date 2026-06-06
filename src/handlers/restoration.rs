use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
use std::sync::Arc;
use crate::AppState;
use crate::middleware::auth::get_current_user;
use crate::services::{restoration_service, artifact_service};

#[derive(Deserialize)]
pub struct CreateRestorationForm {
    pub artifact_id: i64,
    pub restorer: String,
    pub method: String,
    pub start_date: String,
    pub end_date: String,
    pub cost: String,
    pub description: String,
    pub status: String,
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    let db = state.db.lock().unwrap();
    let restorations = restoration_service::list(&db).unwrap_or_default();
    drop(db);
    let mut ctx = tera::Context::new();
    ctx.insert("restorations", &restorations);
    if let Some(u) = &user {
        ctx.insert("user", u);
    }
    let html = state.tera.render("restorations/list.html", &ctx).unwrap();
    Html(html)
}

pub async fn detail(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    let db = state.db.lock().unwrap();
    let restoration = match restoration_service::find_by_id(&db, id) {
        Ok(r) => r,
        Err(_) => return (axum::http::StatusCode::NOT_FOUND, "修复记录未找到").into_response(),
    };
    drop(db);
    let mut ctx = tera::Context::new();
    ctx.insert("restoration", &restoration);
    if let Some(u) = &user {
        ctx.insert("user", u);
    }
    let html = state.tera.render("restorations/detail.html", &ctx).unwrap();
    Html(html).into_response()
}

pub async fn create_page(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let db = state.db.lock().unwrap();
    let artifacts = artifact_service::list(&db).unwrap_or_default();
    drop(db);
    let mut ctx = tera::Context::new();
    ctx.insert("artifacts", &artifacts);
    ctx.insert("user", &user);
    let html = state.tera.render("restorations/create.html", &ctx).unwrap();
    Html(html).into_response()
}

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(form): Form<CreateRestorationForm>,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let cost: f64 = form.cost.parse().unwrap_or(0.0);
    let db = state.db.lock().unwrap();
    let _ = restoration_service::create(&db, form.artifact_id, &form.restorer, &form.method, &form.start_date, &form.end_date, cost, &form.description, &form.status);
    drop(db);
    Redirect::to("/restorations").into_response()
}
