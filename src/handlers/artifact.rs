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
use crate::models::artifact::Artifact;
use crate::services::{artifact_service, conservation_service, restoration_service};

#[derive(Deserialize)]
pub struct CreateArtifactForm {
    pub name: String,
    pub category: String,
    pub era: String,
    pub material: String,
    pub dimensions: String,
    pub origin: String,
    pub description: String,
    pub status: String,
    pub entry_date: String,
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    let db = state.db.lock().unwrap();
    let artifacts = artifact_service::list(&db).unwrap_or_default();
    drop(db);
    let mut ctx = tera::Context::new();
    ctx.insert("artifacts", &artifacts);
    if let Some(u) = &user {
        ctx.insert("user", u);
    }
    let html = state.tera.render("artifacts/list.html", &ctx).unwrap();
    Html(html)
}

pub async fn detail(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    let db = state.db.lock().unwrap();
    let artifact = match artifact_service::find_by_id(&db, id) {
        Ok(a) => a,
        Err(_) => return (axum::http::StatusCode::NOT_FOUND, "文物未找到").into_response(),
    };
    let conservations = conservation_service::list_by_artifact(&db, id).unwrap_or_default();
    let restorations = restoration_service::list_by_artifact(&db, id).unwrap_or_default();
    drop(db);
    let mut ctx = tera::Context::new();
    ctx.insert("artifact", &artifact);
    ctx.insert("conservations", &conservations);
    ctx.insert("restorations", &restorations);
    if let Some(u) = &user {
        ctx.insert("user", u);
    }
    let html = state.tera.render("artifacts/detail.html", &ctx).unwrap();
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
    let mut ctx = tera::Context::new();
    ctx.insert("user", &user);
    let html = state.tera.render("artifacts/create.html", &ctx).unwrap();
    Html(html).into_response()
}

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(form): Form<CreateArtifactForm>,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let artifact = Artifact {
        id: 0,
        name: form.name,
        category: form.category,
        era: form.era,
        material: form.material,
        dimensions: form.dimensions,
        origin: form.origin,
        description: form.description,
        status: form.status,
        entry_date: form.entry_date,
        created_at: String::new(),
    };
    let db = state.db.lock().unwrap();
    let _ = artifact_service::create(&db, &artifact);
    drop(db);
    Redirect::to("/artifacts").into_response()
}

pub async fn edit_page(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let db = state.db.lock().unwrap();
    let artifact = match artifact_service::find_by_id(&db, id) {
        Ok(a) => a,
        Err(_) => return (axum::http::StatusCode::NOT_FOUND, "文物未找到").into_response(),
    };
    drop(db);
    let mut ctx = tera::Context::new();
    ctx.insert("artifact", &artifact);
    ctx.insert("user", &user);
    let html = state.tera.render("artifacts/edit.html", &ctx).unwrap();
    Html(html).into_response()
}

pub async fn edit_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    jar: CookieJar,
    Form(form): Form<CreateArtifactForm>,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let artifact = Artifact {
        id,
        name: form.name,
        category: form.category,
        era: form.era,
        material: form.material,
        dimensions: form.dimensions,
        origin: form.origin,
        description: form.description,
        status: form.status,
        entry_date: form.entry_date,
        created_at: String::new(),
    };
    let db = state.db.lock().unwrap();
    let _ = artifact_service::update(&db, &artifact);
    drop(db);
    Redirect::to("/artifacts").into_response()
}

pub async fn delete(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let db = state.db.lock().unwrap();
    let _ = artifact_service::delete(&db, id);
    drop(db);
    Redirect::to("/artifacts").into_response()
}
