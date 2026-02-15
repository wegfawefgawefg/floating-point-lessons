---
title: Lesson 06 - Domain Profiles and Asymmetric Precision
---

# Lesson 06: Domain Profiles and Asymmetric Precision

## What this lesson is for

Sometimes requirements are asymmetric across value regions.
This lesson shows when standard float formats cannot satisfy that requirement, and what to use instead.

## Claim

IEEE-like formats are sign-symmetric, so they cannot provide finer precision on `+x` than `-x` at the same magnitude.

## Demonstration

Run:

```bash
cargo run -q --bin profile_float_demo
```

Outputs:

- `docs/profile_quantizer_examples.md`
- `docs/profile_quantizer_examples.csv`

Compared quantizers:

- `bf16_like` (uniform)
- `f32_like` (uniform)
- `profile_pos_fine_neg_coarse` (piecewise)

## Piecewise profile definition

```rust
regions: vec![
    Region {
        min: -1.0,
        max: 0.0,
        spec: SoftFloatSpec::new("neg_coarse", 4, -20, 20),
    },
    Region {
        min: 0.0,
        max: 2.0,
        spec: SoftFloatSpec::new("pos_fine", 12, -20, 20),
    },
]
```

## Interpretation

- Uniform formats keep similar behavior for `+x` and `-x` at equal magnitude.
- Piecewise quantizers can intentionally break that symmetry.
- Use piecewise/domain profiles when your error budget is region-specific.

## End state

You now have a complete workflow:

1. Understand float error modes.
2. Model alternative formats.
3. Sweep and rank them for your domain.
4. Use piecewise profiles when uniform formats cannot express the requirement.
