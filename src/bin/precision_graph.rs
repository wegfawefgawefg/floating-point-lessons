use std::error::Error;
use std::fs;
use std::path::Path;

fn ulp32(x: f32) -> f32 {
    x.next_up() - x
}

fn ulp64(x: f64) -> f64 {
    x.next_up() - x
}

fn map(value: f64, src_min: f64, src_max: f64, dst_min: f64, dst_max: f64) -> f64 {
    let t = (value - src_min) / (src_max - src_min);
    dst_min + t * (dst_max - dst_min)
}

fn line_path(points: &[(f64, f64)]) -> String {
    points
        .iter()
        .map(|(x, y)| format!("{x:.2},{y:.2}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn step_path(points: &[(f64, f64)]) -> String {
    if points.is_empty() {
        return String::new();
    }

    let mut out = vec![format!("{:.2},{:.2}", points[0].0, points[0].1)];
    for i in 1..points.len() {
        let (x0, y0) = points[i - 1];
        let (x1, y1) = points[i];
        out.push(format!("{x1:.2},{y0:.2}"));
        out.push(format!("{x1:.2},{y1:.2}"));
        let _ = x0;
    }
    out.join(" ")
}

fn draw_axes(
    svg: &mut String,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    x_ticks: &[i32],
    y_ticks: &[f64],
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    x_label: &str,
    y_label: &str,
) {
    for &tick in x_ticks {
        let x = map(tick as f64, x_min, x_max, x0, x1);
        svg.push_str(&format!(
            "<line x1=\"{x:.2}\" y1=\"{y0:.2}\" x2=\"{x:.2}\" y2=\"{y1:.2}\" stroke=\"#ececec\" stroke-width=\"1\"/>"
        ));
        svg.push_str(&format!(
            "<text x=\"{x:.2}\" y=\"{:.2}\" font-family=\"sans-serif\" font-size=\"12\" text-anchor=\"middle\">{tick}</text>",
            y1 + 22.0
        ));
    }

    for &tick in y_ticks {
        let y = map(tick, y_min, y_max, y1, y0);
        svg.push_str(&format!(
            "<line x1=\"{x0:.2}\" y1=\"{y:.2}\" x2=\"{x1:.2}\" y2=\"{y:.2}\" stroke=\"#ececec\" stroke-width=\"1\"/>"
        ));
        svg.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" font-family=\"sans-serif\" font-size=\"12\" text-anchor=\"end\">{:.2}</text>",
            x0 - 8.0,
            y + 4.0,
            tick
        ));
    }

    svg.push_str(&format!(
        "<line x1=\"{x0:.2}\" y1=\"{y0:.2}\" x2=\"{x0:.2}\" y2=\"{y1:.2}\" stroke=\"#222\" stroke-width=\"2\"/>"
    ));
    svg.push_str(&format!(
        "<line x1=\"{x0:.2}\" y1=\"{y1:.2}\" x2=\"{x1:.2}\" y2=\"{y1:.2}\" stroke=\"#222\" stroke-width=\"2\"/>"
    ));

    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"16\" text-anchor=\"middle\">{x_label}</text>",
        (x0 + x1) / 2.0,
        y1 + 48.0
    ));
    let yc = (y0 + y1) / 2.0;
    svg.push_str(&format!(
        "<text x=\"22\" y=\"{yc}\" font-family=\"sans-serif\" font-size=\"16\" text-anchor=\"middle\" transform=\"rotate(-90 22,{yc})\">{y_label}</text>"
    ));
}

fn main() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("docs")?;

    let output = Path::new("docs/precision_over_range.svg");
    let width = 1400.0;
    let height = 980.0;

    let left = 90.0;
    let right = 40.0;
    let panel_h = 360.0;
    let gap = 110.0;

    let x0 = left;
    let x1 = width - right;
    let top_y0 = 80.0;
    let top_y1 = top_y0 + panel_h;
    let bot_y0 = top_y1 + gap;
    let bot_y1 = bot_y0 + panel_h;

    let k_min = -324.0;
    let k_max = 308.0;

    let dense_f64: Vec<(f64, f64)> = (0..=((k_max - k_min) * 4.0) as usize)
        .filter_map(|i| {
            let k = k_min + i as f64 * 0.25;
            let x = 10f64.powf(k);
            if !x.is_finite() || x <= 0.0 {
                return None;
            }
            let u = ulp64(x);
            if u > 0.0 {
                Some((k, u.log10()))
            } else {
                None
            }
        })
        .collect();

    let dense_f32: Vec<(f64, f64)> = (0..=((38.0 - (-45.0)) * 8.0) as usize)
        .filter_map(|i| {
            let k = -45.0 + i as f64 * 0.125;
            let x = 10f32.powf(k as f32);
            if !x.is_finite() || x <= 0.0 {
                return None;
            }
            let u = ulp32(x);
            if u > 0.0 {
                Some((k, (u as f64).log10()))
            } else {
                None
            }
        })
        .collect();

    let top_y_min = -330.0;
    let top_y_max = 300.0;

    let top_f64_px: Vec<(f64, f64)> = dense_f64
        .iter()
        .map(|(k, y)| {
            (
                map(*k, k_min, k_max, x0, x1),
                map(*y, top_y_min, top_y_max, top_y1, top_y0),
            )
        })
        .collect();

    let top_f32_px: Vec<(f64, f64)> = dense_f32
        .iter()
        .map(|(k, y)| {
            (
                map(*k, k_min, k_max, x0, x1),
                map(*y, top_y_min, top_y_max, top_y1, top_y0),
            )
        })
        .collect();

    let log10_2 = 2f64.log10();

    let residual_f64: Vec<(f64, f64)> = (-20..=20)
        .flat_map(|ki| {
            (0..8).filter_map(move |sub| {
                let k = ki as f64 + sub as f64 / 8.0;
                let x = 10f64.powf(k);
                if !x.is_finite() || x <= 0.0 {
                    return None;
                }
                let u = ulp64(x);
                if u <= 0.0 {
                    return None;
                }
                let y = u.log10();
                let baseline = k - 52.0 * log10_2;
                Some((k, y - baseline))
            })
        })
        .collect();

    let residual_f32: Vec<(f64, f64)> = (-20..=20)
        .flat_map(|ki| {
            (0..8).filter_map(move |sub| {
                let k = ki as f64 + sub as f64 / 8.0;
                let x = 10f32.powf(k as f32);
                if !x.is_finite() || x <= 0.0 {
                    return None;
                }
                let u = ulp32(x);
                if u <= 0.0 {
                    return None;
                }
                let y = (u as f64).log10();
                let baseline = k - 23.0 * log10_2;
                Some((k, y - baseline))
            })
        })
        .collect();

    let bot_x_min = -20.0;
    let bot_x_max = 21.0;
    let bot_y_min = -log10_2 - 0.02;
    let bot_y_max = 0.02;

    let bot_f64_px: Vec<(f64, f64)> = residual_f64
        .iter()
        .map(|(k, r)| {
            (
                map(*k, bot_x_min, bot_x_max, x0, x1),
                map(*r, bot_y_min, bot_y_max, bot_y1, bot_y0),
            )
        })
        .collect();

    let bot_f32_px: Vec<(f64, f64)> = residual_f32
        .iter()
        .map(|(k, r)| {
            (
                map(*k, bot_x_min, bot_x_max, x0, x1),
                map(*r, bot_y_min, bot_y_max, bot_y1, bot_y0),
            )
        })
        .collect();

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\">"
    ));
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"white\" />");
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"42\" font-family=\"sans-serif\" font-size=\"34\" text-anchor=\"middle\">Floating-point precision over range (ULP at x = 10^k)</text>",
        width / 2.0
    ));

    draw_axes(
        &mut svg,
        x0,
        top_y0,
        x1,
        top_y1,
        &[-300, -200, -100, 0, 100, 200, 300],
        &[-300.0, -200.0, -100.0, 0.0, 100.0, 200.0, 300.0],
        k_min,
        k_max,
        top_y_min,
        top_y_max,
        "k where x = 10^k",
        "log10(ULP(x))",
    );

    svg.push_str(&format!(
        "<polyline fill=\"none\" stroke=\"#1565c0\" stroke-width=\"2\" points=\"{}\" />",
        line_path(&top_f64_px)
    ));
    svg.push_str(&format!(
        "<polyline fill=\"none\" stroke=\"#c62828\" stroke-width=\"2\" points=\"{}\" />",
        line_path(&top_f32_px)
    ));

    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"18\" fill=\"#444\" text-anchor=\"start\">Global trend (dense sampling)</text>",
        x0 + 8.0,
        top_y0 - 12.0
    ));

    draw_axes(
        &mut svg,
        x0,
        bot_y0,
        x1,
        bot_y1,
        &[-20, -10, 0, 10, 20],
        &[-0.30, -0.20, -0.10, 0.0],
        bot_x_min,
        bot_x_max,
        bot_y_min,
        bot_y_max,
        "k where x = 10^k",
        "Residual vs baseline",
    );

    svg.push_str(&format!(
        "<polyline fill=\"none\" stroke=\"#1565c0\" stroke-width=\"1.8\" points=\"{}\" />",
        step_path(&bot_f64_px)
    ));
    svg.push_str(&format!(
        "<polyline fill=\"none\" stroke=\"#c62828\" stroke-width=\"1.8\" points=\"{}\" />",
        step_path(&bot_f32_px)
    ));

    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"18\" fill=\"#444\" text-anchor=\"start\">Jagged view (step plot of sawtooth residual)</text>",
        x0 + 8.0,
        bot_y0 - 12.0
    ));

    let legend_y = 64.0;
    svg.push_str(&format!(
        "<line x1=\"{}\" y1=\"{legend_y}\" x2=\"{}\" y2=\"{legend_y}\" stroke=\"#1565c0\" stroke-width=\"3\"/><text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"17\">f64</text>",
        x1 - 180.0,
        x1 - 140.0,
        x1 - 130.0,
        legend_y + 6.0
    ));
    svg.push_str(&format!(
        "<line x1=\"{}\" y1=\"{legend_y}\" x2=\"{}\" y2=\"{legend_y}\" stroke=\"#c62828\" stroke-width=\"3\"/><text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"17\">f32</text>",
        x1 - 90.0,
        x1 - 50.0,
        x1 - 40.0,
        legend_y + 6.0
    ));

    svg.push_str("</svg>");

    fs::write(output, svg)?;
    println!("Wrote {}", output.display());

    Ok(())
}
