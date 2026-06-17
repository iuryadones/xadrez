use crate::board::Board;
use crate::game::Game;
use crate::moves;
use crate::mv::Move;
use crate::piece::{Color, Piece, PieceType};
use crate::square::Square;
use std::time::{SystemTime, UNIX_EPOCH};

const INF: i32 = 1_000_000_000;

const PIECE_VALUES: [i32; 6] = [20000, 900, 500, 330, 320, 100];

#[doc(hidden)]
pub fn piece_value(kind: PieceType) -> i32 {
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

#[doc(hidden)]
pub fn compute_hash(game: &Game) -> u64 {
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

#[allow(clippy::too_many_arguments)]
#[doc(hidden)]
pub fn update_hash(
    mut h: u64,
    mv: &Move,
    piece: Piece,
    captured: Option<Piece>,
    old_ep: Option<Square>,
    old_castling: &crate::game::CastlingRights,
    new_ep: Option<Square>,
    new_castling: &crate::game::CastlingRights,
) -> u64 {
    let zb = zobrist();
    let from_idx = mv.from.rank * 8 + mv.from.file;
    let to_idx = mv.to.rank * 8 + mv.to.file;

    h ^= zb.pieces[piece.kind as usize][from_idx][piece.color as usize];
    h ^= zb.pieces[piece.kind as usize][to_idx][piece.color as usize];

    if let Some(cap) = captured {
        h ^= zb.pieces[cap.kind as usize][to_idx][cap.color as usize];
    }

    if piece.kind == PieceType::Pawn && mv.from.file != mv.to.file && captured.is_none() {
        let ep_idx = mv.from.rank * 8 + mv.to.file;
        h ^= zb.pieces[PieceType::Pawn as usize][ep_idx][piece.color.opponent() as usize];
    }

    if piece.kind == PieceType::King && (mv.to.file as isize - mv.from.file as isize).abs() == 2 {
        let (rook_from_file, rook_to_file) = if mv.to.file > mv.from.file { (7, 5) } else { (0, 3) };
        let rank = mv.from.rank;
        let rf_idx = rank * 8 + rook_from_file;
        let rt_idx = rank * 8 + rook_to_file;
        h ^= zb.pieces[PieceType::Rook as usize][rf_idx][piece.color as usize];
        h ^= zb.pieces[PieceType::Rook as usize][rt_idx][piece.color as usize];
    }

    if let Some(pt) = mv.promotion {
        h ^= zb.pieces[PieceType::Pawn as usize][to_idx][piece.color as usize];
        h ^= zb.pieces[pt as usize][to_idx][piece.color as usize];
    }

    h ^= zb.side;

    if let Some(ep) = old_ep {
        h ^= zb.ep[ep.file];
    }
    if let Some(ep) = new_ep {
        h ^= zb.ep[ep.file];
    }

    if old_castling.white_kingside && !new_castling.white_kingside { h ^= zb.castling[0]; }
    if old_castling.white_queenside && !new_castling.white_queenside { h ^= zb.castling[1]; }
    if old_castling.black_kingside && !new_castling.black_kingside { h ^= zb.castling[2]; }
    if old_castling.black_queenside && !new_castling.black_queenside { h ^= zb.castling[3]; }

    h
}

#[doc(hidden)]
pub fn pst_bonus(kind: PieceType, sq: Square, color: Color) -> i32 {
    let table = PST[kind as usize];
    let idx = match color {
        Color::White => sq.rank * 8 + sq.file,
        Color::Black => (7 - sq.rank) * 8 + (7 - sq.file),
    };
    table[idx]
}

fn pawn_structure_score(
    own_pawns_by_file: &[i32; 8],
    own_advance: &[i32; 8],
    opp_advance: &[i32; 8],
) -> i32 {
    let mut score = 0i32;
    for file in 0..8 {
        let count = own_pawns_by_file[file];
        if count == 0 {
            continue;
        }
        if count > 1 {
            score -= 5 * (count - 1);
        }
        let left = file > 0 && own_pawns_by_file[file - 1] > 0;
        let right = file < 7 && own_pawns_by_file[file + 1] > 0;
        if !left && !right {
            score -= 10;
        }
        let adv = own_advance[file];
        if adv == 0 {
            continue;
        }
        if opp_advance[file] > adv {
            continue;
        }
        let mut passed = true;
        for &adj in &[file.wrapping_sub(1), file + 1] {
            if adj >= 8 {
                continue;
            }
            if opp_advance[adj] > adv {
                passed = false;
                break;
            }
        }
        if passed {
            score += match adv {
                0..=2 => 0,
                3 => 10,
                4 => 20,
                5 => 35,
                6 => 60,
                _ => 100,
            };
        }
    }
    score
}

fn king_safety_score(board: &Board, king_sq: Square, color: Color) -> i32 {
    let kf = king_sq.file as isize;
    let kr = king_sq.rank as isize;
    let forward: isize = match color {
        Color::White => 1,
        Color::Black => -1,
    };
    let mut score = 0i32;
    for df in -1..=1 {
        let file = kf + df;
        if !(0..=7).contains(&file) {
            continue;
        }
        let mut has_shield = false;
        for dr in 1..=3 {
            let rank = kr + dr * forward;
            if !(0..=7).contains(&rank) {
                continue;
            }
            let sq = Square::new_unchecked(file as usize, rank as usize);
            if let Some(p) = board.piece_at(sq) {
                if p.color == color && p.kind == PieceType::Pawn {
                    has_shield = true;
                    score += match dr {
                        1 => 15,
                        2 => 10,
                        _ => 5,
                    };
                }
            }
        }
        if !has_shield {
            score -= 8;
        }
    }
    score
}

fn rook_open_score(
    rook_files: &[bool; 8],
    pawns_by_file: &[[i32; 8]; 2],
    color: Color,
) -> i32 {
    let mut score = 0i32;
    for (file, &has_rook) in rook_files.iter().enumerate() {
        if !has_rook {
            continue;
        }
        let total = pawns_by_file[0][file] + pawns_by_file[1][file];
        if total == 0 {
            score += 15;
        } else if pawns_by_file[color as usize][file] == 0 {
            score += 8;
        }
    }
    score
}

fn bishop_pair_score(bishop_count: &[i32; 2], color: Color) -> i32 {
    if bishop_count[color as usize] >= 2 { 15 } else { 0 }
}

#[doc(hidden)]
pub fn evaluate(game: &Game) -> i32 {
    let board = game.board();
    let turn = game.turn();
    let opp = turn.opponent();

    let mut material_score = 0i32;
    let mut pawns_by_file = [[0i32; 8]; 2];
    let mut pawn_advance = [[0i32; 8]; 2];
    let mut bishop_count = [0i32; 2];
    let mut rook_files = [[false; 8]; 2];
    let mut king_sq = [Square::new_unchecked(4, 0), Square::new_unchecked(4, 7)];

    for rank in 0..8 {
        for file in 0..8 {
            let sq = Square::new_unchecked(file, rank);
            if let Some(piece) = board.piece_at(sq) {
                let val = piece_value(piece.kind)
                    + pst_bonus(piece.kind, sq, piece.color);
                if piece.color == turn {
                    material_score += val;
                } else {
                    material_score -= val;
                }
                let ci = piece.color as usize;
                match piece.kind {
                    PieceType::Pawn => {
                        pawns_by_file[ci][file] += 1;
                        let adv = match piece.color {
                            Color::White => rank as i32,
                            Color::Black => (7 - rank) as i32,
                        };
                        if adv > pawn_advance[ci][file] {
                            pawn_advance[ci][file] = adv;
                        }
                    }
                    PieceType::Bishop => bishop_count[ci] += 1,
                    PieceType::Rook => rook_files[ci][file] = true,
                    PieceType::King => king_sq[ci] = sq,
                    _ => {}
                }
            }
        }
    }

    let ti = turn as usize;
    let oi = opp as usize;
    let total = material_score
        + pawn_structure_score(&pawns_by_file[ti], &pawn_advance[ti], &pawn_advance[oi])
        - pawn_structure_score(&pawns_by_file[oi], &pawn_advance[oi], &pawn_advance[ti])
        + king_safety_score(board, king_sq[ti], turn)
        - king_safety_score(board, king_sq[oi], opp)
        + bishop_pair_score(&bishop_count, turn)
        - bishop_pair_score(&bishop_count, opp)
        + rook_open_score(&rook_files[ti], &pawns_by_file, turn)
        - rook_open_score(&rook_files[oi], &pawns_by_file, opp);

    total * 2
}

use std::cell::RefCell;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub enum TTFlag {
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

#[doc(hidden)]
pub struct TranspositionTable {
    entries: Box<[Option<TTEntry>]>,
    mask: usize,
}

impl TranspositionTable {
    pub fn new() -> Self {
        let size = 1 << 20;
        Self {
            entries: vec![None; size].into_boxed_slice(),
            mask: size - 1,
        }
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new()
    }
}

impl TranspositionTable {
    #[doc(hidden)]
    pub fn index(&self, hash: u64) -> usize {
        (hash as usize) & self.mask
    }
    #[doc(hidden)]
    pub fn probe(&self, hash: u64, depth: u32, alpha: i32, beta: i32) -> (Option<(i32, TTFlag)>, Option<Move>) {
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
    #[doc(hidden)]
    pub fn record(&mut self, hash: u64, depth: u32, score: i32, flag: TTFlag, best_move: Option<Move>) {
        let idx = self.index(hash);
        let replace = match &self.entries[idx] {
            Some(existing) => depth >= existing.depth,
            None => true,
        };
        if replace {
            self.entries[idx] = Some(TTEntry { hash, depth, score, flag, best_move });
        }
    }
    #[doc(hidden)]
    pub fn clear(&mut self) {
        for e in self.entries.iter_mut() {
            *e = None;
        }
    }
}

thread_local! {
    static TT: RefCell<TranspositionTable> = RefCell::new(TranspositionTable::new());
    static KILLERS: RefCell<[[Option<Move>; 2]; 64]> = const { RefCell::new([[None; 2]; 64]) };
    static HISTORY: RefCell<[[i32; 64]; 6]> = const { RefCell::new([[0; 64]; 6]) };
}

#[doc(hidden)]
pub fn order_moves(
    game: &Game,
    moves: &mut [Move],
    in_check: bool,
    killers: &[Option<Move>; 2],
    history: &[[i32; 64]; 6],
) {
    let n = moves.len();
    let mut scores = [0i32; 64];
    for (i, mv) in moves.iter().enumerate() {
        let mut s = 0i32;
        if let Some(victim) = game.board().piece_at(mv.to) {
            let attacker = game.board().piece_at(mv.from);
            let av = attacker.map(|p| piece_value(p.kind)).unwrap_or(0);
            s = piece_value(victim.kind) * 10 - av + 10_000;
        }
        if let Some(pt) = mv.promotion {
            s += piece_value(pt) + 5_000;
        }
        if in_check {
            s += 1_000;
        }
        if s <= 0 {
            if Some(*mv) == killers[0] {
                s += 8_000;
            } else if Some(*mv) == killers[1] {
                s += 7_000;
            }
            if let Some(piece) = game.board().piece_at(mv.from) {
                s += history[piece.kind as usize][mv.to.rank * 8 + mv.to.file] / 100;
            }
        }
        scores[i] = s;
    }
    for i in 0..n {
        let mut best = i;
        for j in i + 1..n {
            if scores[j] > scores[best] {
                best = j;
            }
        }
        if best != i {
            moves.swap(i, best);
            scores.swap(i, best);
        }
    }
}

fn quiescence(
    _tt: &mut TranspositionTable,
    game: &mut Game,
    hash: u64,
    alpha: i32,
    beta: i32,
    color: Color,
) -> i32 {
    let stand_pat = evaluate(game) * if color == game.turn() { 1 } else { -1 };
    if stand_pat >= beta {
        return beta;
    }
    let mut alpha = alpha;
    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let mut captures = moves::capture_moves(
        game.board(),
        game.turn(),
        game.ep_target(),
        game.castling_rights(),
    );
    if captures.is_empty() {
        return alpha;
    }

    order_moves(game, &mut captures, game.in_check(), &[None; 2], &[[0; 64]; 6]);

    for mv in captures {
        let piece = game.board().piece_at(mv.from).expect("piece must exist at from in quiescence");
        let captured = game.board().piece_at(mv.to);
        let old_ep = game.ep_target();
        let old_castling = *game.castling_rights();
        let undo = game.make_move_search(mv);
        let new_hash = update_hash(
            hash, &mv, piece, captured, old_ep, &old_castling,
            game.ep_target(), game.castling_rights(),
        );
        let score = -quiescence(_tt, game, new_hash, -beta, -alpha, color.opponent());
        game.undo_move(undo);
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }
    alpha
}

#[allow(clippy::too_many_arguments)]
fn negamax(
    tt: &mut TranspositionTable,
    game: &mut Game,
    hash: u64,
    depth: u32,
    mut alpha: i32,
    beta: i32,
    color: Color,
    extensions: u32,
    ply: u32,
) -> i32 {
    let (tt_result, tt_move) = tt.probe(hash, depth, alpha, beta);
    if let Some((score, _flag)) = tt_result {
        return score;
    }

    if depth == 0 {
        let score = quiescence(tt, game, hash, alpha, beta, color);
        tt.record(hash, 0, score, TTFlag::Exact, None);
        return score;
    }

    if game.halfmove_clock() >= 150 {
        tt.record(hash, depth, 0, TTFlag::Exact, None);
        return 0;
    }

    let in_check = game.in_check();

    // null-move: reduce by 2 plies; depth must be >= 3 so depth - 3 doesn't underflow u32
    if depth >= 3 && !in_check {
        let old_ep = game.null_move_begin();
        let null_hash = hash ^ zobrist().side;
        let null_hash = match old_ep {
            Some(ep) => null_hash ^ zobrist().ep[ep.file],
            None => null_hash,
        };
        let score = -negamax(tt, game, null_hash, depth - 1 - 2, -beta, -beta + 1, color.opponent(), extensions, ply);
        game.null_move_end(old_ep);
        if score >= beta {
            return beta;
        }
    }

    let mut moves = game.legal_moves();
    if moves.is_empty() {
        let score = if in_check { -INF + 2 } else { 0 };
        tt.record(hash, depth, score, TTFlag::Exact, None);
        return score;
    }

    if let Some(bm) = tt_move {
        if let Some(pos) = moves.iter().position(|m| *m == bm) {
            moves.swap(0, pos);
        }
    }

    let (killers, history) = KILLERS.with(|k| (k.borrow()[ply as usize], *k.borrow()));
    let _ = history;
    order_moves(game, &mut moves, in_check, &killers, &HISTORY.with(|h| *h.borrow()));

    let original_alpha = alpha;
    let mut best_move: Option<Move> = None;

    for (i, mv) in moves.iter().enumerate() {
        let piece = game.board().piece_at(mv.from).expect("piece must exist at from in negamax");
        let captured = game.board().piece_at(mv.to);
        let old_ep = game.ep_target();
        let old_castling = *game.castling_rights();
        let undo = game.make_move_search(*mv);
        let new_hash = update_hash(
            hash, mv, piece, captured, old_ep, &old_castling,
            game.ep_target(), game.castling_rights(),
        );

        let extend = game.in_check() && extensions < 1;
        let next_depth = if extend { depth } else { depth - 1 };
        let next_ext = if extend { extensions + 1 } else { extensions };

        let lmr = !in_check && depth >= 3 && i >= 3 && mv.promotion.is_none() && captured.is_none();

        let score;
        if i == 0 {
            score = -negamax(tt, game, new_hash, next_depth, -beta, -alpha, color.opponent(), next_ext, ply + 1);
        } else {
            let d = if lmr {
                let reduction = 1 + (i - 3) / 6 + depth as usize / 4;
                if reduction >= next_depth as usize { 1 } else { next_depth - reduction as u32 }
            } else {
                next_depth
            };
            let mut s = -negamax(tt, game, new_hash, d, -alpha - 1, -alpha, color.opponent(), next_ext, ply + 1);
            if lmr && s > alpha {
                s = -negamax(tt, game, new_hash, next_depth, -alpha - 1, -alpha, color.opponent(), next_ext, ply + 1);
            }
            if s > alpha && s < beta {
                s = -negamax(tt, game, new_hash, next_depth, -beta, -alpha, color.opponent(), next_ext, ply + 1);
            }
            score = s;
        }

        game.undo_move(undo);

        if score >= beta {
            tt.record(hash, depth, beta, TTFlag::LowerBound, best_move);
            if lmr && ply < 64 {
                KILLERS.with(|k| {
                    let pk = &mut k.borrow_mut()[ply as usize];
                    if pk[0] != Some(*mv) {
                        pk[1] = pk[0];
                        pk[0] = Some(*mv);
                    }
                });
                if let Some(p) = game.board().piece_at(mv.from) {
                    HISTORY.with(|h| {
                        let entry = &mut h.borrow_mut()[p.kind as usize][mv.to.rank * 8 + mv.to.file];
                        *entry = entry.saturating_add(depth as i32 * depth as i32);
                    });
                }
            }
            return beta;
        }
        if score > alpha {
            alpha = score;
            best_move = Some(*mv);
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

fn iterative_deepening(tt: &mut TranspositionTable, game: &mut Game, max_depth: u32) -> Option<Move> {
    KILLERS.with(|k| k.borrow_mut().fill([None; 2]));
    HISTORY.with(|h| h.borrow_mut().fill([0; 64]));

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

        order_moves(game, &mut moves, game.in_check(), &[None; 2], &HISTORY.with(|h| *h.borrow()));

        let mut current_best: Option<Move> = None;
        let mut current_score: i32 = -INF + 1;
        let mut alpha = -INF;
        let beta = INF;

        for mv in moves {
            let piece = game.board().piece_at(mv.from).expect("piece must exist at from in iterative_deepening");
            let captured = game.board().piece_at(mv.to);
            let old_ep = game.ep_target();
            let old_castling = *game.castling_rights();
            let undo = game.make_move_search(mv);
            let new_hash = update_hash(
                hash, &mv, piece, captured, old_ep, &old_castling,
                game.ep_target(), game.castling_rights(),
            );
            let score = -negamax(tt, game, new_hash, depth - 1, -beta, -alpha, color.opponent(), 0, 0);
            game.undo_move(undo);
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
            Difficulty::Medium => 6,
            Difficulty::Hard => 14,
        }
    }
}

pub fn best_move(game: &Game) -> Option<Move> {
    let mut g = game.clone();
    TT.with(|tt| iterative_deepening(&mut tt.borrow_mut(), &mut g, Difficulty::default().depth()))
}

pub fn best_move_with_depth(game: &Game, max_depth: u32) -> Option<Move> {
    let mut g = game.clone();
    TT.with(|tt| iterative_deepening(&mut tt.borrow_mut(), &mut g, max_depth))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Game;

    #[test]
    fn test_piece_value_order() {
        assert!(piece_value(PieceType::Pawn) < piece_value(PieceType::Knight));
        assert!(piece_value(PieceType::Knight) < piece_value(PieceType::Bishop));
        assert!(piece_value(PieceType::Bishop) < piece_value(PieceType::Rook));
        assert!(piece_value(PieceType::Rook) < piece_value(PieceType::Queen));
        assert!(piece_value(PieceType::King) > piece_value(PieceType::Queen));
    }

    #[test]
    fn test_pst_bonus_180_rotation() {
        for kind in &[PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King] {
            for file in 0..8 {
                for rank in 0..8 {
                    let sq_w = Square::new_unchecked(file, rank);
                    let sq_b = Square::new_unchecked(7 - file, 7 - rank);
                    let bonus_w = pst_bonus(*kind, sq_w, Color::White);
                    let bonus_b = pst_bonus(*kind, sq_b, Color::Black);
                    assert_eq!(bonus_w, bonus_b,
                        "PST 180-rotation asymmetry for {:?} at ({},{}): White({}) != Black({})",
                        kind, file, rank, bonus_w, bonus_b,
                    );
                }
            }
        }
    }

    #[test]
    fn test_evaluate_initial() {
        let game = Game::new();
        let score = evaluate(&game);
        assert_eq!(score, 0, "Initial position should be balanced");
    }

    #[test]
    fn test_evaluate_material_imbalance() {
        // White has Queen+K+B vs K only → White should be far ahead
        let game = Game::from_fen("4k3/8/8/8/8/8/4Q3/4KB2 w - - 0 1").unwrap();
        let score = evaluate(&game);
        assert!(score > 200, "White should be ahead by material: score={}", score);
    }

    #[test]
    fn test_compute_hash_deterministic() {
        let game = Game::new();
        let h1 = compute_hash(&game);
        let h2 = compute_hash(&game);
        assert_eq!(h1, h2, "Hash must be deterministic");
    }

    #[test]
    fn test_compute_hash_different_positions() {
        let g1 = Game::new();
        let g2 = Game::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1").unwrap();
        let h1 = compute_hash(&g1);
        let h2 = compute_hash(&g2);
        assert_ne!(h1, h2, "Different positions must have different hashes");
    }

    #[test]
    fn test_initial_best_move_not_none() {
        let game = Game::new();
        let mv = best_move_with_depth(&game, 1);
        assert!(mv.is_some(), "Should find a move from initial position at depth 1");
    }

    #[test]
    fn test_difficulty_depth() {
        assert_eq!(Difficulty::Easy.depth(), 2);
        assert_eq!(Difficulty::Medium.depth(), 6);
        assert_eq!(Difficulty::Hard.depth(), 14);
    }

    #[test]
    fn test_pst_bonus_bounds() {
        let corners = [
            (0, 0, Color::White),
            (7, 7, Color::White),
            (0, 7, Color::Black),
            (7, 0, Color::Black),
        ];
        for &(file, rank, color) in &corners {
            let sq = Square::new_unchecked(file, rank);
            for kind in &[PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King] {
                let bonus = pst_bonus(*kind, sq, color);
                assert!(bonus >= -100 && bonus <= 100, "PST bonus out of bounds for {:?} at ({},{}): {}",
                    kind, file, rank, bonus);
            }
        }
    }
}
