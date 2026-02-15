---
title: Lesson 04 - Sweep Many Float Formats
---

# Lesson 04: Sweep Many Float Formats

## What this lesson is for

A single point test can mislead you. This lesson shows how to evaluate formats across an entire magnitude band.

## Claim

Format quality is distribution-dependent; you need sweeps, not anecdotes.

## Run the sweep

```bash
cargo run -q --bin soft_float_explorer
```

Outputs:

- `docs/soft_float_sweep.svg`
- `docs/soft_float_sweep.csv`
- `docs/soft_float_sweep_summary.md`
- `docs/soft_float_sweep_ranking.md`

![Soft-float sweep](../soft_float_sweep.svg)

## What the tool is doing

1. Samples values as `x = 10^k` over your configured `k` interval.
2. Quantizes each sample with each candidate format.
3. Measures relative error and clipping behavior.
4. Writes plot/table artifacts for comparison.

## How to reason about results

- Long flat low-error regions: stable precision in that band.
- Spikes/discontinuities: clipping boundaries or quantization transitions.
- A good format for one domain can be poor for another.

## Useful variants

```bash
cargo run -q --bin soft_float_explorer -- --format custom,11,-40,40
```

```bash
cargo run -q --bin soft_float_explorer -- --no-presets \
  --add-format tiny,5,-10,10 \
  --add-format medium,10,-30,30 \
  --add-format wide,7,-100,100
```

```bash
cargo run -q --bin soft_float_explorer -- \
  --focus-min -2 --focus-max 1 --focus-weight 8
```

## Continue

Next: [Lesson 05: Discover Good Formats Automatically](05-discover-good-formats)
