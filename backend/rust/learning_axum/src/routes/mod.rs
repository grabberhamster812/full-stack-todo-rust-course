pub mod hello_world;
mod middleware;
mod tasks;
mod users;

use self::{
    tasks::{create_task, get_one_task, mark_completed},
    users::{logout, sign_in},
};
use crate::config::Config;
use axum::{
    routing::{get, post, put},
    Extension, Router,
};
use middleware::auth_required;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use users::create_user;

pub fn create_router(config: Arc<Config>, db: DatabaseConnection) -> Router {
    Router::new()
        .route("/api/v1/users/logout", post(logout))
        .route("/api/v1/tasks", get(tasks::get_all_tasks))
        .route("/api/v1/tasks", post(create_task))
        .route("/api/v1/tasks/:task_id", get(get_one_task))
        .route("/api/v1/tasks/:task_id/completed", put(mark_completed))
        .layer(axum::middleware::from_fn(auth_required))
        .route("/hello_world", get(hello_world::hello_world))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login", post(sign_in))
        .layer(Extension(config))
        .layer(Extension(db))
}
