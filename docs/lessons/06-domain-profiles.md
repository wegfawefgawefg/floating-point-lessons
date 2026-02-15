# Lesson 06: Domain Profiles and Asymmetric Precision

Question: can we make precision high in `[0, 2)` but low in `[-1, 0)`?

Yes, but not with one standard IEEE float format.

## Important clarification

- IEEE-like floats (including bfloat16) are sign-symmetric.
- That means `+x` and `-x` have the same precision at the same magnitude.

So if you want different precision by value region or sign, use a **profile/piecewise quantizer**.

## Concrete example in this repo

Run:

```bash
cargo run -q --bin profile_float_demo
```

Outputs:

- `docs/profile_quantizer_examples.md`
- `docs/profile_quantizer_examples.csv`

This compares:

- `bf16_like` (uniform)
- `f32_like` (uniform)
- `profile_pos_fine_neg_coarse` (piecewise)

The profile quantizer is intentionally:

- coarse on `[-1, 0)`
- fine on `[0, 2)`

This gives exactly the kind of asymmetric behavior you described.
