mod state;
mod models;

use state::AppState;
use models::Boards;

use axum::{routing::get, Json, Router};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex, MutexGuard};
use axum::extract::{Path, State};
use axum::routing::{post};
use tower_http::cors::{CorsLayer};

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
        .layer(CorsLayer::permissive())
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

        let len = board.len();

        match side {
            1 => boards.player1 = board,
            2 => boards.player2 = board,
            _ => panic!("Invalid board side: {}", side),
        }

        let sqrt = (len as f64).sqrt();

        if sqrt.fract() != 0.0 {
            panic!("Length {} is not a perfect square", len);
        }

        boards.size = sqrt as u8;

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

        let size = boards.size as usize;

        let board_to_edit: &mut Vec<u8> = match side {
            1 => &mut boards.player1,
            2 => &mut boards.player2,
            _ => panic!("Invalid board side: {}", side),
        };

        let row = pos as usize / size;
        let col = pos as usize % size;

        let mut has_two = false;

        // check neighbors: up, down, left, right
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in directions {
            let nr = row as isize + dr;
            let nc = col as isize + dc;

            if nr >= 0 && nr < size as isize && nc >= 0 && nc < size as isize {
                let npos = (nr as usize) * size + (nc as usize);
                if board_to_edit[npos] == 2 {
                    has_two = true;
                    break;
                }
            }
        }

        let field = &mut board_to_edit[pos as usize];

        match (*field) {
            2 => {
                if has_two {
                    *field = 3;
                } else {
                    *field = 4;
                }
            }
            _ => *field = 1
        }
    }

    Json(build_game(&state, side))
}

fn build_game(state: &AppState, side: u8) -> Game {
    let mut boards = state.boards.lock().unwrap().clone();
    let winner = state.winner.lock().unwrap().clone();

    // Only modify the clone, not the stored state
    if (side == 1) {
        boards.player2 = boards.player2
            .into_iter()
            .map(|field| if field == 2 { 0 } else { field })
            .collect();
    } else {

        boards.player1 = boards.player1
            .into_iter()
            .map(|field| if field == 2 { 0 } else { field })
            .collect();
    }

    Game { boards, winner }
}

// model

use crate::models::Game;


