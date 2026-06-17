use crate::triangle::TriangleResult;

pub fn print_report(result: &TriangleResult, difficulty: &str, mode: &str, baseline_label: Option<&str>) {
    let s_pct = pct(result.space.normalized);
    let e_pct = pct(result.energy.normalized);
    let t_pct = pct(result.time.normalized);
    let score_pct = pct(result.score);

    let baseline_str = baseline_label.unwrap_or("(nenhuma)");

    println!();
    println!("╔════════════════════════════════════════════════════╗");
    println!("║        ▴  TRIANGLE METRIC                        ║");
    println!("║    espaço · energia · tempo                       ║");
    println!("║         (menor = melhor)                          ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!();

    print_triangle(result.score);

    let bl_s = fmt_val(result.space.baseline);
    let cu_s = fmt_val(result.space.current);
    let bl_e = fmt_val(result.energy.baseline);
    let cu_e = fmt_val(result.energy.current);
    let bl_t = fmt_val(result.time.baseline);
    let cu_t = fmt_val(result.time.current);

    println!();
    println!("  ┌─────────────┬──────────┬──────────┬──────────┐");
    println!("  │ Eixo        │ Baseline │ Atual    │ Δ        │");
    println!("  ├─────────────┼──────────┼──────────┼──────────┤");
    println!("  │   Espaço    │ {:>8} │ {:>8} │ {:>7} │", bl_s, cu_s, s_pct);
    println!("  │   Energia   │ {:>8} │ {:>8} │ {:>7} │", bl_e, cu_e, e_pct);
    println!("  │   Tempo     │ {:>8} │ {:>8} │ {:>7} │", bl_t, cu_t, t_pct);
    println!("  ├─────────────┼──────────┼──────────┼──────────┤");
    println!("  │   SCORE     │  1.000   │ {:>8} │ {:>7} │", fmt_score(result.score), score_pct);
    println!("  └─────────────┴──────────┴──────────┴──────────┘");
    println!();
    println!("  Dificuldade: {} (depth {})", difficulty, depth_from_diff(difficulty));
    println!("  Modo: {}", mode);
    println!("  Baseline: {}", baseline_str);
    println!();
    println!("  Area do triangulo: {:.4} (baseline = {:.4})", result.area, 2.598_076_211);
    println!("  Δ area: {}", pct(result.area / 2.598_076_211));
    println!();
}

fn pct(norm: f64) -> String {
    let delta = (norm - 1.0) * 100.0;
    if delta.abs() < 0.01 {
        "  0.0%".to_string()
    } else if delta < 0.0 {
        format!(" {:>+.1}%", delta)
    } else {
        format!(" {:>+.1}%", delta)
    }
}

fn fmt_val(v: f64) -> String {
    if v >= 1_000_000.0 {
        format!("{:.2}ms", v / 1_000_000.0)
    } else if v >= 1_000.0 {
        format!("{:.3}us", v / 1_000.0)
    } else if v >= 1.0 {
        format!("{:.1}ns", v)
    } else {
        format!("{:.3}", v)
    }
}

fn fmt_score(s: f64) -> String {
    format!("{:.3}", s)
}

fn depth_from_diff(d: &str) -> &str {
    match d.to_lowercase().as_str() {
        "easy" => "2",
        "medium" => "4",
        "hard" => "7",
        _ => "?",
    }
}

fn print_triangle(score: f64) {
    let rows = 7;
    let fill = if score < 0.33 { '░' } else if score < 0.66 { '▒' } else { '█' };
    let fill_count = (score * rows as f64).round() as usize;

    for r in (0..rows).rev() {
        let indent = rows - r;
        let inner = 2 * r + 1;
        print!("    ");
        for _ in 0..indent {
            print!(" ");
        }
        for c in 0..inner {
            if r == 0 && c == inner / 2 {
                print!("▲");
            } else if c == 0 || c == inner - 1 {
                print!("╱");
            } else if r > 0 && r <= fill_count {
                print!("{}", fill);
            } else {
                print!(" ");
            }
        }
        println!();
    }
    // Bottom
    print!("    ╱");
    for _ in 0..2 * rows {
        print!("─");
    }
    println!("╲");
}
