//! Game request matching and animation-completion removal policy; Commands reused.
use crate::resources::roofs::components::RoofAnimation;
use crate::resources::walls::publication_fields::WallId;

#[derive(Clone, Copy, Debug)]
pub struct RoofDeletionMark {
    pub entity: u64,
    pub wall_id: WallId,
    pub pivot: [f32; 3],
    pub sound_event_kind: u8,
}

/// Resolved query rows are visited once: duplicate requests do not create
/// duplicate insert commands or sound events for a single row.
pub fn match_deletion_request(
    requests: &[WallId],
    entity: u64,
    roof: &[u8; 88],
) -> Option<RoofDeletionMark> {
    let wall_id = WallId(u64::from_le_bytes(roof[..8].try_into().unwrap()));
    if !requests.contains(&wall_id) {
        return None;
    }
    let tag = u32::from_le_bytes(roof[8..12].try_into().unwrap()) as usize;
    assert!(tag <= 1, "unrecovered Roof shape for pivot");
    let f = |offset: usize| f32::from_le_bytes(roof[offset..offset + 4].try_into().unwrap());
    Some(RoofDeletionMark {
        entity,
        wall_id,
        pivot: [f(12 + tag * 8), f(0x4c), f(16 + tag * 8)],
        sound_event_kind: 1,
    })
}
/// Original delete_roofs requests despawn only for a deletion-marked entity
/// with an ordered zero in the animation's first channel. -0 counts as zero.
pub fn should_despawn(marked: bool, animation: &RoofAnimation) -> bool {
    marked && animation.deletion_finished()
}
