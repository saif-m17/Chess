//Author: Saif Moolji
//Date: 6/13/24
//Code provides some of the basic board structure for the chess engine


//defining bitboards
struct Bitboard(u64);

impl Bitboard {
    const fn empty() -> Self {
        Bitboard(0)
    }

    fn from_squares(squares: &[usize]) -> Self {
        let mut bitboard = Bitboard::empty(); 
        let mut i = 0;
        while i < squares.len() {
            bitboard.0 |= 1 << squares[i];
            i += 1; 
        }
        bitboard
    }
}

//defining chess board 
struct ChessBoard {
    white_pawns: Bitboard,
    white_knights: Bitboard,
    white_bishops: Bitboard,
    white_rooks: Bitboard,
    white_queen: Bitboard,
    white_king: Bitboard,

    black_pawns: Bitboard,
    black_knights: Bitboard,
    black_bishops: Bitboard,
    black_rooks: Bitboard,
    black_queen: Bitboard,
    black_king: Bitboard
}

impl ChessBoard {
    fn new() -> Self {
        ChessBoard {
            white_pawns: Bitboard::from_squares(&[8, 9, 10, 11, 12, 13, 14, 15]),
            white_knights: Bitboard::from_squares(&[1, 6]),
            white_bishops: Bitboard::from_squares(&[2, 5]),
            white_rooks: Bitboard::from_squares(&[0, 7]),
            white_queen: Bitboard::from_squares(&[3]),
            white_king: Bitboard::from_squares(&[4]),

            black_pawns: Bitboard::from_squares(&[48, 49, 50, 51, 52, 53, 54, 55]),
            black_knights: Bitboard::from_squares(&[57, 62]),
            black_bishops: Bitboard::from_squares(&[58, 61]),
            black_rooks: Bitboard::from_squares(&[56, 63]),
            black_queen: Bitboard::from_squares(&[59]),
            black_king: Bitboard::from_squares(&[60])

        }
    }
}

//print for testing
fn print_chess_board(board: &ChessBoard) {
    let pieces = [
        ('\u{265F}', '\u{2659}'), // Pawns
        ('\u{265E}', '\u{2658}'), // Knights
        ('\u{265D}', '\u{2657}'), // Bishops
        ('\u{265C}', '\u{2656}'), // Rooks
        ('\u{265B}', '\u{2655}'), // Queens
        ('\u{265A}', '\u{2654}') // Kings
    ];

    for rank in (0..8).rev() {
        for file in 0..8 {
            let index = rank * 8 + file;
            let piece = if board.white_pawns.0 & (1 << index) != 0 {
                pieces[0].0
            } else if board.white_knights.0 & (1 << index) != 0 {
                pieces[1].0
            } else if board.white_bishops.0 & (1 << index) != 0 {
                pieces[2].0
            } else if board.white_rooks.0 & (1 << index) != 0 {
                pieces[3].0
            } else if board.white_queen.0 & (1 << index) != 0 {
                pieces[4].0
            } else if board.white_king.0 & (1 << index) != 0 {
                pieces[5].0
            } else if board.black_pawns.0 & (1 << index) != 0 {
                pieces[0].1
            } else if board.black_knights.0 & (1 << index) != 0 {
                pieces[1].1
            } else if board.black_bishops.0 & (1 << index) != 0 {
                pieces[2].1
            } else if board.black_rooks.0 & (1 << index) != 0 {
                pieces[3].1
            } else if board.black_queen.0 & (1 << index) != 0 {
                pieces[4].1
            } else if board.black_king.0 & (1 << index) != 0 {
                pieces[5].1
            } else {
                '.'
            };
            print!("{} ", piece);
        }
        println!();
    }
}



fn main() {
    let chess_board = ChessBoard::new();
    print_chess_board(&chess_board);
}



