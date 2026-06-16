use crate::{Color, Piece, PieceType, Square};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn empty() -> Self {
        Self {
            squares: [[None; 8]; 8],
        }
    }

    pub fn initial() -> Self {
        let mut board = Self::empty();
        let back_rank = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        for (file, &kind) in back_rank.iter().enumerate() {
            board.squares[0][file] = Some(Piece::new(kind, Color::White));
            board.squares[1][file] = Some(Piece::new(PieceType::Pawn, Color::White));
            board.squares[6][file] = Some(Piece::new(PieceType::Pawn, Color::Black));
            board.squares[7][file] = Some(Piece::new(kind, Color::Black));
        }

        board
    }

    pub fn piece_at(&self, sq: Square) -> Option<Piece> {
        if sq.is_valid() {
            self.squares[sq.rank][sq.file]
        } else {
            None
        }
    }

    pub fn set_piece(&mut self, sq: Square, piece: Option<Piece>) {
        if sq.is_valid() {
            self.squares[sq.rank][sq.file] = piece;
        }
    }

    pub fn squares(&self) -> &[[Option<Piece>; 8]; 8] {
        &self.squares
    }

    pub fn king_square(&self, color: Color) -> Option<Square> {
        for rank in 0..8 {
            for file in 0..8 {
                let sq = Square::new_unchecked(file, rank);
                if let Some(p) = self.piece_at(sq) {
                    if p.kind == PieceType::King && p.color == color {
                        return Some(sq);
                    }
                }
            }
        }
        None
    }

    pub fn to_fen(&self) -> String {
        let mut parts = Vec::new();
        for rank in (0..8).rev() {
            let mut empty = 0;
            let mut row = String::new();
            for file in 0..8 {
                let sq = Square::new_unchecked(file, rank);
                match self.piece_at(sq) {
                    Some(p) => {
                        if empty > 0 {
                            row.push_str(&empty.to_string());
                            empty = 0;
                        }
                        row.push(p.to_fen_char());
                    }
                    None => {
                        empty += 1;
                    }
                }
            }
            if empty > 0 {
                row.push_str(&empty.to_string());
            }
            parts.push(row);
        }
        parts.join("/")
    }

    pub fn from_fen(placement: &str) -> Result<Self, String> {
        let mut board = Self::empty();
        let ranks: Vec<&str> = placement.split('/').collect();
        if ranks.len() != 8 {
            return Err("FEN invalido: esperado 8 ranks".into());
        }

        for (i, rank_str) in ranks.iter().enumerate() {
            let rank = 7 - i;
            let mut file = 0;
            for c in rank_str.chars() {
                if file >= 8 {
                    return Err(format!("FEN invalido: muitas casas no rank {}", rank + 1));
                }
                if let Some(count) = c.to_digit(10) {
                    file += count as usize;
                } else if let Some(piece) = Piece::from_fen_char(c) {
                    board.squares[rank][file] = Some(piece);
                    file += 1;
                } else {
                    return Err(format!("Caractere FEN invalido: '{}'", c));
                }
            }
        }

        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_initial() {
        let board = Board::initial();
        assert_eq!(board.piece_at(Square::from_algebraic("e1").unwrap()), Some(Piece::new(PieceType::King, Color::White)));
        assert_eq!(board.piece_at(Square::from_algebraic("d8").unwrap()), Some(Piece::new(PieceType::Queen, Color::Black)));
        assert_eq!(board.piece_at(Square::from_algebraic("e4").unwrap()), None);
    }

    #[test]
    fn test_board_from_fen_valid() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
        let initial = Board::initial();
        assert_eq!(board, initial);
    }

    #[test]
    fn test_board_from_fen_invalid_rank_count() {
        assert!(Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP").is_err());
    }


    #[test]
    fn test_board_king_square() {
        let board = Board::initial();
        assert_eq!(board.king_square(Color::White), Square::from_algebraic("e1"));
        assert_eq!(board.king_square(Color::Black), Square::from_algebraic("e8"));
    }

    #[test]
    fn test_board_to_fen_roundtrip() {
        let board = Board::initial();
        let fen = board.to_fen();
        let parsed = Board::from_fen(&fen).unwrap();
        assert_eq!(board, parsed);
    }
}
