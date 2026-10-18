//! Scalar, array, and glam conversions to IEEE binary16.
use glam::{Quat, Vec2, Vec3, Vec4};
use half::f16;
pub trait Halver {
    type Half;
    fn to_f16(self) -> Self::Half;
}
impl Halver for f32 {
    type Half = f16;
    fn to_f16(self) -> f16 {
        f16::from_f32(self)
    }
}
impl<const N: usize> Halver for [f32; N] {
    type Half = [f16; N];
    fn to_f16(self) -> Self::Half {
        self.map(f16::from_f32)
    }
}
impl Halver for Vec2 {
    type Half = [f16; 2];
    fn to_f16(self) -> Self::Half {
        self.to_array().to_f16()
    }
}
impl Halver for Vec3 {
    type Half = [f16; 3];
    fn to_f16(self) -> Self::Half {
        self.to_array().to_f16()
    }
}
impl Halver for Vec4 {
    type Half = [f16; 4];
    fn to_f16(self) -> Self::Half {
        self.to_array().to_f16()
    }
}
impl Halver for Quat {
    type Half = [f16; 4];
    fn to_f16(self) -> Self::Half {
        self.to_array().to_f16()
    }
}
