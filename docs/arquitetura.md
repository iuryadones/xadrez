# Arquitetura

## Visão Geral

O projeto é dividido em duas camadas:

```
┌─────────────────────────────────────────────┐
│              binary (main.rs)               │
│   Game loop  ·  Render ANSI                 │
│   (acoplado ao terminal)                    │
├─────────────────────────────────────────────┤
│           chess-wasm (Yew + WASM)            │
│   Componentes  ·  State reducer  ·  CSS     │
│   (acoplado ao navegador)                   │
├─────────────────────────────────────────────┤
│              library (lib.rs)               │
│   Board  ·  Game  ·  Moves  ·  Fen          │
│   Notation  ·  (independente de UI)         │
└─────────────────────────────────────────────┘
```

A library (`chess`) não depende de nenhuma crate externa e não faz I/O.
Qualquer UI (terminal, web, desktop) pode consumi-la.

## Módulos

### Library (src/)

```
src/
├── lib.rs            # Re-exports públicos da library
├── board.rs          # Board 8×8, init, from_fen, to_fen, king_square
├── piece.rs          # Color, PieceType, Piece + to_unicode() / to_unicode_square()
├── square.rs         # Square { file, rank } + FromStr
├── mv.rs             # Move { from, to, promotion }
├── moves.rs          # Geração de movimentos legais + Perft
├── game.rs           # Game state, make_move, undo, status, regras de fim
├── fen.rs            # Parse e serialização FEN
└── notation.rs       # Notação algébrica: move_to_algebraic, parse_algebraic, disambiguation
```

### Frontend WASM (chess-wasm/)

```
chess-wasm/src/
├── main.rs           # Entrypoint Yew
├── app.rs            # Componente App (layout)
├── state.rs          # GameState reducer (Select, MakeMove, Undo, NewGame, Promotion)
├── render.rs         # Helpers de render (cor, glyph)
└── components/
    ├── chess_board.rs     # Grid 8×8 com click-to-move
    ├── square_tile.rs     # Casa individual com labels rank/file
    ├── piece_icon.rs      # Ícone Unicode com cor
    ├── status_bar.rs      # Turno, xeque, resultado
    ├── move_input.rs      # Input de notação algébrica
    ├── move_list.rs       # Histórico de lances
    ├── new_game.rs        # Botão novo jogo
    ├── undo_button.rs     # Botão desfazer
    ├── fen_display.rs     # FEN + copiar
    └── promotion_dialog.rs# Seletor de promoção (modal)
```

## Fluxo de Dados

**Terminal:**
```
                  ┌──────────┐
  Entrada ──────► │  main.rs │
  (e4, Nf3, etc)  └────┬─────┘
                        │ parse_algebraic() (em notation.rs)
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

**WASM:**
```
  Click/Input ──► state.dispatch(GameAction)
                       │
                       ▼
                  GameState::reduce()
                       │
              ┌────────┼────────┐
              ▼        ▼        ▼
         Game::     Game::    Game::
         make_move  legal_    status
                    moves
                       │
                       ▼
                  Componentes re-renderizam
```

A notação algébrica (`parse_algebraic`, `move_to_algebraic`) vive em `notation.rs` na library e é compartilhada entre o terminal e o WASM — sem duplicação de código.

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

**Binary (terminal):** ANSI escape codes — sem crate externa.

**Binary (WASM):** Yew 0.21, web-sys, wasm-bindgen.
