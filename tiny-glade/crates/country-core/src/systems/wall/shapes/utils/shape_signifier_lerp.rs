//! B `systems/wall/shapes/utils/shape_signifier_lerp.rs`: shape signifier
//! interpolation.
//!
//! Evidence: `chapters/systems/wall/shapes/utils/shape_signifier_lerp.rs.md`
//! (O, 0x1409ef010). During shape drag the preview radius (circle) or
//! dimensions (rectangle) exponentially chase the target; cross-shape
//! interpolation panics (shape_signifier_lerp.rs:40).

use glam::Vec2;

/// Smooth-step chase (B func_0x142923ec0; CubicInOut family).
fn ease(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        let x = t + t;
        x * x * x * 0.5
    } else {
        let x = t + t - 2.0;
        x * x * x * 0.5 + 1.0
    }
}

/// B ShapeVal: 12 B record `{tag u32@+0, val f32@+4, val2 f32@+8}`.
/// Circle uses val as radius; rectangle uses val/val2 as dimensions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShapeVal {
    Circle(f32),
    Rectangle(Vec2),
}

impl ShapeVal {
    /// Inline B line 80: circle payload radius (+0xc of the record).
    pub fn get_radius(&self) -> f32 {
        match self {
            Self::Circle(r) => *r,
            Self::Rectangle(_) => panic!("get_radius called on a rectangle ShapeVal"),
        }
    }

    /// Inline B line 87: rectangle payload half-dimensions (+0x14/+0x18).
    pub fn get_rectangle_dims(&self) -> Vec2 {
        match self {
            Self::Circle(_) => panic!("get_rectangle_dims called on a circle ShapeVal"),
            Self::Rectangle(d) => *d,
        }
    }

    pub fn tag(&self) -> u32 {
        match self {
            Self::Circle(_) => 0,
            Self::Rectangle(_) => 1,
        }
    }
}

/// B 0x1409ef010 `ShapeSignifierLerp::lerp_radius` = `ShapeVal::interpolate`.
/// The chase rate (0x14295c698) and the exact min-selection rule are 待取证;
/// the observed structure (ease chase + clamped increment, cross-shape panic)
/// is preserved.
pub fn lerp_shape(prev: ShapeVal, cur: ShapeVal, t: f32) -> ShapeVal {
    const RATE: f32 = 10.0;
    match prev {
        ShapeVal::Circle(prev_r) => match cur {
            ShapeVal::Circle(cur_r) => {
                let eased = ease(RATE * t);
                let v1 = (1.0 - eased) * (cur_r - prev_r);
                let v2 = (t * 2.0).min((cur_r - prev_r).abs());
                let increment = if v1.abs() < v2.abs() { v1 } else { v2 };
                ShapeVal::Circle(prev_r + increment)
            }
            ShapeVal::Rectangle(_) => {
                panic!("interpolated a circle shape value toward a rectangle")
            }
        },
        ShapeVal::Rectangle(prev_d) => match cur {
            ShapeVal::Rectangle(cur_d) => {
                let lerp_axis = |a: f32, b: f32| {
                    let eased = ease(RATE * t);
                    let v1 = (1.0 - eased) * (b - a);
                    let v2 = (t * 2.0).min((b - a).abs());
                    let increment = if v1.abs() < v2.abs() { v1 } else { v2 };
                    a + increment
                };
                ShapeVal::Rectangle(Vec2::new(
                    lerp_axis(prev_d.x, cur_d.x),
                    lerp_axis(prev_d.y, cur_d.y),
                ))
            }
            ShapeVal::Circle(_) => {
                panic!("interpolated a rectangle shape value toward a circle")
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_chase_stays_within_target_bounds() {
        // The chase is asymptotic; it never overshoots the target and keeps
        // growing toward it.
        let start = lerp_shape(ShapeVal::Circle(1.0), ShapeVal::Circle(5.0), 0.0);
        let midway = lerp_shape(ShapeVal::Circle(1.0), ShapeVal::Circle(5.0), 0.5);
        let end = lerp_shape(ShapeVal::Circle(1.0), ShapeVal::Circle(5.0), 1.0);
        let value = |s: ShapeVal| match s {
            ShapeVal::Circle(r) => r,
            _ => panic!(),
        };
        let (a, b, c) = (value(start), value(midway), value(end));
        assert!(a >= 1.0 && a <= 5.0);
        assert!(b >= a && b <= 5.0);
        assert!(c >= b && c <= 5.0);
    }

    #[test]
    #[should_panic]
    fn cross_shape_interpolation_panics() {
        let _ = lerp_shape(ShapeVal::Circle(1.0), ShapeVal::Rectangle(Vec2::new(2.0, 3.0)), 0.5);
    }
}
