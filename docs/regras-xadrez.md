# Regras do Xadrez

## O Tabuleiro

O tabuleiro tem 8×8 casas alternando cores claras e escuras.
As colunas são nomeadas de **a** a **h**, as linhas de **1** a **8**.

```
  a b c d e f g h
8 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 8
7 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ 7
6 · · · · · · · · 6
5 · · · · · · · · 5
4 · · · · · · · · 4
3 · · · · · · · · 3
2 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ 2
1 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 1
  a b c d e f g h
```

**Brancas** (♔♕♖♗♘♙) começam na parte de baixo (linhas 1-2).
**Pretas** (♚♛♜♝♞♟) começam na parte de cima (linhas 7-8).

Brancas sempre jogam primeiro.

---

## As Peças

### ♙ Peão
- Anda **1 casa para frente** (ou 2 no primeiro movimento)
- **Captura na diagonal** (1 casa à frente e 1 para o lado)
- **Promoção**: ao alcançar a última fileira (8 para brancas, 1 para pretas), é promovido a ♕ ♖ ♗ ♘
- **En passant**: se um peão avança 2 casas e termina ao lado de um peão adversário na 5ª fileira (brancas) ou 4ª (pretas), o adversário pode capturá-lo como se tivesse andado 1 casa

**Exemplo:** `e4` — peão de e2 para e4

```
Antes:                        Depois:
  a b c d e f g h              a b c d e f g h
2 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ 2         2 ♙ ♙ ♙ ♙ · ♙ ♙ ♙ 2
1 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 1         1 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 1
                              4 · · · · ♙ · · · 4
```

### ♘ Cavalo
- Movimento em "L": 2 casas em uma direção + 1 perpendicular
- **Pula sobre outras peças**

**Exemplo:** `Nf3` — cavalo de g1 para f3

### ♗ Bispo
- Movimento diagonal ilimitado
- Permanece sempre na mesma cor de casa

### ♖ Torre
- Movimento retilíneo (horizontal/vertical) ilimitado

### ♕ Dama
- Combina Torre + Bispo: move-se em qualquer direção (horizontal, vertical, diagonal)

### ♔ Rei
- Move **1 casa** em qualquer direção
- **Roque**: o rei move 2 casas em direção à torre, e a torre pula para o lado oposto
  - **Roque Pequeno (O-O)**: ♔e1→g1, ♖h1→f1
  - **Roque Grande (O-O-O)**: ♔e1→c1, ♖a1→d1
  - Requer: rei e torre nunca movidos, nenhuma peça entre eles, rei não em xeque, rei não passa por casa atacada

**Exemplo Roque Pequeno:**
```
Antes:                        Depois (O-O):
  a b c d e f g h              a b c d e f g h
1 ♖ ♘ ♗ ♕ ♔ · · ♖ 1         1 ♖ ♘ ♗ ♕ · ♖ ♔ · 1
```

---

## Capturas

Uma peça captura ocupando a casa da peça adversária. A peça capturada é removida do tabuleiro.

O peão é a única peça que captura de forma diferente do seu movimento normal: ele anda para frente mas captura na diagonal.

---

## Xeque e Xeque-Mate

**Xeque (+):** o rei está sob ataque. O jogador deve:
1. Mover o rei para uma casa segura
2. Bloquear o xeque (colocar uma peça entre o rei e o atacante)
3. Capturar a peça atacante

**Xeque-Mate (#):** o rei está em xeque e **não há jogada legal** para sair. Fim de jogo — vitória do atacante.

**Exemplo (Scholar's Mate):**
```
1. e4  e5
2. Bc4 Nc6
3. Qh5 Nf6
4. Qxf7#

  a b c d e f g h
8 ♜ · ♝ ♛ ♚ ♝ · ♜ 8
7 ♟ ♟ ♟ ♟ · ♕ ♟ ♟ 7
6 · · ♞ · · ♞ · · 6
5 · · · · ♟ · · · 5
4 · · ♗ · ♙ · · · 4
3 · · · · · · · · 3
2 ♙ ♙ ♙ ♙ · ♙ ♙ ♙ 2
1 ♖ ♘ ♗ · ♔ · ♘ ♖ 1
```

---

## Empate

| Tipo | Descrição |
|------|-----------|
| **Afogamento (Stalemate)** | Rei não está em xeque, mas **nenhuma jogada legal** é possível |
| **Material Insuficiente** | Apenas ♔ vs ♔, ♔+♗ vs ♔, ♔+♘ vs ♔, ♔+♗ vs ♔+♗ (mesma cor) |
| **Tripla Repetição** | Mesma posição (mesmas peças, mesmo turno, mesmos direitos de roque/en passant) ocorre 3 vezes |
| **Regra dos 50 Movimentos** | 50 lances sem captura ou movimento de peão (= 100半-lances) |
| **Acordo** | Ambos os jogadores concordam com o empate |

**Exemplo de Afogamento:**
```
  a b c d e f g h
8 · · · · · · · · 8
7 · · · · · · · · 7
6 · · · · · · · · 6
5 · · · · · · · · 5
4 · · · · · · · · 4
3 · · · · · · · · 3
2 ♛ · · · · · · · 2
1 · ♔ · · · · · · 1
  a b c d e f g h
```
Rei branco em b1, Dama preta em a2. **Vez das brancas** — rei não em xeque, mas a1 e c1 são atacados pela dama. **Afogamento!**

---

## Notação Algébrica

| Notação | Significado |
|---------|-------------|
| `e4` | Peão para e4 |
| `Nf3` | Cavalo para f3 |
| `Nbd2` | Cavalo de b para d2 (desambiguação) |
| `Bxe5` | Bispo captura em e5 |
| `exd5` | Peão captura em d5 |
| `O-O` | Roque pequeno |
| `O-O-O` | Roque grande |
| `e8=Q` | Peão promove a Dama em e8 |
| `Qh4+` | Dama para h4 **dando xeque** |
| `Qxf7#` | Dama captura em f7 **xeque-mate** |
