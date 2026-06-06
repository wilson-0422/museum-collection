use axum::{
    Router,
    routing::{get, post},
};
use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tera::Tera;
use tower_http::services::ServeDir;

mod config;
mod handlers;
mod middleware;
mod models;
mod services;

use config::app::BIND_ADDR;
use middleware::auth::SessionInfo;

pub struct AppState {
    pub db: Mutex<Connection>,
    pub tera: Tera,
    pub sessions: Mutex<HashMap<String, SessionInfo>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let conn = config::database::init_db().expect("数据库初始化失败");
    config::seed::seed_data(&conn).expect("种子数据初始化失败");

    let tera = Tera::new("templates/**/*").expect("模板初始化失败");

    let state = Arc::new(AppState {
        db: Mutex::new(conn),
        tera,
        sessions: Mutex::new(HashMap::new()),
    });

    let app = Router::new()
        .route("/", get(handlers::home::index))
        .route("/dashboard", get(handlers::home::dashboard))
        .route("/auth/login", get(handlers::auth::login_page).post(handlers::auth::login_post))
        .route("/auth/register", get(handlers::auth::register_page).post(handlers::auth::register_post))
        .route("/auth/logout", get(handlers::auth::logout))
        .route("/artifacts", get(handlers::artifact::list))
        .route("/artifacts/create", get(handlers::artifact::create_page).post(handlers::artifact::create_post))
        .route("/artifacts/:id", get(handlers::artifact::detail))
        .route("/artifacts/:id/edit", get(handlers::artifact::edit_page).post(handlers::artifact::edit_post))
        .route("/artifacts/:id/delete", post(handlers::artifact::delete))
        .route("/conservations", get(handlers::conservation::list))
        .route("/conservations/create", get(handlers::conservation::create_page).post(handlers::conservation::create_post))
        .route("/conservations/:id", get(handlers::conservation::detail))
        .route("/exhibitions", get(handlers::exhibition::list))
        .route("/exhibitions/create", get(handlers::exhibition::create_page).post(handlers::exhibition::create_post))
        .route("/exhibitions/:id", get(handlers::exhibition::detail))
        .route("/restorations", get(handlers::restoration::list))
        .route("/restorations/create", get(handlers::restoration::create_page).post(handlers::restoration::create_post))
        .route("/restorations/:id", get(handlers::restoration::detail))
        .route("/reservations", get(handlers::reservation::list))
        .route("/reservations/create", get(handlers::reservation::create_page).post(handlers::reservation::create_post))
        .route("/reservations/:id/confirm", get(handlers::reservation::confirm))
        .route("/reservations/:id/cancel", get(handlers::reservation::cancel))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(BIND_ADDR).await.unwrap();
    tracing::info!("服务器启动于 {}", BIND_ADDR);
    axum::serve(listener, app).await.unwrap();
}
