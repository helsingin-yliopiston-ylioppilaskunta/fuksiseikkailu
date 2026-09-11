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
        .route("/", get(routes::list_photos).post(routes::create_photo))
        .route(
            "/{id}",
            get(routes::list_photos)
                .patch(routes::update_photo)
                .delete(routes::delete_photo),
        )
        .route("/{id}/vote", post(routes::vote_photo))
        .route(
            "/{id}/suggest",
            post(routes::suggest_team).get(routes::list_suggestions),
        )
        .with_state(state)
}
