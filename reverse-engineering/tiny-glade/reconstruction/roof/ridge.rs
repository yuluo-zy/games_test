//! Recovered original RoofRidgeDims numeric algorithms. The byte interface keeps
//! the observed ABI layout distinct from a claim about the original Rust type.
#[cfg(not(feature = "library"))]
use std::io::{self, BufRead};

pub const RIDGE_MIN: f32 = f32::from_bits(0x3dcccccd);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RidgeDims(pub [f32; 2]);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RidgeGeometry {
    Point([f32; 3]),
    Segment([[f32; 3]; 2]),
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RidgeError {
    ShapeMustBeRectangular,
    InvalidHeight,
    DegenerateInverse,
    InvalidInverse,
}

fn at(b: &[u8; 0x58], o: usize) -> f32 {
    f32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}
fn tag(b: &[u8; 0x58]) -> u32 {
    u32::from_le_bytes(b[8..12].try_into().unwrap())
}
fn max_num(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        b
    } else if b.is_nan() || b <= a {
        a
    } else {
        b
    }
}
// SSE MINSS/MAXSS select the second operand on an unordered comparison.
fn maxss(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}
fn minss(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}
fn neg_bits(a: f32) -> f32 {
    f32::from_bits(a.to_bits() ^ 0x8000_0000)
}
fn rotation(b: &[u8; 0x58], p: [f32; 2]) -> [f32; 2] {
    let c = at(b, 0xc);
    let s = at(b, 0x10);
    (glam::Mat2::from_cols(glam::Vec2::new(c, s), glam::Vec2::new(neg_bits(s), c))
        * glam::Vec2::from(p))
    .into()
}

pub fn ridge_dims_calc(ridge_length_01: f32, ridge_dir: u8, shape_dims: [f32; 2]) -> RidgeDims {
    let small = (1.0 - ridge_length_01) * RIDGE_MIN;
    if ridge_dir != 0 {
        RidgeDims([RIDGE_MIN, small + ridge_length_01 * shape_dims[1]])
    } else {
        RidgeDims([small + ridge_length_01 * shape_dims[0], RIDGE_MIN])
    }
}
pub fn ridge_dims_from_observed(roof: &[u8; 0x58]) -> RidgeDims {
    if tag(roof) != 1 {
        RidgeDims([RIDGE_MIN; 2])
    } else {
        ridge_dims_calc(at(roof, 0x34), roof[0x3c], [at(roof, 0x1c), at(roof, 0x20)])
    }
}

pub fn calc_roof_tip_center_ls_observed(roof: &[u8; 0x58], scale: [f32; 2]) -> [f32; 2] {
    let dims = if roof[8] == 0 {
        [at(roof, 0x14) + at(roof, 0x14); 2]
    } else {
        [at(roof, 0x1c), at(roof, 0x20)]
    };
    [
        dims[0] * at(roof, 0x24) * scale[0],
        dims[1] * at(roof, 0x28) * scale[1],
    ]
}
pub fn roof_tip_offset_xz_observed(roof: &[u8; 0x58]) -> [f32; 2] {
    if roof[8] & 1 == 0 {
        calc_roof_tip_center_ls_observed(roof, [0.5, 0.5])
    } else {
        let p = [
            at(roof, 0x24) * at(roof, 0x1c) * 0.5,
            at(roof, 0x28) * at(roof, 0x20) * 0.5,
        ];
        rotation(roof, p)
    }
}

pub fn inverse_ridge_01_param(
    ridge_length: f32,
    ridge_dir: u8,
    shape_dims: [f32; 2],
) -> Result<f32, RidgeError> {
    let d = shape_dims[usize::from(ridge_dir != 0)];
    if d == RIDGE_MIN {
        return Err(RidgeError::DegenerateInverse);
    }
    let ratio = (ridge_length + (-RIDGE_MIN)) / (d + (-RIDGE_MIN));
    // Original rejects NaN; +/- infinity is clamped into [0,1].
    if ratio.is_nan() {
        return Err(RidgeError::InvalidInverse);
    }
    Ok(minss(1.0, maxss(0.0, ratio)))
}

fn roof_height(roof: &[u8; 0x58]) -> Result<f32, RidgeError> {
    let h = at(roof, 0x30);
    if !(RIDGE_MIN < h) && !h.is_nan() {
        return Ok(0.0);
    }
    let (u, low) = if roof[8] & 1 == 0 {
        (
            (at(roof, 0x14) + f32::from_bits(0xbf666667)) / f32::from_bits(0x40e33333),
            8.0,
        )
    } else {
        (
            (max_num(at(roof, 0x1c), at(roof, 0x20)) + (-2.25)) / 12.75,
            9.0,
        )
    };
    let t = minss(1.0, maxss(0.0, u / 0.4));
    let squared = t * t;
    let smooth = (3.0 - (t + t)) * squared;
    let limit = smooth * 12.0 + (1.0 - smooth) * low;
    if limit < 0.0 || limit.is_nan() {
        return Err(RidgeError::InvalidHeight);
    }
    let requested = 12.0 * h + (1.0 - h) * 0.4;
    Ok(minss(limit, maxss(0.0, requested)))
}

/// Complete point/segment branch of original RoofRidgeDims::to_world_space.
/// Valid shape discriminants are 0 (circular) and 1 (rectangular).
pub fn ridge_to_world_space_observed(
    dims: RidgeDims,
    roof: &[u8; 0x58],
) -> Result<RidgeGeometry, RidgeError> {
    let maximum = max_num(dims.0[0], dims.0[1]);
    let height = roof_height(roof)?;
    if maximum <= RIDGE_MIN {
        let p = roof_tip_offset_xz_observed(roof);
        let center = if tag(roof) == 0 {
            [at(roof, 0xc), at(roof, 0x10)]
        } else {
            [at(roof, 0x14), at(roof, 0x18)]
        };
        Ok(RidgeGeometry::Point([
            0.0 * height + (center[0] + p[0]),
            height + (at(roof, 0x4c) + 0.0),
            (center[1] + p[1]) + 0.0 * height,
        ]))
    } else {
        if roof[8] & 1 == 0 {
            return Err(RidgeError::ShapeMustBeRectangular);
        }
        // The original multiplies the normalized tip offset by zero in this
        // branch. Preserve these operations, including signed zero/NaN effects.
        let zero_x = (at(roof, 0x24) * at(roof, 0x1c)) * 0.0;
        let zero_z = (at(roof, 0x28) * at(roof, 0x20)) * 0.0;
        let (a, b) = if roof[0x3c] == 1 {
            let half = dims.0[1] * 0.5;
            (
                rotation(roof, [zero_x + 0.0, zero_z - half]),
                rotation(roof, [zero_x + 0.0, half + zero_z]),
            )
        } else {
            let half = dims.0[0] * 0.5;
            (
                rotation(roof, [zero_x - half, zero_z + 0.0]),
                rotation(roof, [half + zero_x, zero_z + 0.0]),
            )
        };
        let first = [
            0.0 * height + (a[0] + at(roof, 0x14)),
            height + (0.0 + at(roof, 0x4c)),
            (a[1] + at(roof, 0x18)) + 0.0 * height,
        ];
        let second = [
            0.0 * height + (at(roof, 0x14) + b[0]),
            height + (at(roof, 0x4c) + 0.0),
            (b[1] + at(roof, 0x18)) + 0.0 * height,
        ];
        Ok(RidgeGeometry::Segment([first, second]))
    }
}

#[cfg(not(feature = "library"))]
fn decode(s: &str) -> [u8; 0x58] {
    std::array::from_fn(|i| u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap())
}
#[cfg(not(feature = "library"))]
fn ff(s: &str) -> f32 {
    f32::from_bits(u32::from_str_radix(s, 16).unwrap())
}
#[cfg(not(feature = "library"))]
fn bits(v: &[f32]) -> String {
    v.iter()
        .map(|x| format!("{:08x}", x.to_bits()))
        .collect::<Vec<_>>()
        .join(" ")
}
#[cfg(not(feature = "library"))]
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let p: Vec<_> = line.split_whitespace().collect();
        match p[0] {
            "calc" => println!(
                "{}",
                bits(&ridge_dims_calc(ff(p[1]), p[2].parse().unwrap(), [ff(p[3]), ff(p[4])]).0)
            ),
            "from" => println!("{}", bits(&ridge_dims_from_observed(&decode(p[1])).0)),
            "tip" => println!("{}", bits(&roof_tip_offset_xz_observed(&decode(p[1])))),
            "center" => println!(
                "{}",
                bits(&calc_roof_tip_center_ls_observed(
                    &decode(p[1]),
                    [ff(p[2]), ff(p[3])]
                ))
            ),
            "inverse" => {
                match inverse_ridge_01_param(ff(p[1]), p[2].parse().unwrap(), [ff(p[3]), ff(p[4])])
                {
                    Ok(x) => println!("{}", bits(&[x])),
                    Err(_) => println!("panic"),
                }
            }
            "world" => {
                match ridge_to_world_space_observed(RidgeDims([ff(p[2]), ff(p[3])]), &decode(p[1]))
                {
                    Ok(RidgeGeometry::Point(a)) => println!("0 {}", bits(&a)),
                    Ok(RidgeGeometry::Segment(a)) => println!("1 {} {}", bits(&a[0]), bits(&a[1])),
                    Err(_) => println!("panic"),
                }
            }
            _ => panic!("unknown mode"),
        }
    }
}
