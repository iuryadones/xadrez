# Contribuindo

Obrigado pelo interesse em contribuir com este projeto!

## Como contribuir

1. **Issues**: Reporte bugs ou sugira melhorias abrindo uma issue.
2. **Pull Requests**: Envie PRs com suas alterações.

## Pré-requisitos

- Rust nightly (para fmt)
- `trunk` — WASM bundler: `cargo install trunk`
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

## Padrões de código

- **Linguagem**: Rust edition 2021
- **Estilo**: `cargo fmt` antes de commitar
- **Lints**: `cargo clippy -- -D warnings` — sem warnings
- **Testes**: `cargo test` — 67 testes + 1 ignored (Perft 5 ≈ 4.8M nós)
- **Segurança**: zero `unsafe` — código 100% safe

## Estrutura do projeto

```
src/
├── main.rs          ← Binary: terminal UI (ANSI, game loop, comandos)
├── lib.rs           ← Library: Game, Board, Moves, Fen, Perft
├── board.rs
├── piece.rs
├── square.rs
├── mv.rs
├── moves.rs
├── game.rs
├── fen.rs
├── perft.rs
└── notation.rs      ← Shared: parser algébrico (terminal + WASM)
```

Veja [docs/arquitetura.md](docs/arquitetura.md) para descrição detalhada.

## Build e execução

```bash
# Terminal
cargo run --release

# WASM (desenvolvimento — hot reload)
cd chess-wasm && trunk serve --open

# WASM (release)
cd chess-wasm && trunk build --release
```

## Fluxo de contribuição

```
1. Fork o repositório
2. Crie um branch: git checkout -b minha-feature
3. Faça suas alterações
4. Rode cargo fmt && cargo clippy && cargo test
5. Commit e push
6. Abra um Pull Request
```

## Licença

Este projeto está licenciado sob MIT — veja [LICENSE](LICENSE).
