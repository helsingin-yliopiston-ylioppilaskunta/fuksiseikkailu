pub mod db;
pub mod models;
pub mod routes;

use axum::{
    Router,
    routing::{get, post},
};

use crate::domains::auth::AuthState;

pub fn router(state: AuthState) -> Router {
    Router::new()
        .route("/checkpoint", post(routes::rate_checkpoint))
        .route("/team", post(routes::rate_team))
        .route(
            "/checkpoint/{checkpoint_id}",
            get(routes::get_checkpoint_ratings),
        )
        .route("/team/{team_id}", get(routes::get_team_ratings))
        .with_state(state)
}
