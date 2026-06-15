# Contribuindo

Obrigado pelo interesse em contribuir com este projeto!

## Como contribuir

1. **Issues**: Reporte bugs ou sugira melhorias abrindo uma issue.
2. **Pull Requests**: Envie PRs com suas alterações.

## Padrões de código

- **Linguagem**: Rust edition 2021
- **Estilo**: `cargo fmt` antes de commitar
- **Lints**: `cargo clippy -- -D warnings` — sem warnings
- **Testes**: `cargo test` — todos os testes devem passar
- **Segurança**: zero `unsafe` — código 100% safe

## Estrutura do projeto

Veja [docs/arquitetura.md](docs/arquitetura.md) para entender a organização dos módulos.

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
