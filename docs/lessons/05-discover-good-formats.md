---
title: Lesson 05 - Discover Good Formats Automatically
---

# Lesson 05: Discover Good Formats Automatically

Now we use the sweep results to rank candidate formats.

## Run ranking

```bash
cargo run -q --bin soft_float_explorer
```

This now generates:

- `docs/soft_float_sweep.svg`
- `docs/soft_float_sweep.csv`
- `docs/soft_float_sweep_summary.md`
- `docs/soft_float_sweep_ranking.md`

## Ranking formula in code

```rust
score = log10(mean_rel_err)
      + w_max * log10(max_rel_err)
      + p_under * underflow_frac
      + p_over * overflow_frac
```

## How ranking works

Each format gets a score (lower is better):

- `mean_rel_err`: typical precision
- `max_rel_err`: worst-case precision
- `underflow_frac`: how often values collapse to zero
- `overflow_frac`: how often values collapse to infinity

This is a practical default, not universal truth. You can tune these weights from the CLI for your domain.

## Workflow to find a good format

1. Define candidate formats (mantissa + exponent range).
2. Sweep the magnitude range you care about (`k` range).
3. Check ranking table for top candidates.
4. Inspect plot and CSV for edge-case behavior.
5. Iterate with adjusted bit allocations.

## Example: compare three custom designs with domain focus

```bash
cargo run -q --bin soft_float_explorer -- --no-presets \
  --k-min -12 --k-max 12 --k-step 0.05 \
  --focus-min -2 --focus-max 1 --focus-weight 8 \
  --out docs/data/app_domain \
  --add-format small,7,-20,20 \
  --add-format balanced,10,-20,20 \
  --add-format wide,7,-60,60
```

Then read:

- `docs/data/app_domain_ranking.md`

Use this to answer: "Which representation is best for my value range and tolerance?"
