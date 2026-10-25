//! Game-owned wall height rules recovered from the supplied original RNE.
//! Map/query lookup is deliberately represented by resolved game state, rather
//! than by a local reimplementation of hashbrown/Bevy or their memory ABI.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallHeightBounds {
    pub minimum: f32,
    pub maximum: f32,
}

/// Numeric policy inside original clamp_wall_height after resolving the wall
/// and its optional Roof. The observed special flag's original field name is
/// unknown; it is not labelled as a gameplay feature without producer proof.
pub fn wall_height_bounds(roof_height_01: Option<f32>, special_flag: u8) -> WallHeightBounds {
    let minimum = match roof_height_01 {
        Some(h) if h <= f32::from_bits(0x3dcccccd) && !h.is_nan() => f32::from_bits(0x3fcccccd),
        Some(_) => f32::from_bits(0x3f333333),
        None if special_flag != 0 => f32::from_bits(0x3e99999a),
        None => f32::from_bits(0x3f000000),
    };
    WallHeightBounds {
        minimum,
        maximum: if special_flag != 0 { 2.5 } else { 14.0 },
    }
}
pub fn clamp_resolved_wall_height(
    desired: f32,
    roof_height_01: Option<f32>,
    special_flag: u8,
) -> f32 {
    let bounds = wall_height_bounds(roof_height_01, special_flag);
    // The actual MAXSS/MINSS source-operand behavior preserves unordered input.
    let low = if bounds.minimum > desired {
        bounds.minimum
    } else {
        desired
    };
    if bounds.maximum < low {
        bounds.maximum
    } else {
        low
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HeightCacheError {
    MissingMaximum,
    MissingGableMaximum,
}
pub fn cached_wall_max_y(present_flag: u8, max_y: f32) -> Result<f32, HeightCacheError> {
    if present_flag & 1 != 0 {
        Ok(max_y)
    } else {
        Err(HeightCacheError::MissingMaximum)
    }
}
pub fn cached_gable_max_y(present_flag: u8, max_y: f32) -> Result<f32, HeightCacheError> {
    if present_flag & 1 != 0 {
        Ok(max_y)
    } else {
        Err(HeightCacheError::MissingGableMaximum)
    }
}
/// Complete original flat_roof_y leaf: cached max_y minus exactly f32 0.79.
pub fn flat_roof_y(present_flag: u8, max_y: f32) -> Result<f32, HeightCacheError> {
    cached_wall_max_y(present_flag, max_y).map(|y| y + f32::from_bits(0xbf4a3d71))
}

#[cfg(not(feature = "library"))]
fn main() {
    use std::io::{self, BufRead};
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let p: Vec<_> = line.split_whitespace().collect();
        let f = |s: &str| f32::from_bits(u32::from_str_radix(s, 16).unwrap());
        match p[0] {
            "clamp" => {
                let roof = if p[2] == "none" { None } else { Some(f(p[2])) };
                let v = clamp_resolved_wall_height(f(p[1]), roof, p[3].parse().unwrap());
                println!("{:08x}", v.to_bits());
            }
            "max" | "gable" | "flat" => {
                let flag = p[1].parse().unwrap();
                let value = f(p[2]);
                let result = match p[0] {
                    "max" => cached_wall_max_y(flag, value),
                    "gable" => cached_gable_max_y(flag, value),
                    _ => flat_roof_y(flag, value),
                };
                match result {
                    Ok(v) => println!("{:08x}", v.to_bits()),
                    Err(_) => println!("panic"),
                }
            }
            _ => panic!("unknown mode"),
        }
    }
}
