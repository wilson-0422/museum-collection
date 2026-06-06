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
use crate::services::{reservation_service, exhibition_service};

#[derive(Deserialize)]
pub struct CreateReservationForm {
    pub visitor_name: String,
    pub phone: String,
    pub visit_date: String,
    pub visitor_count: String,
    pub exhibition_id: String,
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    let db = state.db.lock().unwrap();
    let reservations = reservation_service::list(&db).unwrap_or_default();
    drop(db);
    let mut ctx = tera::Context::new();
    ctx.insert("reservations", &reservations);
    if let Some(u) = &user {
        ctx.insert("user", u);
    }
    let html = state.tera.render("reservations/list.html", &ctx).unwrap();
    Html(html)
}

pub async fn create_page(
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
    let html = state.tera.render("reservations/create.html", &ctx).unwrap();
    Html(html).into_response()
}

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(form): Form<CreateReservationForm>,
) -> impl IntoResponse {
    let visitor_count: i64 = form.visitor_count.parse().unwrap_or(1);
    let exhibition_id = if form.exhibition_id.is_empty() {
        None
    } else {
        form.exhibition_id.parse::<i64>().ok()
    };
    let db = state.db.lock().unwrap();
    let _ = reservation_service::create(&db, &form.visitor_name, &form.phone, &form.visit_date, visitor_count, exhibition_id, "待确认");
    drop(db);
    Redirect::to("/reservations").into_response()
}

pub async fn confirm(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let db = state.db.lock().unwrap();
    let _ = reservation_service::update_status(&db, id, "已确认");
    drop(db);
    Redirect::to("/reservations").into_response()
}

pub async fn cancel(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let db = state.db.lock().unwrap();
    let _ = reservation_service::update_status(&db, id, "已取消");
    drop(db);
    Redirect::to("/reservations").into_response()
}
