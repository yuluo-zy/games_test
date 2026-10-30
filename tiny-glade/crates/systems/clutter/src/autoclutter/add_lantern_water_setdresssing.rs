//! add_lantern_water_setdresssing（B 主基线；0x1422d2fa0，size 2096）。
//!
//! 2026-10-17 restorer-clutter-remaining：按 fresh 反编译 + 逐指令反汇编重建
//! （`evidence/decomp/0x1422d2fa0_*.c`、`evidence/disasm5_2.txt`、常量实读
//! `0x142a7fdf4=0.65`、`0x1429ddce8=-0.05`、`0x142a7fa50=-0.8`、
//! `0x1429eb6f8=-0.1`、`0x1429259b0=1.3`、`0x1429cd128=1.2`、`0x142925984=0.1`、
//! `0x142925940=(0.25,0.25)`、`0x142925988=5.0`、`0x142a16cb8=0.8`）。
//!
//! 控制流（O）：
//! 1. `TweakerView::as_inner_mut` 借 WaterRaster（+0x298 双缓冲选择）；
//!    `GladeType::is_winter`（ICF 别名）→ 早退。
//! 2. `DecoratorStorage::iter_terrain_decorators`：每装饰物
//!    先按原指令 `(DecoratorType & 0x3e) == 0x3c && flags == 0` 过滤，
//!    `CachedDecoratorTransforms::transform` 只取水平位置 →
//!    `Circle2d::new(center.xz, 0.65)` → `init_rng(装饰物 id)`。
//! 3. 内层 **5 次**：`u = rem_euclid(f32*0.1 + i/5, 1)` → `get_pos_at_u(u)`；
//!    两次 `f32` 抖动 ×0.25 加到位（0x142925940=(0.25,0.25)）；4 角 bilinear（130.0）
//!    采样；**`0.0 < sample` 才放**（入水门）。
//! 4. 入水后：两次 `f32` 归一化（1/SQRT）得方向；键
//!    `calculate_hash<tuple<u32,usize,&str>>`（i、装饰物 id、"lantern" len 7）。
//! 5. 偏移/缩放三路（O）：
//!    - autumn → y = i/5 × -0.05 + base，factor 0.8；
//!    - 非 autumn 且键偶 → y = i/5 × -0.1 + base，factor 1.3；
//!    - 否则 y = -0.8，factor 0.8。
//!    原 `x0y` 将 base 初始化为 `0.0`，随后每轮更新（累加）；不是装饰物 y。
//! 6. `f32` 抽取 → 缩放 `factor × (r×1.2 + (1-r)×0.8)`；写输出候选 stride 0x68
//!    （+0=5、+8 prefab、+0x18=2、+0x1c 键、+0x30=5、+0x38 位置、+0x60 缩放）。
//!
//! PDB 中 `DragState::is_started` 与 `GladeType::is_autumn` 共址，
//! `BackupSaveFileKind::is_corrupted` 与 `GladeType::is_winter` 共址；接收者都是
//! `ActiveSession+0x58`，因此生产参数必须是 `ActiveSession`。输出 mesh 为
//! `ClutterMeshes` fixed enum 槽 Reed/Lilypad/FallenLeaves（native +0x100/+0x108/
//! +0x110，见 direct-producer-comparison-v1/COMPARISON.md）。

use bevy_ecs::prelude::*;
use country_core::resources::{
    clutter_meshes::{ClutterMeshTy, ClutterMeshes},
    session::ActiveSession,
    walls::{
        cached_decorator_transforms::CachedDecoratorTransforms,
        decorator_storage::DecoratorStorage,
    },
};
use glam::{Vec2, Vec3};

use utils::geometry::circle::Circle2d;

use super::{
    AutoClutterPayload, AutoClutterSpawn, AutoClutterState, ResolvedAutoClutterObject,
    ResolvedClutterMesh,
};

/// 圆半径（O，0x142a7fdf4 = 0.65）。
pub const LANTERN_CIRCLE_RADIUS: f32 = 0.65;
/// 内层迭代数（O，0x142925988 = 5.0）。
pub const LANTERN_ITERATIONS: usize = 5;

/// add_lantern_water_setdresssing（0x1422d2fa0）：地形装饰物周围 5 个圆周 jitter 落位，
/// **入水才放**（type 5 灯笼）。
pub fn add_lantern_water_setdresssing(
    mut state: ResMut<AutoClutterState>,
    meshes: Res<ClutterMeshes>,
    water: Res<country_core::resources::world_raster_defs::WaterRaster>,
    decorators: Res<DecoratorStorage>,
    cached: Res<CachedDecoratorTransforms>,
    session: Res<ActiveSession>,
) {
    let w = (*water).as_ref();
    if session.is_winter() {
        return;
    }
    let autumn = session.is_autumn();
    for bucket in decorators.iter_terrain_decorators() {
        for id in bucket.iter_modified_ids() {
            let info = bucket
                .get(country_core::resources::walls::decorator::DecoratorId(id))
                .expect("modified terrain decorator is missing from its authoritative map");
            // O@1422d336f..3388:
            // `(DecoratorType & 0x3e) == 0x3c && DecoratorInfo::state == 0`.
            if info.ty & 0x3e != 0x3c || info.state != 0 {
                continue;
            }
            let transform = cached.transform(id);
            let pos = transform.translation;
            let circle = Circle2d::new(Vec2::new(pos.x, pos.z), LANTERN_CIRCLE_RADIUS);
            let rng = lantern_rng(id as u64);
            // O@1422d3611: `Vec2::x0y` writes `(x, 0, z)` to the position
            // scratch. The vertical accumulator starts at its zero component.
            let mut base_y = 0.0;
            for i in 0..LANTERN_ITERATIONS {
                let base = i as f32 / LANTERN_ITERATIONS as f32;
                // O：fmodf(...,1.0) 且负值 +1.0 = rem_euclid。
                let u = (rng.f32() * 0.1 + base).rem_euclid(1.0);
                let point = circle.get_pos_at_u(u);
                // O：两次 f32 抖动 ×0.25 加到位（0x142925940=(0.25,0.25)）。
                let jitter = Vec2::new(rng.f32(), rng.f32()) * 0.25;
                let point = point + jitter;
                // O：4 角 bilinear（130.0）采样，`0.0 < sample` 才放。
                let sample = w.bilinear_sample_at_world_xz(point.x, point.y, 130.0, 130.0);
                if !(0.0 < sample) {
                    continue;
                }
                // O：两次 f32 → 1/SQRT 归一化方向，随后 x0y 写入 +0x54..+0x5f。
                let nx = rng.f32();
                let nz = rng.f32();
                let inv = 1.0 / (nx * nx + nz * nz).sqrt();
                let direction = Vec2::new(nx * inv, nz * inv);
                // O：键 tuple<u32,usize,&str>（i、装饰物 id、"lantern"）。
                let key = hash_lantern_key(i, id as u64);
                let mesh_ty = lantern_setdressing_mesh_ty(key, autumn);
                let (y, factor) = if autumn {
                    (base_y + base * -0.05, 0.8f32)
                } else if key & 1 == 0 {
                    (base_y + base * -0.1, 1.3f32)
                } else {
                    (-0.8, 0.8f32)
                };
                let mesh = meshes.mesh(mesh_ty);
                // O：base 每轮更新（累加）。
                base_y = y;
                // O：缩放 `factor*(r*1.2+(1-r)*0.8)`。
                let r = rng.f32();
                let scale = factor * (r * 1.2 + (1.0 - r) * 0.8);
                let position = Vec3::new(point.x, y, point.y);
                state.spawn_candidates.push(AutoClutterSpawn {
                    kind: 5,
                    // Native tag 5 carries no active source payload; the
                    // manager still routes by selector 5 for Plant Nani.
                    payload: AutoClutterPayload::Object(ResolvedAutoClutterObject {
                        metadata: ResolvedClutterMesh::from(mesh),
                        source: country_core::components::drawable::AutoClutterSource::None,
                        destination: country_core::components::drawable::AutoClutterDestination::Terrain,
                    }),
                    f18: 2,
                    key,
                    marker: 5,
                    selector_value: 0,
                    position,
                    host_id: id as u64,
                    f48: u64::from(position.x.to_bits())
                        | (u64::from(position.y.to_bits()) << 32),
                    f50: u64::from(position.z.to_bits()),
                    f54: direction.x.to_bits(),
                    f58: 0.0f32.to_bits(),
                    f5c: direction.y.to_bits(),
                    scale,
                });
            }
        }
    }
}

/// O@1422d3401 passes the 32-bit decorator id to init_rng (0x140c96240);
/// its returned state initializes fastrand directly.
fn lantern_rng(decorator_id: u64) -> fastrand::Rng {
    fastrand::Rng::with_seed(utils::rng::init_rng(decorator_id as u32))
}

fn lantern_setdressing_mesh_ty(key: u64, autumn: bool) -> ClutterMeshTy {
    if autumn {
        ClutterMeshTy::FallenLeaves
    } else if key & 1 == 0 {
        ClutterMeshTy::Lilypad
    } else {
        ClutterMeshTy::Reed
    }
}

/// 放置键（`calculate_hash<tuple<u32,usize,&str>>` 0x141cf7980 语义）。
pub fn hash_lantern_key(i: usize, decorator_id: u64) -> u64 {
    utils::calculate_hash(&((i as u32), decorator_id, "lantern"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::schedule::Schedule;
    use country_core::{
        resources::{
            session::{GladeType, SessionId, SessionMeta},
            walls::{
                cached_decorator_transforms::CachedDecoratorTransform,
                decorator::{DecoratorId, DecoratorInfo},
                decorator_storage::DecoratorStorageAddr,
            },
            world_raster::DirtyTilePublisher,
        },
    };

    fn meshes() -> ClutterMeshes {
        use country_core::resources::clutter_meshes::ClutterMesh;
        fn fixed_slot(ty: ClutterMeshTy) -> usize {
            let tag = ty as usize;
            if tag < ClutterMeshTy::WindowFlowerbeds as usize {
                tag
            } else {
                tag - 1
            }
        }
        let mut fixed = std::array::from_fn(|_| {
            ClutterMesh::omitted_in_demo("test/other", 0, None, false, false)
        });
        fixed[fixed_slot(ClutterMeshTy::Reed)] =
            ClutterMesh::omitted_in_demo("reed", 0, None, false, false);
        fixed[fixed_slot(ClutterMeshTy::Lilypad)] =
            ClutterMesh::omitted_in_demo("lilypad", 0, None, false, false);
        fixed[fixed_slot(ClutterMeshTy::FallenLeaves)] =
            ClutterMesh::omitted_in_demo("fallen_leaves", 0, None, false, false);
        ClutterMeshes::from_parts(fixed, vec![])
    }

    fn full_water() -> country_core::resources::world_raster_defs::WaterRaster {
        let mut raster = country_core::resources::world_raster_defs::WaterRaster::new(
            DirtyTilePublisher::unique(),
        );
        raster.write_composite(Some(&vec![1u8; 512 * 512]));
        raster
    }

    fn run(world: &mut World) {
        let mut schedule = Schedule::default();
        schedule.add_systems(add_lantern_water_setdresssing);
        schedule.run(world);
    }

    fn assert_mesh_names(state: &AutoClutterState, expected: impl Fn(u64) -> &'static str) {
        assert_eq!(state.spawn_candidates.len(), LANTERN_ITERATIONS);
        for spawn in &state.spawn_candidates {
            let AutoClutterPayload::Object(object) = &spawn.payload else {
                panic!("lantern producer must emit a resolved mesh");
            };
            assert_eq!(object.metadata.mesh_name, expected(spawn.key));
        }
    }

    #[test]
    fn fixed_catalog_slots_follow_the_original_direct_producer() {
        assert_eq!(lantern_setdressing_mesh_ty(1, false), ClutterMeshTy::Reed);
        assert_eq!(lantern_setdressing_mesh_ty(0, false), ClutterMeshTy::Lilypad);
        assert_eq!(
            lantern_setdressing_mesh_ty(0, true),
            ClutterMeshTy::FallenLeaves
        );
    }

    #[test]
    fn native_lantern_seed_reads_u32_once_and_does_not_remix_the_state() {
        assert_eq!(
            lantern_rng(0x1_0000_0007).get_seed(),
            lantern_rng(7).get_seed()
        );
        let rng = lantern_rng(0x1_0000_0007);
        let bits: Vec<_> = (0..8).map(|_| rng.f32().to_bits()).collect();
        assert_eq!(
            bits,
            [
                0x3f59_0fb6,
                0x3f37_ccc8,
                0x3e93_01b4,
                0x3e03_0f98,
                0x3f36_b640,
                0x3e82_ce0c,
                0x3f77_97e2,
                0x3e75_be48,
            ]
        );
    }

    #[test]
    fn actual_decorator_cache_and_session_owners_publish_the_three_original_slots() {
        let id = 7;
        let mut storage = DecoratorStorage::default();
        storage
            .get_or_insert_default(DecoratorStorageAddr::Terrain)
            .insert(
                DecoratorId(id),
                DecoratorInfo {
                    ty: 0x3c,
                    state: 0,
                    ..DecoratorInfo::default()
                },
            );
        let mut cached = CachedDecoratorTransforms::default();
        cached.insert(
            id,
            CachedDecoratorTransform {
                rotation: glam::Quat::IDENTITY,
                translation: Vec3::ZERO,
                scale: Vec3::ONE,
            },
        );

        let mut world = World::new();
        world.init_resource::<AutoClutterState>();
        world.insert_resource(meshes());
        world.insert_resource(full_water());
        world.insert_resource(storage);
        world.insert_resource(cached);
        world.insert_resource(ActiveSession::new(
            SessionId::new(),
            SessionMeta {
                glade: GladeType::Summer,
                ..Default::default()
            },
        ));

        run(&mut world);
        let state = world.resource::<AutoClutterState>();
        assert_mesh_names(state, |key| {
            let ty = lantern_setdressing_mesh_ty(key, false);
            match ty {
                ClutterMeshTy::Reed => "reed",
                ClutterMeshTy::Lilypad => "lilypad",
                other => panic!("unexpected summer lantern slot {other:?}"),
            }
        });
        assert!(state.spawn_candidates.iter().all(|spawn| {
            let direction = Vec3::new(
                f32::from_bits(spawn.f54),
                f32::from_bits(spawn.f58),
                f32::from_bits(spawn.f5c),
            );
            direction.is_finite() && direction != Vec3::ZERO
        }));
        world.insert_resource(country_core::resources::time::GameTime::default());
        let mut manage = Schedule::default();
        manage.add_systems(super::super::manage_autoclutter_entities::manage_autoclutter_entities);
        manage.run(&mut world);
        let mut managed = world.query::<(
            &country_core::components::drawable::AutoClutter,
            &country_core::utils::components::transform::Transform,
            &country_core::render::nani::gpu_world::InstanceDataMarker<
                country_core::render::nani::instance_data::PlantInstanceData,
            >,
        )>();
        let rows: Vec<_> = managed.iter(&world).collect();
        assert_eq!(rows.len(), LANTERN_ITERATIONS);
        assert!(rows.iter().all(|(_, transform, _)| transform.rotation.is_finite()));
        let mut convertible = world.query::<&country_core::components::drawable::AutoClutterConvertableToDecoClutter>();
        assert_eq!(convertible.iter(&world).count(), 0);

        world.resource_mut::<ActiveSession>().meta.glade = GladeType::Autumn;
        world.resource_mut::<AutoClutterState>().spawn_candidates.clear();
        run(&mut world);
        assert_mesh_names(world.resource::<AutoClutterState>(), |_| "fallen_leaves");

        world.resource_mut::<ActiveSession>().meta.glade = GladeType::Winter;
        world.resource_mut::<AutoClutterState>().spawn_candidates.clear();
        run(&mut world);
        assert!(world.resource::<AutoClutterState>().spawn_candidates.is_empty());
    }

    #[test]
    fn original_type_and_state_filter_reject_non_water_setdressing_decorators() {
        let mut world = World::new();
        world.init_resource::<AutoClutterState>();
        world.insert_resource(meshes());
        world.insert_resource(full_water());
        let mut storage = DecoratorStorage::default();
        let terrain = storage.get_or_insert_default(DecoratorStorageAddr::Terrain);
        for (id, ty, state) in [
            (1, 0x3c, 0),
            (2, 0x3d, 0),
            (3, 0x3e, 0),
            (4, 0x3b, 0),
            (5, 0x3c, 1),
        ] {
            terrain.insert(
                DecoratorId(id),
                DecoratorInfo {
                    ty,
                    state,
                    ..DecoratorInfo::default()
                },
            );
        }
        world.insert_resource(storage);
        let mut cached = CachedDecoratorTransforms::default();
        for id in 1..=5 {
            cached.insert(
                id,
                CachedDecoratorTransform {
                    rotation: glam::Quat::IDENTITY,
                    translation: Vec3::ZERO,
                    scale: Vec3::ONE,
                },
            );
        }
        world.insert_resource(cached);
        world.insert_resource(ActiveSession::new(
            SessionId::new(),
            SessionMeta {
                glade: GladeType::Summer,
                ..Default::default()
            },
        ));

        run(&mut world);
        let hosts: std::collections::HashSet<_> = world
            .resource::<AutoClutterState>()
            .spawn_candidates
            .iter()
            .map(|spawn| spawn.host_id)
            .collect();
        // (ty & 0x3e) == 0x3c accepts 0x3c/0x3d. 0x3e does not match, and
        // the state==0 check rejects id 5.
        assert_eq!(hosts, std::collections::HashSet::from([1, 2]));
        assert_eq!(
            world.resource::<AutoClutterState>().spawn_candidates.len(),
            LANTERN_ITERATIONS * 2
        );
    }

    #[test]
    fn cached_vertical_translation_does_not_replace_the_native_zero_base() {
        let mut world = World::new();
        world.init_resource::<AutoClutterState>();
        world.insert_resource(meshes());
        world.insert_resource(full_water());
        let mut storage = DecoratorStorage::default();
        storage
            .get_or_insert_default(DecoratorStorageAddr::Terrain)
            .insert(
                DecoratorId(7),
                DecoratorInfo {
                    ty: 0x3c,
                    state: 0,
                    ..DecoratorInfo::default()
                },
            );
        world.insert_resource(storage);
        let mut cached = CachedDecoratorTransforms::default();
        cached.insert(
            7,
            CachedDecoratorTransform {
                rotation: glam::Quat::IDENTITY,
                translation: Vec3::new(12.0, 37.0, -9.0),
                scale: Vec3::ONE,
            },
        );
        world.insert_resource(cached);
        world.insert_resource(ActiveSession::new(
            SessionId::new(),
            SessionMeta {
                glade: GladeType::Summer,
                ..Default::default()
            },
        ));

        run(&mut world);
        let candidates = &world.resource::<AutoClutterState>().spawn_candidates;
        assert_eq!(candidates.len(), LANTERN_ITERATIONS);
        for spawn in candidates {
            let transform = world.resource::<CachedDecoratorTransforms>().transform(7);
            let distance = glam::Vec2::new(
                spawn.position.x - transform.translation.x,
                spawn.position.z - transform.translation.z,
            )
            .length();
            assert!(distance.is_finite());
            assert!(
                spawn.position.y.is_finite()
                    && spawn.position.y >= -1.5
                    && spawn.position.y <= 0.0
            );
        }
    }
}
