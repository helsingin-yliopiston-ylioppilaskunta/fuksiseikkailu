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
        .route(
            "/",
            get(routes::list_checkpoints).post(routes::create_checkpoint),
        )
        .route("/batch", post(routes::batch_import))
        .route(
            "/{id}",
            get(routes::get_checkpoint)
                .patch(routes::update_checkpoint)
                .delete(routes::delete_checkpoint),
        )
        .with_state(state)
}
