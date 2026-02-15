fn decimal_representation_demo() {
    let a32: f32 = 0.1;
    let b32: f32 = 0.2;
    let a64: f64 = 0.1;
    let b64: f64 = 0.2;

    println!("== Decimal representation ==");
    println!("f32: 0.1 + 0.2 = {:.10}", a32 + b32);
    println!("f64: 0.1 + 0.2 = {:.17}", a64 + b64);
    println!("Expected mathematically: 0.3");
    println!();
}

fn accumulation_demo() {
    let sum32: f32 = (0..10).map(|_| 0.1f32).sum();
    let sum64: f64 = (0..10).map(|_| 0.1f64).sum();

    println!("== Accumulation error ==");
    println!("f32: sum(0.1 repeated 10x) = {:.10}", sum32);
    println!("f64: sum(0.1 repeated 10x) = {:.17}", sum64);
    println!("Target: 1.0");
    println!();
}

fn cancellation_demo() {
    let big32: f32 = 100_000_000.0;
    let big64: f64 = 100_000_000.0;

    let out32 = (big32 + 1.0) - big32;
    let out64 = (big64 + 1.0) - big64;

    println!("== Catastrophic cancellation ==");
    println!("f32: (1e8 + 1) - 1e8 = {}", out32);
    println!("f64: (1e8 + 1) - 1e8 = {}", out64);
    println!();
}

fn scaling_demo() {
    let small32: f32 = 0.0002298;
    let small64: f64 = 0.0002298;

    println!("== Scaling a small decimal by powers of ten ==");
    println!("Starting value (f32): {}", small32);
    println!("Starting value (f64): {}", small64);
    println!();

    for i in 0..10 {
        let multiplier = 10u64.pow(i) as f64;
        let result32 = small32 * multiplier as f32;
        let result64 = small64 * multiplier;

        println!("Multiplier: 10^{}", i);
        println!("f32 result: {}", result32);
        println!("f64 result: {}", result64);
        println!("Absolute difference: {}", (result64 - result32 as f64).abs());
        println!();
    }
}

fn non_accumulating_demo() {
    println!("== Counterexamples: repeated ops without growing error ==");

    // Exact in binary: dividing/multiplying by powers of two just shifts exponent bits.
    let start_exact = 0.3f64;
    let mut exact = start_exact;
    for _ in 0..100 {
        exact = (exact / 2.0) * 2.0;
    }
    println!("Exact roundtrip (/2 then *2, 100x)");
    println!("start:  {:.17}", start_exact);
    println!("end:    {:.17}", exact);
    println!("drift:  {:.17}", (exact - start_exact).abs());
    println!();

    // Inexact operation pair: first round may change value, then it can settle to a fixed point.
    let start_stable = 0.9f64;
    let mut stable = start_stable;
    let mut after_one = start_stable;
    for i in 0..100 {
        stable = (stable / 3.0) * 3.0;
        if i == 0 {
            after_one = stable;
        }
    }
    println!("Inexact but non-growing (/3 then *3, f64)");
    println!("start:      {:.20}", start_stable);
    println!("after 1x:   {:.20}", after_one);
    println!("after 100x: {:.20}", stable);
    println!("1-step drift:   {:.20}", (after_one - start_stable).abs());
    println!("100-step drift: {:.20}", (stable - start_stable).abs());
    println!();
}

fn main() {
    decimal_representation_demo();
    accumulation_demo();
    cancellation_demo();
    scaling_demo();
    non_accumulating_demo();
}
