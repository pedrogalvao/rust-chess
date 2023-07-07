
use rust_chess::board::{Board, Color, Piece, PieceType};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn init_board() {
        let b : Board = Board::new();
        assert_eq!(4, 4);
    }
}
