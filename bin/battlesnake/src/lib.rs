use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Router,
};
use state::AppState;

pub mod routes;
pub mod snakes;
pub mod state;

pub fn run() -> shuttle_axum::ShuttleAxum {
    let state = AppState {
        games: Default::default(),
        snakes: Default::default(),
    };
    let state = Mutex::new(state);
    let state = Arc::new(state);

    let router = Router::new()
        .route("/", get(routes::handle_root))
        .route("/start", post(routes::handle_start))
        .route("/move", post(routes::handle_move))
        .route("/end", post(routes::handle_end))
        .with_state(state);

    Ok(router.into())
}
