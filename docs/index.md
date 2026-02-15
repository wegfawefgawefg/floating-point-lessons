# Floating Point Lessons

Hands-on lessons and experiments about floating-point behavior, precision, and custom formats.

## Lessons

1. [Integer math first](lessons/01-integer-math.md)
2. [Float basics and error modes](lessons/02-float-basics.md)
3. [Build your own float in software](lessons/03-soft-float.md)
4. [Sweep many formats](lessons/04-format-sweep.md)
5. [Discover good formats automatically](lessons/05-discover-good-formats.md)
6. [Domain profiles and asymmetric precision](lessons/06-domain-profiles.md)

## Key outputs

- [Precision trend graph](precision_over_range.svg)
- [Soft-float sweep summary](soft_float_sweep_summary.md)
- [Soft-float ranking](soft_float_sweep_ranking.md)
- [Profile quantizer example](profile_quantizer_examples.md)

## Run locally

```bash
cargo run -q
cargo run -q --bin precision_graph
cargo run -q --bin soft_float_explorer
cargo run -q --bin profile_float_demo
```
