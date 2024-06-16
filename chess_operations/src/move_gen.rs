//Author: Saif Moolji
//Date: 6/15/24
//Code to generate possible legal moves

mod board;

use board::Bitboard;

pub enum Side {
    White,
    Black
}
pub enum Pieces {
    white_pawn,
    white_bishop,
    white_knight,
    white_rook,
    white_queen,
    white_king,
    black_pawn,
    black_bishop,
    black_knight,
    black_rook,
    black_queen,
    black_king,
    empty
}

pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}

pub fn generate_pawn_moves(square: Square, side: Side) {
    let mut bitboard = Bitboard::empty();
    let mut results = Bitboard::empty();



}

fn main() {
    let mut x = Bitboard::empty();
    x.set_square(5);
    x.set_square(6);
    let y = x.get_square(5);
    x.print_bit_board();
}

