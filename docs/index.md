---
title: Floating Point Lessons
---

# Floating Point Lessons

Hands-on lessons about floating-point behavior, precision, and custom numeric formats.

## How to use this tutorial

Each lesson follows the same structure:

1. State the claim.
2. Run a concrete demo.
3. Interpret the output.
4. Extract a design rule.

Read in order; each lesson depends on the previous one.

## Learning path

1. [Integer math first](lessons/01-integer-math)
2. [Float basics and error modes](lessons/02-float-basics)
3. [Build your own float in software](lessons/03-soft-float)
4. [Sweep many formats](lessons/04-format-sweep)
5. [Discover good formats automatically](lessons/05-discover-good-formats)
6. [Domain profiles and asymmetric precision](lessons/06-domain-profiles)

## Key outputs

- [Precision trend graph](precision_over_range.svg)
- [Soft-float sweep summary](soft_float_sweep_summary)
- [Soft-float ranking](soft_float_sweep_ranking)
- [Profile quantizer example](profile_quantizer_examples)

![Precision trend graph](precision_over_range.svg)

![Soft-float sweep comparison](soft_float_sweep.svg)

## Run locally

```bash
cargo run -q
cargo run -q --bin precision_graph
cargo run -q --bin soft_float_explorer
cargo run -q --bin profile_float_demo
```
