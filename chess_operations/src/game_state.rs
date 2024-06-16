//Author: Saif Moolji
//Date: 6/15/24
//Code to keep track of the state of a game 

mod board;

use board::ChessBoard;
use board::print_chess_board;

enum Side {
    White,
    Black
}

struct GameState {
    side: Side, 
    board: ChessBoard,
    game_over: bool,
    white_castle: bool,
    black_castle: bool
}

impl GameState {
    fn new() -> Self {
        GameState {
            side: Side::White,
            board: ChessBoard::new(),
            game_over: false,
            white_castle: true,
            black_castle: true
        }
    }
}

fn main() {
    let game_state = GameState::new();
}
