use crate::board::Board;
use crate::fen;
use crate::moves;
use crate::mv::Move;
use crate::piece::{Color, Piece, PieceType};
use crate::square::Square;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameStatus {
    Ongoing,
    WhiteWins,
    BlackWins,
    Draw,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastlingRights {
    pub fn all() -> Self {
        Self {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }
    pub fn none() -> Self {
        Self {
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Snapshot {
    board: Board,
    turn: Color,
    castling: CastlingRights,
    ep_target: Option<Square>,
    halfmove_clock: u8,
    fullmove_number: u16,
    position_history: Vec<String>,
    move_history: Vec<Move>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    board: Board,
    turn: Color,
    castling: CastlingRights,
    ep_target: Option<Square>,
    halfmove_clock: u8,
    fullmove_number: u16,
    position_history: Vec<String>,
    move_history: Vec<Move>,
    undo_stack: Vec<Snapshot>,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            board: Board::initial(),
            turn: Color::White,
            castling: CastlingRights::all(),
            ep_target: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            position_history: Vec::new(),
            move_history: Vec::new(),
            undo_stack: Vec::new(),
        };
        game.position_history.push(game.position_key());
        game
    }

    pub fn from_fen(fen_str: &str) -> Result<Self, String> {
        let (board, turn, castling, ep_target, halfmove, fullmove) = fen::parse_fen(fen_str)?;
        let mut game = Self {
            board,
            turn,
            castling,
            ep_target,
            halfmove_clock: halfmove,
            fullmove_number: fullmove,
            position_history: Vec::new(),
            move_history: Vec::new(),
            undo_stack: Vec::new(),
        };
        game.position_history.push(game.position_key());
        Ok(game)
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn turn(&self) -> Color {
        self.turn
    }

    pub fn castling_rights(&self) -> &CastlingRights {
        &self.castling
    }

    pub fn ep_target(&self) -> Option<Square> {
        self.ep_target
    }

    pub fn halfmove_clock(&self) -> u8 {
        self.halfmove_clock
    }

    pub fn fullmove_number(&self) -> u16 {
        self.fullmove_number
    }

    pub fn move_history(&self) -> &[Move] {
        &self.move_history
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        moves::legal_moves(&self.board, self.turn, self.ep_target, &self.castling)
    }

    pub fn in_check(&self) -> bool {
        let king_sq = match self.board.king_square(self.turn) {
            Some(sq) => sq,
            None => return false,
        };
        moves::is_square_attacked(&self.board, king_sq, self.turn.opponent())
    }

    pub fn make_move(&mut self, mv: Move) -> Result<(), String> {
        let legal = self.legal_moves();
        if !legal.contains(&mv) {
            return Err("Jogada ilegal".into());
        }

        self.undo_stack.push(Snapshot {
            board: self.board.clone(),
            turn: self.turn,
            castling: self.castling,
            ep_target: self.ep_target,
            halfmove_clock: self.halfmove_clock,
            fullmove_number: self.fullmove_number,
            position_history: self.position_history.clone(),
            move_history: self.move_history.clone(),
        });

        let piece = self
            .board
            .piece_at(mv.from)
            .ok_or("Nenhuma peca na origem")?;

        let captured = self.board.piece_at(mv.to);

        let is_ep = (self.ep_target == Some(mv.to))
            && piece.kind == PieceType::Pawn
            && mv.from.file != mv.to.file;

        if is_ep && captured.is_some() {
            return Err("En passant: quadrado destino deve estar vazio".into());
        }

        self.move_history.push(mv);

        self.board.set_piece(mv.to, Some(piece));
        self.board.set_piece(mv.from, None);

        if is_ep {
            let captured_sq = Square::new_unchecked(mv.to.file, mv.from.rank);
            self.board.set_piece(captured_sq, None);
        }

        let is_castle = piece.kind == PieceType::King
            && (mv.to.file as isize - mv.from.file as isize).abs() == 2;

        if is_castle {
            let (rook_from_file, rook_to_file) = if mv.to.file > mv.from.file {
                (7, 5)
            } else {
                (0, 3)
            };
            let rank = mv.from.rank;
            let rook_from = Square::new_unchecked(rook_from_file, rank);
            let rook_to = Square::new_unchecked(rook_to_file, rank);
            let rook = self.board.piece_at(rook_from);
            self.board.set_piece(rook_to, rook);
            self.board.set_piece(rook_from, None);
        }

        if let Some(promotion) = mv.promotion {
            self.board
                .set_piece(mv.to, Some(Piece::new(promotion, self.turn)));
        }

        self.ep_target = if piece.kind == PieceType::Pawn {
            let rank_diff = (mv.to.rank as isize - mv.from.rank as isize).abs();
            if rank_diff == 2 {
                let mid_rank = (mv.from.rank + mv.to.rank) / 2;
                Some(Square::new_unchecked(mv.from.file, mid_rank))
            } else {
                None
            }
        } else {
            None
        };

        if piece.kind == PieceType::King {
            match self.turn {
                Color::White => {
                    self.castling.white_kingside = false;
                    self.castling.white_queenside = false;
                }
                Color::Black => {
                    self.castling.black_kingside = false;
                    self.castling.black_queenside = false;
                }
            }
        }

        if piece.kind == PieceType::Rook {
            match (self.turn, mv.from.file) {
                (Color::White, 0) => self.castling.white_queenside = false,
                (Color::White, 7) => self.castling.white_kingside = false,
                (Color::Black, 0) => self.castling.black_queenside = false,
                (Color::Black, 7) => self.castling.black_kingside = false,
                _ => {}
            }
        }

        if mv.to.file == 0 && mv.to.rank == 0 {
            self.castling.white_queenside = false;
        }
        if mv.to.file == 7 && mv.to.rank == 0 {
            self.castling.white_kingside = false;
        }
        if mv.to.file == 0 && mv.to.rank == 7 {
            self.castling.black_queenside = false;
        }
        if mv.to.file == 7 && mv.to.rank == 7 {
            self.castling.black_kingside = false;
        }

        if piece.kind == PieceType::Pawn || captured.is_some() || is_ep {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock = self.halfmove_clock.saturating_add(1);
        }

        if self.turn == Color::Black {
            self.fullmove_number = self.fullmove_number.saturating_add(1);
        }

        self.turn = self.turn.opponent();

        self.position_history.push(self.position_key());

        Ok(())
    }

    pub fn undo(&mut self) -> bool {
        let snap = match self.undo_stack.pop() {
            Some(s) => s,
            None => return false,
        };
        self.board = snap.board;
        self.turn = snap.turn;
        self.castling = snap.castling;
        self.ep_target = snap.ep_target;
        self.halfmove_clock = snap.halfmove_clock;
        self.fullmove_number = snap.fullmove_number;
        self.position_history = snap.position_history;
        self.move_history = snap.move_history;
        true
    }

    pub fn status(&self) -> GameStatus {
        let moves = self.legal_moves();
        let king_sq = match self.board.king_square(self.turn) {
            Some(sq) => sq,
            None => {
                return match self.turn {
                    Color::White => GameStatus::BlackWins,
                    Color::Black => GameStatus::WhiteWins,
                };
            }
        };
        let in_check = moves::is_square_attacked(&self.board, king_sq, self.turn.opponent());

        if moves.is_empty() {
            if in_check {
                match self.turn {
                    Color::White => return GameStatus::BlackWins,
                    Color::Black => return GameStatus::WhiteWins,
                }
            } else {
                return GameStatus::Draw;
            }
        }

        if self.halfmove_clock >= 150 {
            return GameStatus::Draw;
        }

        if self.halfmove_clock >= 100 {
            return GameStatus::Draw;
        }

        if self.is_fivefold_repetition() {
            return GameStatus::Draw;
        }

        if self.is_threefold_repetition() {
            return GameStatus::Draw;
        }

        if self.is_insufficient_material() {
            return GameStatus::Draw;
        }

        GameStatus::Ongoing
    }

    pub fn to_fen(&self) -> String {
        fen::to_fen(
            &self.board,
            self.turn,
            &self.castling,
            self.ep_target,
            self.halfmove_clock,
            self.fullmove_number,
        )
    }

    fn is_insufficient_material(&self) -> bool {
        let pieces: Vec<Piece> = self
            .board
            .squares()
            .iter()
            .flatten()
            .filter_map(|&p| p)
            .collect();

        if pieces.len() == 2 {
            return true;
        }

        if pieces.len() == 3 {
            let non_king = pieces.iter().find(|p| p.kind != PieceType::King);
            if let Some(p) = non_king {
                if p.kind == PieceType::Bishop || p.kind == PieceType::Knight {
                    return true;
                }
            }
        }

        if pieces.len() == 4 {
            let bishops: Vec<&Piece> = pieces
                .iter()
                .filter(|p| p.kind == PieceType::Bishop)
                .collect();
            if bishops.len() == 2 {
                let same_color = bishops
                    .iter()
                    .all(|b| (b.color == Color::White) == (bishops[0].color == Color::White));
                if !same_color {
                    return true;
                }
            }
        }

        false
    }

    fn is_fivefold_repetition(&self) -> bool {
        if self.position_history.len() < 2 {
            return false;
        }

        let current = self.position_key();
        let count = self
            .position_history
            .iter()
            .filter(|&k| *k == current)
            .count();

        count >= 5
    }

    fn is_threefold_repetition(&self) -> bool {
        if self.position_history.len() < 2 {
            return false;
        }

        let current = self.position_key();
        let count = self
            .position_history
            .iter()
            .filter(|&k| *k == current)
            .count();

        count >= 3
    }

    fn position_key(&self) -> String {
        format!(
            "{}-{}-{}-{}",
            self.board.to_fen(),
            if self.turn == Color::White { 'w' } else { 'b' },
            castling_to_string(&self.castling),
            self.ep_target
                .map_or("-".to_string(), |sq| sq.to_algebraic())
        )
    }
}

fn castling_to_string(castling: &CastlingRights) -> String {
    let mut s = String::new();
    if castling.white_kingside {
        s.push('K');
    }
    if castling.white_queenside {
        s.push('Q');
    }
    if castling.black_kingside {
        s.push('k');
    }
    if castling.black_queenside {
        s.push('q');
    }
    if s.is_empty() {
        s.push('-');
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Square;

    #[test]
    fn test_initial_game() {
        let game = Game::new();
        assert_eq!(game.turn, Color::White);
        assert_eq!(game.legal_moves().len(), 20);
        assert_eq!(game.status(), GameStatus::Ongoing);
    }

    #[test]
    fn test_make_move_e4() {
        let mut game = Game::new();
        let e2 = Square::from_algebraic("e2").unwrap();
        let e4 = Square::from_algebraic("e4").unwrap();
        let mv = Move::new(e2, e4);
        assert!(game.make_move(mv).is_ok());
        assert_eq!(game.turn, Color::Black);
        assert_eq!(game.legal_moves().len(), 20);
    }

    #[test]
    fn test_king_in_check() {
        let game =
            Game::from_fen("rnb1kbnr/pppppppp/8/8/8/5q2/PPPPPPPP/RNB1KBNR w KQkq - 0 1").unwrap();
        let king = game.board.king_square(Color::White).unwrap();
        assert!(!moves::is_square_attacked(game.board(), king, Color::Black));
    }

    #[test]
    fn test_ep_target_after_double_push() {
        let mut game = Game::new();
        game.make_move(Move::new(
            Square::from_algebraic("e2").unwrap(),
            Square::from_algebraic("e4").unwrap(),
        ))
        .unwrap();
        assert_eq!(game.ep_target, Square::from_algebraic("e3"));
    }

    #[test]
    fn test_castling() {
        let fen = "r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 4 5";
        let mut game = Game::from_fen(fen).unwrap();
        let e1 = Square::from_algebraic("e1").unwrap();
        let g1 = Square::from_algebraic("g1").unwrap();
        let mv = Move::new(e1, g1);
        assert!(game.make_move(mv).is_ok());
        assert_eq!(
            game.board.piece_at(Square::from_algebraic("f1").unwrap()),
            Some(Piece::new(PieceType::Rook, Color::White))
        );
    }

    #[test]
    fn test_stalemate() {
        let fen = "k7/8/1Q6/8/8/8/8/7K b - - 0 1";
        let game = Game::from_fen(fen).unwrap();
        assert_eq!(game.legal_moves().len(), 0);
        assert!(!game.in_check());
        assert_eq!(game.status(), GameStatus::Draw);
    }

    #[test]
    fn test_fifty_move_rule() {
        let fen = "k7/8/8/8/4r3/8/8/K7 w - - 99 50";
        let game = Game::from_fen(fen).unwrap();
        assert_eq!(game.status(), GameStatus::Ongoing);

        let ka1 = Square::from_algebraic("a1").unwrap();
        let kb1 = Square::from_algebraic("b1").unwrap();

        let mut game = game.clone();
        game.make_move(Move::new(ka1, kb1)).unwrap();
        assert_eq!(game.status(), GameStatus::Draw);
    }

    #[test]
    fn test_en_passant_capture() {
        let mut game = Game::new();
        game.make_move(Move::new(Square::from_algebraic("e2").unwrap(), "e4".parse().unwrap())).unwrap();
        game.make_move(Move::new(Square::from_algebraic("d7").unwrap(), "d5".parse().unwrap())).unwrap();
        game.make_move(Move::new(Square::from_algebraic("e4").unwrap(), "e5".parse().unwrap())).unwrap();
        game.make_move(Move::new(Square::from_algebraic("f7").unwrap(), "f5".parse().unwrap())).unwrap();
        let ep_move = Move::new(Square::from_algebraic("e5").unwrap(), "f6".parse().unwrap());
        assert!(game.legal_moves().contains(&ep_move));
        game.make_move(ep_move).unwrap();
        assert!(game.board.piece_at(Square::from_algebraic("f6").unwrap()).is_some());
        assert!(game.board.piece_at(Square::from_algebraic("f5").unwrap()).is_none());
        assert!(game.board.piece_at(Square::from_algebraic("e5").unwrap()).is_none());
    }

    #[test]
    fn test_en_passant_expiry() {
        let mut game = Game::new();
        game.make_move(Move::new("e2".parse().unwrap(), "e4".parse().unwrap())).unwrap();
        assert_eq!(game.ep_target, Square::from_algebraic("e3"));
        game.make_move(Move::new("d7".parse().unwrap(), "d6".parse().unwrap())).unwrap();
        assert!(game.ep_target.is_none());
    }

    #[test]
    fn test_queenside_castling() {
        let fen = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";
        let mut game = Game::from_fen(fen).unwrap();
        let e1 = Square::from_algebraic("e1").unwrap();
        let c1 = Square::from_algebraic("c1").unwrap();
        let mv = Move::new(e1, c1);
        assert!(game.legal_moves().contains(&mv));
        game.make_move(mv).unwrap();
        assert_eq!(
            game.board.piece_at(Square::from_algebraic("d1").unwrap()),
            Some(Piece::new(PieceType::Rook, Color::White))
        );
        assert_eq!(game.board.piece_at(c1), Some(Piece::new(PieceType::King, Color::White)));
        assert!(game.board.piece_at(Square::from_algebraic("a1").unwrap()).is_none());
    }

    #[test]
    fn test_castling_blocked() {
        let fen = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let game = Game::from_fen(fen).unwrap();
        let e1 = Square::from_algebraic("e1").unwrap();
        let g1 = Square::from_algebraic("g1").unwrap();
        let kingside = Move::new(e1, g1);
        assert!(!game.legal_moves().contains(&kingside));
    }

    #[test]
    fn test_castling_through_check() {
        let fen = "r3k2r/pppppppp/8/8/8/5R2/PPPPPPPP/4K3 b KQkq - 0 1";
        let mut game = Game::from_fen(fen).unwrap();
        game.make_move(Move::new(Square::from_algebraic("e8").unwrap(), "d8".parse().unwrap())).unwrap();
        let e1 = Square::from_algebraic("e1").unwrap();
        let g1 = Square::from_algebraic("g1").unwrap();
        assert!(!game.legal_moves().contains(&Move::new(e1, g1)));
    }

    #[test]
    fn test_pawn_promotion() {
        let fen = "4k3/P7/8/8/8/8/8/4K3 w - - 0 1";
        let mut game = Game::from_fen(fen).unwrap();
        let from = Square::from_algebraic("a7").unwrap();
        let to = Square::from_algebraic("a8").unwrap();
        let queen_mv = Move::new_promotion(from, to, PieceType::Queen);
        assert!(game.legal_moves().contains(&queen_mv));
        game.make_move(queen_mv).unwrap();
        let piece = game.board.piece_at(to).unwrap();
        assert_eq!(piece.kind, PieceType::Queen);
        assert_eq!(piece.color, Color::White);
    }

    #[test]
    fn test_underpromotion() {
        let fen = "4k3/P7/8/8/8/8/8/4K3 w - - 0 1";
        let mut game = Game::from_fen(fen).unwrap();
        let from = Square::from_algebraic("a7").unwrap();
        let to = Square::from_algebraic("a8").unwrap();
        let knight_mv = Move::new_promotion(from, to, PieceType::Knight);
        assert!(game.legal_moves().contains(&knight_mv));
        game.make_move(knight_mv).unwrap();
        let piece = game.board.piece_at(to).unwrap();
        assert_eq!(piece.kind, PieceType::Knight);
    }

    #[test]
    fn test_undo_move() {
        let mut game = Game::new();
        let initial = game.clone();
        let e4 = Move::new(Square::from_algebraic("e2").unwrap(), Square::from_algebraic("e4").unwrap());
        game.make_move(e4).unwrap();
        assert_ne!(game, initial);
        assert!(game.undo());
        assert_eq!(game, initial);
    }

    #[test]
    fn test_undo_twice() {
        let mut game = Game::new();
        let initial = game.clone();
        let e4 = Move::new("e2".parse().unwrap(), "e4".parse().unwrap());
        let d5 = Move::new("d7".parse().unwrap(), "d5".parse().unwrap());
        game.make_move(e4).unwrap();
        game.make_move(d5).unwrap();
        game.undo();
        game.undo();
        assert_eq!(game, initial);
    }

    #[test]
    fn test_undo_empty() {
        let mut game = Game::new();
        assert!(!game.undo());
    }

    #[test]
    fn test_threefold_repetition() {
        let mut game = Game::new();
        let nf3 = Move::new("g1".parse().unwrap(), "f3".parse().unwrap());
        let nf6 = Move::new("b8".parse().unwrap(), "c6".parse().unwrap());
        let ng1 = Move::new("f3".parse().unwrap(), "g1".parse().unwrap());
        let nc6 = Move::new("c6".parse().unwrap(), "b8".parse().unwrap());

        for _ in 0..2 {
            game.make_move(nf3).unwrap();
            game.make_move(nf6).unwrap();
            game.make_move(ng1).unwrap();
            game.make_move(nc6).unwrap();
        }
        assert_eq!(game.status(), GameStatus::Draw);
    }

    #[test]
    fn test_captured_rook_loses_castling() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";
        let mut game = Game::from_fen(fen).unwrap();
        let ra1 = Square::from_algebraic("a1").unwrap();
        let ra8 = Square::from_algebraic("a8").unwrap();
        game.make_move(Move::new(ra1, ra8)).unwrap();
        assert!(!game.castling.black_queenside);
        assert!(!game.castling.white_queenside);
    }

    #[test]
    fn test_rook_move_loses_castling() {
        let fen = "r3k2r/8/8/8/8/8/8/4K2R w KQkq - 0 1";
        let mut game = Game::from_fen(fen).unwrap();
        game.make_move(Move::new(Square::from_algebraic("h1").unwrap(), Square::from_algebraic("g1").unwrap())).unwrap();
        assert!(!game.castling.white_kingside);
    }

    #[test]
    fn test_seventy_five_move_rule() {
        let fen = "k6r/8/8/8/8/8/8/K6R w - - 149 75";
        let mut game = Game::from_fen(fen).unwrap();
        let mv = Move::new("a1".parse().unwrap(), "b1".parse().unwrap());
        game.make_move(mv).unwrap();
        assert_eq!(game.status(), GameStatus::Draw);
    }

    #[test]
    fn test_seventy_five_move_rule_not_yet() {
        let fen = "k6r/8/8/8/8/8/8/K6R w - - 80 40";
        let mut game = Game::from_fen(fen).unwrap();
        let mv = Move::new("a1".parse().unwrap(), "b1".parse().unwrap());
        game.make_move(mv).unwrap();
        assert_eq!(game.status(), GameStatus::Ongoing);
    }

    #[test]
    fn test_fivefold_repetition() {
        let mut game = Game::new();
        let nf3 = Move::new("g1".parse().unwrap(), "f3".parse().unwrap());
        let nc6 = Move::new("b8".parse().unwrap(), "c6".parse().unwrap());
        let ng1 = Move::new("f3".parse().unwrap(), "g1".parse().unwrap());
        let nb8 = Move::new("c6".parse().unwrap(), "b8".parse().unwrap());

        for _ in 0..4 {
            game.make_move(nf3).unwrap();
            game.make_move(nc6).unwrap();
            game.make_move(ng1).unwrap();
            game.make_move(nb8).unwrap();
        }
        assert_eq!(game.status(), GameStatus::Draw);
    }
}
