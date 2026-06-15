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
BG_LIGHT = \x1b[48;5;255m  (fundo claro — bege)
BG_DARK  = \x1b[48;5;236m  (fundo escuro — cinza)
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
