//! Named views of the fixed roof record shared by runtime consumers.
//!
//! Storage stays byte-backed for the policy APIs. These accessors decode only
//! known fields; callers validate geometry before using shape-dependent fields.
use country_core::resources::walls::publication_fields::WallId;

#[derive(Clone, Copy)]
pub(crate) struct RoofRecord<'a>(pub(crate) &'a [u8; 88]);

impl<'a> RoofRecord<'a> {
    pub(crate) fn wall(self) -> WallId {
        WallId(u64::from_le_bytes(self.0[..8].try_into().unwrap()))
    }

    pub(crate) fn shape(self) -> u32 {
        u32::from_le_bytes(self.0[8..12].try_into().unwrap())
    }

    pub(crate) fn pitch(self) -> f32 {
        self.scalar(0x30)
    }

    pub(crate) fn ridge(self) -> f32 {
        self.scalar(0x34)
    }

    pub(crate) fn base_height(self) -> f32 {
        self.scalar(0x4c)
    }

    pub(crate) fn rectangle(self) -> &'a [u8; 24] {
        self.0[12..36].try_into().unwrap()
    }

    pub(crate) fn sound_pivot(self) -> Option<[f32; 3]> {
        let center_offset = match self.shape() {
            0 => 12,
            1 => 20,
            _ => return None,
        };
        let height = country_core::resources::roofs::geometry::surface::height(self.0);
        let pivot = [
            self.scalar(center_offset),
            self.base_height() + height,
            self.scalar(center_offset + 4),
        ];
        pivot.into_iter().all(f32::is_finite).then_some(pivot)
    }

    fn scalar(self, offset: usize) -> f32 {
        f32::from_le_bytes(self.0[offset..offset + 4].try_into().unwrap())
    }
}
