# Profile Quantizer Concrete Example

Goal example: **high precision in [0, 2)** and **low precision in [-1, 0)**.

Important note: true IEEE-like floats (including bfloat16) are sign-symmetric.
At the same magnitude, `+x` and `-x` have the same spacing/precision.
So this asymmetric behavior needs a profile/piecewise quantizer, not a single standard float format.

## Zone metrics

| quantizer | zone | mean abs err | mean rel err |
| --- | --- | ---: | ---: |
| bf16_like | [-1, 0) | 6.495e-4 | 1.324e-3 |
| bf16_like | [0, 2) | 1.300e-3 | 1.338e-3 |
| f32_like | [-1, 0) | 9.911e-9 | 2.039e-8 |
| f32_like | [0, 2) | 1.983e-8 | 2.050e-8 |
| profile_pos_fine_neg_coarse | [-1, 0) | 5.207e-3 | 1.110e-2 |
| profile_pos_fine_neg_coarse | [0, 2) | 4.063e-5 | 5.042e-3 |

## Sample points

| x | bf16_like | f32_like | profile_pos_fine_neg_coarse |
| ---: | ---: | ---: | ---: |
| -0.900 | -0.89843750 | -0.89999998 | -0.90625000 |
| -0.500 | -0.50000000 | -0.50000000 | -0.50000000 |
| -0.100 | -0.10009766 | -0.10000000 | -0.10156250 |
| 0.100 | 0.10009766 | 0.10000000 | 0.10000610 |
| 0.500 | 0.50000000 | 0.50000000 | 0.50000000 |
| 1.000 | 1.00000000 | 1.00000000 | 1.00000000 |
| 1.500 | 1.50000000 | 1.50000000 | 1.50000000 |
