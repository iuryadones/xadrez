#[derive(Debug, Clone)]
pub struct Estimate {
    pub point_estimate: f64,
}

#[derive(Debug, Clone)]
pub struct BenchResult {
    /// Full benchmark name as string (e.g. "board_clone/initial")
    pub name: String,
    /// Which sub-run this comes from ("new", "base", or a baseline name)
    pub variant: String,
    pub mean: Estimate,
}

/// Scan `<root>/criterion/` for all `new/estimates.json` (current runs)
/// and also `<baseline>/estimates.json` if a baseline name is given.
pub fn load_all_results(root: &str, baseline: Option<&str>) -> Vec<BenchResult> {
    let mut results = Vec::new();
    let criterion_dir = std::path::Path::new(root).join("criterion");
    if !criterion_dir.is_dir() {
        return results;
    }
    let mut stack = vec![(criterion_dir.clone(), Vec::new())];
    while let Some((dir, components)) = stack.pop() {
        if components.len() > 3 {
            continue;
        }
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let mut child = components.clone();
                    if let Some(name) = path.file_name() {
                        child.push(name.to_string_lossy().to_string());
                    }
                    stack.push((path, child));
                } else if path.file_name().map_or(false, |n| n == "estimates.json") {
                    if let Some((bench_name, variant)) = classify_path(components.clone(), baseline) {
                        if let Some(est) = read_estimate(&path) {
                            results.push(BenchResult {
                                name: bench_name,
                                variant,
                                mean: est,
                            });
                        }
                    }
                }
            }
        }
    }
    results
}

/// Determine benchmark name and variant from the path components.
///
/// Expected:
///   criterion/<bench_name>/new/estimates.json        → variant = "new"
///   criterion/<bench_name>/base/estimates.json       → variant = "base"
///   criterion/<bench_name>/<baseline>/estimates.json → variant = "<baseline>"
fn classify_path(
    components: Vec<String>,
    baseline: Option<&str>,
) -> Option<(String, String)> {
    if components.len() < 2 {
        return None;
    }
    // components = [bench_name, subdir_or_file]
    // So full_path has at least: .../<bench_name>/<subdir>/estimates.json
    let bench_name = components[0].clone();
    let subdir = &components[1];

    let variant = match subdir.as_str() {
        "new" => "new".to_string(),
        "base" => "base".to_string(),
        "change" => "change".to_string(),
        other => {
            // Could be a named baseline
            if baseline.map_or(false, |b| b == other) {
                other.to_string()
            } else {
                return None; // skip unknown subdirectories
            }
        }
    };

    // Reconstruct the original benchmark name by un-flattening underscores
    // Criterion replaces `/` with `_` in directory names
    // Common patterns:
    //   "board_clone_initial" → "board_clone/initial"
    //   "legal_moves_kiwipete" → "legal_moves/kiwipete"
    // But we can't know which underscore is the group separator.
    // Use a lookup table of known benchmark prefixes.
    let original = unflatten_name(&bench_name);
    Some((original, variant))
}

/// Known benchmark prefixes for name unflattening.
/// Maps from flat-name-prefix → desired group/name format.
fn unflatten_name(flat: &str) -> String {
    let prefixes = [
        ("best_move_", "time/best_move"),
        ("board_clone_", "space/board_clone"),
        ("game_clone_", "space/game_clone"),
        ("compute_hash_", "space/compute_hash"),
        ("update_hash_", "space/update_hash"),
        ("tt_probe_", "space/tt_probe"),
        ("tt_record", "space/tt_record"),
        ("vec_move_", "space/vec_move"),
        ("size_of_types_", "space/size_of_types"),
        ("evaluate_", "energy/evaluate"),
        ("pseudo_legal_", "energy/pseudo_legal"),
        ("legal_moves_", "energy/legal_moves"),
        ("capture_moves_", "energy/capture_moves"),
        ("is_attacked_", "energy/is_attacked"),
        ("is_legal_", "energy/is_legal"),
        ("order_moves_", "energy/order_moves"),
        ("pst_bonus_", "energy/pst_bonus"),
        ("piece_value_", "energy/piece_value"),
        ("perft_", "energy/perft"),
    ];
    for (prefix, replacement) in &prefixes {
        if let Some(suffix) = flat.strip_prefix(prefix) {
            return format!("{}/{}", replacement, suffix);
        }
    }
    // Fallback: keep as-is
    flat.to_string()
}

fn read_estimate(path: &std::path::Path) -> Option<Estimate> {
    let content = std::fs::read_to_string(path).ok()?;
    let val: serde_json::Value = serde_json::from_str(&content).ok()?;
    let mean = val.get("Mean").or_else(|| val.get("mean"))?;
    let point_estimate = mean.get("point_estimate")?.as_f64()?;
    Some(Estimate { point_estimate })
}

/// Find a specific benchmark result by name prefix and variant.
pub fn lookup<'a>(
    results: &'a [BenchResult],
    name_prefix: &str,
    variant: &str,
) -> Option<&'a BenchResult> {
    results.iter().find(|r| {
        r.name.starts_with(name_prefix) && r.variant == variant
    })
}
