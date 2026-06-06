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
use crate::services::{exhibition_service, artifact_service};

#[derive(Deserialize)]
pub struct CreateExhibitionForm {
    pub name: String,
    pub venue: String,
    pub start_date: String,
    pub end_date: String,
    pub curator: String,
    pub description: String,
    pub status: String,
    pub artifact_ids: String,
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    let db = state.db.lock().unwrap();
    let exhibitions = exhibition_service::list(&db).unwrap_or_default();
    drop(db);
    let mut ctx = tera::Context::new();
    ctx.insert("exhibitions", &exhibitions);
    if let Some(u) = &user {
        ctx.insert("user", u);
    }
    let html = state.tera.render("exhibitions/list.html", &ctx).unwrap();
    Html(html)
}

pub async fn detail(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    let db = state.db.lock().unwrap();
    let exhibition = match exhibition_service::find_by_id(&db, id) {
        Ok(e) => e,
        Err(_) => return (axum::http::StatusCode::NOT_FOUND, "展览未找到").into_response(),
    };
    let artifacts = exhibition_service::find_artifacts_by_exhibition(&db, id).unwrap_or_default();
    drop(db);
    let mut ctx = tera::Context::new();
    ctx.insert("exhibition", &exhibition);
    ctx.insert("artifacts", &artifacts);
    if let Some(u) = &user {
        ctx.insert("user", u);
    }
    let html = state.tera.render("exhibitions/detail.html", &ctx).unwrap();
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
    let html = state.tera.render("exhibitions/create.html", &ctx).unwrap();
    Html(html).into_response()
}

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(form): Form<CreateExhibitionForm>,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let db = state.db.lock().unwrap();
    match exhibition_service::create(&db, &form.name, &form.venue, &form.start_date, &form.end_date, &form.curator, &form.description, &form.status) {
        Ok(exhibition_id) => {
            let artifact_ids: Vec<i64> = form.artifact_ids
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            for aid in artifact_ids {
                let _ = exhibition_service::add_artifact(&db, exhibition_id, aid);
            }
            drop(db);
            Redirect::to("/exhibitions").into_response()
        }
        Err(_) => {
            drop(db);
            Redirect::to("/exhibitions/create").into_response()
        }
    }
}
