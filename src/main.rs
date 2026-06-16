use chess::*;
use std::io::{self, BufRead, Write};

const RESET: &str = "\x1b[0m";
const BG_LIGHT: &str = "\x1b[107m";
const BG_DARK: &str = "\x1b[40m";
const FG_DARK: &str = "\x1b[30m";
const FG_LIGHT: &str = "\x1b[97m";

enum Mode {
    PvBot,
    PvP,
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).ok();
    input.trim().to_string()
}

fn choose_mode() -> Mode {
    loop {
        println!("╔══════════════════════════════════╗");
        println!("║        ♔ XADREZ RUST ♚          ║");
        println!("╠══════════════════════════════════╣");
        println!("║  Modo de jogo:                   ║");
        println!("║  1. Jogador vs Computador        ║");
        println!("║  2. Jogador vs Jogador           ║");
        println!("╚══════════════════════════════════╝");
        print!("  Escolha: ");
        io::stdout().flush().unwrap();
        match read_line().as_str() {
            "1" => return Mode::PvBot,
            "2" => return Mode::PvP,
            _ => {
                println!("  Opcao invalida. Digite 1 ou 2.\n");
            }
        }
    }
}

fn choose_difficulty() -> ai::Difficulty {
    loop {
        println!("\n╔══════════════════════════════════╗");
        println!("║        ♔ XADREZ RUST ♚          ║");
        println!("╠══════════════════════════════════╣");
        println!("║  Dificuldade do bot:             ║");
        println!("║  1. Fácil                        ║");
        println!("║  2. Médio                        ║");
        println!("║  3. Difícil                      ║");
        println!("║  4. Aleatório                    ║");
        println!("╚══════════════════════════════════╝");
        print!("  Escolha: ");
        io::stdout().flush().unwrap();
        match read_line().as_str() {
            "1" => return ai::Difficulty::Easy,
            "2" => return ai::Difficulty::Medium,
            "3" => return ai::Difficulty::Hard,
            "4" => return ai::random_difficulty(),
            _ => {
                println!("  Opcao invalida. Digite 1 a 4.");
            }
        }
    }
}

fn show_banner() {
    println!("╔══════════════════════════════════╗");
    println!("║        ♔ XADREZ RUST ♚          ║");
    println!("╠══════════════════════════════════╣");
    println!("║ Comandos:                        ║");
    println!("║  e4, Nf3, O-O   → jogada        ║");
    println!("║  moves          → jogadas legais ║");
    println!("║  fen            → mostrar FEN    ║");
    println!("║  undo           → desfazer       ║");
    println!("║  quit/exit      → sair           ║");
    println!("║  help            → ajuda          ║");
    println!("╚══════════════════════════════════╝");
    println!();
}

fn main() {
    match choose_mode() {
        Mode::PvP => run_pvp(),
        Mode::PvBot => {
            let difficulty = choose_difficulty();
            run_pvbot(difficulty);
        }
    }
}

fn game_loop<F>(on_turn: F)
where
    F: Fn(&mut Game, &mut dyn Write) -> bool,
{
    let mut game = Game::new();
    let mut stdout = io::stdout();

    show_banner();

    loop {
        render(&game);

        match game.status() {
            GameStatus::Ongoing => {}
            GameStatus::WhiteWins => {
                println!("  ♔ XEQUE-MATE! Brancas venceram!");
                break;
            }
            GameStatus::BlackWins => {
                println!("  ♚ XEQUE-MATE! Pretas venceram!");
                break;
            }
            GameStatus::Draw => {
                println!("  Empate!");
                break;
            }
        }

        if !on_turn(&mut game, &mut stdout) {
            break;
        }
    }
}

fn run_pvp() {
    game_loop(|game, stdout| {
        let player = match game.turn() {
            Color::White => "Brancas",
            Color::Black => "Pretas",
        };

        if game.in_check() {
            print!("  XEQUE! ");
        }
        print!("{}: ", player);
        stdout.flush().unwrap();

        handle_player_input(game)
    });
}

fn run_pvbot(difficulty: ai::Difficulty) {
    let bot_color = ai::coin_flip();
    let player_color = bot_color.opponent();

    let diff_name = match difficulty {
        ai::Difficulty::Easy => "Fácil",
        ai::Difficulty::Medium => "Médio",
        ai::Difficulty::Hard => "Difícil",
    };

    println!(
        "  Você joga de {} | Bot joga de {} ({})",
        ai::color_name(player_color),
        ai::color_name(bot_color),
        diff_name,
    );
    println!();

    game_loop(|game, stdout| {
        if game.turn() == bot_color {
            print!("  {} pensando...", ai::king_symbol(bot_color));
            stdout.flush().unwrap();

            match ai::best_move_with_depth(game, difficulty.depth()) {
                Some(mv) => {
                    let alg = move_to_algebraic(game, &mv);
                    println!("\r  {} jogou {}  ", ai::king_symbol(bot_color), alg);
                    game.make_move(mv).ok();
                }
                None => {
                    println!("\r  {} sem jogadas legais.", ai::king_symbol(bot_color));
                }
            }
            true
        } else {
            let player = match game.turn() {
                Color::White => "Brancas",
                Color::Black => "Pretas",
            };

            if game.in_check() {
                print!("  XEQUE! ");
            }
            print!("{}: ", player);
            stdout.flush().unwrap();

            handle_player_input(game)
        }
    });
}

fn handle_player_input(game: &mut Game) -> bool {
    let input = read_line();

    match input.as_str() {
        "quit" | "exit" | "q" => return false,
        "moves" | "mov" => {
            show_legal_moves(game);
            return true;
        }
        "fen" => {
            println!("  {}", game.to_fen());
            return true;
        }
        "undo" => {
            if !game.undo() {
                println!("  Nada para desfazer.");
            }
            return true;
        }
        "help" | "h" => {
            show_help();
            return true;
        }
        "" => return true,
        _ => {}
    }

    match parse_algebraic(game, &input) {
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
    true
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
