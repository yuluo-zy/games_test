//! B init_game_state_from_snapshot command-producing ranges only.
//! Candidate for the canonical snapshot module, not a complete apply function.
//! Original sources: RCX LoadedSnapshotData; R8 is SystemParam targets, never bytes.
use crate::recovered_building::roof_creation::RoofCreateRequestObserved;
use crate::resources::{
    color_id::{SetGateColorIdCmd, SetHalftimberColorIdCmd, SetWallColorIdCmd},
    roofs::{commands::CreateRoofCmd, roof_shape::Roof},
};
use bevy_ecs::event::Events;

/// Original 0x1417a9c50 roof range iterates 0x58B owned Roof rows, emits the
/// actual CreateRoof command and leaves world creation to its real consumer.
/// The leading message-ID word in the current recovered record is ignored by
/// initialize_resolved_roof; Bevy assigns its own event ID in this host.
pub fn queue_snapshot_roofs(roofs: &[Roof], output: &mut Events<CreateRoofCmd>) {
    for roof in roofs {
        let source = roof.observed();
        if u32::from_le_bytes(source[8..12].try_into().unwrap()) == 2 {
            break;
        }
        let mut record = [0u8; 64];
        record[8..16].copy_from_slice(&source[..8]);
        record[16..20].copy_from_slice(&source[0x50..0x54]);
        record[20..32].copy_from_slice(&source[0x40..0x4c]);
        record[32..60].copy_from_slice(&source[0x24..0x40]);
        record[60] = 1;
        output.write(CreateRoofCmd(RoofCreateRequestObserved::from_observed(
            record,
        )));
    }
}

/// B's three loops copy a complete u64 index, color u32 and custom=false.
/// The old current command schema splits the u64 into `id` and `_gap4`; retain
/// every input bit until that canonical field is corrected, never fill it with
/// a guessed zero. The `_pad` bytes have no recovered game meaning.
pub fn queue_snapshot_colors(
    walls: &[(u64, u32)],
    gates: &[(u64, u32)],
    halftimber: &[(u64, u32)],
    wall_events: &mut Events<SetWallColorIdCmd>,
    gate_events: &mut Events<SetGateColorIdCmd>,
    halftimber_events: &mut Events<SetHalftimberColorIdCmd>,
) {
    for &(id, color) in walls {
        wall_events.write(SetWallColorIdCmd {
            id: id as u32,
            _gap4: (id >> 32) as u32,
            color,
            custom: 0,
            _pad: [0; 3],
        });
    }
    for &(id, color) in gates {
        gate_events.write(SetGateColorIdCmd {
            id: id as u32,
            _gap4: (id >> 32) as u32,
            color,
            custom: 0,
            _pad: [0; 3],
        });
    }
    for &(id, color) in halftimber {
        halftimber_events.write(SetHalftimberColorIdCmd {
            id: id as u32,
            _gap4: (id >> 32) as u32,
            color,
            custom: 0,
            _pad: [0; 3],
        });
    }
}
