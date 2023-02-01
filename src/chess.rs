#![allow(dead_code, unused)]
#[derive(Clone, Copy, Default)]
enum Piece {
    #[default]
    EMPTY,
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}

#[derive(Clone, Copy, Default)]
enum Side {
    #[default]
    EMPTY,
    WHITE,
    BLACK,
}


#[derive(Clone, Copy)]
struct Square {
    piece: Piece,
    side: Side,
}

impl Default for Square {
    fn default() -> Self {
        Self {
            piece: Piece::EMPTY,
            side: Side::EMPTY,
        }
    }

}

struct ChessBoard {
    turn: Side,
    board: Vec<Square>,
}

impl ChessBoard {
    // add code here
    pub fn new() -> Self {
        Self {
            turn: Side::WHITE,
            board: vec![(Square::default()); 64],
        }
    }
    fn turn_switch(&mut self) {
        self.turn = match &self.turn {
            Side::WHITE => Side::BLACK,
            Side::BLACK => Side::WHITE,
            Side::EMPTY => Side::EMPTY,
        }
    }
    fn setup(&mut self) {
        // self.board = vec![ ];
    }
    fn print(&self) {

    }
}
