---
title: Lesson 01 - Integer Math First
---

# Lesson 01: Integer Math First

## What this lesson is for

This lesson establishes a baseline: integer arithmetic is exact, but only within range.
You need this baseline before float behavior makes sense.

## Claim

Integer math has no rounding model. Results are exact unless the type cannot represent them.

## Demonstration

```rust
let a: i32 = 7 + 5;      // 12 exactly
let b: i32 = 1_000 * 1_000; // exact if still in range
let c: i32 = 7 / 3;      // 2, because integer division truncates
```

## What to notice

- `a` and `b` are exact because they fit in `i32`.
- `c` is not "wrong"; it follows integer division rules.
- None of these involve representational drift.

## Failure modes you still have

- Overflow/underflow: true result exceeds type bounds.
- Truncation on integer division.

## Why this matters for floats

When we move to floats, the dominant risk changes from **range-only** problems to **representation and rounding** problems.
That is the conceptual jump for the rest of the tutorial.

## Continue

Next: [Lesson 02: Float Math and Error Modes](02-float-basics)
