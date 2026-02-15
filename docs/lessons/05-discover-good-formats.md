---
title: Lesson 05 - Discover Good Formats Automatically
---

# Lesson 05: Discover Good Formats Automatically

## What this lesson is for

After sweeping many candidates, you need a repeatable way to pick one.
This lesson introduces a ranking objective and how to tune it.

## Claim

"Best" format is an objective function, not a universal constant.

## Run ranking

```bash
cargo run -q --bin soft_float_explorer
```

Generated artifacts include:

- `docs/soft_float_sweep_ranking.md`
- `docs/soft_float_sweep_summary.md`

## Scoring model

```rust
score = log10(mean_rel_err)
      + w_max * log10(max_rel_err)
      + p_under * underflow_frac
      + p_over * overflow_frac
```

## Interpretation

- `mean_rel_err`: everyday quality.
- `max_rel_err`: protects against bad tails.
- `underflow_frac`/`overflow_frac`: explicit clipping penalties.

If your application is tail-sensitive, increase `w_max`.
If clipping is unacceptable, increase `p_under` and `p_over`.

## Domain-focused example

```bash
cargo run -q --bin soft_float_explorer -- --no-presets \
  --k-min -12 --k-max 12 --k-step 0.05 \
  --focus-min -2 --focus-max 1 --focus-weight 8 \
  --out docs/data/app_domain \
  --add-format small,7,-20,20 \
  --add-format balanced,10,-20,20 \
  --add-format wide,7,-60,60
```

Then inspect `docs/data/app_domain_ranking.md`.

## Continue

Next: [Lesson 06: Domain Profiles and Asymmetric Precision](06-domain-profiles)
