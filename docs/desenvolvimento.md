# Desenvolvimento

## Pré-requisitos

- **Rust**: edição 2021 (instale via [rustup](https://rustup.rs))
- **Make**: para usar atalhos do Makefile

## Comandos

```bash
make setup    # Instalar/atualizar Rust toolchain
make build    # Compilar o projeto
make clean    # Limpar artefatos de build
make test     # Rodar todos os testes
make run      # Executar o jogo
make fmt      # Formatar código (cargo fmt)
make lint     # Verificar lints (cargo clippy)
make check    # Verificar se compila
```

## Estrutura do Projeto

```
src/
├── main.rs        ← Binary: game loop, terminal I/O
├── lib.rs         ← Re-exports públicos
├── square.rs      ← Square { file, rank }
├── piece.rs       ← Color, PieceType, Piece + to_unicode / to_unicode_square
├── board.rs       ← Board 8×8, init, FEN
├── mv.rs          ← Move { from, to, promotion }
├── moves.rs       ← Geração de movimentos legais + Perft
├── game.rs        ← Game state, make_move, status
└── fen.rs         ← Parse/serialize FEN
```

## Testes

```bash
cargo test                    # Todos os testes
cargo test -- --nocapture     # Com saída visível
cargo test <nome_do_teste>    # Teste específico
```

### Perft

O projeto inclui testes Perft que validam a geração de movimentos:

| Depth | Nós Esperados | Nós Obtidos |
|-------|---------------|-------------|
| 1     | 20            | 20 ✓ |
| 2     | 400           | 400 ✓ |
| 3     | 8.902         | 8.902 ✓ |
| 4     | 197.281       | 197.281 ✓ |

## Diretrizes

- **Zero `unsafe`**: todo código deve ser 100% safe
- **Zero dependências externas**: apenas Rust std
- **Library pura**: `Game`, `Board`, `Move` não fazem I/O
- **Separação UI/lógica**: `main.rs` cuida do terminal, `lib.rs` é independente
- **cargo fmt + clippy**: obrigatório antes de todo commit

## Como adicionar uma nova funcionalidade

1. Crie ou modifique o módulo relevante em `src/`
2. Adicione testes unitários no mesmo arquivo (dentro de `#[cfg(test)]`)
3. Execute `cargo fmt && cargo clippy && cargo test`
4. Se aplicável, atualize a documentação em `docs/`
