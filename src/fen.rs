use crate::game::CastlingRights;
use crate::{Board, Color, Square};

type FenResult = Result<(Board, Color, CastlingRights, Option<Square>, u8, u16), String>;

pub fn parse_fen(
    fen: &str,
) -> FenResult {
    let parts: Vec<&str> = fen.split_whitespace().collect();
    if parts.len() < 4 {
        return Err("FEN precisa de ao menos 4 campos".into());
    }

    let board = Board::from_fen(parts[0])?;

    let turn = match parts[1] {
        "w" => Color::White,
        "b" => Color::Black,
        _ => return Err("Turno FEN invalido: esperado 'w' ou 'b'".into()),
    };

    let castling = parse_castling(parts[2])?;

    let ep_target = match parts[3] {
        "-" => None,
        sq => match Square::from_algebraic(sq) {
            Some(s) => Some(s),
            None => return Err("En passant FEN invalido".into()),
        },
    };

    let halfmove = if parts.len() > 4 {
        parts[4]
            .parse::<u8>()
            .map_err(|_| "Halfmove clock invalido".to_string())?
    } else {
        0
    };

    let fullmove = if parts.len() > 5 {
        parts[5]
            .parse::<u16>()
            .map_err(|_| "Fullmove number invalido".to_string())?
    } else {
        1
    };

    Ok((board, turn, castling, ep_target, halfmove, fullmove))
}

fn parse_castling(s: &str) -> Result<CastlingRights, String> {
    let mut c = CastlingRights::none();
    if s == "-" {
        return Ok(c);
    }
    for ch in s.chars() {
        match ch {
            'K' => c.white_kingside = true,
            'Q' => c.white_queenside = true,
            'k' => c.black_kingside = true,
            'q' => c.black_queenside = true,
            _ => return Err(format!("Caractere de roque invalido: '{}'", ch)),
        }
    }
    Ok(c)
}

pub fn to_fen(
    board: &Board,
    turn: Color,
    castling: &CastlingRights,
    ep_target: Option<Square>,
    halfmove: u8,
    fullmove: u16,
) -> String {
    let turn_str = match turn {
        Color::White => "w",
        Color::Black => "b",
    };

    let mut castling_str = String::new();
    if castling.white_kingside {
        castling_str.push('K');
    }
    if castling.white_queenside {
        castling_str.push('Q');
    }
    if castling.black_kingside {
        castling_str.push('k');
    }
    if castling.black_queenside {
        castling_str.push('q');
    }
    if castling_str.is_empty() {
        castling_str.push('-');
    }

    let ep_str = match ep_target {
        Some(sq) => sq.to_algebraic(),
        None => "-".to_string(),
    };

    format!(
        "{} {} {} {} {} {}",
        board.to_fen(),
        turn_str,
        castling_str,
        ep_str,
        halfmove,
        fullmove
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Board;

    #[test]
    fn test_fen_roundtrip_initial() {
        let board = Board::initial();
        let fen = to_fen(&board, Color::White, &CastlingRights::all(), None, 0, 1);
        let expected = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(fen, expected);

        let (parsed_board, parsed_turn, parsed_castling, parsed_ep, parsed_half, parsed_full) =
            parse_fen(&fen).unwrap();
        assert_eq!(parsed_board, board);
        assert_eq!(parsed_turn, Color::White);
        assert_eq!(parsed_castling, CastlingRights::all());
        assert_eq!(parsed_ep, None);
        assert_eq!(parsed_half, 0);
        assert_eq!(parsed_full, 1);
    }

    #[test]
    fn test_fen_parse_after_e4() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let (board, turn, _castling, ep, half, full) = parse_fen(fen).unwrap();
        assert_eq!(turn, Color::Black);
        assert_eq!(ep, Square::from_algebraic("e3"));
        assert_eq!(half, 0);
        assert_eq!(full, 1);

        let white_pawn_e4 = board.piece_at(Square::from_algebraic("e4").unwrap());
        assert!(white_pawn_e4.is_some());
    }

    #[test]
    fn test_fen_invalid_too_few_fields() {
        assert!(parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").is_err());
    }

    #[test]
    fn test_fen_invalid_piece_char() {
        assert!(Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPX/RNBQKBNR").is_err());
    }

    #[test]
    fn test_fen_castling_dash() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1";
        let (_, _, castling, _, _, _) = parse_fen(fen).unwrap();
        assert_eq!(castling, CastlingRights::none());
    }

    #[test]
    fn test_fen_ep_dash() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let (_, _, _, ep, _, _) = parse_fen(fen).unwrap();
        assert!(ep.is_none());
    }

    #[test]
    fn test_fen_invalid_turn() {
        assert!(parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1").is_err());
    }
}
