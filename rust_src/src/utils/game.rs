use super::data::{Board, Piece, PieceLocation};

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub hold: Option<Piece>
}

#[derive(Debug, Clone)]
pub struct CheeseGame {
    pub wasted: u8,
    pub height: i8,
    pub game: Game
}

impl Game {
    pub fn advance(&mut self, next: Piece, next_next: Piece, loc: &PieceLocation) -> bool {
        if (self.hold.is_none() && loc.piece == next_next) || (self.hold.is_some() && loc.piece == self.hold.unwrap()) {
            self.hold = Some(next);
        }
        self.board.put_piece(&loc);
        let line_mask = self.board.remove_lines();
        line_mask.count_ones() != 0
    }
}

impl CheeseGame {
    pub fn advance(&mut self, next: Piece, next_next: Piece, loc: &PieceLocation) {
        if loc.blocks().iter().any(|&(_, y)| y < self.height) {
            self.height -= 1;
        } else {
            self.wasted += 1;
        }
        self.game.advance(next, next_next, loc);
    }
}