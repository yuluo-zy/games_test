//! `country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve`
//! （B 0x140a8b0c0，1129 B；`set 6 chain` 成员）。
//!
//! 原版 SystemParam 面（三个形参，顺序固定）：
//!
//! | # | 原版形参 | 读取 |
//! |---|---|---|
//! | 1 | `Res<DebugGui>` | `byte +0xf3 == 1`（`show_threshold_curve`）才工作 |
//! | 2 | `Res<PublicWalls>` | 遍历全部墙记录（项 0x2a8 B），每个 id 再探测一次并校验 `simplified_curve_2d.is_valid()` |
//! | 3 | `ResMut<UiSignifierStream>` | `DebugSphere::show` 的提交流 |
//!
//! 逐墙行为：取四个数组长度的最小值 `n`（公共曲线点 `+0x90/+0x98`、段值 `+0xc8/+0xd0`、
//! 地形高度 `terrain_y` `+0xe0/+0xe8`、独立段高度数组 `+0xf8/+0x100`）；`n == 0` 的墙不绘制；否则逐点画 4 个球：
//!
//! 1. 白（tag 0），半径 `0.1`，alpha `1.0`，中心 = 点；
//! 2. 红（tag 1），半径 `0.05`，alpha `2.0`，中心 = 点 + `Y * 段值[i]`；
//! 3. 红（tag 1），半径 `0.15`，alpha `0.5`，中心 = 点 + `Y * 段高度[i]`；
//! 4. 蓝（tag 2），半径 `0.15`，alpha `0.5`，中心 = 点 + `Y * (地形高度[i] - 点.y)`（即 `y = terrain_y[i]`）。
//!
//! 数组身份：`+0xc8` 为 `PreparedWallPath::segment_values`（`fences` 断言其长度等于准备后路径点数）；
//! `+0xe0` 按 `ui_adjust_wall_height_signifier_animation`（0x14085bb50）的用法——地面带曲线的
//! 绝对世界高度——判定为 `geometry.terrain_y`（**身份为推断**，未闭合）；`+0xf8` 为 `segment_heights`。
//!
//! 全部 `solid = false`。常量 `0x3dcccccd`/`0x3d4ccccd`/`0x3e19999a`/`0x3f800000`/`0x40000000`/
//! `0x3f000000` 均取自反编译立即数。

use crate::resources::gui::DebugGui;
use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::ui::signifiers::UiSignifierStream;
use crate::utils::debug_viz::{DebugColor, DebugSphere};
use bevy_ecs::prelude::*;
use glam::Vec3;

const WHITE_RADIUS: f32 = f32::from_bits(0x3dcc_cccd);
const SEGMENT_VALUE_RADIUS: f32 = f32::from_bits(0x3d4c_cccd);
const HEIGHT_RADIUS: f32 = f32::from_bits(0x3e19_999a);
const SEGMENT_VALUE_ALPHA: f32 = f32::from_bits(0x4000_0000);
const HEIGHT_ALPHA: f32 = f32::from_bits(0x3f00_0000);

/// 每点 4 个球的请求（按原版 `show` 调用顺序）。
pub fn threshold_spheres(point: Vec3, segment_value: f32, terrain_height: f32, segment_height: f32) -> [DebugSphere; 4] {
    [
        DebugSphere {
            color: DebugColor::White,
            radius: WHITE_RADIUS,
            center: point,
            alpha: 1.0,
            solid: false,
        },
        DebugSphere {
            color: DebugColor::Red,
            radius: SEGMENT_VALUE_RADIUS,
            center: point + Vec3::Y * segment_value,
            alpha: SEGMENT_VALUE_ALPHA,
            solid: false,
        },
        DebugSphere {
            color: DebugColor::Red,
            radius: HEIGHT_RADIUS,
            center: point + Vec3::Y * segment_height,
            alpha: HEIGHT_ALPHA,
            solid: false,
        },
        DebugSphere {
            color: DebugColor::Blue,
            radius: HEIGHT_RADIUS,
            center: point + Vec3::Y * (terrain_height - point.y),
            alpha: HEIGHT_ALPHA,
            solid: false,
        },
    ]
}

pub fn debug_viz_threshold_curve(
    debug_gui: Res<DebugGui>,
    public: Res<PublicWalls>,
    mut stream: ResMut<UiSignifierStream>,
) {
    if !debug_gui.show_threshold_curve {
        return;
    }
    for wall in public.wall_ids().collect::<Vec<_>>() {
        // 原版再探测一次并带 `is_valid` 断言（`get` 内含）；记录必然存在。
        let state = public
            .get(wall)
            .expect("called `Option::unwrap()` on a `None` value");
        let outline = state.height_outline();
        let segment_values: &[f32] = state
            .prepared_path
            .as_ref()
            .map(|path| path.segment_values.as_slice())
            .unwrap_or(&[]);
        let terrain_heights = state.geometry.terrain_y.as_slice();
        let segment_heights = state.segment_heights.as_slice();
        let count = outline
            .len()
            .min(segment_values.len())
            .min(terrain_heights.len())
            .min(segment_heights.len());
        for index in 0..count {
            let point = Vec3::from_array(outline[index]);
            for sphere in threshold_spheres(
                point,
                segment_values[index],
                terrain_heights[index],
                segment_heights[index],
            ) {
                sphere.show(&mut stream);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::walls::publication_source::TerrainHeightGrid512;
    use crate::resources::walls::{InnerWalls, WallState};
    use crate::systems::wall::wall_state::path_preparation::PathRasters512;
    use crate::systems::wall::wall_state::publish_public_wall_state::WallPublicationTerrain;
    use bevy_ecs::system::RunSystemOnce;
    use std::sync::Arc;

    fn terrain() -> TerrainHeightGrid512 {
        TerrainHeightGrid512 {
            encoded: vec![0u16; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 4.0,
            sampling_fix: true,
        }
    }

    fn published_world(prepare_paths: bool) -> World {
        let mut world = World::new();
        let mut inner = InnerWalls::new();
        let curve = crate::resources::roofs::geometry::surface::Curve2::try_new(
            vec![[0.0, 0.0], [4.0, 0.0]],
            false,
        )
        .unwrap();
        inner.new_wall(WallState::new(curve, 2.0, 0, 0, false));
        let mut public = PublicWalls::default();
        let zero: Arc<[u8]> = vec![0u8; 512 * 512].into();
        let paths = PathRasters512::from_encoded(
            [zero.clone(), zero.clone(), zero.clone(), zero],
            true,
        )
        .unwrap();
        public.consume_with_paths(
            &mut inner,
            &terrain(),
            prepare_paths.then_some(&paths),
            &Default::default(),
            &Default::default(),
        );
        world.insert_resource(public);
        world.insert_resource(inner);
        world.insert_resource(WallPublicationTerrain(terrain()));
        world.insert_resource(UiSignifierStream::default());
        world
    }

    fn gui(show: bool) -> DebugGui {
        DebugGui {
            show_threshold_curve: show,
            ..Default::default()
        }
    }

    #[test]
    fn disabled_flag_draws_nothing() {
        let mut world = published_world(true);
        world.insert_resource(gui(false));
        world.run_system_once(debug_viz_threshold_curve).unwrap();
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
    }

    #[test]
    fn unprepared_wall_has_empty_arrays_and_draws_nothing() {
        let mut world = published_world(false);
        world.insert_resource(gui(true));
        world.run_system_once(debug_viz_threshold_curve).unwrap();
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
    }

    #[test]
    fn enabled_flag_draws_four_spheres_per_common_point() {
        let mut world = published_world(true);
        world.insert_resource(gui(true));
        let expected = {
            let public = world.resource::<PublicWalls>();
            let id = public.wall_ids().next().unwrap();
            let state = public.get(id).unwrap();
            let n = state
                .height_outline()
                .len()
                .min(state.prepared_path.as_ref().unwrap().segment_values.len())
                .min(state.geometry.terrain_y.len())
                .min(state.segment_heights.len());
            assert!(n > 0, "fixture must produce a drawable wall");
            n * 4
        };
        world.run_system_once(debug_viz_threshold_curve).unwrap();
        assert_eq!(
            world.resource::<UiSignifierStream>().committed_len() as usize,
            expected
        );
    }

    #[test]
    fn sphere_table_matches_original_constants() {
        let spheres = threshold_spheres(Vec3::new(1.0, 2.0, 3.0), 0.5, 7.0, 1.5);
        assert_eq!(spheres[0].color, DebugColor::White);
        assert_eq!(spheres[0].radius, 0.1);
        assert_eq!(spheres[0].center, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(spheres[1].color, DebugColor::Red);
        assert_eq!(spheres[1].radius, 0.05);
        assert_eq!(spheres[1].alpha, 2.0);
        assert_eq!(spheres[1].center, Vec3::new(1.0, 2.5, 3.0));
        assert_eq!(spheres[2].color, DebugColor::Red);
        assert_eq!(spheres[2].radius, 0.15);
        assert_eq!(spheres[2].alpha, 0.5);
        assert_eq!(spheres[2].center, Vec3::new(1.0, 3.5, 3.0));
        assert_eq!(spheres[3].color, DebugColor::Blue);
        assert_eq!(spheres[3].center, Vec3::new(1.0, 7.0, 3.0));
        assert!(spheres.iter().all(|s| !s.solid));
    }
}
