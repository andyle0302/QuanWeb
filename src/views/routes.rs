use axum::{routing::get, Router};

use super::{home, static_handler};
use super::front;
use crate::types::AppState;
use crate::consts::STATIC_URL;

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
        .route(&format!("{STATIC_URL}/*file"), get(static_handler))
        .route("/post/:year/:month/:slug", get(front::show_post))
        .route("/category/:category/", get(front::list_posts))
}
