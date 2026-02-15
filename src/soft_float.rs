#[derive(Clone, Debug)]
pub struct SoftFloatSpec {
    pub name: String,
    pub mantissa_bits: u32,
    pub min_exp2: i32,
    pub max_exp2: i32,
}

impl SoftFloatSpec {
    pub fn new(name: impl Into<String>, mantissa_bits: u32, min_exp2: i32, max_exp2: i32) -> Self {
        Self {
            name: name.into(),
            mantissa_bits,
            min_exp2,
            max_exp2,
        }
    }

    pub fn quantize(&self, x: f64) -> f64 {
        if x.is_nan() {
            return f64::NAN;
        }
        if x == 0.0 {
            return x;
        }

        let sign = x.signum();
        let ax = x.abs();
        if !ax.is_finite() {
            return sign * f64::INFINITY;
        }

        let mut exp2 = ax.log2().floor() as i32;
        if exp2 < self.min_exp2 {
            return sign * 0.0;
        }
        if exp2 > self.max_exp2 {
            return sign * f64::INFINITY;
        }

        let base = 2f64.powi(exp2);
        let m = ax / base;
        let frac = m - 1.0;

        let steps = 2f64.powi(self.mantissa_bits as i32);
        let mut frac_q = (frac * steps).round() / steps;

        if frac_q >= 1.0 {
            frac_q = 0.0;
            exp2 += 1;
            if exp2 > self.max_exp2 {
                return sign * f64::INFINITY;
            }
        }

        sign * (1.0 + frac_q) * 2f64.powi(exp2)
    }

    pub fn epsilon_at_one(&self) -> f64 {
        2f64.powi(-(self.mantissa_bits as i32))
    }

    pub fn min_normal(&self) -> f64 {
        2f64.powi(self.min_exp2)
    }

    pub fn max_finite(&self) -> f64 {
        (2.0 - self.epsilon_at_one()) * 2f64.powi(self.max_exp2)
    }

    pub fn ulp_near(&self, x: f64) -> Option<f64> {
        if !(x.is_finite() && x > 0.0) {
            return None;
        }

        let exp2 = x.log2().floor() as i32;
        if exp2 < self.min_exp2 || exp2 > self.max_exp2 {
            return None;
        }

        Some(2f64.powi(exp2 - self.mantissa_bits as i32))
    }
}

pub fn default_presets() -> Vec<SoftFloatSpec> {
    vec![
        SoftFloatSpec::new("tiny8", 3, -6, 7),
        SoftFloatSpec::new("fp16_like", 10, -14, 15),
        SoftFloatSpec::new("bf16_like", 7, -126, 127),
        SoftFloatSpec::new("f32_like", 23, -126, 127),
        SoftFloatSpec::new("f64_like", 52, -1022, 1023),
    ]
}
