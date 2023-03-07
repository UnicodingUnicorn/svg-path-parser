use crate::utils;
use std::f64::consts::PI;

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

// According to the implementation notes in Appendix B in the SVG spec
pub fn compute_arc(start:(f64, f64), r:(f64, f64), rotation:f64, large:bool, sweep:bool, end:(f64, f64), resolution:u64) -> Vec<(f64, f64)> {
    // Handle edge cases
    if r.0 == 0.0 || r.1 == 0.0 { 
        // Return a line according to the spec
        return vec![start, end];
    } else if start.0 == end.0 && start.1 == end.1 { // start == end
        return Vec::new();
    }

    // Normalise values
    let r = (r.0.abs(), r.1.abs());
    // Convert degrees 'rotation' to radians
    let rotation = rotation * PI / 180.0;

    // Find centre
    let x = (start.0 - end.0) / 2.0; // x'
    let y = (start.1 - end.1) / 2.0; // y'
    let (x, y) = utils::rotate(-rotation, (x, y));

    // Scale radius to match
    let a = (x * x) / (r.0 * r.0) + (y * y) / (r.1 * r.1);
    let r = match a > 1.0 {
        true => (r.0 * a.sqrt(), r.1 * a.sqrt()),
        false => r,
    };

    // Actually calculate centre
    let sign = match large == sweep {
        true => -1.0,
        false => 1.0,
    };

    // The abs is a little bit of insurance against floating point errors
    let n = sign * ((r.0 * r.0 * r.1 * r.1 - r.0 * r.0 * y * y - r.1 * r.1 * x * x).abs() / (r.0 * r.0 * y * y + r.1 * r.1 * x * x)).sqrt();
    let cx_prime = n * r.0 * y / r.1;
    let cy_prime = -n * r.1 * x / r.0;

    let (cx, cy) = utils::rotate(rotation, (cx_prime, cy_prime));
    let cx = cx + (start.0 + end.0) / 2.0;
    let cy = cy + (start.1 + end.1) / 2.0;

    // Find angles
    let a = ((x - cx_prime) / r.0, (y - cy_prime) / r.1);
    let starting_angle = utils::find_angle((1.0, 0.0), a);

    let b = ((-x - cx_prime) / r.0, (-y - cy_prime) / r.1);
    let delta_angle = utils::find_angle(a, b) % (2.0 * PI);
    let delta_angle = match (sweep, delta_angle > 0.0) {
        (false, true) => delta_angle - 2.0 * PI,
        (true, false) => delta_angle + 2.0 * PI,
        _ => delta_angle,
    };
    
    // Draw curve
    let step = delta_angle / (resolution as f64);
    let mut res = Vec::new();
    for i in 0..(resolution + 1) {
        let theta = starting_angle + (i as f64) * step;

        let (x, y) = utils::rotate(rotation, (r.0 * theta.cos(), r.1 * theta.sin()));
        res.push((x + cx, y + cy));
    }

    res
}