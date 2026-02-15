use prectest::soft_float::SoftFloatSpec;
use std::error::Error;
use std::fs;

#[derive(Clone)]
struct Region {
    min: f64,
    max: f64,
    spec: SoftFloatSpec,
}

#[derive(Clone)]
struct PiecewiseQuantizer {
    name: String,
    regions: Vec<Region>,
    fallback: SoftFloatSpec,
}

impl PiecewiseQuantizer {
    fn quantize(&self, x: f64) -> f64 {
        for region in &self.regions {
            if x >= region.min && x < region.max {
                return region.spec.quantize(x);
            }
        }
        self.fallback.quantize(x)
    }
}

#[derive(Clone)]
enum Quantizer {
    Uniform(SoftFloatSpec),
    Piecewise(PiecewiseQuantizer),
}

impl Quantizer {
    fn name(&self) -> &str {
        match self {
            Quantizer::Uniform(s) => &s.name,
            Quantizer::Piecewise(p) => &p.name,
        }
    }

    fn quantize(&self, x: f64) -> f64 {
        match self {
            Quantizer::Uniform(s) => s.quantize(x),
            Quantizer::Piecewise(p) => p.quantize(x),
        }
    }
}

#[derive(Clone, Copy)]
struct Zone {
    name: &'static str,
    min: f64,
    max: f64,
}

fn sample_range(min: f64, max: f64, step: f64) -> Vec<f64> {
    let mut xs = Vec::new();
    let mut x = min;
    while x <= max + step * 0.5 {
        xs.push(x);
        x += step;
    }
    xs
}

fn mean_abs_rel_error(q: &Quantizer, zone: Zone, xs: &[f64]) -> (f64, f64) {
    let mut abs_sum = 0.0;
    let mut rel_sum = 0.0;
    let mut n = 0usize;

    for &x in xs {
        if !(x >= zone.min && x < zone.max) {
            continue;
        }
        let y = q.quantize(x);
        let abs_err = (y - x).abs();
        let rel_err = if x == 0.0 { 0.0 } else { abs_err / x.abs() };

        abs_sum += abs_err;
        rel_sum += rel_err;
        n += 1;
    }

    if n == 0 {
        (0.0, 0.0)
    } else {
        (abs_sum / n as f64, rel_sum / n as f64)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("docs")?;

    let bf16_like = Quantizer::Uniform(SoftFloatSpec::new("bf16_like", 7, -126, 127));
    let f32_like = Quantizer::Uniform(SoftFloatSpec::new("f32_like", 23, -126, 127));

    let profile = Quantizer::Piecewise(PiecewiseQuantizer {
        name: "profile_pos_fine_neg_coarse".to_string(),
        regions: vec![
            Region {
                min: -1.0,
                max: 0.0,
                // Intentionally coarse in [-1, 0)
                spec: SoftFloatSpec::new("neg_coarse", 4, -20, 20),
            },
            Region {
                min: 0.0,
                max: 2.0,
                // Intentionally fine in [0, 2)
                spec: SoftFloatSpec::new("pos_fine", 12, -20, 20),
            },
        ],
        // Medium precision elsewhere
        fallback: SoftFloatSpec::new("fallback", 7, -20, 20),
    });

    let quantizers = vec![bf16_like, f32_like, profile];

    let zones = [
        Zone {
            name: "[-1, 0)",
            min: -1.0,
            max: 0.0,
        },
        Zone {
            name: "[0, 2)",
            min: 0.0,
            max: 2.0,
        },
    ];

    let xs = sample_range(-1.0, 2.0, 0.01);

    let mut md = String::new();
    md.push_str("---\n");
    md.push_str("title: Profile Quantizer Concrete Example\n");
    md.push_str("---\n\n");
    md.push_str("# Profile Quantizer Concrete Example\n\n");
    md.push_str("Goal example: **high precision in [0, 2)** and **low precision in [-1, 0)**.\n\n");
    md.push_str("Important note: true IEEE-like floats (including bfloat16) are sign-symmetric.\n");
    md.push_str("At the same magnitude, `+x` and `-x` have the same spacing/precision.\n");
    md.push_str("So this asymmetric behavior needs a profile/piecewise quantizer, not a single standard float format.\n\n");

    md.push_str("## Zone metrics\n\n");
    md.push_str("| quantizer | zone | mean abs err | mean rel err |\n");
    md.push_str("| --- | --- | ---: | ---: |\n");

    for q in &quantizers {
        for &zone in &zones {
            let (mae, mre) = mean_abs_rel_error(q, zone, &xs);
            md.push_str(&format!(
                "| {} | {} | {:.3e} | {:.3e} |\n",
                q.name(),
                zone.name,
                mae,
                mre
            ));
        }
    }

    let sample_points = [-0.9, -0.5, -0.1, 0.1, 0.5, 1.0, 1.5];
    md.push_str("\n## Sample points\n\n");
    md.push_str("| x | bf16_like | f32_like | profile_pos_fine_neg_coarse |\n");
    md.push_str("| ---: | ---: | ---: | ---: |\n");
    for &x in &sample_points {
        let b = quantizers[0].quantize(x);
        let f = quantizers[1].quantize(x);
        let p = quantizers[2].quantize(x);
        md.push_str(&format!("| {:.3} | {:.8} | {:.8} | {:.8} |\n", x, b, f, p));
    }

    fs::write("docs/profile_quantizer_examples.md", md)?;

    let mut csv = String::from("x,quantizer,quantized,abs_error,rel_error\n");
    for &x in &xs {
        for q in &quantizers {
            let y = q.quantize(x);
            let abs = (y - x).abs();
            let rel = if x == 0.0 { 0.0 } else { abs / x.abs() };
            csv.push_str(&format!(
                "{:.6},{},{:.12},{:.12e},{:.12e}\n",
                x,
                q.name(),
                y,
                abs,
                rel
            ));
        }
    }
    fs::write("docs/profile_quantizer_examples.csv", csv)?;

    println!("Wrote docs/profile_quantizer_examples.md");
    println!("Wrote docs/profile_quantizer_examples.csv");

    Ok(())
}
