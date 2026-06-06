use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Deserialize;
use std::sync::Arc;
use crate::AppState;
use crate::middleware::auth::{get_current_user, SessionInfo};
use crate::services::user_service;

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub password: String,
    pub display_name: String,
}

pub async fn login_page(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    if user.is_some() {
        return Redirect::to("/").into_response();
    }
    let mut ctx = tera::Context::new();
    let html = state.tera.render("auth/login.html", &ctx).unwrap();
    Html(html).into_response()
}

pub async fn login_post(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    let db = state.db.lock().unwrap();
    match user_service::verify_user(&db, &form.username, &form.password) {
        Ok(user) => {
            drop(db);
            let session_id = uuid::Uuid::new_v4().to_string();
            let session_info = SessionInfo {
                user_id: user.id,
                username: user.username.clone(),
                display_name: user.display_name.clone(),
                role: user.role.clone(),
            };
            state.sessions.lock().unwrap().insert(session_id.clone(), session_info);
            let cookie = Cookie::new("session_id", session_id);
            (jar.add(cookie), Redirect::to("/")).into_response()
        }
        Err(msg) => {
            drop(db);
            let mut ctx = tera::Context::new();
            ctx.insert("error", &msg);
            let html = state.tera.render("auth/login.html", &ctx).unwrap();
            Html(html).into_response()
        }
    }
}

pub async fn register_page(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&state.sessions, &jar);
    if user.is_some() {
        return Redirect::to("/").into_response();
    }
    let mut ctx = tera::Context::new();
    let html = state.tera.render("auth/register.html", &ctx).unwrap();
    Html(html).into_response()
}

pub async fn register_post(
    State(state): State<Arc<AppState>>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    let db = state.db.lock().unwrap();
    match user_service::create(&db, &form.username, &form.password, &form.display_name) {
        Ok(_) => {
            drop(db);
            Redirect::to("/auth/login").into_response()
        }
        Err(msg) => {
            drop(db);
            let mut ctx = tera::Context::new();
            ctx.insert("error", &msg);
            let html = state.tera.render("auth/register.html", &ctx).unwrap();
            Html(html).into_response()
        }
    }
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    if let Some(cookie) = jar.get("session_id") {
        let session_id = cookie.value().to_string();
        state.sessions.lock().unwrap().remove(&session_id);
    }
    let jar = jar.remove(Cookie::from("session_id"));
    (jar, Redirect::to("/")).into_response()
}
