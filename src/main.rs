use chess::*;
use std::io::{self, BufRead, Write};

const RESET: &str = "\x1b[0m";
const BG_LIGHT: &str = "\x1b[107m";
const BG_DARK: &str = "\x1b[40m";
const FG_DARK: &str = "\x1b[30m";
const FG_LIGHT: &str = "\x1b[97m";


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
                if !game.undo() {
                    println!("  Nada para desfazer.");
                }
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
                println!("  Jogada invalida. Digite 'moves' para ver as jogadas possiveis.");
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
            let fg = if is_light { FG_DARK } else { FG_LIGHT };
            match board.piece_at(sq) {
                Some(p) => {
                    print!("{}{} {} {}", bg, fg, p.kind.to_unicode_square(p.color, is_light), RESET);
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
    let mut algebraic: Vec<String> = moves.iter().map(|mv| move_to_algebraic(game, mv)).collect();
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
