use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opponent(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl PieceType {
    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_lowercase() {
            'k' => Some(PieceType::King),
            'q' => Some(PieceType::Queen),
            'r' => Some(PieceType::Rook),
            'b' => Some(PieceType::Bishop),
            'n' => Some(PieceType::Knight),
            'p' => Some(PieceType::Pawn),
            _ => None,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            PieceType::King => 'k',
            PieceType::Queen => 'q',
            PieceType::Rook => 'r',
            PieceType::Bishop => 'b',
            PieceType::Knight => 'n',
            PieceType::Pawn => 'p',
        }
    }

    pub fn to_unicode(self, color: Color) -> &'static str {
        match (self, color) {
            (PieceType::King, Color::White) => "♔",
            (PieceType::King, Color::Black) => "♚",
            (PieceType::Queen, Color::White) => "♕",
            (PieceType::Queen, Color::Black) => "♛",
            (PieceType::Rook, Color::White) => "♖",
            (PieceType::Rook, Color::Black) => "♜",
            (PieceType::Bishop, Color::White) => "♗",
            (PieceType::Bishop, Color::Black) => "♝",
            (PieceType::Knight, Color::White) => "♘",
            (PieceType::Knight, Color::Black) => "♞",
            (PieceType::Pawn, Color::White) => "♙",
            (PieceType::Pawn, Color::Black) => "♟",
        }
    }

    pub fn to_unicode_square(self, color: Color, is_light: bool) -> &'static str {
        let display_color = match (color, is_light) {
            (Color::White, true) | (Color::Black, false) => Color::White,
            (Color::White, false) | (Color::Black, true) => Color::Black,
        };
        self.to_unicode(display_color)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    pub kind: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(kind: PieceType, color: Color) -> Self {
        Self { kind, color }
    }

    pub fn from_fen_char(c: char) -> Option<Self> {
        let color = if c.is_uppercase() {
            Color::White
        } else {
            Color::Black
        };
        PieceType::from_char(c).map(|kind| Self { kind, color })
    }

    pub fn to_fen_char(self) -> char {
        let c = self.kind.to_char();
        match self.color {
            Color::White => c.to_ascii_uppercase(),
            Color::Black => c,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind.to_unicode(self.color))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_opponent() {
        assert_eq!(Color::White.opponent(), Color::Black);
        assert_eq!(Color::Black.opponent(), Color::White);
        assert_eq!(Color::White.opponent().opponent(), Color::White);
    }

    #[test]
    fn test_piecetype_from_char() {
        assert_eq!(PieceType::from_char('K'), Some(PieceType::King));
        assert_eq!(PieceType::from_char('q'), Some(PieceType::Queen));
        assert_eq!(PieceType::from_char('r'), Some(PieceType::Rook));
        assert_eq!(PieceType::from_char('B'), Some(PieceType::Bishop));
        assert_eq!(PieceType::from_char('N'), Some(PieceType::Knight));
        assert_eq!(PieceType::from_char('p'), Some(PieceType::Pawn));
        assert_eq!(PieceType::from_char('x'), None);
    }

    #[test]
    fn test_piecetype_to_char() {
        assert_eq!(PieceType::King.to_char(), 'k');
        assert_eq!(PieceType::Queen.to_char(), 'q');
        assert_eq!(PieceType::Rook.to_char(), 'r');
        assert_eq!(PieceType::Bishop.to_char(), 'b');
        assert_eq!(PieceType::Knight.to_char(), 'n');
        assert_eq!(PieceType::Pawn.to_char(), 'p');
    }

    #[test]
    fn test_piecetype_to_unicode() {
        assert_eq!(PieceType::King.to_unicode(Color::White), "♔");
        assert_eq!(PieceType::King.to_unicode(Color::Black), "♚");
        assert_eq!(PieceType::Queen.to_unicode(Color::White), "♕");
        assert_eq!(PieceType::Queen.to_unicode(Color::Black), "♛");
        assert_eq!(PieceType::Pawn.to_unicode(Color::White), "♙");
        assert_eq!(PieceType::Pawn.to_unicode(Color::Black), "♟");
    }

    #[test]
    fn test_piece_from_fen_char() {
        assert_eq!(Piece::from_fen_char('K'), Some(Piece::new(PieceType::King, Color::White)));
        assert_eq!(Piece::from_fen_char('k'), Some(Piece::new(PieceType::King, Color::Black)));
        assert_eq!(Piece::from_fen_char('P'), Some(Piece::new(PieceType::Pawn, Color::White)));
        assert_eq!(Piece::from_fen_char('p'), Some(Piece::new(PieceType::Pawn, Color::Black)));
        assert_eq!(Piece::from_fen_char('.'), None);
    }

    #[test]
    fn test_piece_to_fen_char() {
        assert_eq!(Piece::new(PieceType::King, Color::White).to_fen_char(), 'K');
        assert_eq!(Piece::new(PieceType::King, Color::Black).to_fen_char(), 'k');
        assert_eq!(Piece::new(PieceType::Pawn, Color::White).to_fen_char(), 'P');
    }
}
