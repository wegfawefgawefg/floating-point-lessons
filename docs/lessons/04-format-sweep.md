# Lesson 04: Sweep Many Float Formats

Use the general evaluator:

```bash
cargo run -q --bin soft_float_explorer
```

Default outputs:

- `docs/soft_float_sweep.svg`
- `docs/soft_float_sweep.csv`
- `docs/soft_float_sweep_summary.md`
- `docs/soft_float_sweep_ranking.md`

## What it does

- Samples values as `x = 10^k`
- Quantizes each value through each configured software format
- Computes relative error
- Plots `log10(relative error)` across k-range
- Ranks formats by a practical score (see Lesson 05)

## Add your own formats

Replace presets with one format:

```bash
cargo run -q --bin soft_float_explorer -- --format custom,11,-40,40
```

Compare several custom formats:

```bash
cargo run -q --bin soft_float_explorer -- --no-presets \
  --add-format tiny,5,-10,10 \
  --add-format medium,10,-30,30 \
  --add-format wide,7,-100,100
```

Tune sampling and output path:

```bash
cargo run -q --bin soft_float_explorer -- \
  --k-min -30 --k-max 30 --k-step 0.05 \
  --out docs/data/my_sweep
```

Add domain focus weighting:

```bash
cargo run -q --bin soft_float_explorer -- \
  --focus-min -2 --focus-max 1 --focus-weight 8
```

This is the general form: define formats, sweep many representations, and characterize precision over range.
