pub mod db;
pub mod models;
pub mod routes;

use axum::{Router, routing::get};

use crate::domains::auth::AuthState;

pub fn router(state: AuthState) -> Router {
    Router::new()
        .route(
            "/",
            get(routes::list_news).post(routes::create_news_article),
        )
        .route(
            "/{id}",
            get(routes::get_news_article)
                .patch(routes::update_news_article)
                .delete(routes::delete_news_article),
        )
        .with_state(state)
}
