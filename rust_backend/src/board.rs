// enumerating the pieces - capital refer to white pieces, e is empty 
enum pieces {
    e, P, N, B, R, Q, K, p, n, b, r, q, k, o
}

//defining bitboard
struct Bitboard(u64);

impl Bitboard {
    const fn empty() -> Self {
        Bitboard(0); 
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
    whitePawns: Bitboard;
    whiteKnights: Bitboard;
    whiteBishops: Bitboard;
    whiteRooks: Bitboard;
    whiteQueen: Bitboard;
    whiteKing: Bitboard;

    blackPawns: Bitboard;
    blackKnights: Bitboard;
    blackBishops: Bitboard;
    blackRooks: Bitboard;
    blackQueen: Bitboard;
    blackKing: Bitboard; 
}

impl ChessBoard {
    const fn new() -> Self {
        ChessBoard {
            whitePawns: Bitboard::from_squares(&[8, 9, 10, 11, 12, 13, 14, 15]);
            whiteKnights: Bitboard::from_squares(&[1, 6]);
            whiteBishops: Bitboard::from_squares(&[2, 5]);
            whiteRooks: Bitboard::from_squares(&[0, 7]);
            whiteQueen: Bitboard::from_squares(&[3]);
            whiteKing: Bitboard::from_squares(&[4]);

            blackPawns: Bitboard::from_squares(&[48, 49, 50, 51, 52, 53, 54, 55]);
            blackKnights: Bitboard::from_squares(&[57, 62]);
            blackBishops: Bitboard::from_squares(&[58, 61]);
            blackRooks: Bitboard::from_squares(&[56, 63]);
            blackQueen: Bitboard::from_squares(&[59]);
            blackKing: Bitboard::from_squares(&[60]);

        }
    }
}





