pub struct ColorLab {
    pub l: i8,
    pub a: i8,
    pub b: i8,
}

pub struct ColorRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

struct ColorXyz {
    x: i32,
    y: i32,
    z: i32,
}

pub struct Illuminant {
    x_n: i32,
    y_n: i32,
    z_n: i32,
}

pub const D65_ILLUMINANT: Illuminant = Illuminant {
    x_n: 95,
    y_n: 100,
    z_n: 109,
};

const DELTA: i32 = (6 << 12) / 29;

fn f_inv(t: i32) -> i32 {
    let ans = if t > DELTA {
        (((t * t) >> 12) * t) >> 12
    } else {
        ((3 * (DELTA >> 6) * (DELTA >> 6)) >> 6) * ((t >> 6) - (4 << 6) / 29)
    };
    //eprintln!("f_inv({}) = {}", t, ans);
    ans
}

fn lab_to_xyz(lab: ColorLab, illuminant: Illuminant) -> ColorXyz {
    let lum = illuminant;
    let ColorLab { l, a, b } = lab;
    let l_adj: i32 = (((l as i32) + 16) << 12) / 116;
    let a_adj: i32 = ((a as i32) << 12) / 500;
    let b_adj: i32 = -((b as i32) << 12) / 200;
    let ans = ColorXyz {
        x: lum.x_n * f_inv(l_adj + a_adj),
        y: lum.y_n * f_inv(l_adj),
        z: lum.z_n * f_inv(l_adj + b_adj),
    };
    //eprintln!("x:{} y:{}, z:{}", ans.x, ans.y, ans.z);
    ans
}

fn xyz_to_rgb(xyz: ColorXyz) -> ColorRgb {
    let ColorXyz {
        mut x,
        mut y,
        mut z,
    } = xyz;
    x >>= 8;
    y >>= 8;
    z >>= 8;
    let ans = ColorRgb {
        r: ((13273 * x - 6296 * y - 2042 * z) >> 15) as u8,
        g: ((-3970 * x + 7684 * y + 170 * z) >> 15) as u8,
        b: ((228 * x - 836 * y + 4330 * z) >> 15) as u8,
    };
    /*
        let ans = ColorRgb {
            r: ((8361 * x - 2314 * y - 1412 * z) >> 16) as u8,
            g: ((-3970 * x + 7684 * y + 170 * z) >> 16) as u8,
            b: ((55 * x - 485 * y + 4159 * z) >> 16) as u8,
        };
    */
    //eprintln!("r:{} g:{} b:{}", ans.r, ans.g, ans.b);
    ans
}
/*    ColorRgb {
    r: ((1714 * x - 650 * y - 339 * z) >> 16) as u8,
    g: ((-373 * x + 1034 * y + 64 * z) >> 16) as u8,
    b: ((4 * x - 10 * y + 731 * z) >> 16) as u8,
}*/

fn lab_to_rgb(lab: ColorLab, illuminant: Illuminant) -> ColorRgb {
    xyz_to_rgb(lab_to_xyz(lab, illuminant))
}

impl From<ColorLab> for ColorRgb {
    fn from(lab: ColorLab) -> ColorRgb {
        lab_to_rgb(lab, D65_ILLUMINANT)
    }
}
