use crate::board::Board;
use crate::game::CastlingRights;
use crate::mv::Move;
use crate::piece::{Color, PieceType};
use crate::square::Square;

const KNIGHT_OFFSETS: [(isize, isize); 8] = [
    (2, 1),
    (2, -1),
    (-2, 1),
    (-2, -1),
    (1, 2),
    (1, -2),
    (-1, 2),
    (-1, -2),
];

const KING_OFFSETS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

const ROOK_DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

const BISHOP_DIRS: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

pub fn legal_moves(
    board: &Board,
    color: Color,
    ep_target: Option<Square>,
    castling: &CastlingRights,
) -> Vec<Move> {
    let pseudo = pseudo_legal_moves(board, color, ep_target, castling);
    pseudo
        .into_iter()
        .filter(|mv| is_legal(board, mv, color))
        .collect()
}

pub fn is_square_attacked(board: &Board, square: Square, by_color: Color) -> bool {
    let pawn_fwd: isize = match by_color {
        Color::White => 1,
        Color::Black => -1,
    };

    for df in [-1, 1] {
        if let Some(from) = square.offset(-df, -pawn_fwd) {
            if let Some(p) = board.piece_at(from) {
                if p.kind == PieceType::Pawn && p.color == by_color {
                    return true;
                }
            }
        }
    }

    for &(df, dr) in &KNIGHT_OFFSETS {
        if let Some(from) = square.offset(-df, -dr) {
            if let Some(p) = board.piece_at(from) {
                if p.kind == PieceType::Knight && p.color == by_color {
                    return true;
                }
            }
        }
    }

    for &(df, dr) in &KING_OFFSETS {
        if let Some(from) = square.offset(-df, -dr) {
            if let Some(p) = board.piece_at(from) {
                if p.kind == PieceType::King && p.color == by_color {
                    return true;
                }
            }
        }
    }

    for &(df, dr) in &BISHOP_DIRS {
        let mut sq = square;
        loop {
            match sq.offset(df, dr) {
                Some(next) => {
                    sq = next;
                    match board.piece_at(sq) {
                        Some(p) => {
                            if p.color == by_color
                                && (p.kind == PieceType::Bishop || p.kind == PieceType::Queen)
                            {
                                return true;
                            }
                            break;
                        }
                        None => {}
                    }
                }
                None => break,
            }
        }
    }

    for &(df, dr) in &ROOK_DIRS {
        let mut sq = square;
        loop {
            match sq.offset(df, dr) {
                Some(next) => {
                    sq = next;
                    match board.piece_at(sq) {
                        Some(p) => {
                            if p.color == by_color
                                && (p.kind == PieceType::Rook || p.kind == PieceType::Queen)
                            {
                                return true;
                            }
                            break;
                        }
                        None => {}
                    }
                }
                None => break,
            }
        }
    }

    false
}

fn pseudo_legal_moves(
    board: &Board,
    color: Color,
    ep_target: Option<Square>,
    castling: &CastlingRights,
) -> Vec<Move> {
    let mut moves = Vec::new();

    for rank in 0..8 {
        for file in 0..8 {
            let from = Square::new_unchecked(file, rank);
            if let Some(piece) = board.piece_at(from) {
                if piece.color != color {
                    continue;
                }

                match piece.kind {
                    PieceType::Pawn => {
                        add_pawn_moves(board, from, color, ep_target, &mut moves);
                    }
                    PieceType::Knight => {
                        add_knight_moves(board, from, color, &mut moves);
                    }
                    PieceType::Bishop => {
                        add_sliding_moves(board, from, color, &BISHOP_DIRS, &mut moves);
                    }
                    PieceType::Rook => {
                        add_sliding_moves(board, from, color, &ROOK_DIRS, &mut moves);
                    }
                    PieceType::Queen => {
                        add_sliding_moves(board, from, color, &ROOK_DIRS, &mut moves);
                        add_sliding_moves(board, from, color, &BISHOP_DIRS, &mut moves);
                    }
                    PieceType::King => {
                        add_king_moves(board, from, color, &mut moves);
                        add_castling_moves(board, from, color, castling, &mut moves);
                    }
                }
            }
        }
    }

    moves
}

fn add_pawn_moves(
    board: &Board,
    from: Square,
    color: Color,
    ep_target: Option<Square>,
    moves: &mut Vec<Move>,
) {
    let (forward, start_rank, promote_rank) = match color {
        Color::White => (1, 1, 7),
        Color::Black => (-1, 6, 0),
    };

    if let Some(to) = from.offset(0, forward) {
        if board.piece_at(to).is_none() {
            if to.rank == promote_rank {
                for &promo in &[
                    PieceType::Queen,
                    PieceType::Rook,
                    PieceType::Bishop,
                    PieceType::Knight,
                ] {
                    moves.push(Move::new_promotion(from, to, promo));
                }
            } else {
                moves.push(Move::new(from, to));
            }
        }
    }

    if from.rank == start_rank {
        if let Some(to) = from.offset(0, forward * 2) {
            let mid = from.offset(0, forward).unwrap();
            if board.piece_at(to).is_none() && board.piece_at(mid).is_none() {
                moves.push(Move::new(from, to));
            }
        }
    }

    for df in [-1, 1] {
        if let Some(to) = from.offset(df, forward) {
            if let Some(p) = board.piece_at(to) {
                if p.color != color {
                    if to.rank == promote_rank {
                        for &promo in &[
                            PieceType::Queen,
                            PieceType::Rook,
                            PieceType::Bishop,
                            PieceType::Knight,
                        ] {
                            moves.push(Move::new_promotion(from, to, promo));
                        }
                    } else {
                        moves.push(Move::new(from, to));
                    }
                }
            }

            if let Some(ep) = ep_target {
                if to == ep {
                    moves.push(Move::new(from, to));
                }
            }
        }
    }
}

fn add_knight_moves(board: &Board, from: Square, color: Color, moves: &mut Vec<Move>) {
    for &(df, dr) in &KNIGHT_OFFSETS {
        if let Some(to) = from.offset(df, dr) {
            match board.piece_at(to) {
                Some(p) if p.color != color => moves.push(Move::new(from, to)),
                None => moves.push(Move::new(from, to)),
                _ => {}
            }
        }
    }
}

fn add_sliding_moves(
    board: &Board,
    from: Square,
    color: Color,
    dirs: &[(isize, isize)],
    moves: &mut Vec<Move>,
) {
    for &(df, dr) in dirs {
        let mut sq = from;
        loop {
            match sq.offset(df, dr) {
                Some(to) => match board.piece_at(to) {
                    Some(p) if p.color != color => {
                        moves.push(Move::new(from, to));
                        break;
                    }
                    Some(_) => break,
                    None => {
                        moves.push(Move::new(from, to));
                        sq = to;
                    }
                },
                None => break,
            }
        }
    }
}

fn add_king_moves(board: &Board, from: Square, color: Color, moves: &mut Vec<Move>) {
    for &(df, dr) in &KING_OFFSETS {
        if let Some(to) = from.offset(df, dr) {
            match board.piece_at(to) {
                Some(p) if p.color != color => moves.push(Move::new(from, to)),
                None => moves.push(Move::new(from, to)),
                _ => {}
            }
        }
    }
}

fn add_castling_moves(
    board: &Board,
    from: Square,
    color: Color,
    castling: &CastlingRights,
    moves: &mut Vec<Move>,
) {
    if is_square_attacked(board, from, color.opponent()) {
        return;
    }

    let rank = from.rank;

    let (kingside_allowed, queenside_allowed) = match color {
        Color::White => (castling.white_kingside, castling.white_queenside),
        Color::Black => (castling.black_kingside, castling.black_queenside),
    };

    let king_from_file = from.file;

    if kingside_allowed {
        let king_to_file = 6;
        let rook_from_file = 7;

        let king_to = Square::new_unchecked(king_to_file, rank);
        let rook_from = Square::new_unchecked(rook_from_file, rank);

        let path_clear = (king_from_file + 1..rook_from_file)
            .all(|f| board.piece_at(Square::new_unchecked(f, rank)).is_none());

        let path_safe = (king_from_file + 1..=king_to_file)
            .all(|f| !is_square_attacked(board, Square::new_unchecked(f, rank), color.opponent()));

        let rook_ok = board.piece_at(rook_from) == Some(crate::Piece::new(PieceType::Rook, color));

        if path_clear && path_safe && rook_ok {
            moves.push(Move::new(from, king_to));
        }
    }

    if queenside_allowed {
        let king_to_file = 2;
        let rook_from_file = 0;

        let king_to = Square::new_unchecked(king_to_file, rank);
        let rook_from = Square::new_unchecked(rook_from_file, rank);

        let path_clear = (rook_from_file + 1..king_from_file)
            .all(|f| board.piece_at(Square::new_unchecked(f, rank)).is_none());

        let path_safe = (king_to_file..king_from_file)
            .all(|f| !is_square_attacked(board, Square::new_unchecked(f, rank), color.opponent()));

        let rook_ok = board.piece_at(rook_from) == Some(crate::Piece::new(PieceType::Rook, color));

        if path_clear && path_safe && rook_ok {
            moves.push(Move::new(from, king_to));
        }
    }
}

fn is_legal(board: &Board, mv: &Move, color: Color) -> bool {
    let mut new_board = board.clone();

    let piece = new_board.piece_at(mv.from);
    let is_ep = piece.map_or(false, |p| {
        p.kind == PieceType::Pawn && mv.from.file != mv.to.file && board.piece_at(mv.to).is_none()
    });

    new_board.set_piece(mv.to, piece);
    new_board.set_piece(mv.from, None);

    if is_ep {
        let captured_sq = Square::new_unchecked(mv.to.file, mv.from.rank);
        new_board.set_piece(captured_sq, None);
    }

    let king_sq = match new_board.king_square(color) {
        Some(sq) => sq,
        None => return false,
    };

    !is_square_attacked(&new_board, king_sq, color.opponent())
}

pub fn perft(
    board: &Board,
    depth: u32,
    color: Color,
    ep_target: Option<Square>,
    castling: &CastlingRights,
) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = pseudo_legal_moves(board, color, ep_target, castling);

    if depth == 1 {
        return moves.iter().filter(|mv| is_legal(board, mv, color)).count() as u64;
    }

    let mut total = 0;
    for mv in moves {
        if !is_legal(board, &mv, color) {
            continue;
        }

        let mut new_board = board.clone();
        let piece = new_board.piece_at(mv.from);

        let is_ep = piece.map_or(false, |p| {
            p.kind == PieceType::Pawn
                && mv.from.file != mv.to.file
                && board.piece_at(mv.to).is_none()
        });

        new_board.set_piece(mv.to, piece);
        new_board.set_piece(mv.from, None);

        if is_ep {
            let captured_sq = Square::new_unchecked(mv.to.file, mv.from.rank);
            new_board.set_piece(captured_sq, None);
        }

        if let Some(promo) = mv.promotion {
            new_board.set_piece(mv.to, Some(crate::Piece::new(promo, color)));
        }

        let new_ep = next_ep_target(&piece, &mv);
        let new_castling = update_castling_rights(castling, &piece, &mv, &new_board);

        total += perft(
            &new_board,
            depth - 1,
            color.opponent(),
            new_ep,
            &new_castling,
        );
    }

    total
}

fn next_ep_target(piece: &Option<crate::Piece>, mv: &Move) -> Option<Square> {
    if let Some(p) = piece {
        if p.kind == PieceType::Pawn {
            let rank_diff = (mv.to.rank as isize - mv.from.rank as isize).abs();
            if rank_diff == 2 {
                let mid_rank = (mv.from.rank + mv.to.rank) / 2;
                return Some(Square::new_unchecked(mv.from.file, mid_rank));
            }
        }
    }
    None
}

fn update_castling_rights(
    castling: &CastlingRights,
    piece: &Option<crate::Piece>,
    mv: &Move,
    new_board: &Board,
) -> CastlingRights {
    let mut new_castling = *castling;

    if let Some(p) = piece {
        if p.kind == PieceType::King {
            match p.color {
                Color::White => {
                    new_castling.white_kingside = false;
                    new_castling.white_queenside = false;
                }
                Color::Black => {
                    new_castling.black_kingside = false;
                    new_castling.black_queenside = false;
                }
            }
        }

        if p.kind == PieceType::Rook {
            match (p.color, mv.from.file) {
                (Color::White, 0) => new_castling.white_queenside = false,
                (Color::White, 7) => new_castling.white_kingside = false,
                (Color::Black, 0) => new_castling.black_queenside = false,
                (Color::Black, 7) => new_castling.black_kingside = false,
                _ => {}
            }
        }
    }

    let captured = new_board.piece_at(mv.to);
    if let Some(p) = captured {
        if p.kind == PieceType::Rook {
            match (p.color, mv.to.file) {
                (Color::White, 0) => new_castling.white_queenside = false,
                (Color::White, 7) => new_castling.white_kingside = false,
                (Color::Black, 0) => new_castling.black_queenside = false,
                (Color::Black, 7) => new_castling.black_kingside = false,
                _ => {}
            }
        }
    }

    new_castling
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Board;

    fn initial_castling() -> CastlingRights {
        CastlingRights::all()
    }

    #[test]
    fn test_perft_depth_1() {
        let board = Board::initial();
        let count = perft(&board, 1, Color::White, None, &initial_castling());
        assert_eq!(count, 20);
    }

    #[test]
    fn test_perft_depth_2() {
        let board = Board::initial();
        let count = perft(&board, 2, Color::White, None, &initial_castling());
        assert_eq!(count, 400);
    }

    #[test]
    fn test_perft_depth_3() {
        let board = Board::initial();
        let count = perft(&board, 3, Color::White, None, &initial_castling());
        assert_eq!(count, 8902);
    }

    #[test]
    fn test_perft_depth_4() {
        let board = Board::initial();
        let count = perft(&board, 4, Color::White, None, &initial_castling());
        assert_eq!(count, 197281);
    }

    #[test]
    fn test_king_not_in_check_initial() {
        let board = Board::initial();
        let king_sq = board.king_square(Color::White).unwrap();
        assert!(!is_square_attacked(&board, king_sq, Color::Black));
    }

    #[test]
    fn test_scholars_mate() {
        let fen = "r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4";
        let board = Board::from_fen(&fen[..fen.find(' ').unwrap_or(fen.len())]).unwrap();
        let king_sq = board.king_square(Color::Black).unwrap();
        assert!(is_square_attacked(&board, king_sq, Color::White));
    }
}
