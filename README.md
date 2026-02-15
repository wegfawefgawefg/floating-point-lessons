# prectest

Tutorial and scratch repo for understanding floating-point precision in Rust (`f32` vs `f64`), where it works well, and where it breaks down.

## Learning path

Structured lessons now live in `docs/lessons/`:

1. `docs/lessons/01-integer-math.md`
2. `docs/lessons/02-float-basics.md`
3. `docs/lessons/03-soft-float.md`
4. `docs/lessons/04-format-sweep.md`
5. `docs/lessons/05-discover-good-formats.md`
6. `docs/lessons/06-domain-profiles.md`

## What this repo demonstrates

`src/main.rs` includes practical examples:

- Decimal representation mismatch (`0.1 + 0.2` is not exactly `0.3`)
- Error accumulation (adding `0.1` repeatedly)
- Catastrophic cancellation (`(1e8 + 1) - 1e8`)
- Scaling behavior for a small decimal across powers of ten
- Counterexamples where repeated operations do not grow error

`src/bin/precision_graph.rs` generates one SVG with two views:

- Top panel: global trend, `log10(ULP(10^k))` (dense sampling)
- Bottom panel: jagged residual as a step plot (sawtooth made visible)
- Series in both panels: `f32` and `f64`

`src/bin/soft_float_explorer.rs` adds a general software-float evaluator:

- Define formats by mantissa bits + exponent range
- Sweep precision across a chosen magnitude band
- Export SVG + CSV + Markdown summary for many representations

## Quick start

Run tutorial examples:

```bash
cargo run -q
```

Generate precision graph:

```bash
cargo run -q --bin precision_graph
```

Graph output:

- `docs/precision_over_range.svg`

Run the custom-format sweep:

```bash
cargo run -q --bin soft_float_explorer
```

Sweep outputs:

- `docs/soft_float_sweep.svg`
- `docs/soft_float_sweep.csv`
- `docs/soft_float_sweep_summary.md`
- `docs/soft_float_sweep_ranking.md`

Domain-focused ranking example:

```bash
cargo run -q --bin soft_float_explorer -- \
  --focus-min -2 --focus-max 1 --focus-weight 8
```

Concrete asymmetric profile example:

```bash
cargo run -q --bin profile_float_demo
```

Profile outputs:

- `docs/profile_quantizer_examples.md`
- `docs/profile_quantizer_examples.csv`

## Floating-point primer (short version)

Floating-point values are binary scientific notation:

- `value = sign * significand * 2^exponent`
- `f32` has 24 bits of precision in the significand (including the hidden leading bit)
- `f64` has 53 bits of precision in the significand

Two important consequences:

- Most decimal fractions are not exactly representable in binary (for example `0.1`)
- Gaps between representable numbers grow as magnitude grows

So float math is usually very good for approximate real-number calculations, but not exact decimal accounting.

## Common misunderstanding

Floats are not "wrong all the time" and they are not random.

- Floating-point operations are deterministic for a fixed format, operation order, and rounding mode.
- The error is specific to the operation and operands (and can often be bounded/analyzed).
- Some loops accumulate drift, some stay bounded, and some are exact.

The important question is not "are floats wrong?" but "what error model does this computation have?"

## Where precision works well

- Physics/simulation values that are naturally approximate
- Graphics, signal processing, and ML workloads
- Relative-error-tolerant algorithms

Use `f64` when you need tighter error bounds or many chained operations.

## Where precision fails (or surprises)

### 1) Decimal fractions are approximate

`0.1 + 0.2` is close to `0.3`, not exactly equal in binary float.

### 2) Errors accumulate

Adding an inexact value repeatedly can drift from the mathematically exact answer.

### 3) Catastrophic cancellation

Subtracting nearly equal large numbers can wipe out useful digits:

- `(1e8 + 1) - 1e8` may lose the `+1` in `f32`

### 4) Precision is scale-dependent

At larger magnitudes, absolute spacing (ULP) gets larger, so tiny increments disappear.

## Counterexamples: repeat operations without growing error

These are included in `src/main.rs`:

- Exact in binary: repeatedly doing `/2` then `*2` is an exponent shift and does not accumulate drift.
- Inexact but stable: for `f64`, repeating `x = (x / 3.0) * 3.0` with `x = 0.9` introduces a one-time tiny change, then does not keep growing over 100 repeats.

## Reading the graph

In `docs/precision_over_range.svg`:

- Top panel: a lower Y value means finer absolute precision (smaller ULP)
- Top panel: an upward trend means larger spacing between adjacent representable numbers
- Top panel: `f64` stays below `f32` (smaller ULP) over shared ranges
- Bottom panel: the step-like sawtooth is the exponent-bin structure of binary floats
- Bottom panel: this shows why you expected a jagged shape even when the global trend looks linear

This visual is why code that is stable at one scale can become unstable at another.

## Practical rules

- Never compare floats with direct `==` unless you truly require bit-for-bit equality
- Use epsilon-based comparisons:

```rust
fn approx_eq(a: f64, b: f64, rel_tol: f64, abs_tol: f64) -> bool {
    let diff = (a - b).abs();
    diff <= abs_tol.max(rel_tol * a.abs().max(b.abs()))
}
```

- Prefer `f64` by default unless memory/perf constraints force `f32`
- For currency and exact decimal requirements, use fixed-point or decimal crates instead of binary floats
- Reorder numerically sensitive computations where possible (for example, compensated summation)

## Archival note

Original project files date: **2024-08-07** (local filesystem timestamps, `-05:00` offset at creation).

- `.gitignore`: 2024-08-07 08:39:20
- `Cargo.toml`: 2024-08-07 08:39:20
- `Cargo.lock`: 2024-08-07 08:39:23
- `src/main.rs` (last modified): 2024-08-07 08:40:26
