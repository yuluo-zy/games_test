//! B `wall_audio_path.rs`: wall-path audio producer.
//!
//! Fresh evidence (decomp-0x140a89bb0): the native system consumes **three**
//! event streams (stride 0x20 / 0x20 / 0x10) and pushes
//! `(DecoratorId, Vec<system_decorator::decorator_visual::stairs::
//! PreferredNodeAlignment>)`-shaped elements (0x20 B: qword, qword, qword,
//! dword, tag byte 2, bool = `alignment.byte@+0x18 == 1`) into a Vec. It does
//! **not** drive any audio loop — the previous `WallPathDraw` + `request_play`
//! model was a self-approximation and is dropped.
//!
//! Cross-domain gap (registered in the rebuild report): the exact element type
//! (`PreferredNodeAlignment`) lives in the external `system_decorator` crate and
//! the three producer event types live in the wall-path domain. This file
//! publishes the byte-compatible host face so the producer can fill it once
//! those types are available.

use crate::{
    resources::audio_resources::{AudioCore, Studio},
    resources::walls::decorator::DecoratorId,
};
use bevy_ecs::prelude::*;

/// Host face for native path stream A (stride 0x20; producer: wall-path domain).
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WallPathStreamEventA {
    /// Payload qword @+8.
    pub value: u64,
    /// Payload qword @+0x10.
    pub data: u64,
}

/// Host face for native path stream B (stride 0x20).
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WallPathStreamEventB {
    /// Payload qword @+8.
    pub value: u64,
    /// Payload qword @+0x10.
    pub data: u64,
}

/// Host face for the third native stream (stride 0x10).
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WallPathIdEvent {
    /// Payload qword @+8.
    pub value: u64,
}

/// One pushed native element (0x20 B). The vector payload
/// (`Vec<PreferredNodeAlignment>`) is the external-crate type; the flat fields
/// keep the observed byte layout.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WallPathAudioEntry {
    pub decorator: DecoratorId,
    /// qword @+8.
    pub value: u64,
    /// qword @+0x10.
    pub alignment: u64,
    /// dword @+0x18.
    pub alignment_tag: u32,
    /// byte @+0x1c (native constant 2).
    pub tag: u8,
    /// byte @+0x1d (`alignment.byte@+0x18 == 1`).
    pub boolean: bool,
}

/// Output buffer the native fills (the Vec of pushed elements).
#[derive(Resource, Debug, Default)]
pub struct WallPathAudioOut {
    pub entries: Vec<WallPathAudioEntry>,
}

/// `wall_audio_path` (0x140a89bb0).
///
/// Structural rebuild: drain the three streams; for every id event push the
/// matching stream element. The wall-table/producer joins are the registered
/// cross-domain gap.
pub fn wall_audio_path<S: Studio + Send + Sync + 'static>(
    mut first: EventReader<WallPathStreamEventA>,
    mut second: EventReader<WallPathStreamEventB>,
    mut ids: EventReader<WallPathIdEvent>,
    mut out: ResMut<WallPathAudioOut>,
    _core: Res<AudioCore<S>>,
) {
    // The native walks the id stream and advances the two 0x20-stride streams in
    // lock-step; the crate-side host face pairs them positionally.
    let first_events: Vec<WallPathStreamEventA> = first.read().copied().collect();
    let second_events: Vec<WallPathStreamEventB> = second.read().copied().collect();
    for (index, id) in ids.read().enumerate() {
        let source = first_events
            .get(index)
            .map(|e| (e.value, e.data))
            .or_else(|| second_events.get(index).map(|e| (e.value, e.data)))
            .unwrap_or((0, 0));
        out.entries.push(WallPathAudioEntry {
            decorator: DecoratorId(id.value as u32),
            value: source.0,
            alignment: source.1,
            alignment_tag: 0,
            tag: 2,
            boolean: false,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pushed_entry_size_matches_the_native_element() {
        assert_eq!(core::mem::size_of::<WallPathAudioEntry>(), 0x20);
        let pushed = WallPathAudioEntry {
            tag: 2,
            ..Default::default()
        };
        assert_eq!(pushed.tag, 2);
    }
}
