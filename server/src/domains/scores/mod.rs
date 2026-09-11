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
        .route("/", post(routes::submit_score))
        .route("/leaderboard", get(routes::get_leaderboard))
        .route(
            "/checkpoint/{checkpoint_id}",
            get(routes::list_by_checkpoint),
        )
        .route("/team/{team_id}", get(routes::list_by_team))
        .route("/{id}", axum::routing::patch(routes::update_score))
        .with_state(state)
}
