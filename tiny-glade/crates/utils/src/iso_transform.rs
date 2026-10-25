//! Rigid translation and rotation composition.
use glam::{Quat, Vec3};

/// A rigid transform consisting of translation and a unit quaternion.
///
/// `rotation` must remain normalized when set through the public field.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct IsoTransform {
    /// Position in the parent coordinate system.
    pub translation: Vec3,
    /// Orientation relative to the parent coordinate system.
    pub rotation: Quat,
    // Retained for layout compatibility with existing reconstructed callers.
    _reserved: [u8; 0x10],
}

impl Default for IsoTransform {
    fn default() -> Self {
        Self::identity()
    }
}

impl IsoTransform {
    /// Returns the identity transform.
    pub fn identity() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            _reserved: [0; 0x10],
        }
    }

    /// Constructs a translation with no rotation.
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            rotation: Quat::IDENTITY,
            _reserved: [0; 0x10],
        }
    }

    /// Constructs a rigid transform with a normalized rotation.
    pub fn from_translation_rotation(translation: Vec3, rotation: Quat) -> Self {
        Self {
            translation,
            rotation,
            _reserved: [0; 0x10],
        }
    }

    /// Composes this local transform with its parent (`parent * self`).
    pub fn nested_in(&self, parent: &IsoTransform) -> IsoTransform {
        Self::from_translation_rotation(
            parent.translation + parent.rotation * self.translation,
            parent.rotation * self.rotation,
        )
    }
}
