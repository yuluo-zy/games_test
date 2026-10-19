//! Retire the actual dormer roof allocations on canonical roof-delete events.
//!
//! B maintain_dormer_tiles@1407ce920/334B takes ResMut<DormerTiles> and
//! EventReader<DeleteRoofCmd>. Its event-instance metadata belongs to Bevy;
//! the command payload is one WallId, not an application tile array.
use bevy_ecs::event::EventReader;
use bevy_ecs::system::ResMut;
use country_core::resources::roofs::commands::DeleteRoofCmd;
use country_core::resources::roofs::dormer_tiles::DormerTiles;

/// Each real event marks the resource changed before its attempted remove.
/// Empty iteration does not mutably dereference it. Allocation Drop queues
/// retirement; completion of a GPU upload is not implied by this operation.
pub fn maintain_dormer_tiles(
    mut dormer_tiles: ResMut<DormerTiles>,
    mut deleted_roofs: EventReader<DeleteRoofCmd>,
) {
    for event in deleted_roofs.read() {
        drop(dormer_tiles.allocations.remove(&event.0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::change_detection::DetectChanges;
    use bevy_ecs::event::Events;
    use bevy_ecs::schedule::Schedule;
    use bevy_ecs::world::World;
    use country_core::render::sparse_instance_buffer::{SparseInstanceBuffer, SparseSsbo};
    use country_core::render::ssbo::rhapsody_boundary::{
        BackendError, Buffer, BufferDesc, GraphicsDevice,
    };
    use country_core::resources::roof_tile_ssbo::RoofTileSsbo;
    use country_core::resources::walls::WallId;
    use std::ops::Range;

    /// Only the existing GraphicsDevice test boundary is recorded. The real
    /// sparse buffer owns its CPU chunks, allocation handles and waste queue.
    struct CpuDevice;

    impl GraphicsDevice for CpuDevice {
        fn create_buffer(
            &self,
            descriptor: &BufferDesc,
            _name: &str,
            _data: Option<&[u8]>,
        ) -> Result<Buffer, BackendError> {
            Ok(Buffer {
                byte_size: descriptor.size as u64,
                ..Buffer::default()
            })
        }

        fn update_buffer_bytes(
            &self,
            _buffer: &Buffer,
            _data: &[u8],
            _ranges: &[Range<usize>],
            _offset: usize,
        ) -> Result<(), BackendError> {
            panic!("the deletion system must not perform a GPU upload")
        }
    }

    struct Fixture {
        world: World,
        schedule: Schedule,
        buffer: SparseInstanceBuffer<RoofTileSsbo>,
    }

    impl Fixture {
        fn new(walls: &[WallId]) -> Self {
            let mut buffer =
                SparseInstanceBuffer::<RoofTileSsbo>::new_storage_buffer(&CpuDevice, None);
            let mut tiles = DormerTiles::default();
            for (index, &wall) in walls.iter().enumerate() {
                let allocation = buffer.new_allocation();
                buffer
                    .writer(&allocation)
                    .push(&RoofTileSsbo {
                        roof_id: wall.0 as u32,
                        seed: index as i32,
                        ..RoofTileSsbo::default()
                    })
                    .unwrap();
                assert!(buffer.owns_allocation(&allocation));
                assert!(tiles.allocations.insert(wall, allocation).is_none());
            }
            let mut world = World::new();
            world.insert_resource(tiles);
            world.insert_resource(Events::<DeleteRoofCmd>::default());
            let mut schedule = Schedule::default();
            schedule.add_systems(maintain_dormer_tiles);
            world.clear_trackers();
            Self {
                world,
                schedule,
                buffer,
            }
        }

        fn send(&mut self, wall: WallId) {
            self.world
                .resource_mut::<Events<DeleteRoofCmd>>()
                .send(DeleteRoofCmd(wall));
        }

        fn run(&mut self) {
            self.schedule.run(&mut self.world);
        }

        fn chunks(&self, wall: WallId) -> Vec<u32> {
            self.world.resource::<DormerTiles>().allocations[&wall]
                .handle
                .0
                .as_ref()
                .unwrap()
                .chunk_ids
                .lock()
                .clone()
        }

        fn queued_chunks(&self) -> Vec<Vec<u32>> {
            self.buffer
                .waste
                .lock()
                .iter()
                .map(|handle| handle.0.as_ref().unwrap().chunk_ids.lock().clone())
                .collect()
        }
    }

    #[test]
    fn delete_uses_full_wall_identity_and_queues_the_same_real_allocation() {
        let low = WallId(7);
        let high = WallId(0x1_0000_0007);
        let mut fixture = Fixture::new(&[low, high]);
        let deleted_chunks = fixture.chunks(low);
        let retained_chunks = fixture.chunks(high);
        assert_ne!(deleted_chunks, retained_chunks);
        let free_before = fixture.buffer.free_chunks.read().len();

        fixture.send(low);
        fixture.run();

        assert!(
            !fixture
                .world
                .resource::<DormerTiles>()
                .allocations
                .contains_key(&low)
        );
        assert_eq!(fixture.chunks(high), retained_chunks);
        assert_eq!(fixture.queued_chunks(), vec![deleted_chunks]);
        assert_eq!(fixture.buffer.free_chunks.read().len(), free_before);
        fixture.buffer.take_out_waste();
        assert!(fixture.queued_chunks().is_empty());
        assert_eq!(fixture.buffer.free_chunks.read().len(), free_before + 1);
    }

    #[test]
    fn repeated_and_missing_events_do_not_retire_a_second_allocation() {
        let mut fixture = Fixture::new(&[WallId(11), WallId(12)]);
        fixture.send(WallId(11));
        fixture.send(WallId(11));
        fixture.send(WallId(99));
        fixture.run();
        assert_eq!(fixture.queued_chunks().len(), 1);
        assert_eq!(fixture.world.resource::<DormerTiles>().allocations.len(), 1);
        let retained = fixture.chunks(WallId(12));

        fixture.world.clear_trackers();
        fixture.send(WallId(11));
        fixture.run();
        assert!(fixture.world.resource_ref::<DormerTiles>().is_changed());
        assert_eq!(fixture.queued_chunks().len(), 1);
        assert_eq!(fixture.chunks(WallId(12)), retained);
    }

    #[test]
    fn only_an_actual_unread_event_marks_the_resource_changed() {
        let mut fixture = Fixture::new(&[]);
        fixture.run();
        assert!(!fixture.world.resource_ref::<DormerTiles>().is_changed());

        fixture.send(WallId(900));
        fixture.run();
        assert!(fixture.world.resource_ref::<DormerTiles>().is_changed());
        assert!(fixture.queued_chunks().is_empty());

        fixture.world.clear_trackers();
        fixture.run();
        assert!(!fixture.world.resource_ref::<DormerTiles>().is_changed());
    }

    #[test]
    fn actual_event_retention_and_reader_cursor_control_deletion() {
        let mut fixture = Fixture::new(&[WallId(21), WallId(22), WallId(23)]);
        fixture.send(WallId(21));
        fixture
            .world
            .resource_mut::<Events<DeleteRoofCmd>>()
            .update();
        fixture.send(WallId(22));
        fixture.run();
        assert_eq!(fixture.queued_chunks().len(), 2);
        assert_eq!(fixture.world.resource::<DormerTiles>().allocations.len(), 1);

        fixture.world.clear_trackers();
        fixture.run();
        assert!(!fixture.world.resource_ref::<DormerTiles>().is_changed());
        assert_eq!(fixture.queued_chunks().len(), 2);

        fixture.send(WallId(23));
        fixture
            .world
            .resource_mut::<Events<DeleteRoofCmd>>()
            .update();
        fixture
            .world
            .resource_mut::<Events<DeleteRoofCmd>>()
            .update();
        fixture.world.clear_trackers();
        fixture.run();
        assert!(!fixture.world.resource_ref::<DormerTiles>().is_changed());
        assert!(
            fixture
                .world
                .resource::<DormerTiles>()
                .allocations
                .contains_key(&WallId(23))
        );

        fixture.send(WallId(23));
        fixture.run();
        assert!(
            fixture
                .world
                .resource::<DormerTiles>()
                .allocations
                .is_empty()
        );
        assert_eq!(fixture.queued_chunks().len(), 3);
    }

    #[test]
    fn dropping_the_resource_retires_remaining_handles_through_the_same_owner() {
        let mut fixture = Fixture::new(&[WallId(31), WallId(32)]);
        assert_eq!(
            fixture.buffer.free_chunks.read().len(),
            RoofTileSsbo::CHUNK_COUNT - 2
        );
        drop(fixture.world.remove_resource::<DormerTiles>().unwrap());
        assert_eq!(fixture.queued_chunks().len(), 2);
        fixture.buffer.take_out_waste();
        assert!(fixture.queued_chunks().is_empty());
        assert_eq!(
            fixture.buffer.free_chunks.read().len(),
            RoofTileSsbo::CHUNK_COUNT
        );
    }
}
