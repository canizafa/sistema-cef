use axum::{Router, routing::get};

use crate::routes::health_checker::health_checker;

pub fn router() -> Router {
    Router::new().route("/", get(health_checker))
}
