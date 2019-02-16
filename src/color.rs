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

// delta, scaled by 12 bits
const DELTA: i32 = (6 << 12) / 29;

// The inverse of function f, defined here:
//    https://en.wikipedia.org/wiki/CIELAB_color_space#CIELAB
// Both input and output scaled by 12 bits.
fn f_inv(t: i32) -> i32 {
    let ans = if t > DELTA {
        (((t * t) >> 12) * t) >> 12
    } else {
        ((3 * (DELTA >> 6) * (DELTA >> 6)) >> 6) * ((t >> 6) - (4 << 6) / 29)
    };
    //eprintln!("f_inv({}) = {}", t, ans);
    ans
}

// Convert from CIE-lab to CIE-XYZ, defined here:
//    https://en.wikipedia.org/wiki/CIELAB_color_space#CIELAB
// Output scaled by 12 bits.
fn lab_to_xyz(lab: ColorLab) -> ColorXyz {
    let ColorLab { l, a, b } = lab;
    let l_adj: i32 = (((l as i32) + 16) << 12) / 116;
    let a_adj: i32 = ((a as i32) << 12) / 500;
    let b_adj: i32 = -((b as i32) << 12) / 200;
    // Using the D65 Illuminant
    let ans = ColorXyz {
        x: 95 * f_inv(l_adj + a_adj),
        y: 100 * f_inv(l_adj),
        z: 109 * f_inv(l_adj + b_adj),
    };
    //eprintln!("x:{} y:{}, z:{}", ans.x, ans.y, ans.z);
    ans
}

// Convert from CIE-XYZ to sRGB, defined here:
//    https://en.wikipedia.org/wiki/SRGB
// Input and output scaled by 12 bits.
fn xyz_to_srgb(xyz: ColorXyz) -> ColorRgb {
    let ColorXyz {
        mut x,
        mut y,
        mut z,
    } = xyz;
    x >>= 8;
    y >>= 8;
    z >>= 8;
    let ans = ColorRgb {
        r: gamma((13273 * x - 6296 * y - 2042 * z) >> 4),
        g: gamma((-3969 * x + 7683 * y + 170 * z) >> 4),
        b: gamma((228 * x - 836 * y + 4329 * z) >> 4)
    };
    ans
}

fn gamma(u: i32) -> u8 {
    match GAMMA_INV.binary_search(&u) {
        Ok(i) => i as u8,
        Err(i) => (i - 1) as u8
    }
}

const GAMMA_INV: [i32; 256] =
    [0, 123, 247, 371, 495, 619, 743, 866, 
     990, 1114, 1238, 1365, 1499, 1641, 1790, 1947, 
     2111, 2284, 2464, 2652, 2849, 3054, 3267, 3489, 
     3719, 3957, 4205, 4461, 4726, 5000, 5283, 5576, 
     5877, 6188, 6508, 6838, 7177, 7526, 7884, 8252, 
     8631, 9019, 9416, 9825, 10243, 10671, 11110, 11559,
     12018, 12488, 12969, 13460, 13961, 14474, 14997, 15531,
     16076, 16632, 17200, 17778, 18367, 18968, 19580, 20203,
     20838, 21484, 22142, 22812, 23493, 24186, 24890, 25607,
     26335, 27075, 27827, 28592, 29368, 30157, 30957, 31770,
     32596, 33434, 34284, 35146, 36021, 36909, 37810, 38723,
     39648, 40587, 41539, 42503, 43480, 44470, 45474, 46490,
     47519, 48562, 49618, 50687, 51769, 52865, 53974, 55097,
     56233, 57383, 58546, 59723, 60914, 62119, 63337, 64569,
     65815, 67074, 68348, 69636, 70938, 72254, 73584, 74928,
     76286, 77659, 79046, 80447, 81863, 83293, 84737, 86196,
     87670, 89158, 90661, 92179, 93711, 95258, 96820, 98396,
     99988, 101594, 103216, 104852, 106504, 108170, 109852, 111548,
     113260, 114988, 116730, 118488, 120261, 122049, 123853, 125672,
     127507, 129358, 131224, 133105, 135002, 136915, 138844, 140788,
     142748, 144724, 146716, 148724, 150748, 152787, 154843, 156915,
     159002, 161106, 163226, 165362, 167515, 169684, 171869, 174070,
     176288, 178522, 180772, 183039, 185322, 187622, 189939, 192272,
     194622, 196988, 199372, 201771, 204188, 206622, 209072, 211539,
     214023, 216524, 219042, 221577, 224129, 226698, 229284, 231887,
     234508, 237145, 239800, 242472, 245162, 247868, 250592, 253334,
     256093, 258869, 261663, 264474, 267303, 270149, 273013, 275895,
     278794, 281711, 284646, 287599, 290569, 293557, 296563, 299587,
     302628, 305688, 308766, 311861, 314975, 318107, 321257, 324425,
     327611, 330815, 334037, 337278, 340537, 343814, 347110, 350424,
     353756, 357107, 360476, 363864, 367270, 370695, 374138, 377600,
     381080, 384579, 388097, 391633, 395189, 398763, 402355, 405967];

fn lab_to_srgb(lab: ColorLab) -> ColorRgb {
    xyz_to_srgb(lab_to_xyz(lab))
}

impl From<ColorLab> for ColorRgb {
    fn from(lab: ColorLab) -> ColorRgb {
        lab_to_srgb(lab)
    }
}
