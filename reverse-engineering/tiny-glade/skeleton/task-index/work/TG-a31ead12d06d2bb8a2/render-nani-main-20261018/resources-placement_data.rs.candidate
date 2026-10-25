//! Placement records shared by tree instance uploads and placement assets.
//! B `TypedStorageBuffer<PlacementDataSsbo>@1416164c0` uses a 48-byte
//! stride, matching the original packaged canopy shader's member offsets.
use glam::{Affine3A, Quat, Vec2, Vec3};

use crate::{render::ssbo::StorageElement, utils::components::transform::Transform};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, bytemuck::NoUninit)]
pub struct PlacementDataSsbo {
    pub position: Vec3,
    pub scale: f32,
    // Native serde accepts both spellings; the missing-field name is
    // rotation_degrees. Do not convert the stored payload during loading.
    #[serde(rename = "rotation_degrees", alias = "rotation_angle")]
    pub rotation_angle: f32,
    pub id: i32,
    #[serde(default)]
    pub custom0: u32,
    #[serde(default)]
    pub custom1: u32,
    #[serde(skip, default = "default_anim_t")]
    pub anim_t_packed: u32,
    #[serde(default)]
    pub bend_vector: Vec2,
    #[serde(default)]
    pub pad0: i32,
}

/// Deserialization initializes both half-float animation channels to one:
/// original 14143021c loads 1.0 in xmm0/xmm1 before pack_2x16f_uint.
pub fn default_anim_t() -> u32 {
    crate::render::nani::instance_updates::pack_2x16f_uint(1.0, 1.0)
}

impl Default for PlacementDataSsbo {
    /// Unallocated owned slots use the native hidden sentinel, not the
    /// deserialized animation defaults. Original constructor 141567533
    /// repeats {0,-100,0,0,...,0} at a 48-byte stride.
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, -100.0, 0.0),
            scale: 0.0,
            rotation_angle: 0.0,
            id: 0,
            custom0: 0,
            custom1: 0,
            anim_t_packed: 0,
            bend_vector: Vec2::ZERO,
            pad0: 0,
        }
    }
}

impl StorageElement for PlacementDataSsbo {
    const NAME: &'static str = "PlacementDataSsbo";
}

impl PlacementDataSsbo {
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Preserve native CPU behavior: to_affine3a@140ad5060 multiplies the
    /// stored angle only by 0.5 before sin/cos. The original GPU shader
    /// separately applies Radians; that difference is not silently corrected.
    pub fn to_affine3a(&self) -> Affine3A {
        Affine3A::from_scale_rotation_translation(
            Vec3::splat(self.scale),
            Quat::from_rotation_y(self.rotation_angle),
            self.position,
        )
    }

    pub fn to_transform(&self) -> Transform {
        Transform {
            translation: self.position,
            rotation: Quat::from_rotation_y(self.rotation_angle),
            scale: Vec3::splat(self.scale),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn original_placement_payload_and_asset_defaults() {
        assert_eq!(size_of::<PlacementDataSsbo>(), 48);
        assert_eq!(offset_of!(PlacementDataSsbo, scale), 12);
        assert_eq!(offset_of!(PlacementDataSsbo, rotation_angle), 16);
        assert_eq!(offset_of!(PlacementDataSsbo, id), 20);
        assert_eq!(offset_of!(PlacementDataSsbo, anim_t_packed), 32);
        assert_eq!(offset_of!(PlacementDataSsbo, bend_vector), 36);
        assert_eq!(offset_of!(PlacementDataSsbo, pad0), 44);
        let value: PlacementDataSsbo = serde_json::from_str(
            r#"{"position":[1,2,3],"scale":2,"rotation_angle":90,"id":-3,"pad1":7}"#,
        )
        .unwrap();
        assert_eq!(value.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(value.rotation_angle, 90.0);
        assert_eq!(value.id, -3);
        assert_eq!(value.anim_t_packed, 0x3c00_3c00);
        assert_eq!(value.bend_vector, Vec2::ZERO);
        assert_eq!((value.custom0, value.custom1, value.pad0), (0, 0, 0));
        let missing: Result<PlacementDataSsbo, _> =
            serde_json::from_str(r#"{"position":[1,2,3],"scale":2,"id":3}"#);
        assert!(
            missing
                .unwrap_err()
                .to_string()
                .contains("rotation_degrees")
        );
        assert_eq!(PlacementDataSsbo::default().position.y, -100.0);
        assert_eq!(PlacementDataSsbo::default().anim_t_packed, 0);
    }
}
