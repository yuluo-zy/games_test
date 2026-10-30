//! add_tree_vegetation_setdressing（B 主基线；0x1422d3800，size 2566）。
//!
//! 2026-10-17 restorer-clutter-remaining：按 fresh 反编译 + 逐指令反汇编重建
//! （`evidence/decomp/0x1422d3800_*.c`、`evidence/disasm6_0.txt`；常量实读
//! `0x142a7c88c=0.9`、`0x142925984=0.1`、`0x1429cd128=1.2`、`0x142a16cb8=0.8`、
//! `0x1429ddce8=-0.05`、`0x1429eb6f8=-0.1`、`0x142a7fa50=-0.8`、`0x1429259b0=1.3`、
//! `0x142925970=0.4`）。
//!
//! 控制流（O）：
//! - 每树：`PlacementDataSsbo::position` ×2 取水平位置；两处 `x0y` 把垂直
//!   基准初始化为 `0.0`。flowery/autumn/winter 季节门控，其中 flowery 分支
//!   还要求树记录 kind(+0x2c)==2。
//! - 半径 = `树 scale(+0xc) + 0.9`，且 autumn 或 winter 时再 +0.4。
//! - `Circle2d::new(center.xz, radius)` + `init_rng(树 id)`。
//! - 内层 **12 次**：`u = rem_euclid(i/12 + f32×0.1, 1)` → `get_pos_at_u`；
//!   两次 `f32` 抖动（flowery/autumn/winter 时 ×1，否则 ×0.5）加到位；
//!   4 角 bilinear（130.0）采样，`0.0 < sample` 才放。
//! - 入水后两次 `f32` 归一化方向；键 `calculate_hash<tuple<TreeStorageIndex,usize,&str>>`
//!   （树 id、i、"tree" len 4）。
//! - 偏移/缩放（O）：
//!   - autumn/winter 或 flowery → y = i/12 × -0.05 + base，factor 0.8；
//!   - 否则默认 y = -0.8、factor 0.8；键偶 → y = i/12 × -0.1 + base，factor 1.3。
//!   base 每轮更新。
//! - `f32` 抽取 → 缩放 `factor × (r×1.2 + (1-r)×0.8)`；写输出候选 stride 0x68。
//!
//! 资源身份以 PDB 符号和接收者共同核对：三个 `7 B` 共址符号实际是
//! `GladeType::{is_flowery,is_autumn,is_winter}`，调用者是 `ActiveSession+0x58`。
//! 树位置/缩放来自 canonical `PrefabInstanceData`，输出 mesh 来自 `ClutterMeshes`
//! 的 fixed enum 槽 Reed/Lilypad/FallenLeaves/FallenPetals（native +0x100/+0x108/
//! +0x110/+0x118，见 direct-producer-comparison-v1/COMPARISON.md）。

use bevy_ecs::prelude::*;
use country_core::{
    components::TreeComponent,
    resources::{
        clutter_meshes::{ClutterMeshTy, ClutterMeshes},
        session::ActiveSession,
    },
    systems::{prefab::PrefabInstanceData, ui::signifiers::UiSignifierStream},
};
use glam::{Vec2, Vec3};

use utils::geometry::circle::Circle2d;

use super::{
    AutoClutterPayload, AutoClutterSpawn, AutoClutterState, ResolvedAutoClutterObject,
    ResolvedClutterMesh,
};

/// 内层迭代数（O：i/12.0）。
pub const TREE_ITERATIONS: usize = 12;

/// add_tree_setdressing（0x1422d3800）：每树 12 个圆周 jitter 落位，入水才放（type 5）。
pub fn add_tree_setdressing(
    mut state: ResMut<AutoClutterState>,
    _signifiers: Res<UiSignifierStream>,
    meshes: Res<ClutterMeshes>,
    water: Res<country_core::resources::world_raster_defs::WaterRaster>,
    trees: Query<(&TreeComponent, &PrefabInstanceData)>,
    session: Res<ActiveSession>,
) {
    let w = (*water).as_ref();
    let autumn = session.is_autumn();
    let winter = session.is_winter();
    for (tree, prefab) in &trees {
        let placement = prefab.0;
        // `SpawnPrefabCmd.mode` is copied into PlacementDataSsbo+0x2c. The
        // original tree branch compares that native kind word with 2.
        let flowery_tree = session.is_flowery() && placement.pad0 == 2;
        // O：半径 = scale + 0.9（0x142a7c88c），autumn/winter 再 +0.4。
        let radius = tree_radius(placement.scale, flowery_tree, autumn, winter);
        let circle = Circle2d::new(
            Vec2::new(placement.position.x, placement.position.z),
            radius,
        );
        let tree_rng = tree_rng(tree.id);
        // O@1422d3f5b..3f65: `Vec2::x0y` writes `(x, 0, z)` to the position
        // scratch. The vertical accumulator starts at its zero component.
        let mut base_y = 0.0;
        for i in 0..TREE_ITERATIONS {
            let base = i as f32 / TREE_ITERATIONS as f32;
            let u = (base + tree_rng.f32() * 0.1).rem_euclid(1.0);
            let point = circle.get_pos_at_u(u);
            // O：两次 f32 抖动；flowery-tree、autumn、winter 时 ×1，否则 ×0.5。
            let jitter_scale = if flowery_tree || autumn || winter {
                1.0
            } else {
                0.5
            };
            let jitter = Vec2::new(tree_rng.f32(), tree_rng.f32()) * jitter_scale;
            let point = point + jitter;
            let s = w.bilinear_sample_at_world_xz(point.x, point.y, 130.0, 130.0);
            if !(s > 0.0) {
                continue;
            }
            // O：两次 f32 → 1/SQRT 归一化方向。
            let nx = tree_rng.f32();
            let nz = tree_rng.f32();
            let inv = 1.0 / (nx * nx + nz * nz).sqrt();
            let direction = Vec2::new(nx * inv, nz * inv);
            let key = hash_tree_key(tree.id, i);
            let mesh_ty = tree_setdressing_mesh_ty(key, flowery_tree, autumn, winter);
            let (y, factor) = if autumn || winter {
                let y = if winter {
                    base_y + base * -0.05 - 0.1
                } else {
                    base_y + base * -0.05
                };
                (y, 0.8f32)
            } else if flowery_tree {
                (base_y + base * -0.05, 0.8f32)
            } else if key & 1 == 0 {
                (base_y + base * -0.1, 1.3f32)
            } else {
                (-0.8, 0.8f32)
            };
            let mesh = meshes.mesh(mesh_ty);
            base_y = y;
            let rr = tree_rng.f32();
            let scale = factor * (rr * 1.2 + (1.0 - rr) * 0.8);
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
                host_id: tree.id,
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

/// O@1422d3d25 reads dword [tree], @1422d3d27 mixes it once via
/// init_rng(u32), and the returned state initializes fastrand directly.
fn tree_rng(tree_id: u64) -> fastrand::Rng {
    fastrand::Rng::with_seed(utils::rng::init_rng(tree_id as u32))
}

/// `Circle2d` radius at 0x1422d3c6c..d00: base+0.4 for a flowery kind-2
/// tree and for either non-flowery seasonal branch (autumn or winter).
fn tree_radius(scale: f32, flowery_tree: bool, autumn: bool, winter: bool) -> f32 {
    let radius = scale + 0.9;
    if flowery_tree || autumn || winter {
        radius + 0.4
    } else {
        radius
    }
}

fn tree_setdressing_mesh_ty(
    key: u64,
    flowery_tree: bool,
    autumn: bool,
    winter: bool,
) -> ClutterMeshTy {
    if autumn || winter {
        ClutterMeshTy::FallenLeaves
    } else if flowery_tree {
        ClutterMeshTy::FallenPetals
    } else if key & 1 == 0 {
        ClutterMeshTy::Lilypad
    } else {
        ClutterMeshTy::Reed
    }
}

/// 放置键（`calculate_hash<tuple<TreeStorageIndex,usize,&str>>` 0x141cf7600 语义）。
pub fn hash_tree_key(tree_index: u64, i: usize) -> u64 {
    utils::calculate_hash(&(tree_index, i, "tree"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::schedule::Schedule;
    use country_core::{
        resources::{
            placement_data::PlacementDataSsbo,
            session::{GladeType, SessionId, SessionMeta},
            world_raster::DirtyTilePublisher,
        },
        systems::prefab::PrefabInstanceData,
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
        fixed[fixed_slot(ClutterMeshTy::FallenPetals)] =
            ClutterMesh::omitted_in_demo("fallen_petals", 0, None, false, false);
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
        schedule.add_systems(add_tree_setdressing);
        schedule.run(world);
    }

    fn assert_mesh_names(state: &AutoClutterState, expected: impl Fn(u64) -> &'static str) {
        assert_eq!(state.spawn_candidates.len(), TREE_ITERATIONS);
        for spawn in &state.spawn_candidates {
            let AutoClutterPayload::Object(object) = &spawn.payload else {
                panic!("tree producer must emit a resolved mesh");
            };
            assert_eq!(object.metadata.mesh_name, expected(spawn.key));
        }
    }

    #[test]
    fn fixed_catalog_slots_follow_the_original_direct_producer() {
        assert_eq!(
            tree_setdressing_mesh_ty(1, false, false, false),
            ClutterMeshTy::Reed
        );
        assert_eq!(
            tree_setdressing_mesh_ty(0, false, false, false),
            ClutterMeshTy::Lilypad
        );
        assert_eq!(
            tree_setdressing_mesh_ty(0, false, true, false),
            ClutterMeshTy::FallenLeaves
        );
        assert_eq!(
            tree_setdressing_mesh_ty(0, false, false, true),
            ClutterMeshTy::FallenLeaves
        );
        assert_eq!(
            tree_setdressing_mesh_ty(0, true, false, false),
            ClutterMeshTy::FallenPetals
        );
    }

    #[test]
    fn flowery_and_seasonal_radius_use_the_original_plus_point_four_branch() {
        assert_eq!(tree_radius(1.0, false, false, false).to_bits(), 1.9f32.to_bits());
        for (flowery, autumn, winter) in [
            (true, false, false),
            (false, true, false),
            (false, false, true),
        ] {
            assert_eq!(
                tree_radius(1.0, flowery, autumn, winter).to_bits(),
                2.3f32.to_bits()
            );
        }
    }

    #[test]
    fn native_tree_seed_reads_u32_once_and_does_not_remix_the_state() {
        assert_eq!(
            tree_rng(0x1_0000_0001).get_seed(),
            tree_rng(1).get_seed()
        );
        let rng = tree_rng(0x1_0000_0001);
        let bits: Vec<_> = (0..8).map(|_| rng.f32().to_bits()).collect();
        assert_eq!(
            bits,
            [
                0x3f50_36d4,
                0x3e93_c8f4,
                0x3f6f_460a,
                0x3ebc_b498,
                0x3e9b_4aa4,
                0x3efc_6560,
                0x3ea9_2a90,
                0x3ecf_04c0,
            ]
        );
    }

    #[test]
    fn actual_tree_and_session_owners_publish_the_four_original_slots() {
        let tree_id = 0x1234;
        let mut world = World::new();
        world.init_resource::<AutoClutterState>();
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(meshes());
        world.insert_resource(full_water());
        world.insert_resource(ActiveSession::new(
            SessionId::new(),
            SessionMeta {
                glade: GladeType::Summer,
                ..Default::default()
            },
        ));
        let tree = world
            .spawn((
                TreeComponent {
                    id: tree_id,
                    is_preplaced: false,
                },
                PrefabInstanceData(PlacementDataSsbo {
                    position: Vec3::new(18.0, 29.0, -14.0),
                    scale: 1.0,
                    ..Default::default()
                }),
            ))
            .id();

        run(&mut world);
        let state = world.resource::<AutoClutterState>();
        assert_mesh_names(state, |key| {
            let ty = tree_setdressing_mesh_ty(key, false, false, false);
            match ty {
                ClutterMeshTy::Reed => "reed",
                ClutterMeshTy::Lilypad => "lilypad",
                other => panic!("unexpected summer tree slot {other:?}"),
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
        assert_eq!(rows.len(), TREE_ITERATIONS);
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
        assert_mesh_names(world.resource::<AutoClutterState>(), |_| "fallen_leaves");

        world.resource_mut::<ActiveSession>().meta.glade = GladeType::Flowery;
        world.get_mut::<PrefabInstanceData>(tree).unwrap().0.pad0 = 2;
        world.resource_mut::<AutoClutterState>().spawn_candidates.clear();
        run(&mut world);
        assert_mesh_names(world.resource::<AutoClutterState>(), |_| "fallen_petals");
    }

    #[test]
    fn placement_height_does_not_replace_the_native_zero_base() {
        let mut world = World::new();
        world.init_resource::<AutoClutterState>();
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(meshes());
        world.insert_resource(full_water());
        world.insert_resource(ActiveSession::new(
            SessionId::new(),
            SessionMeta {
                glade: GladeType::Summer,
                ..Default::default()
            },
        ));
        world.spawn((
            TreeComponent {
                id: 7,
                is_preplaced: false,
            },
            PrefabInstanceData(PlacementDataSsbo {
                position: Vec3::new(12.0, 37.0, -9.0),
                scale: 1.0,
                ..Default::default()
            }),
        ));

        run(&mut world);
        let candidates = &world.resource::<AutoClutterState>().spawn_candidates;
        assert_eq!(candidates.len(), TREE_ITERATIONS);
        for spawn in candidates {
            let distance = glam::Vec2::new(spawn.position.x - 12.0, spawn.position.z + 9.0).length();
            assert!(distance > 1.0);
            assert!(
                spawn.position.y.is_finite()
                    && spawn.position.y >= -1.5
                    && spawn.position.y <= 0.0
            );
        }
    }
}
