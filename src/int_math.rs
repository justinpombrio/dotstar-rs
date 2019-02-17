pub fn sin(deg: isize, multiplier: isize) -> isize {
    if deg < 0 {
        return -sin(-deg, multiplier);
    }
    let deg = deg % 360;
    if deg > 180 {
        return -sin(360 - deg, multiplier);
    }
    // Thanks, Bhaskara I
    multiplier * 4 * deg * (180 - deg) / (40500 - deg * (180 - deg))
}

pub fn cos(deg: isize, multiplier: isize) -> isize {
    sin(90 - deg, multiplier)
}

pub fn inc(x: &mut i8, delta: i8, min: i8, max: i8) {
    let new_x = x.saturating_add(delta);
    if new_x < min {
        *x = min;
    } else if new_x > max {
        *x = max;
    } else {
        *x = new_x;
    }
}
