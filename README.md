# ♔ Xadrez Rust ♚

[![Rust](https://img.shields.io/badge/Rust-1.85%2B-blue)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)
![Zero unsafe](https://img.shields.io/badge/unsafe-0%25-brightgreen)

Jogo de xadrez completo implementado em Rust, jogável no terminal.

## Funcionalidades

- ✅ Jogo completo 2 jogadores no terminal
- ✅ Todas as regras: roque, en passant, promoção
- ✅ Detecção de xeque, xeque-mate, afogamento
- ✅ Material insuficiente, regra dos 50 movimentos, tripla repetição
- ✅ Notação algébrica (e4, Nf3, O-O, Bxe5, exd5)
- ✅ Cores ANSI com padrão xadrez
- ✅ Zero dependências externas, 100% safe Rust
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
make setup     # Instalar Rust toolchain
make build     # Compilar
make test      # Rodar testes (16 testes)
make run       # Executar jogo
make fmt       # Formatar código
make lint      # Verificar lints
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

## Licença

MIT
