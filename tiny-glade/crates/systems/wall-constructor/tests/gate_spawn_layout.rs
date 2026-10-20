//! WC-C layout contracts for the gate spawn record and the gate command.
//!
//! Native layouts (`evidence/building-gates`, `wc-constructor-CONTRACT.md` §1.2):
//! * 120B `ConstructGateCmd` = `MaterializedWallSegment` 104B + `wall_id` 8B +
//!   `ordinal` 8B (the Bevy event adds a separate 8B EventId).
//! * 136B door spawn payload: TransientMesh 0..16, Transform 16..56, mesh@64,
//!   shader@72, top-position Vec 80..104, `WallId`@104, endpoint directions
//!   112..128, seed@128, Public29a@132, padded to 136.
//!
//! The producers are `construct_gates::systems::construct_gates` and the arch
//! 120B command; this test pins the byte layout with the *real* constituent
//! types so a field reorder/type change fails immediately.
use country_core::asset_libraries::transient_mesh::TransientMesh;
use country_core::asset_libraries::Handle;
use country_core::render::mesh::Mesh;
use country_core::resources::walls::publication_fields::WallId as PublicWallId;
use country_core::resources::walls::WallId;
use country_core::systems::wall::construction::arches::ConstructGateCmd120Projection;
use country_core::systems::wall::construction::structures::MaterializedWallSegment;
use country_core::utils::components::transform::Transform;
use system_wall_constructor::construct_gates::systems::{ConstructGateCmd, WoodenGateState};

#[repr(C)]
struct GateCmd120 {
    segment: MaterializedWallSegment,
    wall: u64,
    ordinal: u64,
}

/// Mirror of the native 136B spawn payload using the real component types.
/// `_gap56` covers the 8B hole before the mesh handle; `_tail` pads 133..136.
#[repr(C)]
struct DoorSpawn136 {
    transient: TransientMesh,
    transform: Transform,
    _gap56: [u8; 8],
    mesh: Handle<Mesh>,
    shader: u64,
    top_positions: Vec<[f32; 3]>,
    wall: WallId,
    endpoint_directions: [[f32; 2]; 2],
    seed: u32,
    clutter_version: u8,
    _tail: [u8; 3],
}

#[test]
fn gate_command_120b_is_104b_segment_plus_wall_and_ordinal() {
    assert_eq!(size_of::<MaterializedWallSegment>(), 104);
    assert_eq!(size_of::<GateCmd120>(), 120);
    assert_eq!(offset_of!(GateCmd120, segment), 0);
    assert_eq!(offset_of!(GateCmd120, wall), 104);
    assert_eq!(offset_of!(GateCmd120, ordinal), 112);

    // The owned Event form carries the same three logical fields.
    let projection = ConstructGateCmd120Projection {
        segment: MaterializedWallSegment {
            heights: vec![1.0],
            positions: vec![[0.0, 0.0, 0.0]],
            points_u: vec![0.0],
            source_indices: [0, 1],
            range: country_core::systems::wall::construction::segmentation::WallLoopRange::for_open(
                0, 1, 1,
            )
            .unwrap(),
        },
        wall_id: PublicWallId(9),
        ordinal: 4,
    };
    let event = ConstructGateCmd {
        wall: projection.wall_id.0,
        ordinal: projection.ordinal,
        segment: projection.segment.clone(),
    };
    assert_eq!(event.wall, 9);
    assert_eq!(event.ordinal, 4);
    assert_eq!(event.segment.source_indices, [0, 1]);
    assert_eq!(size_of::<ConstructGateCmd>(), 120);
}

#[test]
fn door_spawn_136b_field_offsets_match_native_contract() {
    // Constituent type sizes are the native layout's assumptions.
    assert_eq!(size_of::<TransientMesh>(), 16);
    assert_eq!(size_of::<Transform>(), 40);
    assert_eq!(size_of::<Handle<Mesh>>(), 8);
    assert_eq!(size_of::<Vec<[f32; 3]>>(), 24);
    assert_eq!(size_of::<WallId>(), 8);
    assert_eq!(size_of::<[[f32; 2]; 2]>(), 16);

    assert_eq!(size_of::<DoorSpawn136>(), 136);
    assert_eq!(offset_of!(DoorSpawn136, transient), 0);
    assert_eq!(offset_of!(DoorSpawn136, transform), 16);
    assert_eq!(offset_of!(DoorSpawn136, mesh), 64);
    assert_eq!(offset_of!(DoorSpawn136, shader), 72);
    assert_eq!(offset_of!(DoorSpawn136, top_positions), 80);
    assert_eq!(offset_of!(DoorSpawn136, wall), 104);
    assert_eq!(offset_of!(DoorSpawn136, endpoint_directions), 112);
    assert_eq!(offset_of!(DoorSpawn136, seed), 128);
    assert_eq!(offset_of!(DoorSpawn136, clutter_version), 132);

    // `WoodenGateState` is the owned equivalent of the same payload fields.
    let state = WoodenGateState {
        top_positions: vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]],
        wall: WallId(9),
        endpoint_directions: [[0.0, 1.0], [0.0, -1.0]],
        seed: 0xdead_beef,
        clutter_version: 0x29,
    };
    assert_eq!(state.top_positions.len(), 2);
    assert_eq!(state.wall.0, 9);
    assert_eq!(state.seed.to_le_bytes(), [0xef, 0xbe, 0xad, 0xde]);
    assert_eq!(state.clutter_version, 0x29);
    // Seed/29a occupy the same 4B + 1B tail slots at 128/132 in the payload.
    assert_eq!(offset_of!(DoorSpawn136, seed) + 4, offset_of!(DoorSpawn136, clutter_version));
}
