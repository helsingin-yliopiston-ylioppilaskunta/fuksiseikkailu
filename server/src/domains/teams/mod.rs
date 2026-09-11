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
        .route("/", get(routes::list_teams).post(routes::create_team))
        .route("/batch", post(routes::batch_import))
        .route(
            "/{id}",
            get(routes::get_team)
                .patch(routes::update_team)
                .delete(routes::delete_team),
        )
        .with_state(state)
}
