use crate::color_constants::*;

// TODO: hide impl, have constructor return option. (b.c. some labs invalid)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ColorLab {
    pub l: i8,
    pub a: i8,
    pub b: i8,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ColorRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct ColorXyz {
    x: i32,
    y: i32,
    z: i32,
}

// The inverse of function f, defined here:
//    https://en.wikipedia.org/wiki/CIELAB_color_space#CIELAB
// Both input and output scaled by 12 bits.
fn f_inv(t: i32) -> i32 {
    if t > DELTA {
        (((t * t) >> 12) * t) >> 12
    } else {
        ((3 * (DELTA >> 6) * (DELTA >> 6)) * (t - (4 << 12) / 29)) >> 12
    }
}

// Convert from CIE-lab to CIE-XYZ, defined here:
//    https://en.wikipedia.org/wiki/CIELAB_color_space#CIELAB
// Output scaled by 12 bits.
fn lab_to_xyz(lab: ColorLab) -> ColorXyz {
    let ColorLab { l, a, b } = lab;
    let l_adj: i32 = (((l as i32) + 16) << 12) / 116;
    let a_adj: i32 = ((a as i32) << 12) / 500;
    let b_adj: i32 = -((b as i32) << 12) / 200;
    ColorXyz {
        x: D65_ILLUMINANT[0] * f_inv(l_adj + a_adj),
        y: D65_ILLUMINANT[1] * f_inv(l_adj),
        z: D65_ILLUMINANT[2] * f_inv(l_adj + b_adj),
    }
}

// Convert from CIE-XYZ to sRGB, defined here:
//    https://en.wikipedia.org/wiki/SRGB
// Input and output scaled by 12 bits.
// Assumes that the range of LAB's L value is [0, 100]
// and that the range of the output RGB values is [0, 256]
fn xyz_to_srgb(xyz: ColorXyz) -> ColorRgb {
    let ColorXyz {
        mut x,
        mut y,
        mut z,
    } = xyz;
    x >>= 6;
    y >>= 6;
    z >>= 6;
    let t = XYZ_TO_RGB_LINEAR;
    let gamma = linear_rgb_to_srgb;
    ColorRgb {
        r: gamma((t[0][0] * x + t[0][1] * y + t[0][2] * z) >> 6),
        g: gamma((t[1][0] * x + t[1][1] * y + t[1][2] * z) >> 6),
        b: gamma((t[2][0] * x + t[2][1] * y + t[2][2] * z) >> 6)
    }
}

// Convert from linear RGB to sRGB, defined here:
//     https://en.wikipedia.org/wiki/SRGB
// Input scaled by 12 bits.
fn linear_rgb_to_srgb(u: i32) -> u8 {
    match LINEAR_RGB_TO_SRGB.binary_search(&u) {
        Ok(i) => i as u8,
        Err(i) => (i - 1) as u8
    }
}

fn lab_to_srgb(lab: ColorLab) -> ColorRgb {
    xyz_to_srgb(lab_to_xyz(lab))
}

impl From<ColorLab> for ColorRgb {
    fn from(lab: ColorLab) -> ColorRgb {
        lab_to_srgb(lab)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn convert(l: i8, a: i8, b: i8) -> (u8, u8, u8) {
        let ColorRgb{r, g, b} = lab_to_srgb(ColorLab{l, a, b});
        (r, g, b)
    }

    // Tested against http://colorizer.org/
    // All within 1/256, or occassionally 2/256.

    #[test]
    fn test_grayscale() {
        assert_eq!(convert(0, 0, 0), (0, 0, 0));
        assert_eq!(convert(1, 0, 0), (2, 3, 2));
        assert_eq!(convert(2, 0, 0), (5, 6, 6));
        assert_eq!(convert(3, 0, 0), (10, 10, 10));
        assert_eq!(convert(4, 0, 0), (13, 13, 13));
        assert_eq!(convert(10, 0, 0), (26, 27, 27));
        assert_eq!(convert(25, 0, 0), (59, 59, 59));
        assert_eq!(convert(50, 0, 0), (119, 119, 119));
        assert_eq!(convert(75, 0, 0), (185, 185, 185));
        assert_eq!(convert(90, 0, 0), (226, 227, 227));
        assert_eq!(convert(97, 0, 0), (247, 247, 247));
        assert_eq!(convert(98, 0, 0), (249, 250, 250));
        assert_eq!(convert(99, 0, 0), (252, 253, 253));
        assert_eq!(convert(100, 0, 0), (255, 255, 255));
    }

    #[test]
    fn test_color() {
        assert_eq!(convert(3, -3, 0), (1, 12, 10));
        assert_eq!(convert(10, 0, 10), (31, 26, 10));
        assert_eq!(convert(70, 40, 0), (239, 143, 173));
        assert_eq!(convert(70, 25, -30), (190, 158, 226));
        assert_eq!(convert(40, -40, 40), (34, 108, 18));
        assert_eq!(convert(72, 0, -43), (115, 181, 255));
    }
}
