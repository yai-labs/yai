# TEST_EMBED (Manuale)

## Precondizioni
- `yai` buildato
- Modello embeddings scaricato in `~/.yai/models/embeddings/<model>`

## Test 1 — Fallback hash
```bash
yai embed --text "ciao mondo"
```
Atteso:
- `model: hash (fallback)`
- `dim: 16`

## Test 2 — Candle
```bash
yai embed --model mini --text "ciao mondo"
```
Atteso:
- `model: mini`
- `path: ~/.yai/models/embeddings/mini`
- `dim: <model_dim>`

