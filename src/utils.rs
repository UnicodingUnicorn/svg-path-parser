pub fn is_separator(ch:char) -> bool {
    ch.is_whitespace() || ch == ','
}

pub fn is_number_part(ch:char) -> bool {
    ch == '-' || ch == '.' || ch.is_digit(10)
}

pub fn add_point((ax, ay):(f64, f64), (bx, by):(f64, f64)) -> (f64, f64) {
    (ax + bx, ay + by)
}