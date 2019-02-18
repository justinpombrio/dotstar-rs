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

pub fn sqrt(x: i32) -> i8 {
    match SQUARES.binary_search(&x) {
        Ok(i) => i as i8,
        Err(128) => 127 as i8,
        Err(i) => i as i8,
    }
}

/// Table of squares, for computing the square root of an `i8`.
pub const SQUARES: [i32; 128] = [
    0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289,
    324, 361, 400, 441, 484, 529, 576, 625, 676, 729, 784, 841, 900, 961, 1024,
    1089, 1156, 1225, 1296, 1369, 1444, 1521, 1600, 1681, 1764, 1849, 1936,
    2025, 2116, 2209, 2304, 2401, 2500, 2601, 2704, 2809, 2916, 3025, 3136,
    3249, 3364, 3481, 3600, 3721, 3844, 3969, 4096, 4225, 4356, 4489, 4624,
    4761, 4900, 5041, 5184, 5329, 5476, 5625, 5776, 5929, 6084, 6241, 6400,
    6561, 6724, 6889, 7056, 7225, 7396, 7569, 7744, 7921, 8100, 8281, 8464,
    8649, 8836, 9025, 9216, 9409, 9604, 9801, 10000, 10201, 10404, 10609,
    10816, 11025, 11236, 11449, 11664, 11881, 12100, 12321, 12544, 12769,
    12996, 13225, 13456, 13689, 13924, 14161, 14400, 14641, 14884, 15129,
    15376, 15625, 15876, 16129,
];

#[test]
fn test_inc() {
    let mut x: i8 = 0;
    inc(&mut x, 100, -10, 10);
    assert_eq!(x, 10);
    inc(&mut x, -21, -10, 10);
    assert_eq!(x, -10);
    x = 0;
    inc(&mut x, -10, -10, 10);
    assert_eq!(x, -10);
}
