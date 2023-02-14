// Doing a dynamic Bezier Curve would waste too much compute
fn scale(t:f64, i:u64, n:u64) -> f64 {
    (1.0 - t).powf((n - i) as f64) * t.powf(i as f64)
}

pub fn compute_cubic_bezier(start:(f64, f64), p1:(f64, f64), p2:(f64, f64), end:(f64, f64), n:u64) -> Vec<(f64, f64)> {
    let mut res = Vec::new();
    for i in 0..(n + 1) {
        let t = (i as f64) / (n as f64);
        let x = scale(t, 0, 3) * start.0 + 3.0 * scale(t, 1, 3) * p1.0 + 3.0 * scale(t, 2, 3) * p2.0 + scale(t, 3, 3) * end.0;
        let y = scale(t, 0, 3) * start.1 + 3.0 * scale(t, 1, 3) * p1.1 + 3.0 * scale(t, 2, 3) * p2.1 + scale(t, 3, 3) * end.1;

        res.push((x, y));
    }

    res
}

pub fn compute_quadratic_bezier(start:(f64, f64), p1:(f64, f64), end:(f64, f64), n:u64) -> Vec<(f64, f64)> {
    let mut res = Vec::new();
    for i in 0..(n + 1) {
        let t = (i as f64) / (n as f64);
        let x = scale(t, 0, 2) * start.0 + 2.0 * scale(t, 1, 2) * p1.0 + scale(t, 2, 2) * end.0;
        let y = scale(t, 0, 2) * start.1 + 2.0 * scale(t, 1, 2) * p1.1 + scale(t, 2, 2) * end.1;

        res.push((x, y))
    }

    res
}