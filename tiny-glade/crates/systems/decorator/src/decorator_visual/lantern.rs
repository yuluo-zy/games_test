//! 原B `decorator_visual/lantern.rs`：14078d320生成，140790400光源维护。
//! 原生成7组/维护8组SystemParam和140790220局部缓存共同恢复；不生成影子灯列表。
//! 真实SSBO和世界组件由唯一core owner提供，fullvisual负责创建/替换/卸载实体。
use bevy_ecs::prelude::*;
use country_core::resources::session::ActiveSession;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::{
    DecoratorLibKey, DerivedDecoratorInfoResources,
};
use glam::Vec3;

use crate::blueprint::DecoratorBlueprints;

pub const LIGHT_RECORD_SIZE: usize = 28;
pub const LIGHT_RECORD_CAP: usize = 0x8000;
pub type LanternLightRecord = [u8; LIGHT_RECORD_SIZE];
fn light_record_key(record: &LanternLightRecord) -> u32 {
    record_u32(record, 0)
}

/// 原 B14078d320：只发布蓝图，实体和GPU由唯一fullvisual宿主负责。
/// 原七组参数保留真实墙/屋顶查询；地形与楼梯桶不能被墙迭代截断。
pub fn generate_lanterns(
    derived: DerivedDecoratorInfoResources,
    storage: Res<DecoratorStorage>,
    cached: Res<
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms,
    >,
    mut blueprints: ResMut<DecoratorBlueprints>,
    walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    roofs: Query<&country_core::resources::roofs::roof_shape::Roof>,
    _signifiers: Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,
) {
    use country_core::resources::walls::{
        decorator::DecoratorId, decorator_storage::DecoratorStorageAddr,
    };
    let view = derived.as_view();
    let buckets = storage
        .iter_walls()
        .map(|(wall, bucket)| (DecoratorStorageAddr::Wall(wall), bucket))
        .chain(
            storage
                .iter_terrain_decorators()
                .map(|bucket| (DecoratorStorageAddr::Terrain, bucket)),
        )
        .chain(
            storage
                .iter_stair_decorators()
                .map(|bucket| (DecoratorStorageAddr::Stair, bucket)),
        );
    for (address, bucket) in buckets {
        for id in bucket.iter_modified_ids() {
            let info = bucket
                .get(DecoratorId(id))
                .expect("modified decorator is missing from its storage");
            let subtype = view.subtype(DecoratorId(id)).0;
            if subtype.wrapping_sub(16) > 2 {
                continue;
            }
            if subtype == 18 {
                match address {
                    DecoratorStorageAddr::Wall(wall) => {
                        if !walls
                            .roof_component(wall, |entity| roofs.get(entity).ok())
                            .is_some_and(|roof| roof.is_flat())
                        {
                            continue;
                        }
                    }
                    DecoratorStorageAddr::Stair => {
                        panic!("internal error: entered unreachable code")
                    }
                    DecoratorStorageAddr::Terrain => {}
                }
            }
            let blueprint = blueprints.insert_for(id, address, info.state());
            let original = cached.transform(id);
            let mut pose = country_core::utils::components::transform::Transform {
                rotation: original.rotation,
                translation: original.translation,
                scale: original.scale,
            };
            // 原墙挂灯先沿自身Z前移0.28；碰撞位姿与最终描边中心各有原写面。
            if subtype == 16 {
                pose.translation +=
                    pose.rotation * Vec3::new(0.0, 0.0, f32::from_bits(0x3e8f_5c29));
            } else if subtype == 18 {
                pose.scale = Vec3::splat(f32::from_bits(0x3f4c_cccd));
            }
            let key = DecoratorLibKey::new(subtype, info.rank());
            blueprint.set_positioned_colliders(
                std::sync::Arc::clone(&view.mesh_library().collision_record(key).mesh),
                std::sync::Arc::clone(&view.mesh_library().interaction_record(key).mesh),
                &pose.translation,
                &pose.rotation,
            );
            let (body_name, inside_name, local_light) = match subtype {
                16 => (
                    "decorators/lantern_wall",
                    "decorators/lantern_wall_inside",
                    [0.0, 0.0, f32::from_bits(0x3ebb_e76d)],
                ),
                17 => (
                    "decorators/lantern_terrain",
                    "decorators/lantern_terrain_inside",
                    [
                        0.0,
                        f32::from_bits(0x4041_47ae),
                        f32::from_bits(0x3f9d_70a4),
                    ],
                ),
                18 => (
                    "decorators/lantern_flat_roof",
                    "decorators/lantern_flat_roof_inside",
                    [0.0, f32::from_bits(0x3ecc_cccd), 0.0],
                ),
                _ => unreachable!(),
            };
            let semi = info.state() != 0;
            let mut body = super::cottage_wall_window::window_builder(
                DecoratorId(id),
                pose,
                body_name,
                "nani_solid_vertex_color",
                None,
                semi,
            );
            // 原terrain支架独立Uber实体，鸟停靠位置(0,3.9,0.4)。
            blueprint.visual_entities.push(body.clone());
            if subtype == 17 {
                body.mesh_name = "decorators/lantern_terrain_support";
                body.nani_name = "nani_uber";
                body.albedo_map = Some(0xe14f_8a4b_e230_cb70);
                body.sittable_for_birds = Some((u64::from(0x4079_999au32) << 32, 0x3ecc_cccd));
                blueprint.visual_entities.push(body);
            }
            let brightness = info.extra().as_lantern().0;
            let mut inside = super::cottage_wall_window::window_builder(
                DecoratorId(id),
                pose,
                inside_name,
                "nani_lantern_emissive",
                Some(country_core::resources::color_id::ColorId(info.u28)),
                semi,
            );
            inside.lantern_light_params =
                Some([local_light[0], local_light[1], local_light[2], brightness]);
            blueprint.visual_entities.push(inside);
            if subtype == 16 {
                pose.translation += Vec3::new(0.0, 0.5, 0.0);
            }
            blueprint.set_cached_transform(country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform {
                rotation: pose.rotation, translation: pose.translation, scale: pose.scale });
        }
    }
}

/// 原140790220的局部状态：GPU槽从0开始复用，释放后降序排序并pop最小槽。
/// 两份28B列表不是另一张装饰物权威表，仅保存上一帧真正点亮的查询结果。
pub struct LanternLightCache {
    old: Vec<LanternLightRecord>,
    next: Vec<LanternLightRecord>,
    free: Vec<u16>,
    gpu: Vec<country_core::resources::render_ssbo::LanternSsbo>,
}
impl Default for LanternLightCache {
    fn default() -> Self {
        Self {
            old: Vec::new(),
            next: Vec::new(),
            free: (0..0x8000u16).rev().collect(),
            gpu: vec![
                country_core::resources::render_ssbo::LanternSsbo::default();
                LIGHT_RECORD_CAP
            ],
        }
    }
}
fn record_u32(record: &LanternLightRecord, at: usize) -> u32 {
    u32::from_le_bytes(record[at..at + 4].try_into().unwrap())
}
fn record_float(record: &LanternLightRecord, at: usize) -> f32 {
    f32::from_bits(record_u32(record, at))
}
fn record_slot(record: &LanternLightRecord) -> u16 {
    u16::from_le_bytes(record[12..14].try_into().unwrap())
}
fn record_position(record: &LanternLightRecord) -> Vec3 {
    Vec3::new(
        record_float(record, 16),
        record_float(record, 20),
        record_float(record, 24),
    )
}
impl LanternLightCache {
    /// 对应两个compare_light_lists闭包：先回收消失槽，再分配新槽/更新保留槽。
    /// 不把相同ID误当完全相同记录，颜色、强度或世界位置变化均触发上传。
    fn publish_changes(
        &mut self,
        sound: &mut impl FnMut(Vec3),
    ) -> (usize, Option<std::ops::Range<usize>>) {
        self.next.sort_unstable_by_key(light_record_key);
        let mut first = usize::MAX;
        let mut last = 0;
        let mut sounds = 0;
        let (mut i, mut j) = (0, 0);
        while j < self.old.len() {
            while i < self.next.len()
                && light_record_key(&self.next[i]) < light_record_key(&self.old[j])
            {
                i += 1;
            }
            if i == self.next.len()
                || light_record_key(&self.next[i]) != light_record_key(&self.old[j])
            {
                let record = self.old[j];
                if sounds < 4 {
                    sound(record_position(&record));
                    sounds += 1;
                }
                let slot = usize::from(record_slot(&record));
                self.free.push(slot as u16);
                self.gpu[slot].intensity_color_id &= 0xffff_0000;
                first = first.min(slot);
                last = last.max(slot);
            }
            j += 1;
        }
        if first != usize::MAX {
            self.free.sort_unstable_by(|a, b| b.cmp(a));
        }
        let mut count = 0;
        let mut previous = 0;
        sounds = 0;
        for record in &mut self.next {
            let id = light_record_key(record);
            while previous < self.old.len() && light_record_key(&self.old[previous]) < id {
                previous += 1;
            }
            let old = self
                .old
                .get(previous)
                .filter(|old| light_record_key(old) == id);
            let slot = if let Some(old) = old {
                record_slot(old)
            } else {
                if sounds < 4 {
                    sound(record_position(record));
                    sounds += 1;
                }
                self.free.pop().expect("ran out of GPU light IDs")
            };
            record[12..14].copy_from_slice(&slot.to_le_bytes());
            let slot = usize::from(slot);
            count = count.max(slot + 1);
            let unchanged = old.is_some_and(|old| {
                record_u32(old, 4) == record_u32(record, 4)
                    && record_float(old, 8) == record_float(record, 8)
                    && record_float(old, 16) == record_float(record, 16)
                    && record_float(old, 20) == record_float(record, 20)
                    && record_float(old, 24) == record_float(record, 24)
            });
            if unchanged {
                continue;
            }
            self.gpu[slot] = country_core::resources::render_ssbo::LanternSsbo {
                position: record_position(record).to_array(),
                intensity_color_id: u32::from(
                    half::f16::from_f32(record_float(record, 8)).to_bits(),
                ) | (record_u32(record, 4) & 0xffff) << 16,
            };
            first = first.min(slot);
            last = last.max(slot);
        }
        std::mem::swap(&mut self.old, &mut self.next);
        (count, (first != usize::MAX).then_some(first..last + 1))
    }
}

/// 原140790400八组SystemParam；资产库在本安全宿主用ResMut取得真实实例计数写权，
/// 其余参数及顺序不变。没有storage.modified替身或随机开关。
pub fn maintain_lantern_lights(
    mut lights: Query<(
        &mut country_core::components::drawable::LanternLightParams,
        &country_core::resources::color_id::ColorId,
        Option<&country_core::components::SemiTransparent>,
    )>,
    mut library: ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,
    device: Res<country_core::render::device::GraphicsDeviceResource>,
    mut cache: Local<LanternLightCache>,
    mut sound: EventWriter<country_core::systems::audio::PlayDecoratorSoundCmd>,
    mut commands: EventReader<country_core::resources::events::LightLanternCmd>,
    session: Res<ActiveSession>,
    tod: Res<country_core::resources::time_of_day::TimeOfDay>,
) {
    let selected = commands.read().last().map(|command| command.0);
    cache.next.clear();
    for (mut params, color, preview) in &mut lights {
        let id = u32::from_le_bytes(params.0[28..32].try_into().unwrap());
        let threshold = fastrand::Rng::with_seed(utils::rng::init_rng(id)).f32() * 0.14 + 0.01;
        let on = if selected == Some(id) {
            true
        } else if preview.is_some() {
            false
        } else {
            (session.is_olden()
                && threshold * 0.2 + 0.45 <= tod.time_of_day
                && tod.time_of_day < 0.72)
                || (tod.day_night_factor < 1.0 && threshold <= tod.day_night_factor)
        };
        params.0[32] = u8::from(on);
        if !on || cache.next.len() >= LIGHT_RECORD_CAP {
            continue;
        }
        let mut record = [0u8; LIGHT_RECORD_SIZE];
        record[..4].copy_from_slice(&id.to_le_bytes());
        record[4..8].copy_from_slice(&color.0.to_le_bytes());
        record[8..12].copy_from_slice(&params.0[24..28]);
        record[16..28].copy_from_slice(&params.0[..12]);
        cache.next.push(record);
    }
    let (count, changed) = cache.publish_changes(&mut |position| {
        sound.write(country_core::systems::audio::PlayDecoratorSoundCmd::at(
            0x0e, position,
        ));
    });
    let storage = library
        .get_mut::<country_core::resources::render_ssbo::LanternSsbo>()
        .expect("called `Option::unwrap()` on a `None` value");
    if let Some(range) = changed {
        use country_core::render::ssbo::rhapsody_boundary::{GraphicsDevice, slice_as_bytes};
        let bytes = slice_as_bytes(&cache.gpu[range.clone()]);
        if range.end * 16 <= storage.buffer.byte_size() {
            // 原版丢弃Result值；发布计数不因GPU上传失败而虚构回滚。
            let _ = GraphicsDevice::update_buffer_bytes(
                device.device(),
                &storage.buffer.buffer,
                bytes,
                &[0..bytes.len()],
                range.start * 16,
            );
        }
    }
    storage.instance_count = count;
}

#[cfg(test)]
mod tests {
    use super::*;
    fn record(id: u32, color: u32, intensity: f32, position: [f32; 3]) -> LanternLightRecord {
        let mut record = [0; 28];
        for (slot, word) in record[..12]
            .chunks_exact_mut(4)
            .zip([id, color, intensity.to_bits()])
        {
            slot.copy_from_slice(&word.to_le_bytes());
        }
        for (slot, word) in record[16..28]
            .chunks_exact_mut(4)
            .zip(position.map(f32::to_bits))
        {
            slot.copy_from_slice(&word.to_le_bytes());
        }
        record
    }
    #[test]
    fn original_three_lantern_builders_reach_fullvisual_with_world_light_points_and_unload() {
        use super::super::flag::full_visual_test_support::*;
        use bevy_ecs::system::RunSystemOnce;
        use country_core::resources::walls::{
            cached_decorator_transforms::CachedDecoratorTransforms,
            decorator::{DecoratorExtraInfo, DecoratorInfo, LanternExtra},
            decorator_storage::DecoratorStorageAddr,
        };
        for (subtype, state) in (16..=18).flat_map(|subtype| [0, 1].map(|state| (subtype, state))) {
            let mut info = DecoratorInfo {
                rank: 1,
                state,
                u28: 0x1234,
                ..Default::default()
            };
            info.set_extra(DecoratorExtraInfo::Lantern(LanternExtra(0.7)));
            let mut world = scene(info, subtype, DecoratorStorageAddr::Terrain);
            let source = world.resource::<CachedDecoratorTransforms>().transform(41);
            world.run_system_once(generate_lanterns).unwrap();
            let bp = world.resource::<DecoratorBlueprints>().get(41).unwrap();
            assert!(bp.transform.is_some());
            assert_eq!(bp.visual_entities.len(), if subtype == 17 { 3 } else { 2 });
            let inside = bp.visual_entities.last().unwrap();
            let local = inside.lantern_light_params.unwrap();
            assert_eq!(local[3].to_bits(), 0.7f32.to_bits());
            let world_point =
                inside.translation + inside.rotation * Vec3::new(local[0], local[1], local[2]);
            if subtype == 16 {
                assert_ne!(inside.translation, source.translation);
                assert_eq!(
                    bp.transform.unwrap().translation.y,
                    inside.translation.y + 0.5
                );
            }
            if subtype == 18 {
                assert_eq!(inside.scale, Vec3::splat(0.8));
            }
            let entities = instantiate(&mut world);
            assert_eq!(entities.len(), if subtype == 17 { 3 } else { 2 });
            let light = world
                .get::<country_core::components::drawable::LanternLightParams>(
                    *entities.last().unwrap(),
                )
                .unwrap();
            for (index, value) in world_point.to_array().into_iter().enumerate() {
                assert_eq!(
                    u32::from_le_bytes(light.0[index * 4..index * 4 + 4].try_into().unwrap()),
                    value.to_bits()
                );
            }
            assert_eq!(
                u32::from_le_bytes(light.0[24..28].try_into().unwrap()),
                0.7f32.to_bits()
            );
            assert_eq!(u32::from_le_bytes(light.0[28..32].try_into().unwrap()), 41);
            assert_unload(&mut world, &entities);
        }
    }

    #[test]
    fn flat_roof_lantern_requires_live_flat_roof_but_accepts_terrain_and_rejects_stairs() {
        use super::super::flag::full_visual_test_support::*;
        use bevy_ecs::system::RunSystemOnce;
        use country_core::resources::{
            roofs::roof_shape::Roof,
            walls::{
                WallId,
                decorator::{DecoratorExtraInfo, DecoratorInfo, LanternExtra},
                decorator_storage::DecoratorStorageAddr,
                public_walls::PublicWalls,
            },
        };
        let mut info = DecoratorInfo {
            rank: 1,
            ..Default::default()
        };
        info.set_extra(DecoratorExtraInfo::Lantern(LanternExtra(1.)));
        let mut world = scene(info, 18, DecoratorStorageAddr::Wall(WallId(7)));
        world.run_system_once(generate_lanterns).unwrap();
        assert!(world.resource::<DecoratorBlueprints>().map.is_empty());
        let roof = Roof::new(
            7u64.to_le_bytes(),
            [0; 28],
            [0; 28],
            [0; 4],
            [0; 4],
            0,
            [0; 12],
        );
        assert!(roof.is_flat());
        let entity = world.spawn(roof).id();
        world
            .resource_mut::<PublicWalls>()
            .associate_roof(WallId(7), entity);
        world.run_system_once(generate_lanterns).unwrap();
        assert!(world.resource::<DecoratorBlueprints>().get(41).is_some());
        let mut stair = scene(info, 18, DecoratorStorageAddr::Stair);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| stair
                .run_system_once(generate_lanterns)
                .unwrap()))
            .is_err()
        );
    }

    #[test]
    fn native_light_slot_callback_bits() {
        type Case = ([u32; 7], [u32; 7], u8, [u32; 4], usize, [usize; 2], u16);
        let cases: &[Case] = include!(
            "../../../../../reports/module-recovery-2026-10-23/m01-lantern-slots-native-cases.rs"
        );
        for (index, (old, new, only, expected, count, bounds, slot)) in cases.iter().enumerate() {
            let bytes = |words: &[u32; 7]| {
                let mut value = [0; 28];
                for (to, from) in value.chunks_exact_mut(4).zip(words) {
                    to.copy_from_slice(&from.to_le_bytes());
                }
                value
            };
            let mut cache = LanternLightCache::default();
            cache.free.clear();
            cache.free.push(5);
            if *only == 0 {
                cache.old.push(bytes(old));
            }
            cache.next.push(bytes(new));
            let (actual_count, range) = cache.publish_changes(&mut |_| {});
            let gpu = cache.gpu[5];
            let got = [
                gpu.position[0].to_bits(),
                gpu.position[1].to_bits(),
                gpu.position[2].to_bits(),
                gpu.intensity_color_id,
            ];
            assert_eq!(got, *expected, "原槽callback GPU字：case {index}");
            assert_eq!(actual_count, *count, "原槽callback计数：case {index}");
            let actual_bounds = range.map_or([usize::MAX, 0], |r| [r.start, r.end - 1]);
            assert_eq!(actual_bounds, *bounds, "原槽callback脏范围：case {index}");
            assert_eq!(record_slot(&cache.old[0]), *slot);
        }
    }

    #[test]
    fn original_cache_reuses_slots_keeps_live_highwater_and_zeroes_only_removed_intensity() {
        let mut cache = LanternLightCache::default();
        let mut sounds = Vec::new();
        cache.next.extend([
            record(8, 9, 1.5, [1., 2., 3.]),
            record(2, 3, 0.25, [4., 5., 6.]),
        ]);
        assert_eq!(
            cache.publish_changes(&mut |p| sounds.push(p)),
            (2, Some(0..2))
        );
        assert_eq!(
            cache.old.iter().map(record_slot).collect::<Vec<_>>(),
            [0, 1]
        );
        assert_eq!(cache.gpu[0].position, [4., 5., 6.]);
        assert_eq!(cache.gpu[1].intensity_color_id, 0x0009_3e00);
        assert_eq!(sounds, [Vec3::new(4., 5., 6.), Vec3::new(1., 2., 3.)]);
        cache.next.clear();
        cache.next.push(record(8, 9, 1.5, [1., 2., 3.]));
        sounds.clear();
        assert_eq!(
            cache.publish_changes(&mut |p| sounds.push(p)),
            (2, Some(0..1))
        );
        assert_eq!(cache.gpu[0].intensity_color_id, 0x0003_0000);
        assert_eq!(cache.gpu[0].position, [4., 5., 6.]);
        assert_eq!(sounds, [Vec3::new(4., 5., 6.)]);
        cache.next.clear();
        cache.next.extend([
            record(8, 9, 1.5, [1., 2., 3.]),
            record(10, 0x10011, 2., [7., 8., 9.]),
        ]);
        assert_eq!(cache.publish_changes(&mut |_| {}), (2, Some(0..1)));
        assert_eq!(record_slot(&cache.old[1]), 0);
        assert_eq!(cache.gpu[0].intensity_color_id, 0x0011_4000);
        cache.next.clear();
        cache.next.extend(cache.old.iter().copied());
        assert_eq!(
            cache.publish_changes(&mut |_| panic!("相同列表不发声音")),
            (2, None)
        );
        cache.next.clear();
        assert_eq!(cache.publish_changes(&mut |_| {}), (0, Some(0..2)));
        assert_eq!(cache.free.len(), 0x8000);
    }
    #[test]
    fn original_same_id_color_intensity_position_and_nan_all_drive_differential_upload() {
        let mut cache = LanternLightCache::default();
        let first = record(1, 2, 3., [4., 5., 6.]);
        cache.next.push(first);
        cache.publish_changes(&mut |_| {});
        for (at, word) in [
            (4, 7),
            (8, 4f32.to_bits()),
            (16, 9f32.to_bits()),
            (20, (-0f32).to_bits()),
            (24, 0x7fc01234),
        ] {
            cache.next.clear();
            let mut changed = first;
            changed[at..at + 4].copy_from_slice(&word.to_le_bytes());
            cache.next.push(changed);
            assert_eq!(
                cache.publish_changes(&mut |_| panic!("保留ID不发声音")),
                (1, Some(0..1))
            );
        }
        cache.next.clear();
        cache.next.extend(cache.old.iter().copied());
        assert_eq!(
            cache.publish_changes(&mut |_| {}),
            (1, Some(0..1)),
            "NaN不能被按字节相同快捷跳过"
        );
    }
    #[test]
    fn original_sound_limits_are_independent_for_removal_and_insertion_passes() {
        let mut cache = LanternLightCache::default();
        cache
            .next
            .extend((0..8).map(|id| record(id, 0, 1., [id as f32, 0., 0.])));
        let mut sounds = Vec::new();
        cache.publish_changes(&mut |p| sounds.push(p));
        assert_eq!(sounds.len(), 4);
        cache.next.clear();
        cache
            .next
            .extend((8..16).map(|id| record(id, 0, 1., [id as f32, 0., 0.])));
        sounds.clear();
        cache.publish_changes(&mut |p| sounds.push(p));
        assert_eq!(
            sounds.iter().map(|p| p.x as u32).collect::<Vec<_>>(),
            [0, 1, 2, 3, 8, 9, 10, 11]
        );
    }
}
