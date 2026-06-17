mod reader;
mod triangle;
mod art;

use std::env;

const SPACE_BENCH: &str = "space/board_clone/initial";
const ENERGY_BENCH: &str = "energy/legal_moves/kiwipete";
const TIME_BENCH_EASY: &str = "time/best_move/easy_depth_2";
const TIME_BENCH_MEDIUM: &str = "time/best_move/medium_depth_4";
const TIME_BENCH_HARD: &str = "time/best_move/hard_depth_7";

fn usage() -> ! {
    eprintln!("Usage: triangle [OPTIONS]");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --mode <mode>          both | dev | release (default: both)");
    eprintln!("  --difficulty <diff>    easy | medium | hard (default: medium)");
    eprintln!("  --baseline <name>      baseline name for comparison");
    eprintln!("  --dir <path>           criterion output dir (default: target)");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  triangle                                   # both modes, medium");
    eprintln!("  triangle --mode release --difficulty hard");
    eprintln!("  triangle --baseline v1                     # compare against baseline 'v1'");
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        usage();
    }

    let mut mode = String::from("both");
    let mut difficulty = String::from("medium");
    let mut baseline = None::<String>;
    let mut dir = String::from("target");

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--mode" => {
                i += 1;
                mode = args.get(i).cloned().unwrap_or_else(|| String::from("both"));
            }
            "--difficulty" => {
                i += 1;
                difficulty = args.get(i).cloned().unwrap_or_else(|| String::from("medium"));
            }
            "--baseline" => {
                i += 1;
                baseline = Some(args.get(i).cloned().unwrap_or_else(|| String::from("baseline")));
            }
            "--dir" => {
                i += 1;
                dir = args.get(i).cloned().unwrap_or_else(|| String::from("target"));
            }
            _ => {
                eprintln!("Unknown option: {}", args[i]);
                usage();
            }
        }
        i += 1;
    }

    let time_bench = match difficulty.to_lowercase().as_str() {
        "easy" => TIME_BENCH_EASY,
        "medium" => TIME_BENCH_MEDIUM,
        "hard" => TIME_BENCH_HARD,
        _ => {
            eprintln!("Unknown difficulty: {}. Use easy, medium, or hard.", difficulty);
            std::process::exit(1);
        }
    };

    match mode.to_lowercase().as_str() {
        "both" => {
            println!("══════════════════ RELEASE ══════════════════");
            analyze(&dir, time_bench, &difficulty, baseline.as_deref(), "release (opt-level=3, lto, codegen-units=1)");
            println!();
            println!("═══════════════════ DEV ═════════════════════");
            analyze(&dir, time_bench, &difficulty, baseline.as_deref(), "dev (opt-level=1)");
        }
        "release" => {
            analyze(&dir, time_bench, &difficulty, baseline.as_deref(), "release (opt-level=3, lto, codegen-units=1)");
        }
        "dev" => {
            analyze(&dir, time_bench, &difficulty, baseline.as_deref(), "dev (opt-level=1)");
        }
        _ => {
            eprintln!("Unknown mode: {}. Use both, dev, or release.", mode);
            std::process::exit(1);
        }
    }
}

fn load_bench(results: &[reader::BenchResult], name_prefix: &str, variant: &str) -> f64 {
    reader::lookup(results, name_prefix, variant)
        .map(|r| r.mean.point_estimate)
        .unwrap_or_else(|| {
            eprintln!(
                "Warning: benchmark '{}' (variant '{}') not found",
                name_prefix, variant
            );
            1.0
        })
}

fn analyze(root: &str, time_bench: &str, difficulty: &str, baseline: Option<&str>, mode_label: &str) {
    let all_results = reader::load_all_results(root, baseline);
    if all_results.is_empty() {
        eprintln!("No benchmark results found in {}/criterion/", root);
        eprintln!("Run `cargo bench` first.");
        return;
    }

    let s_cur = load_bench(&all_results, SPACE_BENCH, "new");
    let e_cur = load_bench(&all_results, ENERGY_BENCH, "new");
    let t_cur = load_bench(&all_results, time_bench, "new");

    let (s_bl, e_bl, t_bl) = if let Some(ref bl) = baseline {
        (
            load_bench(&all_results, SPACE_BENCH, bl),
            load_bench(&all_results, ENERGY_BENCH, bl),
            load_bench(&all_results, time_bench, bl),
        )
    } else {
        (s_cur, e_cur, t_cur)
    };

    let result = triangle::compute_triangle(s_bl, s_cur, e_bl, e_cur, t_bl, t_cur);
    art::print_report(&result, difficulty, mode_label, baseline);
}
