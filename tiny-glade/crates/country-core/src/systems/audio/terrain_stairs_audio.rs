//! B `terrain_stairs_audio.rs`: terrain-stair draw sounds — a
//! `event:/SFX/StairDraw_OneShot` ring plus per-stroke one-shot instances.
//!
//! Fresh evidence (decomp-0x141462dc0, disasm 0x141462dc0..0x141463242):
//! - `AppMode == 5` → early return.
//! - the system consumes one event stream; no new events → return.
//! - `editing_speed = clamp01(inverse_lerp(0, 60, mode == 0xd ? 1.0 :
//!   CursorTerrainRaycast.speed))` (the clamped value feeds both the one-shot
//!   and the ring `editing_speed` parameters).
//! - `Vec<AudioInstance>` retain: keeps instances that are Active **and**
//!   Playing (drop the rest).
//! - when the vector is empty and the ring's current instance is Active and
//!   Playing, an 8-byte bookkeeping event is pushed and a counter is
//!   incremented (the event type lives in the terrain-stairs domain; registered
//!   as a host gap).
//! - an SSBO buffer value (SsboId at 0x142c2e1a0, name assertion length 0x45) is
//!   compared with the stored value: equal → the ring is stopped and the system
//!   returns; different → store the value, instantiate
//!   `event:/SFX/StairDraw_OneShot`, start it with `editing_speed`, push it into
//!   the vector, start the ring and set the ring's `editing_speed`.
//!   The SSBO id / grid indexing is host-side (registered); this file consumes
//!   the host-provided value.

use crate::{
    resources::{
        audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
        audio_instance::PlaybackState,
        cursor_terrain_raycast::CursorTerrainRaycast,
    },
    systems::mode_manager::AppMode,
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;
/// Original B14161ddb0 mapped counter: one u32, explicit zero upload for None.
/// The pass alias and startup library use this same canonical element.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, bytemuck::Zeroable, bytemuck::Pod)]
pub struct StairsCounterSsbo(pub u32);

impl crate::render::ssbo::StorageElement for StairsCounterSsbo {
    const NAME: &'static str = "Stairs Counter";
    const BUFFER_LEN: usize = 1;
    const MAPPED_MEMORY: u8 = crate::render::ssbo::MAPPED_MEMORY_READBACK;
}
impl crate::asset_libraries::ssbo_library::SsboType for StairsCounterSsbo {
    type Storage = crate::render::ssbo::MappedStorageBuffer<'static, Self>;
    fn new_storage_buffer(
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        use crate::render::ssbo::{MappedStorageBuffer, StorageElement};
        use crate::render::ssbo::rhapsody_boundary::BufferDesc;
        let zero = [0u32];
        let data = if let Some(bytes) = initial {
            assert_eq!(bytes.len() % 4, 0);
            assert_eq!(bytes.as_ptr() as usize % 4, 0);
            if bytes.len() >= 8 {
                panic!("Excessive data size supplied for buffer {}", core::any::type_name::<Self>());
            }
            bytes
        } else {
            bytemuck::cast_slice(&zero)
        };
        let buffer = device.create_buffer(&BufferDesc::new(4, 0x22, 4).alignment(4), Self::NAME, Some(data))
            .expect("called `Result::unwrap()` on an `Err` value: create_buffer");
        let bytes = device.mapped_slice_mut(&buffer).expect("called `Option::unwrap()` on a `None` value");
        assert!(bytes.len() as u64 >= buffer.byte_size, "assertion failed: data.len() >= buffer.buffer.desc.size");
        assert_eq!(bytes.len() % 4, 0);
        assert_eq!(bytes.as_ptr() as usize % 4, 0);
        let ptr = bytes.as_mut_ptr().cast::<Self>();
        let count = bytes.len() / 4;
        // SAFETY: Self is Pod and length/alignment were checked. Retain the
        // current device/library's persistent mapped owner, as for BushCPU.
        let mapped = unsafe { core::slice::from_raw_parts_mut(ptr, count) };
        MappedStorageBuffer::from_mapping_with_access(mapped, buffer, device.mapped_upload_writer(&buffer))
    }
}
impl crate::render::pass_builder::pass_resource_access::SsboType for StairsCounterSsbo {
    fn rg_buffer_hash() -> u64 { 0x0e13_781d_7271_3b06 }
}

/// `event:/SFX/StairDraw_OneShot` (O: 0x142c62e70).
pub const STAIR_DRAW_ONE_SHOT: &str = "event:/SFX/StairDraw_OneShot";
/// Native1417e3389 initializes the persistent ring with this distinct event.
pub const STAIR_DRAW_LOOP_EVENT: &str = "event:/SFX/StairDraw_LoopTrem";
/// Instance parameter (O: 0x142c62943).
pub const STAIR_EDITING_SPEED_PARAM: &str = "editing_speed";

/// Host face for the native stair-edit event stream (producer: terrain stairs
/// domain, 待接线). The audio handler only needs the per-frame presence.
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TerrainStairsAudioEvent;

/// O (native SSBO buffer first u32; the exact `SsboType::NAME`/id is 待取证 —
/// the terrain-domain writer fills this host face).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct StairsRasterValue(pub u32);

/// StairsCounter (native `+0x40` counter, incremented when a ring event is
/// pushed).
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct StairsCounter {
    pub count: u32,
}

/// TerrainStairsAudioState (O: ring + retained instances + last raster value).
#[derive(Resource)]
pub struct TerrainStairsAudioState<E: EventInstance> {
    pub ring: AudioLoop<AudioInstance<E>>,
    pub instances: Vec<AudioInstance<E>>,
    pub counter: StairsCounter,
    /// Last observed host SSBO value (O: the state comparison).
    pub last_raster: u32,
}

impl<E: EventInstance> TerrainStairsAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        TerrainStairsAudioState {
            ring: core.instantiate_loop(
                &crate::resources::audio_resources::AudioLoopDesc::new(STAIR_DRAW_LOOP_EVENT),
            ),
            instances: Vec::new(),
            counter: StairsCounter::default(),
            last_raster: 0,
        }
    }
}

/// `terrain_stairs_audio_init_system` (native inline init).
pub fn terrain_stairs_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(TerrainStairsAudioState::<S::Event>::new(&core));
}

fn clamp01(value: f32) -> f32 {
    value.max(0.0).min(1.0)
}

/// `terrain_stairs_audio_system` (0x141462dc0).
pub fn terrain_stairs_audio_system<S: Studio + Send + Sync + 'static>(
    mode: Res<AppMode>,
    mut state: ResMut<TerrainStairsAudioState<S::Event>>,
    mut events: EventReader<TerrainStairsAudioEvent>,
    core: Res<AudioCore<S>>,
    raster: Res<StairsRasterValue>,
    raycast: Res<CursorTerrainRaycast>,
) {
    // O: AppMode == 5 (terrain edit) → return.
    if mode.native_tag() == 5 {
        return;
    }
    // O: no new stair-edit events → return (the ring is not stopped).
    if events.read().next().is_none() {
        return;
    }

    // O: editing_speed = clamp01(inverse_lerp(0, 60, mode == Replay ? 1.0 :
    // raycast.speed)).
    let source = if mode.native_tag() == 0xd {
        1.0
    } else {
        raycast.speed
    };
    let editing_speed = clamp01(source / 60.0);

    // O: retain Active + Playing instances only.
    state.instances.retain(|instance| match instance {
        AudioInstance::Active { event, .. } => {
            event.get_playback_state().map_or(false, |s| s == PlaybackState::Playing)
        }
        _ => false,
    });

    // O: empty vector + ring current Active and Playing → push bookkeeping
    // event / counter (the native event type is the terrain-stairs host gap).
    if state.instances.is_empty() {
        let ring_current = &state.ring.entries[state.ring.cursor];
        let playing = match ring_current {
            AudioInstance::Active { event, .. } => event
                .get_playback_state()
                .map_or(false, |s| s == PlaybackState::Playing),
            _ => false,
        };
        if playing {
            state.counter.count = state.counter.count.wrapping_add(1);
        }
    }

    // O: SSBO value compare.
    if raster.0 == state.last_raster {
        // O: same value → stop the ring + return.
        let _ = state.ring.stop_now();
        return;
    }
    state.last_raster = raster.0;

    // O: changed → StairDraw_OneShot + editing_speed, push, ring start + param.
    let mut one_shot = core.instantiate_audio(STAIR_DRAW_ONE_SHOT);
    let _ = one_shot.start_if_not_playing();
    let _ = one_shot.set_parameter(STAIR_EDITING_SPEED_PARAM, editing_speed);
    state.instances.push(one_shot);
    let ring = &mut state.ring;
    let _ = ring.start_if_not_playing();
    let cursor = ring.cursor;
    let _ = ring.entries[cursor].set_parameter(STAIR_EDITING_SPEED_PARAM, editing_speed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stair_draw_one_shot_event_and_param_are_original() {
        assert_eq!(STAIR_DRAW_ONE_SHOT, "event:/SFX/StairDraw_OneShot");
        assert_eq!(STAIR_EDITING_SPEED_PARAM, "editing_speed");
    }
}

#[cfg(test)]
mod canonical_producer_tests {
    use super::StairsCounterSsbo;
    use crate::{
        asset_libraries::ssbo_library::{AssetSsboLibrary, SsboId},
        render::ssbo::{StorageElement, tests::MockDevice},
        resources::render_ssbo::LanternSsbo,
        systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,
    };
    #[test]
    fn actual_startup_library_keeps_counter_mapped_and_instance_none_unuploaded() {
        let device = MockDevice::default();
        let library = AssetSsboLibrary::new();
        library.add_new::<StairsCounterSsbo>(&device);
        library.add_new::<LanternSsbo>(&device);
        library.add_new::<GroundedBuildingsSsbo>(&device);
        let counter = library.get::<StairsCounterSsbo>().unwrap();
        assert_eq!(counter.mapped.len(), 1);
        assert_eq!(counter.buffer.byte_size, 4);
        assert_eq!(library.instance_count::<LanternSsbo>(), 0);
        assert_eq!(library.instance_count::<GroundedBuildingsSsbo>(), 0);
        assert_eq!(SsboId::of::<StairsCounterSsbo>().0.0, 0x0e13781d72713b06);
        assert_eq!(SsboId::of::<GroundedBuildingsSsbo>().0.0, 0x1c53e91811919db1);
        let calls = device.created.borrow();
        assert_eq!(calls.len(), 3);
        assert_eq!((calls[0].0.size, calls[0].0.usage, calls[0].0.memory, calls[0].0.alignment), (4, 0x22, 4, 4));
        assert_eq!(calls[0].1, "Stairs Counter");
        assert_eq!(calls[0].2.as_deref(), Some([0u8; 4].as_slice()));
        for (index, size, name) in [(1, 0x80000, "LanternSsbo"), (2, 0x27100, "Building Shapes")] {
            assert_eq!((calls[index].0.size, calls[index].0.usage, calls[index].0.memory, calls[index].0.alignment), (size, 0x22, 1, 4));
            assert_eq!(calls[index].1, name);
            assert!(calls[index].2.is_none());
        }
        assert_eq!(core::mem::size_of::<crate::systems::render_loop::compute_passes::pass_kit::StairsCounterBuf>(), 4);
        assert_eq!(StairsCounterSsbo::NAME, "Stairs Counter");
    }
    #[test]
    fn actual_initial_rows_keep_payload_and_instance_counts() {
        let device = MockDevice::default();
        let library = AssetSsboLibrary::new();
        let lanterns = [LanternSsbo { position:[1.0, -0.0, 3.0], intensity_color_id:0x12345678 };2];
        let buildings = [GroundedBuildingsSsbo([0x7fc12345;8]);3];
        library.add_new_with_data::<StairsCounterSsbo>(&device, &[StairsCounterSsbo(71)]);
        library.add_new_with_data::<LanternSsbo>(&device, &lanterns);
        library.add_new_with_data::<GroundedBuildingsSsbo>(&device, &buildings);
        assert_eq!(library.instance_count::<LanternSsbo>(), 2);
        assert_eq!(library.instance_count::<GroundedBuildingsSsbo>(), 3);
        let calls = device.created.borrow();
        assert_eq!(calls[0].2.as_deref(), Some(71u32.to_le_bytes().as_slice()));
        assert_eq!(calls[1].2.as_deref(), Some(bytemuck::cast_slice(&lanterns)));
        assert_eq!(calls[2].2.as_deref(), Some(bytemuck::cast_slice(&buildings)));
    }
    #[test]
    #[should_panic(expected = "data.len() >= buffer.buffer.desc.size")]
    fn mapped_page_must_cover_the_returned_descriptor_not_only_the_request() {
        use crate::render::ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice};
        struct ReturnedLargerDescriptor(MockDevice);
        impl GraphicsDevice for ReturnedLargerDescriptor {
            fn create_buffer(&self, desc: &BufferDesc, name: &str, initial: Option<&[u8]>) -> Result<Buffer, BackendError> {
                let mut buffer = self.0.create_buffer(desc, name, initial)?;
                buffer.byte_size = 8;
                Ok(buffer)
            }
            fn update_buffer_bytes(&self, buffer: &Buffer, bytes: &[u8], ranges: &[core::ops::Range<usize>], offset: usize) -> Result<(), BackendError> {
                self.0.update_buffer_bytes(buffer, bytes, ranges, offset)
            }
            fn mapped_slice_mut(&self, buffer: &Buffer) -> Option<&mut [u8]> { self.0.mapped_slice_mut(buffer) }
        }
        let device = ReturnedLargerDescriptor(MockDevice::default());
        device.0.map.borrow_mut().resize(4, 0);
        let _ = <StairsCounterSsbo as crate::asset_libraries::ssbo_library::SsboType>::new_storage_buffer(&device, None);
    }

}
