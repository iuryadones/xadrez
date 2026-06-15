use chess::*;
use std::io::{self, BufRead, Write};

const RESET: &str = "\x1b[0m";
const BG_LIGHT: &str = "\x1b[48;5;255m";
const BG_DARK: &str = "\x1b[48;5;236m";
const FG_WHITE: &str = "\x1b[97m";
const FG_BLACK: &str = "\x1b[90m";

fn main() {
    let mut game = Game::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("╔══════════════════════════════════╗");
    println!("║        ♔ XADREZ RUST ♚          ║");
    println!("╠══════════════════════════════════╣");
    println!("║ Comandos:                       ║");
    println!("║  e4, Nf3, O-O   → jogada        ║");
    println!("║  moves          → jogadas legais║");
    println!("║  fen            → mostrar FEN   ║");
    println!("║  undo           → desfazer      ║");
    println!("║  quit/exit      → sair          ║");
    println!("╚══════════════════════════════════╝");
    println!();

    loop {
        render(&game);

        match game.status() {
            GameStatus::Ongoing => {}
            GameStatus::WhiteWins => {
                println!("♔ XEQUE-MATE! Brancas venceram!");
                break;
            }
            GameStatus::BlackWins => {
                println!("♚ XEQUE-MATE! Pretas venceram!");
                break;
            }
            GameStatus::Draw => {
                println!("Empate!");
                break;
            }
        }

        let player = match game.turn() {
            Color::White => "Brancas",
            Color::Black => "Pretas",
        };

        if game.in_check() {
            print!("  XEQUE! ");
        }
        print!("{}: ", player);
        stdout.flush().unwrap();

        let mut input = String::new();
        if stdin.lock().read_line(&mut input).is_err() {
            break;
        }
        let input = input.trim();

        match input {
            "quit" | "exit" | "q" => break,
            "moves" | "mov" => {
                show_legal_moves(&game);
                continue;
            }
            "fen" => {
                println!("{}", game.to_fen());
                continue;
            }
            "undo" => {
                eprintln!("Undo nao implementado (sem historico de board)");
                continue;
            }
            "help" | "h" => {
                show_help();
                continue;
            }
            "" => continue,
            _ => {}
        }

        match parse_algebraic(&game, input) {
            Some(mv) => match game.make_move(mv) {
                Ok(()) => {}
                Err(e) => {
                    println!("  Erro: {}", e);
                }
            },
            None => {
                println!(
                    "  Jogada invalida. Digite 'moves' para ver as jogadas possiveis."
                );
            }
        }
    }
}

fn render(game: &Game) {
    println!();
    let board = game.board();
    println!("   a  b  c  d  e  f  g  h");
    for rank in (0..8).rev() {
        print!("{} ", rank + 1);
        for file in 0..8 {
            let sq = Square::new_unchecked(file, rank);
            let is_light = (rank + file) % 2 == 0;
            let bg = if is_light { BG_LIGHT } else { BG_DARK };
            match board.piece_at(sq) {
                Some(p) => {
                    let fg = match p.color {
                        Color::White => FG_WHITE,
                        Color::Black => FG_BLACK,
                    };
                    print!("{}{} {} {}", bg, fg, p.kind.to_unicode(p.color), RESET);
                }
                None => {
                    print!("{}   {}", bg, RESET);
                }
            }
        }
        println!(" {}", rank + 1);
    }
    println!("   a  b  c  d  e  f  g  h");
    println!();
}

fn show_legal_moves(game: &Game) {
    let moves = game.legal_moves();
    println!("  Jogadas legais ({}):", moves.len());
    let mut algebraic: Vec<String> = moves
        .iter()
        .map(|mv| move_to_algebraic(game, mv))
        .collect();
    algebraic.sort();

    for chunk in algebraic.chunks(6) {
        print!("    ");
        for (i, m) in chunk.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", m);
        }
        println!();
    }
}

fn show_help() {
    println!("  Comandos:");
    println!("    e4, Nf3, O-O   → jogada em notacao algebraica");
    println!("    e2e4           → jogada em notacao de coordenadas");
    println!("    moves          → listar jogadas legais");
    println!("    fen            → mostrar FEN atual");
    println!("    quit / exit    → sair");
    println!();
    println!("  Exemplos de notacao:");
    println!("    e4        → peao para e4");
    println!("    Nf3       → cavalo para f3");
    println!("    Nbd2      → cavalo de b para d2");
    println!("    Bxe5      → bispo captura em e5");
    println!("    exd5      → peao captura em d5");
    println!("    O-O       → roque pequeno");
    println!("    O-O-O     → roque grande");
    println!("    e8=Q      → promocao para dama");
}

fn move_to_algebraic(game: &Game, mv: &Move) -> String {
    let piece = match game.board().piece_at(mv.from) {
        Some(p) => p,
        None => return mv.to_coordinate(),
    };

    let is_capture = game.board().piece_at(mv.to).is_some()
        || (piece.kind == PieceType::Pawn
            && mv.from.file != mv.to.file
            && game.ep_target().map_or(false, |ep| mv.to == ep));

    match piece.kind {
        PieceType::King => {
            let file_diff = mv.to.file as isize - mv.from.file as isize;
            if file_diff.abs() == 2 {
                return if file_diff > 0 {
                    "O-O".to_string()
                } else {
                    "O-O-O".to_string()
                };
            }
            let mut s = String::from("K");
            if is_capture {
                s.push('x');
            }
            s.push_str(&mv.to.to_algebraic());
            s
        }
        PieceType::Queen => {
            let mut s = String::from("Q");
            s.push_str(&disambiguation(game, mv, piece));
            if is_capture {
                s.push('x');
            }
            s.push_str(&mv.to.to_algebraic());
            if let Some(p) = mv.promotion {
                s.push('=');
                s.push(p.to_char().to_ascii_uppercase());
            }
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
    }
}

fn disambiguation(game: &Game, mv: &Move, piece: Piece) -> String {
    let moves = game.legal_moves();
    let same_target: Vec<&Move> = moves
        .iter()
        .filter(|lm| {
            lm.to == mv.to
                && lm.from != mv.from
                && game.board().piece_at(lm.from) == Some(piece)
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

fn parse_algebraic(game: &Game, input: &str) -> Option<Move> {
    let input = input.trim().replace('+', "").replace('#', "");

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

    let input = input.replace('=', "");

    if let Some(mv) = try_parse_coordinate(game, &input) {
        return Some(mv);
    }

    try_parse_algebraic_move(game, &input)
}

fn try_parse_coordinate(game: &Game, input: &str) -> Option<Move> {
    let clean = input.replace('=', "");
    let promo = clean.as_bytes().len() > 4
        && PieceType::from_char(clean.as_bytes()[4] as char).is_some();

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

    game.legal_moves().into_iter().find(|mv| {
        mv.from == from
            && mv.to == to
            && mv.promotion == promotion
    })
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
        return parse_pawn_move(game, &rest, promotion);
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
            p.map_or(false, |p| p.kind == piece_type && p.color == game.turn())
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
        } else if disambig.len() >= 2 {
            if from.to_algebraic().starts_with(disambig) {
                return Some(*mv);
            }
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
            game.board().piece_at(mv.from)
                .map_or(false, |p| p.kind == PieceType::Pawn && p.color == game.turn())
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

    let x_index = input.rfind('x');
    if let Some(idx) = x_index {
        let from_file = (input.as_bytes()[idx.saturating_sub(1).max(0)] - b'a') as usize;
        return candidates.into_iter().find(|mv| mv.from.file == from_file);
    }

    candidates.into_iter().next()
}
