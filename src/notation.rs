use crate::game::Game;
use crate::mv::Move;
use crate::piece::{Piece, PieceType};
use crate::square::Square;

pub fn move_to_algebraic(game: &Game, mv: &Move) -> String {
    let piece = match game.board().piece_at(mv.from) {
        Some(p) => p,
        None => return mv.to_coordinate(),
    };

    let is_capture = game.board().piece_at(mv.to).is_some()
        || (piece.kind == PieceType::Pawn
            && mv.from.file != mv.to.file
            && (game.ep_target() == Some(mv.to)));

    let s = match piece.kind {
        PieceType::King => {
            let file_diff = mv.to.file as isize - mv.from.file as isize;
            if file_diff.abs() == 2 {
                if file_diff > 0 {
                    "O-O".to_string()
                } else {
                    "O-O-O".to_string()
                }
            } else {
                let mut s = String::from("K");
                if is_capture {
                    s.push('x');
                }
                s.push_str(&mv.to.to_algebraic());
                s
            }
        }
        PieceType::Queen => {
            let mut s = String::from("Q");
            s.push_str(&disambiguation(game, mv, piece));
            if is_capture {
                s.push('x');
            }
            s.push_str(&mv.to.to_algebraic());
            s
        }
        PieceType::Rook => {
            let mut s = String::from("R");
            s.push_str(&disambiguation(game, mv, piece));
            if is_capture {
                s.push('x');
            }
            s.push_str(&mv.to.to_algebraic());
            s
        }
        PieceType::Bishop => {
            let mut s = String::from("B");
            s.push_str(&disambiguation(game, mv, piece));
            if is_capture {
                s.push('x');
            }
            s.push_str(&mv.to.to_algebraic());
            s
        }
        PieceType::Knight => {
            let mut s = String::from("N");
            s.push_str(&disambiguation(game, mv, piece));
            if is_capture {
                s.push('x');
            }
            s.push_str(&mv.to.to_algebraic());
            s
        }
        PieceType::Pawn => {
            let mut s = String::new();
            if is_capture {
                s.push((b'a' + mv.from.file as u8) as char);
                s.push('x');
            }
            s.push_str(&mv.to.to_algebraic());
            if let Some(p) = mv.promotion {
                s.push('=');
                s.push(p.to_char().to_ascii_uppercase());
            }
            s
        }
    };

    let mut test = game.clone();
    if test.make_move(*mv).is_ok() {
        let is_check = test.in_check();
        let is_mate = is_check && test.legal_moves().is_empty();
        if is_mate {
            format!("{}#", s)
        } else if is_check {
            format!("{}+", s)
        } else {
            s
        }
    } else {
        s
    }
}

pub fn disambiguation(game: &Game, mv: &Move, piece: Piece) -> String {
    let moves = game.legal_moves();
    let same_target: Vec<&Move> = moves
        .iter()
        .filter(|lm| {
            lm.to == mv.to && lm.from != mv.from && game.board().piece_at(lm.from) == Some(piece)
        })
        .collect();

    if same_target.is_empty() {
        return String::new();
    }

    let same_file = same_target.iter().any(|lm| lm.from.file == mv.from.file);
    let same_rank = same_target.iter().any(|lm| lm.from.rank == mv.from.rank);

    if !same_file {
        format!("{}", (b'a' + mv.from.file as u8) as char)
    } else if !same_rank {
        format!("{}", mv.from.rank + 1)
    } else {
        format!(
            "{}{}",
            (b'a' + mv.from.file as u8) as char,
            mv.from.rank + 1
        )
    }
}

pub fn parse_algebraic(game: &Game, input: &str) -> Option<Move> {
    let input = input.trim().replace(['+', '#'], "");

    if input == "O-O" || input == "0-0" || input == "o-o" {
        let king_sq = game.board().king_square(game.turn())?;
        let to_sq = Square::new_unchecked(6, king_sq.rank);
        return game
            .legal_moves()
            .into_iter()
            .find(|mv| mv.from == king_sq && mv.to == to_sq);
    }

    if input == "O-O-O" || input == "0-0-0" || input == "o-o-o" {
        let king_sq = game.board().king_square(game.turn())?;
        let to_sq = Square::new_unchecked(2, king_sq.rank);
        return game
            .legal_moves()
            .into_iter()
            .find(|mv| mv.from == king_sq && mv.to == to_sq);
    }

    if let Some(mv) = try_parse_coordinate(game, &input) {
        return Some(mv);
    }

    try_parse_algebraic_move(game, &input)
}

fn try_parse_coordinate(game: &Game, input: &str) -> Option<Move> {
    let clean = input.replace('=', "");
    let promo =
        clean.len() > 4 && PieceType::from_char(clean.as_bytes()[4] as char).is_some();

    let (coord, promotion_char) = if promo {
        (&clean[..4], clean.as_bytes()[4] as char)
    } else if clean.len() >= 4 {
        (&clean[..4], ' ')
    } else {
        return None;
    };

    let from = Square::from_algebraic(&coord[..2])?;
    let to = Square::from_algebraic(&coord[2..4])?;

    let piece = game.board().piece_at(from)?;
    if piece.color != game.turn() {
        return None;
    }

    let promotion = if promotion_char != ' ' {
        PieceType::from_char(promotion_char)
    } else {
        None
    };

    game.legal_moves()
        .into_iter()
        .find(|mv| mv.from == from && mv.to == to && mv.promotion == promotion)
}

fn try_parse_algebraic_move(game: &Game, input: &str) -> Option<Move> {
    let promotion = if input.contains('=') {
        let parts: Vec<&str> = input.split('=').collect();
        if parts.len() == 2 {
            let promo_char = parts[1].chars().next()?;
            Some(PieceType::from_char(promo_char)?)
        } else {
            None
        }
    } else {
        None
    };

    let input = input.replace('=', "");

    let chars: Vec<char> = input.chars().collect();

    if chars.len() < 2 {
        return None;
    }

    let first = chars[0];

    let (piece_type, idx) = match first {
        'K' => (PieceType::King, 1),
        'Q' => (PieceType::Queen, 1),
        'R' => (PieceType::Rook, 1),
        'B' => (PieceType::Bishop, 1),
        'N' => (PieceType::Knight, 1),
        _ => (PieceType::Pawn, 0),
    };

    let rest: String = chars[idx..].iter().collect();
    let rest = rest.trim_start_matches('x');

    if piece_type == PieceType::Pawn {
        let rest = if promotion.is_some() && rest.len() > 2 {
            &rest[..rest.len() - 1]
        } else {
            rest
        };
        return parse_pawn_move(game, rest, promotion);
    }

    let target_sq = if rest.len() >= 2 {
        let possible_sq = &rest[rest.len() - 2..];
        Square::from_algebraic(possible_sq)
    } else {
        None
    };

    let target_sq = target_sq?;

    let disambig = &rest[..rest.len() - 2];

    let candidates: Vec<Move> = game
        .legal_moves()
        .into_iter()
        .filter(|mv| {
            let p = game.board().piece_at(mv.from);
            p.is_some_and(|p| p.kind == piece_type && p.color == game.turn())
                && mv.to == target_sq
                && mv.promotion == promotion
        })
        .collect();

    if candidates.is_empty() {
        return None;
    }

    if candidates.len() == 1 {
        return Some(candidates[0]);
    }

    for mv in &candidates {
        let from = mv.from;
        if disambig.len() == 1 {
            let c = disambig.chars().next().unwrap();
            if c.is_ascii_digit() {
                let rank = c.to_digit(10).unwrap() as usize - 1;
                if from.rank == rank {
                    return Some(*mv);
                }
            } else {
                let file = (c as u8 - b'a') as usize;
                if from.file == file {
                    return Some(*mv);
                }
            }
        } else if disambig.len() >= 2
            && from.to_algebraic().starts_with(disambig) {
                return Some(*mv);
            }
    }

    candidates.into_iter().next()
}

fn parse_pawn_move(game: &Game, input: &str, promotion: Option<PieceType>) -> Option<Move> {
    let dest = if let Some(x_pos) = input.find('x') {
        &input[x_pos + 1..]
    } else {
        input
    };

    let target_sq = Square::from_algebraic(dest)?;

    let candidates: Vec<Move> = game
        .legal_moves()
        .into_iter()
        .filter(|mv| {
            game.board().piece_at(mv.from).is_some_and(|p| {
                p.kind == PieceType::Pawn && p.color == game.turn()
            }) && mv.to == target_sq
                && mv.promotion == promotion
        })
        .collect();

    if candidates.is_empty() {
        return None;
    }

    if candidates.len() == 1 {
        return Some(candidates[0]);
    }

    let x_index = input.rfind('x');
    if let Some(idx) = x_index {
        let from_file = (input.as_bytes()[idx.saturating_sub(1).max(0)] - b'a') as usize;
        return candidates.into_iter().find(|mv| mv.from.file == from_file);
    }

    candidates.into_iter().next()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Game;

    #[test]
    fn test_notation_pawn_move() {
        let game = Game::new();
        let e4 = Move::new("e2".parse().unwrap(), "e4".parse().unwrap());
        assert_eq!(move_to_algebraic(&game, &e4), "e4");
    }

    #[test]
    fn test_notation_pawn_capture() {
        let fen = "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2";
        let game = Game::from_fen(fen).unwrap();
        let exd5 = Move::new("e4".parse().unwrap(), "d5".parse().unwrap());
        assert_eq!(move_to_algebraic(&game, &exd5), "exd5");
    }

    #[test]
    fn test_notation_knight_move() {
        let game = Game::new();
        let nf3 = Move::new("g1".parse().unwrap(), "f3".parse().unwrap());
        assert_eq!(move_to_algebraic(&game, &nf3), "Nf3");
    }

    #[test]
    fn test_notation_bishop_capture() {
        let fen = "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2";
        let game = Game::from_fen(fen).unwrap();
        let bc4 = Move::new("f1".parse().unwrap(), "c4".parse().unwrap());
        assert_eq!(move_to_algebraic(&game, &bc4), "Bc4");
    }

    #[test]
    fn test_notation_castling_kingside() {
        let fen = "r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 4 5";
        let game = Game::from_fen(fen).unwrap();
        let oo = Move::new("e1".parse().unwrap(), "g1".parse().unwrap());
        assert_eq!(move_to_algebraic(&game, &oo), "O-O");
    }

    #[test]
    fn test_notation_castling_queenside() {
        let fen = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";
        let game = Game::from_fen(fen).unwrap();
        let ooo = Move::new("e1".parse().unwrap(), "c1".parse().unwrap());
        assert_eq!(move_to_algebraic(&game, &ooo), "O-O-O");
    }

    #[test]
    fn test_notation_promotion() {
        let fen = "4k3/P7/8/8/8/8/8/4K3 w - - 0 1";
        let game = Game::from_fen(fen).unwrap();
        let promo = Move::new_promotion("a7".parse().unwrap(), "a8".parse().unwrap(), PieceType::Queen);
        let legal = game.legal_moves().into_iter().find(|m| *m == promo && m.promotion == Some(PieceType::Queen)).unwrap();
        assert_eq!(move_to_algebraic(&game, &legal), "a8=Q+");
    }

    #[test]
    fn test_notation_disambiguation_file() {
        let fen = "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 4 5";
        let game = Game::from_fen(fen).unwrap();
        let mv = Move::new("b1".parse().unwrap(), "d2".parse().unwrap());
        assert_eq!(move_to_algebraic(&game, &mv), "Nbd2");
    }

    #[test]
    fn test_notation_disambiguation_rank() {
        let fen = "R7/8/8/8/8/8/8/R3K3 w - - 0 1";
        let game = Game::from_fen(fen).unwrap();
        let mv = Move::new("a1".parse().unwrap(), "a5".parse().unwrap());
        assert_eq!(move_to_algebraic(&game, &mv), "R1a5");
    }

    #[test]
    fn test_notation_check() {
        let fen = "4k3/8/8/8/8/8/4Q3/4K3 w - - 0 1";
        let game = Game::from_fen(fen).unwrap();
        let qe5 = Move::new("e2".parse().unwrap(), "e5".parse().unwrap());
        assert_eq!(move_to_algebraic(&game, &qe5), "Qe5+");
    }

    #[test]
    fn test_notation_checkmate() {
        let fen = "k7/2K5/8/8/8/8/1Q6/8 w - - 0 1";
        let game = Game::from_fen(fen).unwrap();
        let qb7 = Move::new("b2".parse().unwrap(), "b7".parse().unwrap());
        assert_eq!(move_to_algebraic(&game, &qb7), "Qb7#");
    }

    #[test]
    fn test_parse_pawn_move() {
        let game = Game::new();
        let mv = parse_algebraic(&game, "e4").unwrap();
        assert_eq!(mv.from.to_algebraic(), "e2");
        assert_eq!(mv.to.to_algebraic(), "e4");
    }

    #[test]
    fn test_parse_knight_move() {
        let game = Game::new();
        let mv = parse_algebraic(&game, "Nf3").unwrap();
        assert_eq!(mv.from.to_algebraic(), "g1");
        assert_eq!(mv.to.to_algebraic(), "f3");
    }

    #[test]
    fn test_parse_castling_kingside() {
        let fen = "r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 4 5";
        let game = Game::from_fen(fen).unwrap();
        let mv = parse_algebraic(&game, "O-O").unwrap();
        assert_eq!(mv.from.to_algebraic(), "e1");
        assert_eq!(mv.to.to_algebraic(), "g1");
    }

    #[test]
    fn test_parse_castling_queenside() {
        let fen = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";
        let game = Game::from_fen(fen).unwrap();
        let mv = parse_algebraic(&game, "O-O-O").unwrap();
        assert_eq!(mv.from.to_algebraic(), "e1");
        assert_eq!(mv.to.to_algebraic(), "c1");
    }

    #[test]
    fn test_parse_coordinate() {
        let game = Game::new();
        let mv = parse_algebraic(&game, "e2e4").unwrap();
        assert_eq!(mv.from.to_algebraic(), "e2");
        assert_eq!(mv.to.to_algebraic(), "e4");
    }

    #[test]
    fn test_parse_promotion() {
        let fen = "4k3/P7/8/8/8/8/8/4K3 w - - 0 1";
        let game = Game::from_fen(fen).unwrap();
        let mv = parse_algebraic(&game, "a8=Q").unwrap();
        assert_eq!(mv.promotion, Some(PieceType::Queen));
    }

    #[test]
    fn test_parse_with_check_symbol() {
        let game = Game::new();
        let mv = parse_algebraic(&game, "e4+").unwrap();
        assert_eq!(mv.to.to_algebraic(), "e4");
    }

    #[test]
    fn test_parse_pawn_capture() {
        let fen = "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2";
        let game = Game::from_fen(fen).unwrap();
        let mv = parse_algebraic(&game, "exd5").unwrap();
        assert_eq!(mv.from.to_algebraic(), "e4");
        assert_eq!(mv.to.to_algebraic(), "d5");
    }

    #[test]
    fn test_parse_bishop_capture() {
        let fen = "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2";
        let game = Game::from_fen(fen).unwrap();
        let mv = parse_algebraic(&game, "Bc4").unwrap();
        assert_eq!(mv.from.to_algebraic(), "f1");
        assert_eq!(mv.to.to_algebraic(), "c4");
    }

    #[test]
    fn test_parse_disambiguation_file() {
        let fen = "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 4 5";
        let game = Game::from_fen(fen).unwrap();
        let mv = parse_algebraic(&game, "Nbd2").unwrap();
        assert_eq!(mv.from.to_algebraic(), "b1");
        assert_eq!(mv.to.to_algebraic(), "d2");
    }
}
