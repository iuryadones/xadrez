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
