pub mod db;
pub mod models;
pub mod routes;

use axum::{Router, routing::get};

use crate::domains::auth::AuthState;

pub fn router(state: AuthState) -> Router {
    Router::new()
        .route("/", get(routes::list_areas).post(routes::create_area))
        .route(
            "/{id}",
            get(routes::get_area)
                .patch(routes::update_area)
                .delete(routes::delete_area),
        )
        .with_state(state)
}
