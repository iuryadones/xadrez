use crate::board::Board;
use crate::game::Game;
use crate::mv::Move;
use crate::piece::{Color, PieceType};
use crate::square::Square;
use std::time::{SystemTime, UNIX_EPOCH};

const INF: i32 = 1_000_000_000;

const PIECE_VALUES: [i32; 6] = [100, 320, 330, 500, 900, 20000];

fn piece_value(kind: PieceType) -> i32 {
    PIECE_VALUES[kind as usize]
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
    &PST_PAWN, &PST_KNIGHT, &PST_BISHOP, &PST_ROOK, &PST_QUEEN, &PST_KING,
];

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
    game: &Game,
    depth: u32,
    alpha: i32,
    beta: i32,
    color: Color,
) -> i32 {
    if depth == 0 {
        return quiescence(game, alpha, beta, color);
    }

    if game.status() != crate::GameStatus::Ongoing {
        return if game.status() == crate::GameStatus::WhiteWins {
            if color == Color::White { INF - 1 } else { -INF + 1 }
        } else if game.status() == crate::GameStatus::BlackWins {
            if color == Color::Black { INF - 1 } else { -INF + 1 }
        } else {
            0
        };
    }

    let mut moves = game.legal_moves();
    if moves.is_empty() {
        return if game.in_check() {
            -INF + 2
        } else {
            0
        };
    }

    order_moves(game, &mut moves);

    let mut alpha = alpha;
    let mut searched = 0;

    for mv in moves {
        let mut g = game.clone();
        if g.make_move(mv).is_err() {
            continue;
        }
        let score;
        if searched == 0 {
            score = -negamax(&g, depth - 1, -beta, -alpha, color.opponent());
        } else {
            let mut s = -negamax(&g, depth - 1, -alpha - 1, -alpha, color.opponent());
            if s > alpha && s < beta {
                s = -negamax(&g, depth - 1, -beta, -alpha, color.opponent());
            }
            score = s;
        }
        searched += 1;
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }
    alpha
}

fn iterative_deepening(game: &Game, max_depth: u32) -> Option<Move> {
    let mut best_move: Option<Move> = None;
    let mut _best_score: i32 = -INF + 1;
    let color = game.turn();

    for depth in 1..=max_depth {
        let mut moves = game.legal_moves();
        if moves.is_empty() {
            break;
        }

        if let Some(ref bm) = best_move {
            let idx = moves.iter().position(|m| m == bm);
            if let Some(i) = idx {
                moves.swap(0, i);
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
            let score = -negamax(&g, depth - 1, -beta, -alpha, color.opponent());
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

#[derive(Default)]
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
    iterative_deepening(game, Difficulty::default().depth())
}

pub fn best_move_with_depth(game: &Game, max_depth: u32) -> Option<Move> {
    iterative_deepening(game, max_depth)
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
