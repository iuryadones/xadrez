use crate::board::Board;
use crate::game::Game;
use crate::mv::Move;
use crate::piece::{Color, PieceType};
use crate::square::Square;
use std::time::{SystemTime, UNIX_EPOCH};

const INF: i32 = 1_000_000_000;

const PIECE_VALUES: [i32; 6] = [20000, 900, 500, 330, 320, 100];

fn piece_value(kind: PieceType) -> i32 {
    PIECE_VALUES[kind as usize]
}

struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = z.wrapping_mul(0xbf58476d1ce4e5b9);
        z ^= z >> 30;
        z = z.wrapping_mul(0x94d049bb133111eb);
        z ^= z >> 27;
        z
    }
}

const PST_PAWN: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    5,  5, 10, 25, 25, 10,  5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5, -5,-10,  0,  0,-10, -5,  5,
    5, 10, 10,-20,-20, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0,
];

const PST_KNIGHT: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

const PST_BISHOP: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

const PST_ROOK: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0,
];

const PST_QUEEN: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20,
];

const PST_KING: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20,
];

const PST: [&[i32; 64]; 6] = [
    &PST_KING, &PST_QUEEN, &PST_ROOK, &PST_BISHOP, &PST_KNIGHT, &PST_PAWN,
];

use std::sync::OnceLock;

struct Zobrist {
    pieces: [[[u64; 2]; 64]; 6],
    side: u64,
    castling: [u64; 4],
    ep: [u64; 8],
}

static ZOBRIST: OnceLock<Zobrist> = OnceLock::new();

fn zobrist() -> &'static Zobrist {
    ZOBRIST.get_or_init(|| {
        let mut rng = SplitMix64::new(42);
        let mut rand_u64 = || rng.next();
        Zobrist {
            pieces: [[[rand_u64(); 2]; 64]; 6],
            side: rand_u64(),
            castling: [rand_u64(), rand_u64(), rand_u64(), rand_u64()],
            ep: [rand_u64(), rand_u64(), rand_u64(), rand_u64(), rand_u64(), rand_u64(), rand_u64(), rand_u64()],
        }
    })
}

fn compute_hash(game: &Game) -> u64 {
    let zb = zobrist();
    let mut h = 0u64;
    for rank in 0..8 {
        for file in 0..8 {
            let sq = Square::new_unchecked(file, rank);
            if let Some(piece) = game.board().piece_at(sq) {
                let idx = rank * 8 + file;
                h ^= zb.pieces[piece.kind as usize][idx][piece.color as usize];
            }
        }
    }
    if game.turn() == Color::Black {
        h ^= zb.side;
    }
    let cr = game.castling_rights();
    if cr.white_kingside { h ^= zb.castling[0]; }
    if cr.white_queenside { h ^= zb.castling[1]; }
    if cr.black_kingside { h ^= zb.castling[2]; }
    if cr.black_queenside { h ^= zb.castling[3]; }
    if let Some(ep) = game.ep_target() {
        h ^= zb.ep[ep.file];
    }
    h
}

fn pst_bonus(kind: PieceType, sq: Square, color: Color) -> i32 {
    let table = PST[kind as usize];
    let idx = match color {
        Color::White => sq.rank * 8 + sq.file,
        Color::Black => (7 - sq.rank) * 8 + (7 - sq.file),
    };
    table[idx]
}

fn evaluate_board(board: &Board, color: Color) -> i32 {
    let mut score = 0;
    for rank in 0..8 {
        for file in 0..8 {
            let sq = Square::new_unchecked(file, rank);
            if let Some(piece) = board.piece_at(sq) {
                let val = piece_value(piece.kind)
                    + pst_bonus(piece.kind, sq, piece.color);
                if piece.color == color {
                    score += val;
                } else {
                    score -= val;
                }
            }
        }
    }
    score
}

fn evaluate(game: &Game) -> i32 {
    evaluate_board(game.board(), game.turn())
        - evaluate_board(game.board(), game.turn().opponent())
}

use std::cell::RefCell;

#[derive(Clone, Copy, PartialEq, Eq)]
enum TTFlag {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Clone, Copy)]
struct TTEntry {
    hash: u64,
    depth: u32,
    score: i32,
    flag: TTFlag,
    best_move: Option<Move>,
}

struct TranspositionTable {
    entries: Box<[Option<TTEntry>]>,
    mask: usize,
}

impl TranspositionTable {
    fn new() -> Self {
        let size = 1 << 20;
        Self {
            entries: vec![None; size].into_boxed_slice(),
            mask: size - 1,
        }
    }
    fn index(&self, hash: u64) -> usize {
        (hash as usize) & self.mask
    }
    fn probe(&self, hash: u64, depth: u32, alpha: i32, beta: i32) -> (Option<(i32, TTFlag)>, Option<Move>) {
        match self.entries[self.index(hash)].as_ref() {
            Some(entry) if entry.hash == hash => {
                let best = entry.best_move;
                if entry.depth >= depth {
                    match entry.flag {
                        TTFlag::Exact => (Some((entry.score, TTFlag::Exact)), best),
                        TTFlag::LowerBound if entry.score >= beta => (Some((entry.score, TTFlag::LowerBound)), best),
                        TTFlag::UpperBound if entry.score <= alpha => (Some((entry.score, TTFlag::UpperBound)), best),
                        _ => (None, best),
                    }
                } else {
                    (None, best)
                }
            }
            _ => (None, None),
        }
    }
    fn record(&mut self, hash: u64, depth: u32, score: i32, flag: TTFlag, best_move: Option<Move>) {
        let idx = self.index(hash);
        self.entries[idx] = Some(TTEntry { hash, depth, score, flag, best_move });
    }
    #[allow(dead_code)]
    fn clear(&mut self) {
        for e in self.entries.iter_mut() {
            *e = None;
        }
    }
}

thread_local! {
    static TT: RefCell<TranspositionTable> = RefCell::new(TranspositionTable::new());
}

fn is_capture(game: &Game, mv: &Move) -> bool {
    game.board().piece_at(mv.to).is_some()
}

fn mvv_lva_score(game: &Game, mv: &Move) -> i32 {
    if let Some(victim) = game.board().piece_at(mv.to) {
        let attacker = game.board().piece_at(mv.from);
        let attacker_val = match attacker {
            Some(p) => piece_value(p.kind),
            None => 0,
        };
        piece_value(victim.kind) * 10 - attacker_val
    } else {
        0
    }
}

fn order_moves(game: &Game, moves: &mut [Move]) {
    let mut scored: Vec<(i32, Move)> = moves
        .iter()
        .map(|mv| {
            let mut score = 0;
            if is_capture(game, mv) {
                score += mvv_lva_score(game, mv) + 10_000;
            }
            if let Some(promotion) = mv.promotion {
                score += piece_value(promotion) + 5_000;
            }
            if game.in_check() {
                score += 1_000;
            }
            (score, *mv)
        })
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    for (i, (_, mv)) in scored.iter().enumerate() {
        moves[i] = *mv;
    }
}

fn quiescence(game: &Game, alpha: i32, beta: i32, color: Color) -> i32 {
    let stand_pat = evaluate(game) * if color == game.turn() { 1 } else { -1 };
    if stand_pat >= beta {
        return beta;
    }
    let mut alpha = alpha;
    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let mut captures: Vec<Move> = game
        .legal_moves()
        .into_iter()
        .filter(|mv| is_capture(game, mv) || mv.promotion.is_some())
        .collect();

    if captures.is_empty() {
        return alpha;
    }

    order_moves(game, &mut captures);

    for mv in captures {
        let mut g = game.clone();
        if g.make_move(mv).is_err() {
            continue;
        }
        let score = -quiescence(&g, -beta, -alpha, color.opponent());
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }
    alpha
}

fn negamax(
    tt: &mut TranspositionTable,
    game: &Game,
    depth: u32,
    alpha: i32,
    beta: i32,
    color: Color,
) -> i32 {
    let hash = compute_hash(game);
    let (tt_result, tt_move) = tt.probe(hash, depth, alpha, beta);
    if let Some((score, _flag)) = tt_result {
        return score;
    }

    if depth == 0 {
        let score = quiescence(game, alpha, beta, color);
        tt.record(hash, 0, score, TTFlag::Exact, None);
        return score;
    }

    if game.status() != crate::GameStatus::Ongoing {
        let score = match game.status() {
            crate::GameStatus::WhiteWins => {
                if color == Color::White { INF - 1 } else { -INF + 1 }
            }
            crate::GameStatus::BlackWins => {
                if color == Color::Black { INF - 1 } else { -INF + 1 }
            }
            _ => 0,
        };
        tt.record(hash, depth, score, TTFlag::Exact, None);
        return score;
    }

    let mut moves = game.legal_moves();
    if moves.is_empty() {
        let score = if game.in_check() {
            -INF + 2
        } else {
            0
        };
        tt.record(hash, depth, score, TTFlag::Exact, None);
        return score;
    }

    if let Some(bm) = tt_move {
        if let Some(pos) = moves.iter().position(|m| *m == bm) {
            moves.swap(0, pos);
        }
    }

    order_moves(game, &mut moves);

    let mut alpha = alpha;
    let original_alpha = alpha;
    let mut best_move: Option<Move> = None;
    let mut searched = 0;

    for mv in moves {
        let mut g = game.clone();
        if g.make_move(mv).is_err() {
            continue;
        }
        let score;
        if searched == 0 {
            score = -negamax(tt, &g, depth - 1, -beta, -alpha, color.opponent());
        } else {
            let mut s = -negamax(tt, &g, depth - 1, -alpha - 1, -alpha, color.opponent());
            if s > alpha && s < beta {
                s = -negamax(tt, &g, depth - 1, -beta, -alpha, color.opponent());
            }
            score = s;
        }
        searched += 1;
        if score >= beta {
            tt.record(hash, depth, beta, TTFlag::LowerBound, best_move);
            return beta;
        }
        if score > alpha {
            alpha = score;
            best_move = Some(mv);
        }
    }

    let flag = if alpha <= original_alpha {
        TTFlag::UpperBound
    } else {
        TTFlag::Exact
    };
    tt.record(hash, depth, alpha, flag, best_move);
    alpha
}

fn iterative_deepening(tt: &mut TranspositionTable, game: &Game, max_depth: u32) -> Option<Move> {
    let mut best_move: Option<Move> = None;
    let mut _best_score: i32 = -INF + 1;
    let color = game.turn();

    for depth in 1..=max_depth {
        let mut moves = game.legal_moves();
        if moves.is_empty() {
            break;
        }

        let hash = compute_hash(game);
        let (_tt_result, tt_move) = tt.probe(hash, 0, -INF, INF);
        if let Some(bm) = tt_move.or(best_move) {
            if let Some(pos) = moves.iter().position(|m| *m == bm) {
                moves.swap(0, pos);
            }
        }

        order_moves(game, &mut moves);

        let mut current_best: Option<Move> = None;
        let mut current_score: i32 = -INF + 1;
        let mut alpha = -INF;
        let beta = INF;

        for mv in moves {
            let mut g = game.clone();
            if g.make_move(mv).is_err() {
                continue;
            }
            let score = -negamax(tt, &g, depth - 1, -beta, -alpha, color.opponent());
            if score > current_score {
                current_score = score;
                current_best = Some(mv);
            }
            if score > alpha {
                alpha = score;
            }
        }

        if let Some(mv) = current_best {
            best_move = Some(mv);
            _best_score = current_score;
        }
    }

    best_move
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Difficulty {
    Easy,
    #[default]
    Medium,
    Hard,
}

impl Difficulty {
    pub fn depth(&self) -> u32 {
        match self {
            Difficulty::Easy => 2,
            Difficulty::Medium => 4,
            Difficulty::Hard => 6,
        }
    }
}

pub fn best_move(game: &Game) -> Option<Move> {
    TT.with(|tt| iterative_deepening(&mut tt.borrow_mut(), game, Difficulty::default().depth()))
}

pub fn best_move_with_depth(game: &Game, max_depth: u32) -> Option<Move> {
    TT.with(|tt| iterative_deepening(&mut tt.borrow_mut(), game, max_depth))
}

pub fn coin_flip() -> Color {
    if nanos() % 2 == 0 {
        Color::White
    } else {
        Color::Black
    }
}

fn nanos() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0)
}

pub fn random_difficulty() -> Difficulty {
    match nanos() % 3 {
        0 => Difficulty::Easy,
        1 => Difficulty::Medium,
        _ => Difficulty::Hard,
    }
}

pub fn color_name(color: Color) -> &'static str {
    match color {
        Color::White => "Brancas",
        Color::Black => "Pretas",
    }
}

pub fn king_symbol(color: Color) -> &'static str {
    match color {
        Color::White => "♔",
        Color::Black => "♚",
    }
}
