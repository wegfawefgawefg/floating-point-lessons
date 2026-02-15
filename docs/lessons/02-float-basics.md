---
title: Lesson 02 - Float Math and Error Modes
---

# Lesson 02: Float Math and Error Modes

## What this lesson is for

This lesson shows the core error patterns that make float code behave differently from integer code.

## Claims

1. Most decimal fractions are not exactly representable in binary.
2. Repeated operations can accumulate or expose error.
3. Precision depends on scale.

## Run the demo

```bash
cargo run -q
```

The executable in `src/main.rs` prints five sections:

- decimal representation mismatch
- accumulation drift
- catastrophic cancellation
- scaling across powers of ten
- non-accumulating counterexamples

## One concrete code slice

```rust
let a32: f32 = 0.1;
let b32: f32 = 0.2;
println!("f32: 0.1 + 0.2 = {:.10}", a32 + b32);
```

This demonstrates that the stored values are nearby binary approximations, not exact decimal values.

## Visual interpretation

Generate the graph:

```bash
cargo run -q --bin precision_graph
```

![Precision over range](../precision_over_range.svg)

How to read it:

- Top panel: global trend of ULP size versus magnitude.
- Bottom panel: step-like residual structure from exponent bins.
- Practical meaning: the same algorithm can behave differently at different scales.

## Takeaway

Float arithmetic is deterministic but approximate.
The correct question is not "is float wrong?" but "what error model does this computation induce?"

## Continue

Next: [Lesson 03: Build Your Own Float in Software](03-soft-float)
