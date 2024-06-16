//Author: Saif Moolji
//Date: 6/15/24
//Code to keep track of the state of a game 

mod board;

use board::ChessBoard;
use board::print_chess_board;

struct GameState {
    side: u16, 
    board: ChessBoard,
    game_over: bool,
    white_castle: bool,
    black_castle: bool
}

impl GameState {
    fn new() -> Self {
        GameState {
            side: 0,
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
