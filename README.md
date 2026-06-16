# ♔ Xadrez Rust ♚

[![Rust](https://img.shields.io/badge/Rust-1.85%2B-blue)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)
![Zero unsafe](https://img.shields.io/badge/unsafe-0%25-brightgreen)

Jogo de xadrez completo implementado em Rust, jogável no terminal e via navegador (WebAssembly).

## Funcionalidades

- ✅ Jogo completo 2 jogadores no terminal + WebAssembly
- ✅ Modo PvBot com dificuldade ajustável (Fácil/Médio/Difícil/Aleatório)
- ✅ Todas as regras: roque, en passant, promoção
- ✅ Detecção de xeque, xeque-mate, afogamento
- ✅ Material insuficiente, regra dos 50/75 movimentos, tripla/quíntupla repetição
- ✅ Notação algébrica (e4, Nf3, O-O, Bxe5, exd5)
- ✅ Notação de coordenadas (e2e4)
- ✅ Cores ANSI com padrão xadrez (terminal)
- ✅ Interface gráfica responsiva (WASM)
- ✅ Desfazer jogadas (undo)
- ✅ Diálogo de promoção (WASM)
- ✅ Posições via FEN (importar/exportar/copiar)
- ✅ Zero dependências externas (core), 100% safe Rust
- ✅ API modular para integrar com GUI

## Quick Start

```bash
make run
```

## Documentação

| Documento | Descrição |
|-----------|-----------|
| [Arquitetura](docs/arquitetura.md) | Estrutura de módulos e fluxo de dados |
| [API](docs/api.md) | Documentação da API pública |
| [Regras do Xadrez](docs/regras-xadrez.md) | Regras completas com exemplos |
| [Desenvolvimento](docs/desenvolvimento.md) | Setup, build, teste |

## Comandos

```bash
make setup       # Instalar Rust toolchain
make build       # Compilar
make test        # Rodar testes (67 testes + 1 ignorado)
make run         # Executar jogo no terminal
make web-run     # Servir frontend WASM com hot reload (trunk serve)
make web-build   # Compilar WASM release
make web-deploy  # Compilar e publicar no GitHub Pages
make fmt         # Formatar código
make lint        # Verificar lints
```

## Como Jogar

```
Comandos do jogo:
  e4, Nf3, O-O   → jogada em notação algébrica
  e2e4           → jogada em coordenadas
  moves          → listar jogadas legais
  fen            → mostrar FEN atual
  quit / exit    → sair
```

Ao executar `make run`, o jogo pergunta:
1. **Modo**: `1` — Jogador vs Computador / `2` — Jogador vs Jogador
2. **Dificuldade** (modo PvBot): `1` Fácil / `2` Médio / `3` Difícil / `4` Aleatório
3. O bot começa com cor sorteada automaticamente.

## Licença

MIT
