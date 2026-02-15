use prectest::soft_float::{default_presets, SoftFloatSpec};
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct Config {
    k_min: f64,
    k_max: f64,
    k_step: f64,
    out_prefix: String,
    formats: Vec<SoftFloatSpec>,
    focus_min: Option<f64>,
    focus_max: Option<f64>,
    focus_weight: f64,
    max_err_weight: f64,
    underflow_penalty: f64,
    overflow_penalty: f64,
}

#[derive(Debug, Clone)]
struct FormatMetrics {
    name: String,
    mean_rel_err: f64,
    max_rel_err: f64,
    underflow_frac: f64,
    overflow_frac: f64,
    finite_frac: f64,
    score: f64,
}

fn parse_format(spec: &str) -> Result<SoftFloatSpec, String> {
    let parts: Vec<&str> = spec.split(',').collect();
    if parts.len() != 4 {
        return Err(format!(
            "invalid --format '{spec}', expected name,mantissa_bits,min_exp2,max_exp2"
        ));
    }

    let name = parts[0].trim();
    let mantissa_bits = parts[1]
        .trim()
        .parse::<u32>()
        .map_err(|e| format!("invalid mantissa_bits in '{spec}': {e}"))?;
    let min_exp2 = parts[2]
        .trim()
        .parse::<i32>()
        .map_err(|e| format!("invalid min_exp2 in '{spec}': {e}"))?;
    let max_exp2 = parts[3]
        .trim()
        .parse::<i32>()
        .map_err(|e| format!("invalid max_exp2 in '{spec}': {e}"))?;

    Ok(SoftFloatSpec::new(name, mantissa_bits, min_exp2, max_exp2))
}

fn parse_args() -> Result<Config, String> {
    let mut k_min = -20.0;
    let mut k_max = 20.0;
    let mut k_step = 0.1;
    let mut out_prefix = String::from("docs/soft_float_sweep");
    let mut formats = default_presets();
    let mut include_presets = true;
    let mut focus_min: Option<f64> = None;
    let mut focus_max: Option<f64> = None;
    let mut focus_weight = 5.0;
    let mut max_err_weight = 0.5;
    let mut underflow_penalty = 4.0;
    let mut overflow_penalty = 4.0;

    let mut args = env::args().skip(1).peekable();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--k-min" => {
                let v = args
                    .next()
                    .ok_or_else(|| "--k-min requires a value".to_string())?;
                k_min = v
                    .parse::<f64>()
                    .map_err(|e| format!("invalid --k-min '{v}': {e}"))?;
            }
            "--k-max" => {
                let v = args
                    .next()
                    .ok_or_else(|| "--k-max requires a value".to_string())?;
                k_max = v
                    .parse::<f64>()
                    .map_err(|e| format!("invalid --k-max '{v}': {e}"))?;
            }
            "--k-step" => {
                let v = args
                    .next()
                    .ok_or_else(|| "--k-step requires a value".to_string())?;
                k_step = v
                    .parse::<f64>()
                    .map_err(|e| format!("invalid --k-step '{v}': {e}"))?;
            }
            "--out" => {
                out_prefix = args
                    .next()
                    .ok_or_else(|| "--out requires a value".to_string())?;
            }
            "--focus-min" => {
                let v = args
                    .next()
                    .ok_or_else(|| "--focus-min requires a value".to_string())?;
                focus_min = Some(
                    v.parse::<f64>()
                        .map_err(|e| format!("invalid --focus-min '{v}': {e}"))?,
                );
            }
            "--focus-max" => {
                let v = args
                    .next()
                    .ok_or_else(|| "--focus-max requires a value".to_string())?;
                focus_max = Some(
                    v.parse::<f64>()
                        .map_err(|e| format!("invalid --focus-max '{v}': {e}"))?,
                );
            }
            "--focus-weight" => {
                let v = args
                    .next()
                    .ok_or_else(|| "--focus-weight requires a value".to_string())?;
                focus_weight = v
                    .parse::<f64>()
                    .map_err(|e| format!("invalid --focus-weight '{v}': {e}"))?;
            }
            "--maxerr-weight" => {
                let v = args
                    .next()
                    .ok_or_else(|| "--maxerr-weight requires a value".to_string())?;
                max_err_weight = v
                    .parse::<f64>()
                    .map_err(|e| format!("invalid --maxerr-weight '{v}': {e}"))?;
            }
            "--underflow-penalty" => {
                let v = args
                    .next()
                    .ok_or_else(|| "--underflow-penalty requires a value".to_string())?;
                underflow_penalty = v
                    .parse::<f64>()
                    .map_err(|e| format!("invalid --underflow-penalty '{v}': {e}"))?;
            }
            "--overflow-penalty" => {
                let v = args
                    .next()
                    .ok_or_else(|| "--overflow-penalty requires a value".to_string())?;
                overflow_penalty = v
                    .parse::<f64>()
                    .map_err(|e| format!("invalid --overflow-penalty '{v}': {e}"))?;
            }
            "--format" => {
                let v = args
                    .next()
                    .ok_or_else(|| "--format requires a value".to_string())?;
                if include_presets {
                    formats.clear();
                    include_presets = false;
                }
                formats.push(parse_format(&v)?);
            }
            "--add-format" => {
                let v = args
                    .next()
                    .ok_or_else(|| "--add-format requires a value".to_string())?;
                formats.push(parse_format(&v)?);
            }
            "--no-presets" => {
                formats.clear();
                include_presets = false;
            }
            "--help" | "-h" => {
                return Err(help_text());
            }
            _ => {
                return Err(format!("unknown argument '{arg}'\n\n{}", help_text()));
            }
        }
    }

    if formats.is_empty() {
        return Err("no formats configured; use --format or remove --no-presets".to_string());
    }
    if !(k_step.is_finite() && k_step > 0.0) {
        return Err("--k-step must be > 0".to_string());
    }
    if !(k_min.is_finite() && k_max.is_finite() && k_max > k_min) {
        return Err("require finite k range with --k-max > --k-min".to_string());
    }
    if let (Some(a), Some(b)) = (focus_min, focus_max) {
        if !(b > a) {
            return Err("require --focus-max > --focus-min".to_string());
        }
    }
    if !(focus_weight.is_finite() && focus_weight >= 1.0) {
        return Err("--focus-weight must be >= 1".to_string());
    }
    if !(max_err_weight.is_finite() && max_err_weight >= 0.0) {
        return Err("--maxerr-weight must be >= 0".to_string());
    }
    if !(underflow_penalty.is_finite() && underflow_penalty >= 0.0) {
        return Err("--underflow-penalty must be >= 0".to_string());
    }
    if !(overflow_penalty.is_finite() && overflow_penalty >= 0.0) {
        return Err("--overflow-penalty must be >= 0".to_string());
    }

    Ok(Config {
        k_min,
        k_max,
        k_step,
        out_prefix,
        formats,
        focus_min,
        focus_max,
        focus_weight,
        max_err_weight,
        underflow_penalty,
        overflow_penalty,
    })
}

fn help_text() -> String {
    [
        "Usage:",
        "  cargo run --bin soft_float_explorer -- [options]",
        "",
        "Options:",
        "  --k-min <f64>                  Default: -20",
        "  --k-max <f64>                  Default: 20",
        "  --k-step <f64>                 Default: 0.1",
        "  --out <path-prefix>            Default: docs/soft_float_sweep",
        "  --focus-min <f64>              Optional focus interval lower k",
        "  --focus-max <f64>              Optional focus interval upper k",
        "  --focus-weight <f64>           Default: 5 (>=1)",
        "  --maxerr-weight <f64>          Default: 0.5",
        "  --underflow-penalty <f64>      Default: 4",
        "  --overflow-penalty <f64>       Default: 4",
        "  --no-presets                   Start with no built-in formats",
        "  --format name,m,min_e,max_e    Replace presets with one format",
        "  --add-format name,m,min_e,max_e Add another format",
        "",
        "Examples:",
        "  cargo run --bin soft_float_explorer",
        "  cargo run --bin soft_float_explorer -- --format custom,11,-40,40",
        "  cargo run --bin soft_float_explorer -- --no-presets --add-format a,5,-10,10 --add-format b,12,-20,20",
    ]
    .join("\n")
}

fn sample_k_values(k_min: f64, k_max: f64, step: f64) -> Vec<f64> {
    let mut out = Vec::new();
    let mut k = k_min;
    while k <= k_max + step * 0.5 {
        out.push(k);
        k += step;
    }
    out
}

fn map(value: f64, src_min: f64, src_max: f64, dst_min: f64, dst_max: f64) -> f64 {
    let t = (value - src_min) / (src_max - src_min);
    dst_min + t * (dst_max - dst_min)
}

fn sample_weight(config: &Config, k: f64) -> f64 {
    if let (Some(a), Some(b)) = (config.focus_min, config.focus_max) {
        if k >= a && k <= b {
            return config.focus_weight;
        }
    }
    1.0
}

fn compute_metrics(fmt: &SoftFloatSpec, config: &Config, ks: &[f64]) -> FormatMetrics {
    let mut total_weight = 0.0f64;
    let mut finite_count = 0usize;
    let mut underflow_weight = 0.0f64;
    let mut overflow_weight = 0.0f64;
    let mut rel_sum_weighted = 0.0f64;
    let mut rel_weight_total = 0.0f64;
    let mut rel_max = 0.0f64;

    for &k in ks {
        let w = sample_weight(config, k);
        total_weight += w;
        let x = 10f64.powf(k);
        let q = fmt.quantize(x);

        if q == 0.0 && x != 0.0 {
            underflow_weight += w;
        }
        if !q.is_finite() {
            overflow_weight += w;
            continue;
        }

        let rel = (q - x).abs() / x.abs();
        if rel.is_finite() {
            finite_count += 1;
            rel_sum_weighted += rel * w;
            rel_weight_total += w;
            if rel > rel_max {
                rel_max = rel;
            }
        }
    }

    let finite_frac = finite_count as f64 / ks.len() as f64;
    let underflow_frac = if total_weight > 0.0 {
        underflow_weight / total_weight
    } else {
        0.0
    };
    let overflow_frac = if total_weight > 0.0 {
        overflow_weight / total_weight
    } else {
        0.0
    };

    let mean_rel_err = if rel_weight_total > 0.0 {
        rel_sum_weighted / rel_weight_total
    } else {
        f64::INFINITY
    };
    let max_rel_err = if finite_count > 0 { rel_max } else { f64::INFINITY };

    // Lower is better:
    // - average and worst relative error matter
    // - underflow/overflow rates are strongly penalized
    let score = if finite_count > 0 {
        mean_rel_err.max(1e-30).log10()
            + config.max_err_weight * max_rel_err.max(1e-30).log10()
            + config.underflow_penalty * underflow_frac
            + config.overflow_penalty * overflow_frac
    } else {
        f64::INFINITY
    };

    FormatMetrics {
        name: fmt.name.clone(),
        mean_rel_err,
        max_rel_err,
        underflow_frac,
        overflow_frac,
        finite_frac,
        score,
    }
}

fn ranked_metrics(config: &Config, ks: &[f64]) -> Vec<FormatMetrics> {
    let mut metrics: Vec<FormatMetrics> = config
        .formats
        .iter()
        .map(|fmt| compute_metrics(fmt, config, ks))
        .collect();

    metrics.sort_by(|a, b| a.score.total_cmp(&b.score));
    metrics
}

fn write_csv(config: &Config, ks: &[f64], csv_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut out = String::from("format,k,x,quantized,abs_error,rel_error\n");

    for fmt in &config.formats {
        for &k in ks {
            let x = 10f64.powf(k);
            let q = fmt.quantize(x);
            let abs_err = (q - x).abs();
            let rel_err = if x != 0.0 { abs_err / x.abs() } else { 0.0 };
            out.push_str(&format!(
                "{},{:.6},{:.16e},{:.16e},{:.16e},{:.16e}\n",
                fmt.name, k, x, q, abs_err, rel_err
            ));
        }
    }

    fs::write(csv_path, out)?;
    Ok(())
}

fn write_summary(config: &Config, summary_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut out = String::new();
    out.push_str("---\n");
    out.push_str("title: Soft Float Sweep Summary\n");
    out.push_str("---\n\n");
    out.push_str("# Soft Float Sweep Summary\n\n");
    out.push_str("This file is generated by `soft_float_explorer`.\n\n");
    out.push_str("## Config\n\n");
    out.push_str(&format!(
        "- k range: [{:.2}, {:.2}] step {:.3}\n",
        config.k_min, config.k_max, config.k_step
    ));
    out.push_str(&format!("- number of formats: {}\n\n", config.formats.len()));

    out.push_str("## Formats\n\n");
    out.push_str("| name | mantissa bits | min exp2 | max exp2 | min normal | max finite | epsilon at 1 |\n");
    out.push_str("| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    for fmt in &config.formats {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {:.3e} | {:.3e} | {:.3e} |\n",
            fmt.name,
            fmt.mantissa_bits,
            fmt.min_exp2,
            fmt.max_exp2,
            fmt.min_normal(),
            fmt.max_finite(),
            fmt.epsilon_at_one()
        ));
    }

    fs::write(summary_path, out)?;
    Ok(())
}

fn write_ranking(config: &Config, ks: &[f64], ranking_path: &Path) -> Result<(), Box<dyn Error>> {
    let metrics = ranked_metrics(config, ks);

    let mut out = String::new();
    out.push_str("---\n");
    out.push_str("title: Soft Float Ranking\n");
    out.push_str("---\n\n");
    out.push_str("# Soft Float Ranking\n\n");
    out.push_str("This file is generated by `soft_float_explorer`.\n\n");
    out.push_str("Formats are ranked by a heuristic score (lower is better):\n\n");
    out.push_str(&format!(
        "- `score = log10(mean_rel_err) + {:.3}*log10(max_rel_err) + {:.3}*underflow_frac + {:.3}*overflow_frac`\n",
        config.max_err_weight, config.underflow_penalty, config.overflow_penalty
    ));
    if let (Some(a), Some(b)) = (config.focus_min, config.focus_max) {
        out.push_str(&format!(
            "- Focus weighting enabled: k in [{:.3}, {:.3}] gets weight {:.3} in mean error and clipping rates\n",
            a, b, config.focus_weight
        ));
    } else {
        out.push_str("- Focus weighting disabled: all k samples weighted equally\n");
    }
    out.push_str("- This favors low relative error while penalizing clipping to zero/infinity.\n\n");

    out.push_str("| rank | format | score | mean rel err | max rel err | underflow % | overflow % | finite % |\n");
    out.push_str("| ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    for (idx, m) in metrics.iter().enumerate() {
        out.push_str(&format!(
            "| {} | {} | {:.4} | {:.3e} | {:.3e} | {:.2}% | {:.2}% | {:.2}% |\n",
            idx + 1,
            m.name,
            m.score,
            m.mean_rel_err,
            m.max_rel_err,
            m.underflow_frac * 100.0,
            m.overflow_frac * 100.0,
            m.finite_frac * 100.0,
        ));
    }

    fs::write(ranking_path, out)?;
    Ok(())
}

fn write_svg(config: &Config, ks: &[f64], svg_path: &Path) -> Result<(), Box<dyn Error>> {
    let width = 1400.0;
    let height = 860.0;
    let left = 90.0;
    let right = 40.0;
    let top = 80.0;
    let bottom = 90.0;

    let x0 = left;
    let x1 = width - right;
    let y0 = top;
    let y1 = height - bottom;

    let y_min = -18.0;
    let y_max = 0.5;
    let err_floor = 1e-18;

    let palette = [
        "#1565c0", "#c62828", "#2e7d32", "#6a1b9a", "#ef6c00", "#00695c", "#283593",
        "#ad1457", "#0277bd", "#5d4037",
    ];

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\">"
    ));
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"white\"/>");

    svg.push_str(&format!(
        "<text x=\"{}\" y=\"42\" font-family=\"sans-serif\" font-size=\"34\" text-anchor=\"middle\">Soft float precision sweep (relative error at x = 10^k)</text>",
        width / 2.0
    ));

    for x_tick in ((config.k_min as i32)..=(config.k_max as i32)).step_by(5) {
        let x = map(x_tick as f64, config.k_min, config.k_max, x0, x1);
        svg.push_str(&format!(
            "<line x1=\"{x:.2}\" y1=\"{y0:.2}\" x2=\"{x:.2}\" y2=\"{y1:.2}\" stroke=\"#ececec\" stroke-width=\"1\"/>"
        ));
        svg.push_str(&format!(
            "<text x=\"{x:.2}\" y=\"{:.2}\" font-family=\"sans-serif\" font-size=\"12\" text-anchor=\"middle\">{x_tick}</text>",
            y1 + 22.0
        ));
    }

    for y_tick in -18..=0 {
        let y = map(y_tick as f64, y_min, y_max, y1, y0);
        svg.push_str(&format!(
            "<line x1=\"{x0:.2}\" y1=\"{y:.2}\" x2=\"{x1:.2}\" y2=\"{y:.2}\" stroke=\"#ececec\" stroke-width=\"1\"/>"
        ));
        svg.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" font-family=\"sans-serif\" font-size=\"12\" text-anchor=\"end\">{}</text>",
            x0 - 8.0,
            y + 4.0,
            y_tick
        ));
    }

    svg.push_str(&format!(
        "<line x1=\"{x0:.2}\" y1=\"{y0:.2}\" x2=\"{x0:.2}\" y2=\"{y1:.2}\" stroke=\"#222\" stroke-width=\"2\"/>"
    ));
    svg.push_str(&format!(
        "<line x1=\"{x0:.2}\" y1=\"{y1:.2}\" x2=\"{x1:.2}\" y2=\"{y1:.2}\" stroke=\"#222\" stroke-width=\"2\"/>"
    ));

    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"16\" text-anchor=\"middle\">k where x = 10^k</text>",
        (x0 + x1) / 2.0,
        height - 30.0
    ));
    svg.push_str(&format!(
        "<text x=\"24\" y=\"{}\" font-family=\"sans-serif\" font-size=\"16\" text-anchor=\"middle\" transform=\"rotate(-90 24,{})\">log10(relative error)</text>",
        (y0 + y1) / 2.0,
        (y0 + y1) / 2.0
    ));

    for (idx, fmt) in config.formats.iter().enumerate() {
        let color = palette[idx % palette.len()];
        let mut pts: Vec<String> = Vec::new();
        for &k in ks {
            let x = 10f64.powf(k);
            let q = fmt.quantize(x);
            let rel_err = if x != 0.0 { (q - x).abs() / x.abs() } else { 0.0 };
            let y_val = rel_err.max(err_floor).log10();
            let px = map(k, config.k_min, config.k_max, x0, x1);
            let py = map(y_val, y_min, y_max, y1, y0);
            pts.push(format!("{px:.2},{py:.2}"));
        }
        svg.push_str(&format!(
            "<polyline fill=\"none\" stroke=\"{}\" stroke-width=\"2\" points=\"{}\" />",
            color,
            pts.join(" ")
        ));
    }

    let mut legend_y = 58.0;
    for (idx, fmt) in config.formats.iter().enumerate() {
        let color = palette[idx % palette.len()];
        let lx0 = x1 - 210.0;
        let lx1 = x1 - 170.0;
        let tx = x1 - 160.0;
        svg.push_str(&format!(
            "<line x1=\"{lx0}\" y1=\"{legend_y}\" x2=\"{lx1}\" y2=\"{legend_y}\" stroke=\"{color}\" stroke-width=\"3\"/>"
        ));
        svg.push_str(&format!(
            "<text x=\"{tx}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"14\">{} (m={}, e=[{},{}])</text>",
            legend_y + 5.0,
            fmt.name,
            fmt.mantissa_bits,
            fmt.min_exp2,
            fmt.max_exp2
        ));
        legend_y += 22.0;
    }

    svg.push_str("</svg>");
    fs::write(svg_path, svg)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = match parse_args() {
        Ok(c) => c,
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(2);
        }
    };

    let ks = sample_k_values(config.k_min, config.k_max, config.k_step);

    let svg_path_s = format!("{}.svg", config.out_prefix);
    let csv_path_s = format!("{}.csv", config.out_prefix);
    let md_path_s = format!("{}_summary.md", config.out_prefix);
    let ranking_path_s = format!("{}_ranking.md", config.out_prefix);

    if let Some(parent) = Path::new(&svg_path_s).parent() {
        fs::create_dir_all(parent)?;
    }
    if let Some(parent) = Path::new(&csv_path_s).parent() {
        fs::create_dir_all(parent)?;
    }

    write_svg(&config, &ks, Path::new(&svg_path_s))?;
    write_csv(&config, &ks, Path::new(&csv_path_s))?;
    write_summary(&config, Path::new(&md_path_s))?;
    write_ranking(&config, &ks, Path::new(&ranking_path_s))?;

    println!("Wrote {}", svg_path_s);
    println!("Wrote {}", csv_path_s);
    println!("Wrote {}", md_path_s);
    println!("Wrote {}", ranking_path_s);

    Ok(())
}
