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
```

### `PieceType`

```rust
enum PieceType { King, Queen, Rook, Bishop, Knight, Pawn }
impl PieceType {
    fn from_char(c: char) -> Option<Self>;  // 'k'/'K' → King
    fn to_unicode(self, color: Color) -> &'static str;  // ♔♕♖♗♘♙♚♛♜♝♞♟
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
```

### `Move`

```rust
struct Move { pub from: Square, pub to: Square, pub promotion: Option<PieceType> }
impl Move {
    fn new(from: Square, to: Square) -> Self;
    fn new_promotion(from: Square, to: Square, promotion: PieceType) -> Self;
    fn to_coordinate(&self) -> String;  // "e2e4" ou "e7e8q"
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

### `moves` module

```rust
fn legal_moves(board: &Board, color: Color, ep_target: Option<Square>,
               castling: &CastlingRights) -> Vec<Move>;
fn is_square_attacked(board: &Board, square: Square, by_color: Color) -> bool;
fn perft(board: &Board, depth: u32, color: Color, ep_target: Option<Square>,
         castling: &CastlingRights) -> u64;
```

## Exemplo Completo de Integração

```rust
use chess::*;

fn main() {
    let mut game = Game::new();

    // Loop de jogo (substitua por seu próprio render/input)
    loop {
        // Render
        let board = game.board();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let sq = Square::new_unchecked(file, rank);
                if let Some(p) = board.piece_at(sq) {
                    print!("{}", p.kind.to_unicode(p.color));
                } else {
                    print!(".");
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
