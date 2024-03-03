use std::sync::{Arc, Mutex};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use game::GameState;
use search::Direction;
use crate::state::AppState;

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

pub async fn handle_start(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<GameState>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    state.games.insert(payload.game.id.clone(), payload.game);

    StatusCode::OK
}

#[derive(Serialize, Deserialize)]
pub struct Move {
    r#move: String,
}

impl From<Direction> for Move {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Move {r#move: "up".to_string()},
            Direction::Right => Move {r#move:"right".to_string()},
            Direction::Down => Move{r#move:"down".to_string()},
            Direction::Left => Move{r#move: "left".to_string()},
        }
    }
}

pub async fn handle_move(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<GameState>,
) -> (StatusCode, Json<Move>) {
    let state = state.lock().unwrap();
    let snake = state.snakes.get(&payload.game.id).unwrap();

    let m = snake.get_move(&payload);
    (StatusCode::OK, Json(m.into()))
}

pub async fn handle_end(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<GameState>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    state.games.remove(&payload.game.id);

    StatusCode::OK
}
