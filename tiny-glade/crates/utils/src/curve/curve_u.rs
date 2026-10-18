use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};
use std::marker::PhantomData;
/// Finite normalized coordinate, including both endpoints.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CurveU<S = super::CurveCoordSemantic2D> {
    value: f32,
    semantic: PhantomData<S>,
}
/// Finite curve coordinate before clamping to the normalized range.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CurveUBoundless<S = super::CurveCoordSemantic2D> {
    value: f32,
    semantic: PhantomData<S>,
}
impl<S> CurveUBoundless<S> {
    pub fn new(value: f32) -> Self {
        assert!(value.is_finite(), "nonfinite curve coordinate");
        Self {
            value,
            semantic: PhantomData,
        }
    }
    pub fn get(self) -> f32 {
        self.value
    }
    pub fn clamp(self) -> CurveU<S> {
        let value = self.value.clamp(0.0, 1.0);
        CurveU::new(value)
    }
}
impl<S> Serialize for CurveU<S> {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        serializer.serialize_f32(self.value)
    }
}
impl<'de, S> Deserialize<'de> for CurveU<S> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct CoordinateVisitor<S>(PhantomData<S>);
        impl<'de, S> Visitor<'de> for CoordinateVisitor<S> {
            type Value = CurveU<S>;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a finite curve coordinate")
            }
            fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Self::Value, E> {
                if !value.is_finite() {
                    return Err(E::custom("curve coordinate must be finite"));
                }
                // Clamp before narrowing so every finite f64 remains valid.
                Ok(CurveU::new(value.clamp(0.0, 1.0) as f32))
            }
        }
        deserializer.deserialize_f64(CoordinateVisitor(PhantomData))
    }
}
impl<S> CurveU<S> {
    pub fn new(value: f32) -> Self {
        assert!((0.0..=1.0).contains(&value), "CurveU outside [0,1]");
        Self {
            value,
            semantic: PhantomData,
        }
    }
    pub fn get(self) -> f32 {
        self.value
    }
}
