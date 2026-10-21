//! B canonical40B chimney params, real sparse construction and seven-input host.
//!
//! The host consumes the shared strict tracker, performs native clear/push and
//! Added/Changed/Removed sounds on actual resources. The Entity/lane adapter is
//! a bounded current-architecture mapping; full member14 registration, anchor,
//! loader, audio backend and session cleanup remain separate integration gates.

use bevy_ecs::prelude::{EventWriter, Res, ResMut};
use country_core::{
    geometry::{
        instanced_wall::{BrickSourceId, BrickTransformSsbo},
        runtime::WallBrickAssemblyResources,
    },
    render::sparse_instance_buffer::SparseInstanceBufferWriter,
    resources::{
        render::Affine3Packed,
        walls::{WallId, decorator::DecoratorId, publication_fields::WallId as BrickSourceWallId},
    },
};
use glam::{Affine3A, Vec2, Vec3};

use super::deco_differential_state::{DecoDiff, WallAttachedDecoTracker};
use country_core::resources::{
    terrain_height::{TerrainHeightSsbo, TerrainHeightsData},
    walls::{decorator_subtype::DerivedDecoratorInfoResources, public_walls::PublicWalls},
};
use country_core::systems::{audio::PlayDecoratorSoundCmd, ui::signifiers::UiSignifierStream};

/// Exact 40-byte carrier. Field meanings beyond the native offset contract are
/// not invented. 140824fe0/172B proves the comparison used by fullvisual.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ChimneyAssemblyParams {
    pub words: [u32; 10],
}

impl PartialEq for ChimneyAssemblyParams {
    fn eq(&self, other: &Self) -> bool {
        (2..8).all(|i| f32::from_bits(self.words[i]) == f32::from_bits(other.words[i]))
            && self.words[0] == other.words[0]
            && (self.words[0] == 1
                || f32::from_bits(self.words[1]) == f32::from_bits(other.words[1]))
            && self.words[8] == other.words[8]
            && f32::from_bits(self.words[9]) == f32::from_bits(other.words[9])
    }
}

const _: () = assert!(std::mem::size_of::<ChimneyAssemblyParams>() == 40);
const _: () = assert!(std::mem::align_of::<ChimneyAssemblyParams>() == 4);

/// The original constructor's two endpoints and rectangle size controls.
/// These values are supplied by the original per-current-instance branches.
#[derive(Clone, Copy, Debug)]
pub struct ChimneyBrickSection {
    pub end_2: f32,
    pub end_3: f32,
    pub base_size: Vec2,
    pub size_delta: f32,
}

/// Original 14200bcd0/2836B. Appends to the actual shared sparse writer.
/// `true` means at least one NoFreeChunks failure; all later bricks are still
/// attempted. Clearing the per-wall lane belongs to the surrounding host.
pub fn construct_crossbrick_chimney(
    params: &ChimneyAssemblyParams,
    section: ChimneyBrickSection,
    wall: WallId,
    rng: &fastrand::Rng,
    writer: &mut SparseInstanceBufferWriter<'_, BrickTransformSsbo>,
) -> bool {
    emit_crossbrick_chimney(params, section, wall, rng, &mut |brick| {
        writer.push(brick).is_err()
    })
}

fn emit_crossbrick_chimney(
    params: &ChimneyAssemblyParams,
    section: ChimneyBrickSection,
    wall: WallId,
    rng: &fastrand::Rng,
    emit: &mut impl FnMut(&BrickTransformSsbo) -> bool,
) -> bool {
    let center = Vec2::new(
        f32::from_bits(params.words[2]),
        f32::from_bits(params.words[3]),
    );
    let axis = Vec2::new(
        f32::from_bits(params.words[6]),
        f32::from_bits(params.words[7]),
    );
    let height = section.end_2 - section.end_3;
    let count = ((height / 0.4).ceil() as usize).max(3);
    // The original borrows the normalized split list, collects a second list,
    // then drops the first. Preserve interpolation order and allocation failure.
    let heights: Vec<f32> = {
        let splits = utils::random_splits(count, 0.2 / height, rng);
        splits
            .iter()
            .map(|&t| t * section.end_2 + (1.0 - t) * section.end_3)
            .collect()
    };
    // The last emitted layer has index len-2 and reaches t=1.
    let last_layer_index = (heights.len() - 2) as f32;
    let material = BrickSourceId::from_wall_id(BrickSourceWallId(params.words[8] as u64)).0;
    let mut distance = 0.0;
    let mut failed = false;
    for (layer, pair) in heights.windows(2).enumerate() {
        let low = Vec3::new(center.x, pair[0], center.y);
        let high = Vec3::new(center.x, pair[1], center.y);
        let middle_xz = Vec2::new((low.x + high.x) * 0.5, (low.z + high.z) * 0.5);
        let middle_y = (low.y + high.y) * 0.5;
        let layer_height = (high - low).length();
        let t = layer as f32 / last_layer_index;
        let s = ((t + f32::from_bits(0xbf59_999a)) / f32::from_bits(0x3e19_9998)).clamp(0.0, 1.0);
        let smooth = (3.0 - (s + s)) * (s * s);
        let size_offset = smooth * section.size_delta;
        let size = Vec2::new(
            size_offset + section.base_size.x,
            size_offset + section.base_size.y,
        );
        let rectangle = utils::geometry::Rectangle2d::new(middle_xz, size, axis);
        let sign = if layer & 1 == 0 { -1.0 } else { 1.0 };
        for edge in 0..4 {
            let a = rectangle.get_point(edge);
            let b = rectangle.get_point((edge + 1) & 3);
            let delta = b - a;
            let offset = ((delta * (1.0 / delta.length())) * 0.3) * 0.5 * sign;
            let a = a + offset;
            let b = b + offset;
            let middle = a * 0.5 + b * 0.5;
            let r = rng.f32();
            let thickness = (r * 0.05 - (1.0 - r) * 0.05) + 0.3;
            let rotation = utils::xz_rotation_bad(rectangle.get_edge_normal(edge));
            let length = (a - b).length();
            let affine = Affine3A::from_scale_rotation_translation(
                Vec3::new(length, layer_height, thickness),
                rotation,
                Vec3::new(middle.x, middle_y, middle.y),
            );
            let next_distance = length + distance;
            let brick = BrickTransformSsbo {
                affine_columns: Affine3Packed::from(affine)
                    .cols
                    .map(|column| column.to_array()),
                seed: (wall.0 as u32)
                    .wrapping_add(layer as u32)
                    .wrapping_add(edge as u32),
                material,
                wall_distance: [distance, next_distance],
                ..BrickTransformSsbo::default()
            };
            distance = next_distance;
            failed |= emit(&brick);
        }
    }
    failed
}

/// Original current-instance geometry branch. Uses the same deterministic rng
/// for the body and the optional lower section; no audio or diff normalization
/// is performed here. The real host must do those original side effects.
pub fn write_chimney_instance(
    params: &ChimneyAssemblyParams,
    wall: WallId,
    decorator: DecoratorId,
    terrain: &country_core::resources::terrain_height::TerrainHeightsData<
        country_core::resources::terrain_height::TerrainHeightSsbo,
    >,
    writer: &mut SparseInstanceBufferWriter<'_, BrickTransformSsbo>,
) -> bool {
    let rng = fastrand::Rng::with_seed(utils::rng::init_rng(decorator.0));
    let base = f32::from_bits(params.words[4]);
    let mut failed = construct_crossbrick_chimney(
        params,
        ChimneyBrickSection {
            end_2: f32::from_bits(params.words[5]),
            end_3: base,
            base_size: Vec2::splat(0.5),
            size_delta: 0.3,
        },
        wall,
        &rng,
        writer,
    );
    let extra = match params.words[0] {
        1 => Some(ChimneyBrickSection {
            end_2: base,
            end_3: base + -0.4,
            base_size: Vec2::splat(0.2),
            size_delta: 0.1,
        }),
        0 => {
            let center = Vec2::new(
                f32::from_bits(params.words[2]),
                f32::from_bits(params.words[3]),
            );
            let axis = Vec2::new(
                f32::from_bits(params.words[6]),
                f32::from_bits(params.words[7]),
            );
            let along = (axis * 1.3) * 0.5;
            let across = Vec2::new(-axis.y, axis.x) * 0.5;
            let mut end = base - f32::from_bits(params.words[1]);
            for point in [center + along + across, center - along + across] {
                end = end.min(terrain.sample_at_world_xz(point.x, point.y));
            }
            Some(ChimneyBrickSection {
                end_2: base,
                end_3: end,
                base_size: Vec2::new(1.3, 1.0),
                size_delta: -0.3,
            })
        }
        _ => {
            let height = f32::from_bits(params.words[1]);
            (height > 0.1).then_some(ChimneyBrickSection {
                end_2: base,
                end_3: base - height,
                base_size: Vec2::splat(0.8),
                size_delta: 0.0,
            })
        }
    };
    if let Some(section) = extra {
        failed |= construct_crossbrick_chimney(params, section, wall, &rng, writer);
    }
    failed
}

/// Original1407f8db0/117B and1412ae660 business inputs. The shared third
/// SystemParam also borrows current WallEntities/Query mapping; it never owns
/// an alternate wall map. Root mounts the complete original member14 group.
pub fn chimney_assemble_bricks(
    derived: DerivedDecoratorInfoResources,
    public: Res<PublicWalls>,
    mut owners: WallBrickAssemblyResources,
    mut tracker: ResMut<WallAttachedDecoTracker<ChimneyAssemblyParams>>,
    terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    _signifiers: Res<UiSignifierStream>,
    mut sounds: EventWriter<PlayDecoratorSoundCmd>,
) {
    let derived = derived.as_view();
    tracker.with_diffs(|wall, log| {
        // Native ae937 typed entry/type check precedes the PublicWalls gate,
        // even when the actual wall is absent. No dirty => no entry lookup.
        let mut batch = owners.for_batch();
        if public.get(wall).is_none() {
            return log.into_removals().with_removed(|_, _| {});
        }
        let mut writer = batch.chimney_writer(wall);
        writer.clear();
        let mut _failed = false;
        let removed = log.with_each_deco_diff(|id, diff| {
            if let Some(params) = diff.current() {
                _failed |= write_chimney_instance(params, wall, id, &terrain, &mut writer);
                if let Some(kind) = current_sound_kind(diff) {
                    sounds.write(PlayDecoratorSoundCmd::at(kind, sound_position(params)));
                }
            }
        });
        let completed = removed.with_removed(|id, previous| {
            if derived.try_get_subtype(id).is_some_and(|subtype| subtype.0 == 0x14) {
                sounds.write(PlayDecoratorSoundCmd::at(0x15, sound_position(previous)));
            }
        });
        // Native af077 removals/normalization precedes af087..af09f unlock.
        // Capacity failures do not skip normalization, create UI or request retry.
        drop(writer);
        completed
    });
}

fn sound_position(params: &ChimneyAssemblyParams) -> Vec3 {
    Vec3::new(
        f32::from_bits(params.words[2]),
        f32::from_bits(params.words[4]),
        f32::from_bits(params.words[3]),
    )
}

fn current_sound_kind(diff: &DecoDiff<ChimneyAssemblyParams>) -> Option<u8> {
    match diff {
        DecoDiff::Added(current) if current.words[0] == 0 => Some(0x13),
        DecoDiff::Changed { current, previous } if previous.words[0] != 0 && current.words[0] == 0 => Some(0x13),
        DecoDiff::Changed { current, previous } if previous.words[0] == 0 && current.words[0] != 0 => Some(0x14),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_float_equality_and_tag_one_inactive_word() {
        let a = ChimneyAssemblyParams { words: [0; 10] };
        let mut b = a;
        b.words[2] = (-0.0f32).to_bits();
        assert_eq!(a, b);
        b.words[2] = f32::NAN.to_bits();
        assert_ne!(b, b);
        let mut c = a;
        c.words[0] = 1;
        let mut d = c;
        d.words[1] = f32::NAN.to_bits();
        assert_eq!(c, d);
        d.words[8] = 1;
        assert_ne!(c, d);
    }

    #[test]
    fn last_layer_reaches_full_size_delta_with_three_height_splits() {
        let mut params = ChimneyAssemblyParams { words: [0; 10] };
        params.words[6] = 1.0f32.to_bits();
        let mut bricks = Vec::new();
        assert!(!emit_crossbrick_chimney(
            &params,
            ChimneyBrickSection {
                end_2: 0.8,
                end_3: 0.0,
                base_size: Vec2::splat(0.5),
                size_delta: 0.3,
            },
            WallId(100),
            &fastrand::Rng::with_seed(123),
            &mut |brick| {
                bricks.push(*brick);
                false
            },
        ));
        assert_eq!(bricks.len(), 8);
        // Native index normalization ends at the final layer (index1/1).
        // A len-1 denominator would leave its four edge lengths at0.5.
        for brick in &bricks[..4] {
            assert!((Vec3::from_array(brick.affine_columns[0]).length() - 0.5).abs() < 1e-6);
        }
        for brick in &bricks[4..] {
            assert!((Vec3::from_array(brick.affine_columns[0]).length() - 0.8).abs() < 1e-6);
        }
    }

    #[test]
    fn emitted_payload_and_rng_continue_after_a_failed_brick() {
        let mut params = ChimneyAssemblyParams { words: [0; 10] };
        params.words[6] = 1.0f32.to_bits();
        params.words[8] = 100_001;
        let section = ChimneyBrickSection {
            end_2: 0.8,
            end_3: 0.0,
            base_size: Vec2::splat(0.5),
            size_delta: 0.0,
        };
        let rng = fastrand::Rng::with_seed(123);
        let mut bricks = Vec::new();
        assert!(!emit_crossbrick_chimney(
            &params,
            section,
            WallId(100),
            &rng,
            &mut |brick| {
                bricks.push(*brick);
                false
            },
        ));
        assert_eq!(bricks.len(), 8);
        assert_eq!(
            bricks.iter().map(|brick| brick.seed).collect::<Vec<_>>(),
            [100, 101, 102, 103, 101, 102, 103, 104]
        );
        assert_eq!(bricks[0].wall_distance[0], 0.0);
        assert_eq!(bricks.last().unwrap().wall_distance[1], 4.0);
        for pair in bricks.windows(2) {
            assert_eq!(pair[0].wall_distance[1], pair[1].wall_distance[0]);
        }
        assert!(bricks.iter().all(|brick| brick.material == 0x8000_0001
            && brick.deformation == [0.0; 4]
            && brick.surface == [-100.0; 3]
            && brick.flags == 0));
        // One split sample followed by one thickness sample per brick.
        let expected_rng = fastrand::Rng::with_seed(123);
        for _ in 0..9 {
            expected_rng.f32();
        }
        assert_eq!(rng.get_seed(), expected_rng.get_seed());

        let failed_rng = fastrand::Rng::with_seed(123);
        let mut attempted = Vec::new();
        assert!(emit_crossbrick_chimney(
            &params,
            section,
            WallId(100),
            &failed_rng,
            &mut |brick| {
                attempted.push(*brick);
                attempted.len() == 1
            },
        ));
        assert_eq!(attempted, bricks);
        assert_eq!(failed_rng.get_seed(), rng.get_seed());
    }
}

#[cfg(test)]
mod actual_host_tests {
    use super::*;
    use bevy_ecs::prelude::*;
    use country_core::{
        asset_libraries::ssbo_library::AssetSsboLibrary,
        geometry::instanced_wall::InstancedWall,
        render::{sparse_instance_buffer::SparseInstanceBuffer},
        resources::walls::{InnerWalls, WallState, public_walls::WallEntities, publication_source::TerrainHeightGrid512, decorator_storage::DecoratorStorageAddr, decorator_subtype::{DerivedDecoratorSubtypes, DecoratorMeshLibrary, DecoratorSubtype}},
    };
    use std::panic::{AssertUnwindSafe, catch_unwind};


    /// CPU-only range-checking boundary for these CPU ownership/state tests.
    /// It creates no physical resource, maps no memory, and submits no GPU work.
    /// Successful updates mean only that the canonical uploader requested valid
    /// nonoverlapping byte ranges inside the declared CPU fixture capacity.
    #[derive(Debug)]
    struct CpuBufferDevice(u64);

    impl country_core::render::ssbo::rhapsody_boundary::GraphicsDevice for CpuBufferDevice {
        fn create_buffer(
            &self,
            desc: &country_core::render::ssbo::rhapsody_boundary::BufferDesc,
            _name: &str,
            data: Option<&[u8]>,
        ) -> Result<country_core::render::ssbo::rhapsody_boundary::Buffer, country_core::render::ssbo::rhapsody_boundary::BackendError> {
            assert!(data.is_none_or(|bytes| bytes.len() <= desc.size));
            Ok(country_core::render::ssbo::rhapsody_boundary::Buffer {
                id: self.0,
                byte_size: desc.size as u64,
                ..Default::default()
            })
        }

        fn update_buffer_bytes(
            &self,
            buffer: &country_core::render::ssbo::rhapsody_boundary::Buffer,
            data: &[u8],
            ranges: &[core::ops::Range<usize>],
            offset: usize,
        ) -> Result<(), country_core::render::ssbo::rhapsody_boundary::BackendError> {
            let mut previous_end = 0;
            for range in ranges {
                assert!(previous_end <= range.start);
                assert!(range.start <= range.end && range.end <= data.len());
                let end = offset.checked_add(range.end).expect("CPU fixture byte offset overflow");
                assert!((end as u64) <= buffer.byte_size);
                previous_end = range.end;
            }
            Ok(())
        }
    }

    fn params(tag: u32) -> ChimneyAssemblyParams {
        let mut p = ChimneyAssemblyParams { words: [0;10] };
        p.words[0]=tag; p.words[2]=3.0f32.to_bits(); p.words[3]=4.0f32.to_bits();
        p.words[4]=2.0f32.to_bits(); p.words[5]=3.0f32.to_bits(); p.words[6]=1.0f32.to_bits(); p.words[8]=49;
        p
    }
    fn actual_world() -> (World, WallId, Entity) {
        let wall = WallId(49); let mut inner = InnerWalls::new();
        let curve = country_core::resources::roofs::geometry::surface::Curve2::try_new(vec![[0.,0.],[4.,0.]], false).unwrap();
        inner.new_wall_with_id(wall, WallState::new(curve, 2., 0, 0, false));
        let mut public = PublicWalls::default();
        public.consume(&mut inner, &TerrainHeightGrid512 { encoded: vec![0u16;512*512].into(), minimum_y: 0., maximum_y: 4., sampling_fix: true });
        let storage = SparseInstanceBuffer::<BrickTransformSsbo>::try_new(&CpuBufferDevice(1)).unwrap();
        let owner = InstancedWall::new_awaiting_production(wall.0, &storage);
        let library = AssetSsboLibrary::new(); library.register::<BrickTransformSsbo>(storage);
        let mut world = World::new(); world.insert_resource(public); world.insert_resource(library);
        world.init_resource::<WallEntities>(); world.init_resource::<WallAttachedDecoTracker<ChimneyAssemblyParams>>();
        world.init_resource::<DerivedDecoratorSubtypes>(); world.init_resource::<DecoratorMeshLibrary>();
        world.init_resource::<UiSignifierStream>(); world.init_resource::<Events<PlayDecoratorSoundCmd>>();
        world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
        let entity = world.spawn(owner).id(); world.resource_mut::<WallEntities>().0.insert(wall,entity);
        (world, wall, entity)
    }

    #[test]
    fn current_audio_transitions_use_real_diff_kind_and_current_position() {
        assert_eq!(current_sound_kind(&DecoDiff::Added(Box::new(params(0)))), Some(0x13));
        assert_eq!(current_sound_kind(&DecoDiff::Current(Box::new(params(0)))), None);
        assert_eq!(current_sound_kind(&DecoDiff::Changed { previous: Box::new(params(1)), current: Box::new(params(0)) }), Some(0x13));
        assert_eq!(current_sound_kind(&DecoDiff::Changed { previous: Box::new(params(0)), current: Box::new(params(1)) }), Some(0x14));
        assert_eq!(current_sound_kind(&DecoDiff::Changed { previous: Box::new(params(0)), current: Box::new(params(0)) }), None);
        assert_eq!(current_sound_kind(&DecoDiff::Changed { previous: Box::new(params(1)), current: Box::new(params(2)) }), None);
        assert_eq!(current_sound_kind(&DecoDiff::Removed(Box::new(params(0)))), None);
        assert_eq!(sound_position(&params(1)), Vec3::new(3.,2.,4.));
    }

    #[test]
    fn removed_audio_borrows_the_actual_subtype_owner_and_uses_previous_position() {
        let (mut world, wall, entity) = actual_world(); let id = DecoratorId(7);
        // This fixture only exercises the canonical subtype lookup, not the
        // still-unrecovered collision payload/asset loader.
        world.resource_mut::<DerivedDecoratorSubtypes>().insert(id.0, DecoratorSubtype(0x14));
        world.resource_mut::<WallAttachedDecoTracker<ChimneyAssemblyParams>>().add(DecoratorStorageAddr::Wall(wall), id, &params(1));
        let mut schedule = Schedule::default(); schedule.add_systems(chimney_assemble_bricks);
        schedule.run(&mut world);
        assert!(world.resource::<Events<PlayDecoratorSoundCmd>>().is_empty());
        world.resource_mut::<WallAttachedDecoTracker<ChimneyAssemblyParams>>().remove(DecoratorStorageAddr::Wall(wall), id);
        schedule.run(&mut world);
        let sounds: Vec<_> = world.resource_mut::<Events<PlayDecoratorSoundCmd>>().drain().collect();
        assert_eq!(sounds, [PlayDecoratorSoundCmd::at(0x15, Vec3::new(3.,2.,4.))]);
        let owner = world.get::<InstancedWall<BrickTransformSsbo>>(entity).unwrap();
        assert!(world.resource::<AssetSsboLibrary>().get::<BrickTransformSsbo>().unwrap().allocation_to_draw_multi_instanced(owner.chimney_allocation().unwrap()).is_empty());
    }

    #[test]
    fn geometry_panic_releases_actual_lane_mutex_without_normalizing_added() {
        let (mut world, wall, entity) = actual_world(); let id = DecoratorId(7);
        let mut invalid = params(1); invalid.words[6] = 0;
        world.resource_mut::<WallAttachedDecoTracker<ChimneyAssemblyParams>>().add(DecoratorStorageAddr::Wall(wall), id, &invalid);
        let mut schedule = Schedule::default(); schedule.add_systems(chimney_assemble_bricks);
        assert!(catch_unwind(AssertUnwindSafe(|| schedule.run(&mut world))).is_err());
        let owner = world.get::<InstancedWall<BrickTransformSsbo>>(entity).unwrap();
        assert!(owner.chimney_allocation().unwrap().handle.0.as_deref().unwrap().chunk_ids.try_lock().is_some());
        let mut tracker = world.resource_mut::<WallAttachedDecoTracker<ChimneyAssemblyParams>>();
        tracker.on_wall_changed(wall);
        tracker.with_diffs(|_,log| log.with_each_deco_diff(|_,diff| assert!(matches!(diff, DecoDiff::Added(_)))).with_removed(|_,_|panic!("Added cannot be Removed")));
    }
}

