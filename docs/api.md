# API Pública

A library `chess` exporta todos os tipos necessários para integrar o xadrez em qualquer interface.

## Tipos Exportados

```rust
// src/lib.rs
pub use square::Square;
pub use piece::{Color, PieceType, Piece};
pub use board::Board;
pub use mv::Move;
pub use game::{Game, GameStatus, CastlingRights};
pub use moves;
pub use notation::{move_to_algebraic, disambiguation, parse_algebraic};
```

### `Color`

```rust
enum Color { White, Black }
impl Color {
    fn opponent(self) -> Self;
}
```

### `Square`

```rust
struct Square { pub file: usize, pub rank: usize }
// file 0-7 (a=0, h=7), rank 0-7 (1=0, 8=7)

impl Square {
    fn new(file: usize, rank: usize) -> Option<Self>;
    fn new_unchecked(file: usize, rank: usize) -> Self;
    fn from_algebraic(s: &str) -> Option<Self>;  // "e4" → Square
    fn to_algebraic(self) -> String;             // Square → "e4"
    fn offset(self, df: isize, dr: isize) -> Option<Self>;
}

// FromStr — permite "e4".parse::<Square>()
impl FromStr for Square;
```

### `PieceType`

```rust
enum PieceType { King, Queen, Rook, Bishop, Knight, Pawn }
impl PieceType {
    fn from_char(c: char) -> Option<Self>;  // 'k'/'K' → King
    fn to_char(self) -> char;               // King → 'k'
    fn to_unicode(self, color: Color) -> &'static str;       // ♔♕♖♗♘♙♚♛♜♝♞♟
    fn to_unicode_square(self, color: Color, is_light: bool) -> &'static str;  // glifo baseado na casa
}
```

### `Piece`

```rust
struct Piece { pub kind: PieceType, pub color: Color }
impl Piece {
    fn new(kind: PieceType, color: Color) -> Self;
    fn from_fen_char(c: char) -> Option<Self>; // 'P' → White Pawn
    fn to_fen_char(self) -> char;               // White Pawn → 'P'
}

impl fmt::Display for Piece;  // mostra glifo Unicode
```

### `Move`

```rust
struct Move { pub from: Square, pub to: Square, pub promotion: Option<PieceType> }
impl Move {
    fn new(from: Square, to: Square) -> Self;
    fn new_promotion(from: Square, to: Square, promotion: PieceType) -> Self;
    fn to_coordinate(&self) -> String;  // "e2e4" ou "e7e8=Q"
}
```

### `Board`

```rust
struct Board { /* campos privados */ }
impl Board {
    fn empty() -> Self;
    fn initial() -> Self;          // Posição inicial
    fn from_fen(placement: &str) -> Result<Self, String>;
    fn piece_at(&self, sq: Square) -> Option<Piece>;
    fn set_piece(&mut self, sq: Square, piece: Option<Piece>);
    fn squares(&self) -> &[[Option<Piece>; 8]; 8];
    fn king_square(&self, color: Color) -> Option<Square>;
    fn to_fen(&self) -> String;
}
```

### `CastlingRights`

```rust
struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}
impl CastlingRights {
    fn all() -> Self;
    fn none() -> Self;
}
```

### `GameStatus`

```rust
enum GameStatus { Ongoing, WhiteWins, BlackWins, Draw }
```

### `Game`

```rust
struct Game { /* campos privados */ }
impl Game {
    fn new() -> Self;                                   // Posição inicial
    fn from_fen(fen: &str) -> Result<Self, String>;     // Posição customizada
    fn make_move(&mut self, mv: Move) -> Result<(), String>;
    fn legal_moves(&self) -> Vec<Move>;
    fn status(&self) -> GameStatus;
    fn board(&self) -> &Board;
    fn turn(&self) -> Color;
    fn in_check(&self) -> bool;
    fn to_fen(&self) -> String;
    fn castling_rights(&self) -> &CastlingRights;
    fn ep_target(&self) -> Option<Square>;
    fn halfmove_clock(&self) -> u8;
    fn fullmove_number(&self) -> u16;
}
```

### `Game` — métodos adicionais

```rust
impl Game {
    fn undo(&mut self) -> bool;
    fn move_history(&self) -> &[Move];
}
```

### `moves` module

```rust
fn legal_moves(board: &Board, color: Color, ep_target: Option<Square>,
               castling: &CastlingRights) -> Vec<Move>;
fn is_square_attacked(board: &Board, square: Square, by_color: Color) -> bool;
fn perft(board: &Board, depth: u32, color: Color, ep_target: Option<Square>,
         castling: &CastlingRights) -> u64;
```

### `notation` module

```rust
fn move_to_algebraic(game: &Game, mv: &Move) -> String;
fn parse_algebraic(game: &Game, input: &str) -> Option<Move>;
fn disambiguation(game: &Game, mv: &Move, piece: Piece) -> String;
```

## Exemplo Completo de Integração

```rust
use chess::*;

fn main() {
    let mut game = Game::new();

    // Constantes ANSI para cores (mesmo esquema de main.rs)
    const BG_LIGHT: &str = "\x1b[107m";
    const BG_DARK: &str = "\x1b[40m";
    const FG_DARK: &str = "\x1b[30m";
    const FG_LIGHT: &str = "\x1b[97m";
    const RESET: &str = "\x1b[0m";

    // Loop de jogo (substitua por seu próprio render/input)
    loop {
        // Render
        let board = game.board();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let sq = Square::new_unchecked(file, rank);
                let is_light = (rank + file) % 2 == 0;
                let bg = if is_light { BG_LIGHT } else { BG_DARK };
                let fg = if is_light { FG_DARK } else { FG_LIGHT };
                if let Some(p) = board.piece_at(sq) {
                    print!("{}{}{}{}", bg, fg, p.kind.to_unicode_square(p.color, is_light), RESET);
                } else {
                    print!("{} {}", bg, RESET);
                }
            }
            println!();
        }

        // Mostrar status
        match game.status() {
            GameStatus::Ongoing => {},
            GameStatus::WhiteWins => { println!("White wins!"); break; },
            GameStatus::BlackWins => { println!("Black wins!"); break; },
            GameStatus::Draw => { println!("Draw!"); break; },
        }

        // Input + fazer jogada
        // let mv = ... sua UI ...
        // game.make_move(mv).unwrap();
    }
}
```
