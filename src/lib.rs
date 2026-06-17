#![deny(warnings)]
#![deny(clippy::all)]

pub mod ai;
pub mod board;
pub mod fen;
pub mod game;
pub mod moves;
pub mod mv;
pub mod notation;
pub mod piece;
pub mod square;

pub use board::Board;
pub use game::{CastlingRights, Game, GameStatus};
pub use mv::Move;
pub use notation::{disambiguation, move_to_algebraic, parse_algebraic};
pub use piece::{Color, Piece, PieceType};
pub use square::Square;
