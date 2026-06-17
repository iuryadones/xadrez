/// Triangle metric computation.
///
/// Normalizes each axis against baseline, then computes score = S × E × T
/// and area of the triangle formed by the 3 normalized values on a radar plot.
///
/// Angles between axes on a 3-axis radar = 120°.

#[derive(Debug, Clone)]
pub struct TrianglePoint {
    pub label: String,
    pub baseline: f64,
    pub current: f64,
    pub normalized: f64,
}

#[derive(Debug)]
pub struct TriangleResult {
    pub space: TrianglePoint,
    pub energy: TrianglePoint,
    pub time: TrianglePoint,
    pub score: f64,
    pub area: f64,
}

pub fn compute_triangle(
    space_baseline: f64,
    space_current: f64,
    energy_baseline: f64,
    energy_current: f64,
    time_baseline: f64,
    time_current: f64,
) -> TriangleResult {
    let s = TrianglePoint {
        label: "Espaço".into(),
        baseline: space_baseline,
        current: space_current,
        normalized: space_current / space_baseline,
    };
    let e = TrianglePoint {
        label: "Energia".into(),
        baseline: energy_baseline,
        current: energy_current,
        normalized: energy_current / energy_baseline,
    };
    let t = TrianglePoint {
        label: "Tempo".into(),
        baseline: time_baseline,
        current: time_current,
        normalized: time_current / time_baseline,
    };

    // Score = product of normalized values (smaller = better)
    let score = s.normalized * e.normalized * t.normalized;

    // Area of triangle with side lengths between points on 3-axis radar (angle 120°)
    // Side lengths (law of cosines with 120° between axes):
    let d_se = (s.normalized.powi(2) + e.normalized.powi(2) + s.normalized * e.normalized).sqrt();
    let d_et = (e.normalized.powi(2) + t.normalized.powi(2) + e.normalized * t.normalized).sqrt();
    let d_ts = (t.normalized.powi(2) + s.normalized.powi(2) + t.normalized * s.normalized).sqrt();

    // Heron's formula
    let p = (d_se + d_et + d_ts) / 2.0;
    let area = (p * (p - d_se) * (p - d_et) * (p - d_ts)).sqrt();

    TriangleResult { space: s, energy: e, time: t, score, area }
}
