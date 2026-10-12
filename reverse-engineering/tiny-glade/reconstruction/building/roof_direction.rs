//! Original ridge-direction event consumer after wall/entity/query resolution.
//! The Bevy storage and writer buffers are dependencies; notification policy is game logic.
use crate::building_wall_state::WallId;
#[derive(Clone, Copy, Debug)]
pub struct RoofDirectionRequest {
    pub wall: WallId,
    pub direction: u8,
}
#[derive(Clone, Copy, Debug)]
pub struct RoofDirectionChange {
    pub entity: u64,
    pub wall: WallId,
    pub geometry_kind: u8,
    pub sound_kind: u8,
    pub sound_pivot: [f32; 3],
}
/// The original marks Changed and emits geometry, gable and sound even when
/// the selected direction equals the current byte. Accepted roof shapes are 0/1.
pub fn apply_resolved_direction(
    roof: &mut [u8; 88],
    request: RoofDirectionRequest,
    entity: u64,
    changed_tick: &mut u32,
    current_tick: u32,
) -> RoofDirectionChange {
    let shape = u32::from_le_bytes(roof[8..12].try_into().unwrap()) as usize;
    assert!(shape <= 1, "unrecovered Roof pivot shape");
    *changed_tick = current_tick;
    roof[0x3c] = request.direction;
    let f = |offset| f32::from_le_bytes(roof[offset..offset + 4].try_into().unwrap());
    let pivot = [f(12 + shape * 8), f(0x4c), f(16 + shape * 8)];
    let height = crate::surface::height(roof);
    RoofDirectionChange {
        entity,
        wall: request.wall,
        geometry_kind: 1,
        sound_kind: 7,
        sound_pivot: [
            pivot[0] + height * 0.0,
            pivot[1] + height,
            pivot[2] + height * 0.0,
        ],
    }
}
