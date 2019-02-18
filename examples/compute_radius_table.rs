use dotstar::{sqrt, ColorLab};

/// Find the radius of the largest valid A,B circle centered on this color.
pub fn max_radius(color: ColorLab) -> i8 {
    let mut max_radius = 100;
    for a in -50..50 {
        for b in -50..50 {
            let color = ColorLab {
                l: color.l,
                a: color.a.saturating_add(a as i8),
                b: color.b.saturating_add(b as i8),
            };
            let radius = sqrt(a * a + b * b);
            if !color.is_valid() && radius < max_radius {
                max_radius = radius;
            }
        }
    }
    max_radius
}

fn main() {
    println!("[");
    for l in 1..13 {
        println!("[");
        for a in -11..11 {
            print!("[");
            for b in -11..11 {
                let color = ColorLab {
                    l: l << 3,
                    a: a << 3,
                    b: b << 3,
                };
                let radius = max_radius(color);
                print!("{},", radius);
            }
            println!("],");
        }
        println!("],");
    }
    println!("]");
}
