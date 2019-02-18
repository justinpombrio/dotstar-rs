use crate::color_constants::*;

/// [CIE-LAB](https://en.wikipedia.org/wiki/CIELAB_color_space#CIELAB) colors.
/// `l` ranges from 0 to 99. The range of `a` and `b` is complex and reflects
/// human vision, but invalid LAB colors can be clamped.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ColorLab {
    pub l: i8,
    pub a: i8,
    pub b: i8,
}

/// [sRGB](https://en.wikipedia.org/wiki/SRGB) colors.
/// `r`, `g`, and `b` range from 0 to 255.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ColorRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorLab {
    /// Convert to sRGB. If this color is invalid, return an `Err` of a clamped
    /// version of it.
    pub fn to_srgb(self) -> Result<ColorRgb, ColorRgb> {
        lab_to_srgb(self)
    }

    /// Convert to sRGB. If this color is invalid, automatically clamp it to a
    /// valid color.
    pub fn to_srgb_clamped(self) -> ColorRgb {
        match lab_to_srgb(self) {
            Ok(color) => color,
            Err(color) => color,
        }
    }

    /// Check if this is a valid (i.e., representable) color.
    pub fn is_valid(&self) -> bool {
        lab_to_srgb(*self).is_ok()
    }

    /// Find the radius of the largest valid A,B circle centered on this color.
    pub fn max_radius(&self) -> i8 {
        if self.a > 40 || self.a < -40 || self.b > 40 || self.b < -40 {
            return 0;
        }
        for radius in 0..100 {
            // Test only eight points on the circle, because speed is important.
            let hr = ((radius as i32) * 7071 / 10000) as i8; // sqrt(2) * radius
            let mut colors = [*self; 8];
            colors[0].a += radius;
            colors[1].a -= radius;
            colors[2].b += radius;
            colors[3].b -= radius;
            colors[4].a += hr;
            colors[4].b += hr;
            colors[5].a += hr;
            colors[5].b -= hr;
            colors[6].a -= hr;
            colors[6].b += hr;
            colors[7].a -= hr;
            colors[7].b -= hr;
            for color in &colors {
                if !color.is_valid() {
                    return radius;
                }
            }
        }
        // Something's gone wrong; return 0 to be safe.
        return 0;
    }
}

impl ColorRgb {
    pub fn black() -> ColorRgb {
        ColorRgb { r: 0, g: 0, b: 0 }
    }

    /// Apply gamma correction. This is needed when displaying the color on an
    /// RGB LED. Other devices like a computer monitor already have the
    /// correction built-in.
    pub fn correct_gamma(&self) -> ColorRgb {
        ColorRgb {
            r: GAMMA_CORRECTION[self.r as usize],
            g: GAMMA_CORRECTION[self.g as usize],
            b: GAMMA_CORRECTION[self.b as usize],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct ColorXyz {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct ColorLinearRgb {
    r: i32,
    g: i32,
    b: i32,
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

// Convert from CIE-XYZ to linear RGB, described here:
//    https://en.wikipedia.org/wiki/SRGB
// Input and output scaled by 12 bits.
fn xyz_to_linear_rgb(xyz: ColorXyz) -> ColorLinearRgb {
    let ColorXyz {
        mut x,
        mut y,
        mut z,
    } = xyz;
    x >>= 6;
    y >>= 6;
    z >>= 6;
    let t = XYZ_TO_RGB_LINEAR;
    ColorLinearRgb {
        r: (t[0][0] * x + t[0][1] * y + t[0][2] * z) >> 6,
        g: (t[1][0] * x + t[1][1] * y + t[1][2] * z) >> 6,
        b: (t[2][0] * x + t[2][1] * y + t[2][2] * z) >> 6,
    }
}

// Convert from linear RGB to sRGB, described here:
//    https://en.wikipedia.org/wiki/SRGB
// Assumes that the range of LAB's L value is [0, 100]
// and that the range of the output RGB values is [0, 256]
fn linear_rgb_to_srgb(rgb: ColorLinearRgb) -> Result<ColorRgb, ColorRgb> {
    let ColorLinearRgb { r, g, b } = rgb;
    match (gamma(r), gamma(g), gamma(b)) {
        (Ok(r), Ok(g), Ok(b)) => Ok(ColorRgb { r, g, b }),
        (Ok(r), Ok(g), Err(b)) => Err(ColorRgb { r, g, b }),
        (Ok(r), Err(g), Ok(b)) => Err(ColorRgb { r, g, b }),
        (Ok(r), Err(g), Err(b)) => Err(ColorRgb { r, g, b }),
        (Err(r), Ok(g), Ok(b)) => Err(ColorRgb { r, g, b }),
        (Err(r), Ok(g), Err(b)) => Err(ColorRgb { r, g, b }),
        (Err(r), Err(g), Ok(b)) => Err(ColorRgb { r, g, b }),
        (Err(r), Err(g), Err(b)) => Err(ColorRgb { r, g, b }),
    }
}

// The gamma function for converting from linear RGB to sRGB.
// Return Ok(answer) if in range, or Err(clamped_answer) if out of range.
//     https://en.wikipedia.org/wiki/SRGB
// Input scaled by 12 bits.
fn gamma(u: i32) -> Result<u8, u8> {
    match LINEAR_RGB_TO_SRGB.binary_search(&u) {
        Ok(i) => Ok(i as u8),
        Err(0) => Err(0),
        Err(256) => Err(255),
        Err(i) => Ok((i - 1) as u8),
    }
}

fn lab_to_srgb(lab: ColorLab) -> Result<ColorRgb, ColorRgb> {
    linear_rgb_to_srgb(xyz_to_linear_rgb(lab_to_xyz(lab)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn convert(l: i8, a: i8, b: i8) -> (u8, u8, u8) {
        let ColorRgb { r, g, b } = ColorLab { l, a, b }.to_srgb().unwrap();
        (r, g, b)
    }

    fn clamp(l: i8, a: i8, b: i8) -> (u8, u8, u8) {
        let ColorRgb { r, g, b } = ColorLab { l, a, b }.to_srgb().unwrap_err();
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
    }

    #[test]
    fn test_color() {
        assert_eq!(convert(3, -3, 0), (1, 12, 10));
        assert_eq!(convert(10, 0, 10), (31, 26, 10));
        assert_eq!(convert(70, 40, 0), (239, 143, 173));
        assert_eq!(convert(70, 25, -30), (190, 158, 226));
        assert_eq!(convert(40, -40, 40), (34, 108, 18));
        assert_eq!(convert(72, 0, -42), (117, 180, 253));
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(110, 0, 0), (255, 255, 255));
        assert_eq!(clamp(2, 20, 20), (42, 0, 0));
        assert_eq!(clamp(100, -100, 50), (0, 255, 151));
        // Make sure that extreme values don't cause under/overflow errors.
        clamp(-128, -128, -128);
        clamp(-128, -128, 127);
        clamp(-128, 127, -128);
        clamp(-128, 127, 127);
        clamp(127, -128, -128);
        clamp(127, -128, 127);
        clamp(127, 127, -128);
        clamp(127, 127, 127);
    }
}
