use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Info {
    apiversion: String,
    author: String,
    color: String,
    head: String,
    tail: String,
}

pub async fn handle_root() -> (StatusCode, Json<Info>) {
    let info = Info {
        apiversion: "0.1.0".to_string(),
        author: "alexfertel".to_string(),
        color: "#000000".to_string(),
        head: "sand-worm".to_string(),
        tail: "default".to_string(),
    };

    (StatusCode::OK, Json(info))
}

pub async fn handle_start() -> impl IntoResponse {
    StatusCode::OK
}

#[derive(Serialize, Deserialize)]
pub struct Move {
    r#move: String,
}

pub async fn handle_move() -> (StatusCode, Json<Move>) {
    let m = Move {
        r#move: "up".to_string(),
    };

    (StatusCode::OK, Json(m))
}

pub async fn handle_end() -> impl IntoResponse {
    StatusCode::OK
}
