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
use crate::services::{conservation_service, artifact_service};

#[derive(Deserialize)]
pub struct CreateConservationForm {
    pub artifact_id: i64,
    pub method: String,
    pub performer: String,
    pub start_date: String,
    pub end_date: String,
    pub notes: String,
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    let db = state.db.lock().unwrap();
    let conservations = conservation_service::list(&db).unwrap_or_default();
    drop(db);
    let mut ctx = tera::Context::new();
    ctx.insert("conservations", &conservations);
    if let Some(u) = &user {
        ctx.insert("user", u);
    }
    let html = state.tera.render("conservations/list.html", &ctx).unwrap();
    Html(html)
}

pub async fn detail(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    let db = state.db.lock().unwrap();
    let conservation = match conservation_service::find_by_id(&db, id) {
        Ok(c) => c,
        Err(_) => return (axum::http::StatusCode::NOT_FOUND, "养护记录未找到").into_response(),
    };
    drop(db);
    let mut ctx = tera::Context::new();
    ctx.insert("conservation", &conservation);
    if let Some(u) = &user {
        ctx.insert("user", u);
    }
    let html = state.tera.render("conservations/detail.html", &ctx).unwrap();
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
    let html = state.tera.render("conservations/create.html", &ctx).unwrap();
    Html(html).into_response()
}

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(form): Form<CreateConservationForm>,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let db = state.db.lock().unwrap();
    let _ = conservation_service::create(&db, form.artifact_id, &form.method, &form.performer, &form.start_date, &form.end_date, &form.notes);
    drop(db);
    Redirect::to("/conservations").into_response()
}
