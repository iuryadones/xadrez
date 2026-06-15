# Diagrama da Arquitetura

## Visão Geral dos Módulos

```
╔═══════════════════════════════════════════╗
║              main.rs (binary)             ║
║  ┌──────────────────────────────────┐    ║
║  │         Game Loop                │    ║
║  │  ┌─────────┐  ┌──────────────┐  │    ║
║  │  │ Render  │  │ Input Parse  │  │    ║
║  │  │ (ANSI)  │  │ (algébrica)  │  │    ║
║  │  └────┬────┘  └──────┬───────┘  │    ║
║  └───────┼──────────────┼──────────┘    ║
╚══════════╪══════════════╪═══════════════╝
           │              │
───────────┼──────────────┼────────────────
╔══════════╪══════════════╪═══════════════╗
║          ▼              ▼               ║
║  ┌──────────────────────────────────┐   ║
║  │            Game                  │   ║
║  │  ┌─────────┐  ┌──────────────┐  │   ║
║  │  │ Board   │  │ GameStatus   │  │   ║
║  │  │ 8×8     │  │ Ongoing,     │  │   ║
║  │  │ array   │  │ Checkmate,   │  │   ║
║  │  └────┬────┘  │ Draw, etc    │  │   ║
║  │       │       └──────────────┘  │   ║
║  │  ┌────▼────┐  ┌──────────────┐  │   ║
║  │  │ Moves  │  │     Fen      │  │   ║
║  │  │ (legal │  │ (parse/      │  │   ║
║  │  │  gen)  │  │  serialize)  │  │   ║
║  │  └────────┘  └──────────────┘  │   ║
║  │  ┌─────────────────────────┐   │   ║
║  │  │  Perft (testes)         │   │   ║
║  │  └─────────────────────────┘   │   ║
║  └──────────────────────────────────┘   ║
║          lib.rs (library)               ║
╚═══════════════════════════════════════════╝
```

## Fluxo de uma Jogada

```
Jogador digita "e4"
        │
        ▼
  parse_algebraic(game, "e4")
        │
        ├── O-O/O-O-O? → busca roque
        ├── coordenada (e2e4)? → busca exata
        └── algébrica (e4, Nf3)? → busca por tipo+destino
        │
        ▼
  game.make_move(mv)
        │
        ├── 1. Valida: mv ∈ legal_moves()
        ├── 2. board.set_piece(to, piece)
        ├── 3. board.set_piece(from, None)
        ├── 4. Trata en passant / roque / promoção
        ├── 5. Atualiza: ep_target, castling rights, relógios
        ├── 6. turn = turn.opponent()
        └── 7. Salva posição no histórico (repetição)
        │
        ▼
  game.status()
        │
        ├── legal_moves() vazia + in_check? → Checkmate
        ├── legal_moves() vazia + !in_check? → Stalemate
        ├── halfmove_clock >= 100? → 50-move rule
        ├── material insuficiente? → Draw
        └── tripla repetição? → Draw
        │
        ▼
  render(game) → ANSI colors no terminal
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
 └── move_history: Vec<Move>
      └── Move { from: Square, to: Square, promotion: Option<PieceType> }
```

## Sistema de Cores ANSI

```
BG_LIGHT = \x1b[48;5;255m  (fundo claro)
BG_DARK  = \x1b[48;5;236m  (fundo escuro)
FG_WHITE = \x1b[97m        (texto branco brilhante — peças brancas)
FG_BLACK = \x1b[90m        (texto cinza — peças pretas)
RESET    = \x1b[0m         (reset)
```

Cada célula do tabuleiro alterna entre BG_LIGHT e BG_DARK,
criando o padrão xadrez.
