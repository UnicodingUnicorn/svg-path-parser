pub fn is_separator(ch:char) -> bool {
    ch.is_whitespace() || ch == ','
}

pub fn is_number_part(ch:char) -> bool {
    ch == '+' || ch == '-' || ch == '.' || ch.is_digit(10)
}

pub fn add_point((ax, ay):(f64, f64), (bx, by):(f64, f64)) -> (f64, f64) {
    (ax + bx, ay + by)
}

// a about b
pub fn reflect_point((ax, ay):(f64, f64), (bx, by):(f64, f64)) -> (f64, f64) {
    let x = bx + (bx - ax);
    let y = by + (by - ay);

    (x, y)
}

// Some simple matrix operations
fn magnitude((x, y):(f64, f64)) -> f64 {
    (x * x + y * y).sqrt()
}

pub fn find_angle(u:(f64, f64), v:(f64, f64)) -> f64 {
    let angle = ((u.0 * v.0 + u.1 * v.1) / (magnitude(u) * magnitude(v))).acos();
    let sign = (u.0 * v.1 - u.1 * v.0).signum();

    sign * angle
}

// Applies a rotation matrix
pub fn rotate(theta:f64, (x, y):(f64, f64)) -> (f64, f64) {
    (theta.cos() * x - theta.sin() * y, theta.sin() * x + theta.cos() * y)
}