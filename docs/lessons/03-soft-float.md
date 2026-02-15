---
title: Lesson 03 - Build Your Own Float in Software
---

# Lesson 03: Build Your Own Float in Software

## What this lesson is for

You will move from observing float behavior to modeling it.
The model lets you test precision/range tradeoffs directly.

## Claim

A float-like quantizer can be expressed as a small, understandable pipeline.

## Core quantization pipeline

```rust
pub fn quantize(&self, x: f64) -> f64 {
    let sign = x.signum();
    let ax = x.abs();
    let mut exp2 = ax.log2().floor() as i32;
    if exp2 < self.min_exp2 { return sign * 0.0; }

    let base = 2f64.powi(exp2);
    let frac = ax / base - 1.0;
    let steps = 2f64.powi(self.mantissa_bits as i32);
    let frac_q = (frac * steps).round() / steps;

    sign * (1.0 + frac_q) * 2f64.powi(exp2)
}
```

## What each parameter controls

- `mantissa_bits`: local precision within an exponent bucket.
- `min_exp2` and `max_exp2`: dynamic range.

This is exposed as `SoftFloatSpec` in `src/soft_float.rs`.

## Why this matters

With this model, you can test statements like:

- "More mantissa bits reduce local quantization error."
- "Narrow exponent range clips values sooner."
- "There is no free lunch: precision and range compete."

## Continue

Next: [Lesson 04: Sweep Many Float Formats](04-format-sweep)
