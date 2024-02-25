use axum::{
    routing::{get, post},
    Router,
};

use battlesnake::routes;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(routes::handle_root))
        .route("/start", post(routes::handle_start))
        .route("/move", post(routes::handle_move))
        .route("/end", post(routes::handle_end));

    Ok(router.into())
}
