# Lesson 01: Integer Math First

Before floats, start with integers.

## Key idea

Integer arithmetic is exact as long as the true result fits in range.

- `7 + 5` is exactly `12`
- `1000 * 1000` is exact if the type is wide enough
- No rounding error is involved

## What can still go wrong

- Overflow/underflow: result does not fit the integer type
- Division truncation: `7 / 3` in integer math is `2`

## Why this matters for later lessons

Floats are different from ints in one important way:

- Int errors are usually range/overflow issues
- Float errors are representation/rounding issues

That distinction is the foundation for the rest of this tutorial.
