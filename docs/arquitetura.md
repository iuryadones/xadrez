# Arquitetura

## Visão Geral

O projeto é dividido em duas camadas:

```
┌─────────────────────────────────────────────┐
│              binary (main.rs)               │
│   Game loop  ·  Render ANSI  ·  Input       │
│   (acoplado ao terminal)                    │
├─────────────────────────────────────────────┤
│              library (lib.rs)               │
│   Board  ·  Game  ·  Moves  ·  Fen          │
│   (independente de UI)                      │
└─────────────────────────────────────────────┘
```

A library (`chess`) não depende de nenhuma crate externa e não faz I/O.
Qualquer UI (terminal, web, desktop) pode consumi-la.

## Módulos

```
src/
├── lib.rs            # Re-exports públicos da library
├── main.rs           # Binary: game loop, render, parsedor de entrada
│
├── board.rs          # Board 8×8, init, from_fen, to_fen, king_square
├── piece.rs          # Color, PieceType, Piece + to_unicode()
├── square.rs         # Square { file, rank } + algebraica
├── mv.rs             # Move { from, to, promotion }
├── moves.rs          # Geração de movimentos legais + Perft
├── game.rs           # Game state, make_move, status, regras de fim
└── fen.rs            # Parse e serialização FEN
```

## Fluxo de Dados

```
                  ┌──────────┐
  Entrada ──────► │  main.rs │
  (e4, Nf3, etc)  └────┬─────┘
                        │ parse_algebraic()
                        ▼
                  ┌──────────┐
                  │  Game    │
                  │ .legal   │
                  │ _moves() │
                  │ .make    │
                  │ _move()  │
                  │ .status()│
                  └────┬─────┘
                       │
              ┌────────┼────────┐
              ▼        ▼        ▼
         ┌────────┐┌──────┐┌────────┐
         │ Board  ││Moves ││  Fen   │
         │ 8×8    ││gen   ││parse/  │
         │ array  ││legal ││serial  │
         └────────┘└──────┘└────────┘
```

## Como integrar com GUI

```rust
use chess::*;

fn main() {
    let mut game = Game::new();

    loop {
        // Seu próprio render() aqui
        render_board_gui(game.board());

        let moves = game.legal_moves();
        // Mostrar movimentos no seu GUI

        if let Some(mv) = player_input() {  // seu input handler
            game.make_move(mv).ok();
        }

        match game.status() {
            GameStatus::Ongoing => continue,
            status => { show_result(status); break; }
        }
    }
}
```

## Dependências

**Library (`chess`):** Nenhuma. Rust std apenas.

**Binary:** Usa ANSI escape codes para cores no terminal — sem crate externa.
