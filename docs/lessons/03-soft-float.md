# Lesson 03: Build Your Own Float in Software

This repo includes a configurable software float model in `src/soft_float.rs`.

## Is the implementation complicated?

Not really. The core quantization idea is just a few steps:

1. Write the value as `sign * (1 + fraction) * 2^exp`.
2. Clamp `exp` to your allowed exponent range.
3. Round `fraction` to a fixed number of mantissa bits.
4. Rebuild the value from sign, rounded fraction, and exponent.

That is the whole model in plain terms.

## Model

`SoftFloatSpec` lets you pick:

- Mantissa bits (`mantissa_bits`)
- Minimum binary exponent (`min_exp2`)
- Maximum binary exponent (`max_exp2`)

This gives a practical way to explore precision/range tradeoffs without new hardware types.

## What you can inspect

For each format, the model can report:

- Quantized value of a real input (`quantize`)
- Epsilon near 1 (`epsilon_at_one`)
- Min normal value (`min_normal`)
- Max finite value (`max_finite`)

## Why this is useful

You can directly test ideas like:

- "What if I use bf16-like precision but keep wide exponent range?"
- "What if I tighten range but keep more mantissa bits?"
- "How much relative error do I get across a target magnitude band?"
