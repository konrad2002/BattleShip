mod state;
mod models;

use state::AppState;
use models::Boards;

use axum::{routing::get, Json, Router};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use axum::extract::{Path, State};
use axum::routing::post;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let state = AppState {
        boards: Arc::new(Mutex::new(Boards {
            size: 10,
            player1: Vec::new(),
            player2: Vec::new(),
        })),
        winner: Arc::new(Mutex::new(0)),
    };

    let app = Router::new()
        .route("/", get(hello))
        .route("/board/:side", get(get_boards_by_side))
        .route("/board/:side/init", post(init_board))
        .route("/board/:side/shoot/:pos", post(shoot))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8300));
    println!("Server running at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    "Hello, World!"
}

async fn get_boards_by_side(State(state): State<AppState>, Path(side): Path<u8>) -> Json<Game> {
    let game = build_game(&state, side);
    Json(game)
}

async fn init_board(
    State(state): State<AppState>,
    Path(side): Path<u8>,
    Json(board): Json<Vec<u8>>
) -> Json<Game> {
    println!("{}", format!("Initialising Board for side {}...", side));

    {
        let mut boards = state.boards.lock().unwrap();

        match side {
            1 => boards.player1 = board,
            2 => boards.player2 = board,
            _ => panic!("Invalid board side: {}", side),
        }

        println!("Done!")
    }

    Json(build_game(&state, side))
}

async fn shoot(
    State(state): State<AppState>,
    Path((side, pos)): Path<(u8, u8)>,
) -> Json<Game> {
    println!("{}", format!("player {} shoots on {}...", side, pos));

    {
        let mut boards = state.boards.lock().unwrap();

        match side {
            1 => boards.player1[pos as usize] = 1,
            2 => boards.player2[pos as usize] = 1,
            _ => panic!("Invalid board side: {}", side),
        }
    }

    Json(build_game(&state, side))
}

fn build_game(state: &AppState, side: u8) -> Game {
    let boards = state.boards.lock().unwrap().clone();
    let winner = state.winner.lock().unwrap().clone();

    Game { boards, winner }
}

// model

use crate::models::Game;


