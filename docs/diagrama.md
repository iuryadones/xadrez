# Diagrama da Arquitetura

## Visão Geral dos Módulos

```
╔══════════════════════════════════════════════════╗
║           main.rs (binary terminal)              ║
║  ┌──────────────────────────────────┐           ║
║  │         Game Loop                │           ║
║  │  ┌─────────┐  ┌──────────────┐  │           ║
║  │  │ Render  │  │  Comandos    │  │           ║
║  │  │ (ANSI)  │  │ moves/fen/   │  │           ║
║  │  └────┬────┘  │ undo/help    │  │           ║
║  └───────┼───────┴──────────────┘  │           ║
╚══════════╪══════════════════════════════════════╝
           │
╔══════════╪══════════════════════════════════════╗
║          ▼               chess-wasm/            ║
║  ┌──────────────────────────────────────┐      ║
║  │   Yew App (Componentes)              │      ║
║  │  ┌─────────┐ ┌──────────┐ ┌───────┐ │      ║
║  │  │Chess    │ │MoveInput │ │Status │ │      ║
║  │  │Board    │ │(parse    │ │Bar    │ │      ║
║  │  │(click)  │ │algébrico)│ │       │ │      ║
║  │  └────┬────┘ └────┬─────┘ └───┬───┘ │      ║
║  │  ┌────▼────┐ ┌────▼─────┐ ┌───▼───┐ │      ║
║  │  │Promotion│ │NewGame / │ │Fen    │ │      ║
║  │  │Dialog   │ │Undo      │ │Display│ │      ║
║  │  └─────────┘ └──────────┘ └───────┘ │      ║
║  └──────────────┬───────────────────────┘      ║
╚═════════════════╪═══════════════════════════════╝
                  │
──────────────────┼────────────────────────────────
╔═════════════════╪═══════════════════════════════╗
║                 ▼              lib.rs (library) ║
║  ┌─────────────────────────────────────────┐   ║
║  │                Game                     │   ║
║  │  ┌─────────┐  ┌─────────────────┐       │   ║
║  │  │ Board   │  │   GameStatus    │       │   ║
║  │  │ 8×8     │  │ Ongoing,        │       │   ║
║  │  │ array   │  │ WhiteWins, etc  │       │   ║
║  │  └────┬────┘  └─────────────────┘       │   ║
║  │       │                                  │   ║
║  │  ┌────▼────┐  ┌──────────────┐  ┌──────┐┌──────┐║
║  │  │ Moves   │  │  Notation    │  │ Fen  ││ AI   │║
║  │  │ (legal  │  │ move_to_     │  │parse ││(TT + │║
║  │  │  gen)   │  │ algebraic /  │  │/ser  ││Zobr.)│║
║  │  │ Perft   │  │ parse_       │  │ialize│└──────┘║
║  │  └─────────┘  │ algebraic    │  └──────┘        ║
║  │                └──────────────┘          │   ║
║  └──────────────────────────────────────────┘   ║
╚══════════════════════════════════════════════════╝
```

## Fluxo de uma Jogada (Terminal)

```
Jogador digita "e4"
        │
        ▼
  parse_algebraic(game, "e4")    ← em notation.rs
        │
        ├── O-O/O-O-O? → busca roque
        ├── coordenada (e2e4)? → busca exata
        └── algébrica (e4, Nf3)? → busca por tipo+destino
        │
        ▼
  game.make_move(mv)
        │
        ├── 1. Valida: mv ∈ legal_moves()
        ├── 2. Salva Snapshot (undo)
        ├── 3. board.set_piece(to, piece)
        ├── 4. board.set_piece(from, None)
        ├── 5. Trata en passant / roque / promoção
        ├── 6. Atualiza: ep_target, castling rights, relógios
        ├── 7. turn = turn.opponent()
        └── 8. Salva posição no histórico (repetição)
        │
        ▼
  game.status()
        │
        ├── legal_moves() vazia + in_check? → Checkmate
        ├── legal_moves() vazia + !in_check? → Stalemate
        ├── halfmove_clock >= 150? → 75-move rule (Draw automático)
        ├── halfmove_clock >= 100? → 50-move rule (Draw)
        ├── fivefold repetition (≥ 5)? → Draw automático
        ├── threefold repetition (≥ 3)? → Draw
        ├── material insuficiente? → Draw
        └── senão → Ongoing
        │
        ▼
  render(game) → ANSI colors no terminal
```

## Fluxo de uma Jogada do Bot

```
   Bot joga (PvBot):
         │
         ▼
   ai::best_move_with_depth(game, depth)
         │
         ├── 1. TT probe (hash Zobrist)
         ├── 2. Se hit com flag válida → retorna score
         ├── 3. Gera legal_moves()
         ├── 4. Ordena: TT best move → MVV-LVA capturas → resto
         ├── 5. Negamax com PID + LMR
         ├── 6. Quiescence (capturas até quietude)
         └── 7. TT record (hash, depth, score, flag, best_move)
         │
         ▼
   game.make_move(mv)  →  loop volta ao topo
```

## Fluxo de uma Jogada (WASM)

```
Usuário clica em peça ou digita notação
        │
        ▼
  state.dispatch(GameAction)
        │
        ├── Select(sq) → highlight + legal_moves_for_selected
        ├── MakeMove(mv)
        │     ├── Se mv.promotion.is_none() e existem candidatos com
        │     │   promoção → pending_promotion = Some(candidates)
        │     │   → PromotionDialog aparece
        │     └── Senão → game.make_move(mv) + move_history.push()
        ├── RequestPromotion(from, to) → abre diálogo
        ├── PromotionSelected(mv) → faz jogada de promoção
        ├── Undo → game.undo() + pop history
        └── NewGame → GameState::default()
        │
        ▼
  Componentes re-renderizam
```

## Hierarquia de Dados

```
Game
 ├── board: Board
 │    └── squares: [[Option<Piece>; 8]; 8]
 │         └── Piece { kind: PieceType, color: Color }
 ├── turn: Color
 ├── castling: CastlingRights
 ├── ep_target: Option<Square>
 ├── halfmove_clock: u8
 ├── fullmove_number: u16
 ├── position_history: Vec<String>
 ├── move_history: Vec<Move>
 │    └── Move { from: Square, to: Square, promotion: Option<PieceType> }
 └── undo_stack: Vec<Snapshot>
      └── Snapshot { board, turn, castling, ep_target, halfmove_clock,
                     fullmove_number, position_history, move_history }
```

## Sistema de Cores (Terminal ANSI)

```
BG_LIGHT = \x1b[107m       (fundo claro — branco brilhante)
BG_DARK  = \x1b[40m        (fundo escuro — preto)
FG_DARK  = \x1b[30m        (texto preto — usado em casas claras)
FG_LIGHT = \x1b[97m        (texto branco brilhante — usado em casas escuras)
RESET    = \x1b[0m         (reset)
```

A cor do texto (foreground) é definida pela cor da **casa**, não pela cor da peça:
- Casas claras → texto escuro (contraste máximo)
- Casas escuras → texto claro (contraste máximo)

O glifo (outline vs sólido) depende da **cor da peça combinada com a casa**,
criando regras inversas para cada jogador:

| Jogador | Casa clara | Casa escura |
|---------|------------|-------------|
| Brancas (♔) | outline ♙ + preto | sólido ♟ + branco |
| Pretas (♚)  | sólido ♟ + preto | outline ♔ + branco |

Isto garante que peças de jogadores diferentes no mesmo tipo de casa
usem glifos opostos, mantendo-se sempre distinguíveis.

## Dependências

**Library (`chess`):** Nenhuma. Rust std apenas.

**Binary (terminal):** ANSI escape codes — sem crate externa.

**Binary (WASM):** Yew 0.21, web-sys, wasm-bindgen.
