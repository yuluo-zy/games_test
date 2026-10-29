//! `country_core::systems::wall::adjust_wall_height::signifier_animation`.
//!
//! 原版 `ui_adjust_wall_height_signifier_animation` 0x14085bb50（3458 B），
//! 由 FunctionSystem 包装 0x1403347e0 调用；组件 `WallHeightSignifier`
//! 注册于 init_state 0x140335080（size 0x60 / align 8 / drop 0x14014a160 /
//! 无 required components，名字 89 B）。
//!
//! 参数面（0x1403347e0 取参顺序，init_state 0x140335080 的
//! `(&mut WallHeightSignifier, &SignifierAnimation, &Handle<Mesh>,
//!   Option<&SignifierMarkedForDeletion>)`）：
//! 1. `Query<(&mut WallHeightSignifier, &SignifierAnimation, &Handle<Mesh>,
//!    Option<&SignifierMarkedForDeletion>)>`
//! 2. `ResMut<AssetMeshLibrary>`（`*puVar9 = tick` 在 `update_curve_mesh` 前标记 changed）
//! 3. `Res<PublicWalls>`（0x2a8 B 记录哈希表）
//! 4. `Query<&Roof>`（`PublicWalls::roof_component` 的 r9）
//! 5. `Res<UiState>`（+0x18 hover 字节、+0x28 交点判别、+0x4c 交点位置）
//!
//! `WallHeightSignifier` 布局（0x60）：Curve3 @+0x00..0x38；`Option<f32>` @+0x38/+0x3c；
//! `Option<f32>` @+0x40/+0x44；`Option<Vec3>` @+0x48/+0x4c；`WallId` @+0x58。
//! 字段名为推断（I），偏移与读写面为机器码证据。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use utils::curve::Curve3;
use ::utils::scalar::clamp01;

use crate::asset_libraries::Handle;
use crate::asset_libraries::mesh_library::AssetMeshLibrary;
use crate::render::mesh::Mesh;
use crate::resources::roofs::Roof;
use crate::resources::walls::WallId;
use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::ui::resources::ui_state::UiState;
use crate::systems::ui::signifiers::animation::SignifierAnimation;
use crate::systems::ui::signifiers::signifier_mark_for_delete::SignifierMarkedForDeletion;
use crate::utils::geo::update_curve_mesh;

/// `eased` 门限（0x142a7d8d0 = 0x3f733333）。
pub const EASED_TRACK_THRESHOLD: f32 = f32::from_bits(0x3f73_3333);
/// u 窗口半宽的下偏移（0x142a82410 = 0xbc23d70a = -0.01）。
pub const WINDOW_LOW_OFFSET: f32 = f32::from_bits(0xbc23_d70a);
/// u 窗口半宽的上偏移（0x142960674 = 0x3c23d70a = +0.01）。
pub const WINDOW_HIGH_OFFSET: f32 = f32::from_bits(0x3c23_d70a);
/// 平顶带边屋顶时的下沉量（0x142a94920 = 0xbe947ae2）。
pub const FLAT_RIM_ROOF_LIFT: f32 = f32::from_bits(0xbe94_7ae2);
/// 无记录 u 时的默认值（0x142925870 = 0.5）。
pub const DEFAULT_U: f32 = 0.5;
/// 平顶判定阈值（0x142925984 = 0x3dcccccd = 0.1）。
const FLAT_ROOF_HEIGHT_01: f32 = f32::from_bits(0x3dcc_cccd);

/// 原版 `WallHeightSignifier`（Component，0x60 B）。
#[derive(Component, Debug, Clone)]
pub struct WallHeightSignifier {
    /// +0x00..+0x38：地形高度上的外轮廓曲线（`try_new_from_points<Vec3>`，3D 弧长）。
    pub curve: Curve3,
    /// +0x38 tag / +0x3c 值：首次确定的 u（无光标时 0.5）。
    pub anchor_u: Option<f32>,
    /// +0x40 tag / +0x44 值：标记删除且 eased 满后锁存的 u。
    pub latched_u: Option<f32>,
    /// +0x48 tag / +0x4c..0x58：最近一次 UI 交点。
    pub cursor: Option<Vec3>,
    /// +0x58。
    pub wall: WallId,
}

/// `CurveU::new`（0x14085c10f..c17c 内联）：`UCOMISS x,0; JC` / `UCOMISS 1,x; JC`
/// 对 NaN 同样置 CF，因此 NaN 走区间 panic；其后的 `!u.is_nan()` 断言不可达但机器码保留。
fn curve_u(value: f32) -> f32 {
    if !(value >= 0.0) || 1.0 < value {
        panic!("Tried to init CurveU from an invalid u: {}", value);
    }
    assert!(!value.is_nan(), "assertion failed: !u.is_nan()");
    value
}

/// 0x14085bb50。
pub fn ui_adjust_wall_height_signifier_animation(
    mut query: Query<(
        &mut WallHeightSignifier,
        &SignifierAnimation,
        &Handle<Mesh>,
        Option<&SignifierMarkedForDeletion>,
    )>,
    mut meshes: ResMut<AssetMeshLibrary>,
    public: Res<PublicWalls>,
    roofs: Query<&Roof>,
    ui: Res<UiState>,
) {
    for (mut signifier, animation, mesh_handle, marked) in &mut query {
        // 0x14085bcc..：hover 字节非 0 且交点判别 != 2 → 记录 UI 交点（写 changed tick）。
        if ui.hover_byte() != 0 && ui.intersection_tag() != 2 {
            signifier.cursor = Some(ui.intersection_point());
        }

        // PublicWalls 探针：空表或未命中都 `continue`（0x14085bd20 → 0x14085c2b0）；
        // 命中后 count<2 || length<=0 → panic（0x32 B 断言串，PublicWalls::get 内含）。
        let Some(record) = public.get(signifier.wall) else {
            continue;
        };

        // 屋顶：平顶（height_01 <= 0.1，NaN 不通过）且 byte[+0x3d] != 0 → 下沉 0.29。
        let lift = match public.roof_component(signifier.wall, |entity| roofs.get(entity).ok()) {
            Some(roof) if roof.is_flat() && roof.0[0x3d] != 0 => FLAT_RIM_ROOF_LIFT,
            _ => 0.0,
        };

        // 原+0x88是已准备的轮廓Curve3，+0xe0是arch_segment_heights；不是地形采样高。
        let outline = record.height_outline();
        let heights = record.arch_segment_heights().unwrap_or(&[]);
        let count = outline.len().min(heights.len());
        let points: Vec<Vec3> = (0..count)
            .map(|i| Vec3::new(outline[i][0], heights[i] + lift, outline[i][2]))
            .collect();
        // `try_new_from_points` Err → `Result::unwrap` panic（0x14085c81d）。
        let curve = Curve3::try_new_from_points(points, false).unwrap();
        signifier.curve = curve;

        let eased = animation.eased;
        let approx_u = |signifier: &WallHeightSignifier, cursor: Vec3| -> f32 {
            clamp01(
                signifier
                    .curve
                    .get_approx_u_from_pos(Vec2::new(cursor.x, cursor.z)),
            )
        };

        // 0x14085bfcc：`anchor_u` 缺省时先建立（光标 → 近似 u，否则 0.5）。
        if signifier.anchor_u.is_none() {
            let anchor = match signifier.cursor {
                Some(cursor) => approx_u(&signifier, cursor),
                None => DEFAULT_U,
            };
            signifier.anchor_u = Some(anchor);
        }

        // 0x14085c051：eased > 0.95 且（带删除标记）→ 锁存 u。
        let v = if eased > EASED_TRACK_THRESHOLD && marked.is_some() {
            let latched = match signifier.cursor {
                Some(cursor) => approx_u(&signifier, cursor),
                None => signifier.anchor_u.unwrap(),
            };
            signifier.latched_u = Some(latched);
            latched
        } else if let Some(latched) = signifier.latched_u {
            // 0x14085c09a：已锁存 → +0x44。
            latched
        } else if let Some(anchor) = signifier.anchor_u {
            anchor
        } else {
            DEFAULT_U
        };

        // 窗口：start = eased*0 + clamp01(v-0.01)*(1-eased)；end = eased + (1-eased)*clamp01(v+0.01)。
        let low = clamp01(v + WINDOW_LOW_OFFSET);
        let start = curve_u(eased * 0.0 + low * (1.0 - eased));
        let high = clamp01(v + WINDOW_HIGH_OFFSET);
        let end = curve_u(eased + (1.0 - eased) * high);

        let mut sliced = Vec::new();
        signifier.curve.slice_into(start, end, &mut sliced);
        // 切片再过一次 `try_new_from_points`（0x14085c871 Err → unwrap panic），仅用其点集。
        let slice_curve = Curve3::try_new_from_points(sliced, false).unwrap();
        update_curve_mesh(
            meshes.get_mut(*mesh_handle),
            slice_curve.points.iter().copied(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::roofs::geometry::surface::Curve2;
    use crate::resources::walls::publication_source::TerrainHeightGrid512;
    use crate::resources::walls::{InnerWalls, WallState};
    use crate::systems::ui::resources::ui_state::{UiInteraction, WidgetId};
    use crate::asset_libraries::mesh_library::MeshAsset;

    struct Fixture {
        world: World,
        wall: WallId,
        entity: Entity,
        handle: Handle<Mesh>,
    }

    fn terrain() -> TerrainHeightGrid512 {
        TerrainHeightGrid512 {
            encoded: vec![0u16; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: false,
        }
    }

    fn fixture(eased: f32, marked: bool, with_roof: Option<[u8; 88]>) -> Fixture {
        let mut world = World::new();
        let mut inner = InnerWalls::new();
        let curve = Curve2::try_new(vec![[0.0, 0.0], [10.0, 0.0]], false).unwrap();
        let wall = inner.new_wall(WallState::new(curve, 4.0, 0, 0, false));
        let mut public = PublicWalls::default();
        let paths = crate::systems::wall::wall_state::path_preparation::PathRasters512::from_encoded(
            std::array::from_fn(|_| vec![0; 512 * 512].into()), false).unwrap();
        public.consume_with_paths(&mut inner, &terrain(), Some(&paths), &crate::resources::walls::public_walls::WallPublicationDirty::default(), &crate::resources::walls::public_walls::WallSpatialHash::default());
        let mut meshes = AssetMeshLibrary::new();
        let handle = meshes.add(MeshAsset::new(
            crate::utils::geo::mesh_from_pts("t", &[Vec3::ZERO]),
            None,
        ));
        if let Some(bytes) = with_roof {
            let roof = world.spawn(Roof::from_observed(bytes)).id();
            public.associate_roof(wall, roof);
        }
        world.insert_resource(public);
        world.insert_resource(meshes);
        world.insert_resource(UiState::new(UiInteraction {
            native_tag: 0,
            native_phase: 0,
            widget: WidgetId(0),
        }));
        let placeholder = Curve3::try_new_from_points(vec![Vec3::ZERO, Vec3::X], false).unwrap();
        let mut entity = world.spawn((
            WallHeightSignifier {
                curve: placeholder,
                anchor_u: None,
                latched_u: None,
                cursor: None,
                wall,
            },
            SignifierAnimation {
                eased,
                ..Default::default()
            },
            handle,
        ));
        if marked {
            entity.insert(SignifierMarkedForDeletion);
        }
        let entity = entity.id();
        Fixture {
            world,
            wall,
            entity,
            handle,
        }
    }

    fn run(world: &mut World) {
        use bevy_ecs::system::RunSystemOnce;
        world
            .run_system_once(ui_adjust_wall_height_signifier_animation)
            .unwrap();
    }

    fn signifier(f: &Fixture) -> &WallHeightSignifier {
        f.world.get::<WallHeightSignifier>(f.entity).unwrap()
    }

    #[test]
    fn rebuilds_curve_from_prepared_outline_and_arch_heights_and_defaults_u_to_half() {
        let mut f = fixture(0.0, false, None);
        run(&mut f.world);
        let s = signifier(&f);
        assert!(s.curve.points.len() >= 2);
        let heights = f.world.resource::<PublicWalls>().get(f.wall).unwrap()
            .arch_segment_heights().unwrap();
        assert!(s.curve.points.iter().zip(heights).all(|(p, height)| p.y == *height));
        assert_eq!(s.curve.points[0].x, 0.0);
        assert_eq!(s.curve.points.last().unwrap().x, 10.0);
        assert_eq!(s.anchor_u, Some(0.5));
        assert_eq!(s.latched_u, None);
        // eased = 0 → window [0.49, 0.51] of the 10 m curve.
        let mesh = f.world.resource::<AssetMeshLibrary>().get(f.handle).unwrap();
        let pts = mesh.positions_without_indices();
        assert!((pts[0].x - 4.9).abs() < 1e-4, "{pts:?}");
        assert!((pts.last().unwrap().x - 5.1).abs() < 1e-4, "{pts:?}");
    }

    #[test]
    fn fully_eased_window_covers_the_whole_curve() {
        let mut f = fixture(1.0, false, None);
        run(&mut f.world);
        let mesh = f.world.resource::<AssetMeshLibrary>().get(f.handle).unwrap();
        let pts = mesh.positions_without_indices();
        assert!((pts[0].x - 0.0).abs() < 1e-5);
        assert!((pts.last().unwrap().x - 10.0).abs() < 1e-5);
    }

    #[test]
    fn hover_with_intersection_records_cursor_and_anchors_u_to_it() {
        let mut f = fixture(0.0, false, None);
        {
            let mut ui = f.world.resource_mut::<UiState>();
            ui.replace_interaction(UiInteraction {
                native_tag: 1,
                native_phase: 0,
                widget: WidgetId(0),
            });
            ui.set_intersection_tag(0);
            ui.set_intersection_point(Vec3::new(2.0, 0.0, 0.0));
        }
        run(&mut f.world);
        let s = signifier(&f);
        assert_eq!(s.cursor, Some(Vec3::new(2.0, 0.0, 0.0)));
        let u = s.anchor_u.unwrap();
        assert!((u - 0.2).abs() < 1e-4, "u = {u}");
    }

    #[test]
    fn intersection_tag_two_does_not_record_cursor() {
        let mut f = fixture(0.0, false, None);
        {
            let mut ui = f.world.resource_mut::<UiState>();
            ui.replace_interaction(UiInteraction {
                native_tag: 1,
                native_phase: 0,
                widget: WidgetId(0),
            });
            ui.set_intersection_tag(2);
            ui.set_intersection_point(Vec3::new(2.0, 0.0, 0.0));
        }
        run(&mut f.world);
        assert_eq!(signifier(&f).cursor, None);
    }

    #[test]
    fn marked_and_fully_eased_latches_u() {
        let mut f = fixture(0.99, true, None);
        run(&mut f.world);
        assert_eq!(signifier(&f).latched_u, Some(0.5));
        // 不带标记不锁存。
        let mut g = fixture(0.99, false, None);
        run(&mut g.world);
        assert_eq!(signifier(&g).latched_u, None);
    }

    #[test]
    fn latched_u_wins_over_anchor_below_threshold() {
        let mut f = fixture(0.0, false, None);
        f.world.get_mut::<WallHeightSignifier>(f.entity).unwrap().latched_u = Some(0.8);
        f.world.get_mut::<WallHeightSignifier>(f.entity).unwrap().anchor_u = Some(0.3);
        run(&mut f.world);
        let mesh = f.world.resource::<AssetMeshLibrary>().get(f.handle).unwrap();
        let pts = mesh.positions_without_indices();
        assert!((pts[0].x - 7.9).abs() < 1e-4, "{pts:?}");
    }

    #[test]
    fn flat_rim_roof_lowers_the_curve() {
        let mut bytes = [0u8; 88];
        bytes[0x30..0x34].copy_from_slice(&0.05f32.to_le_bytes());
        bytes[0x3d] = 1;
        let mut f = fixture(0.0, false, Some(bytes));
        run(&mut f.world);
        let s = signifier(&f);
        let base = f.world.resource::<PublicWalls>().get(f.wall).unwrap()
            .arch_segment_heights().unwrap();
        assert!(s.curve.points.iter().zip(base).all(|(p, h)| p.y == *h + FLAT_RIM_ROOF_LIFT));
        // 无边平顶不下沉。
        bytes[0x3d] = 0;
        let mut g = fixture(0.0, false, Some(bytes));
        run(&mut g.world);
        let base = g.world.resource::<PublicWalls>().get(g.wall).unwrap()
            .arch_segment_heights().unwrap();
        assert!(signifier(&g).curve.points.iter().zip(base).all(|(p, h)| p.y == *h));
    }

    #[test]
    fn missing_public_wall_skips_the_entity_after_cursor_capture() {
        let mut f = fixture(0.0, false, None);
        f.world.insert_resource(PublicWalls::default());
        {
            let mut ui = f.world.resource_mut::<UiState>();
            ui.replace_interaction(UiInteraction {
                native_tag: 1,
                native_phase: 0,
                widget: WidgetId(0),
            });
            ui.set_intersection_tag(0);
            ui.set_intersection_point(Vec3::new(1.0, 2.0, 3.0));
        }
        run(&mut f.world);
        let s = signifier(&f);
        assert_eq!(s.cursor, Some(Vec3::new(1.0, 2.0, 3.0)));
        assert_eq!(s.anchor_u, None);
        assert_eq!(s.curve.points.len(), 2);
        let _ = f.wall;
    }

    #[test]
    #[should_panic(expected = "Tried to init CurveU from an invalid u")]
    fn nan_eased_panics_with_the_curve_u_message() {
        let mut f = fixture(f32::NAN, false, None);
        run(&mut f.world);
    }

    #[test]
    fn clamp_semantics_follow_maxss_minss() {
        assert_eq!(clamp01(-1.0), 0.0);
        assert_eq!(clamp01(2.0), 1.0);
        assert!(clamp01(f32::NAN).is_nan());
        assert_eq!(FLAT_RIM_ROOF_LIFT.to_bits(), 0xbe94_7ae2);
    }
}
