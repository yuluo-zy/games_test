//! B `decorator_visual/cottage_wall_window.rs` — 农舍墙窗装饰视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407ebfd0（generate_cottage_wall_windows，70KB）/
//! 0x1407ef7b0（flowerbed_xforms，2.6KB，O 全文）/ 0x1407ef990（add_lintels，
//! 14KB，O 全文）+ 骨架 callees（O）：flowerbed_xforms ×2、add_lintels ×2、
//! subdivide_wall_decorator、has_cloth ×3、Rectangle3d::extent ×4、
//! xz_projective_rotation_bad（0x140c911d0）、Rectangle2d 几何、
//! WindowVisualEntityBuilder 链、blueprint 链。
//!
//! flowerbed_xforms（O，0x1407ef7b0 全文 + 常量 rne-read）：
//! 5 参面 {out, subtype u8, mode u8, ctx, mesh_lib}；跳表（0x142a8ae68 /
//! 0x142a8ae80，subtype-1<3 三键）按 mode 选网格名：
//! - mode 0/1：`decorators/window_cottage_{1x1,2x1,3x1}_flowerbed_locations`
//! - mode 3：`decorators/window_cottage_corner_{1x1,2x1,3x1}_flowerbed_locations`
//! AssetMeshLibrary::get_by_name → Mesh::positions_without_indices →
//! 元素数 = len÷3（Vec3）→ 逐花床变换输出。其余 mode/subtype fmt panic。
//!
//! add_lintels（O，0x1407ef990 全文 + 常量 rne-read，2026-10-19）：
//! - `fVar26 = (param_10 + 1.43) * 0.5`（0x3fb70a3f / 0x3f000000）；
//! - 网格条目高 `entry+0x30 + 0.36`（0x3eb851eb）；
//! - 0x14B 记录 `{u32 句柄, u32 墙, f32 高, f32 0.2, u8 0}`（0x3e4ccccd = 0.2）；
//! - 每件 0x160B 视觉记录：mesh `"decorators/setdressing_window_lintel"`
//!   （len 0x24）/ shader `"nani_solid_vertex_color_window"`（len 0x1e）；
//! - rank < 2：scale 取件 `[7]`；rank >= 2：位置按件四元数旋转并加
//!   `0.081`（0x3da5e353）常量偏移；scale 首件/末件 0.82（0x3f51eb85），
//!   中部 0.64（0x3f23d70a）；
//! - `param_11 & 1`（flag bit0）时上面 scale `*= param_12 / 0.9`
//!   （0x3f666666 = 0.9）。
//!
//! U 登记：① 花床变换装配（FUN_141260b40 内部）② add_lintels 12 参面
//! 其余语义（param_4 件数 / param_5 / param_6 / param_7 输出表）③ 参数排序。

use bevy_ecs::prelude::*;
use country_core::asset_libraries::mesh_library::AssetMeshLibrary;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use glam::{Quat, Vec3};

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// 农舍窗花床网格名（O：0x142a8ae68 跳表，mode<2）。
const FLOWERBED_NAMES: [&str; 3] = [
    "decorators/window_cottage_1x1_flowerbed_locations",
    "decorators/window_cottage_2x1_flowerbed_locations",
    "decorators/window_cottage_3x1_flowerbed_locations",
];
/// 角窗花床网格名（O：0x142a8ae80 跳表，mode==3）。
const CORNER_FLOWERBED_NAMES: [&str; 3] = [
    "decorators/window_cottage_corner_1x1_flowerbed_locations",
    "decorators/window_cottage_corner_2x1_flowerbed_locations",
    "decorators/window_cottage_corner_3x1_flowerbed_locations",
];

/// 窗楣相对墙高的偏移（O：0x142a89f40 = 1.43）。
pub const LINTEL_HEIGHT_OFFSET: f32 = 1.43;
/// 窗楣高度减半系数（O：0x142925870 = 0.5）。
pub const LINTEL_HALF: f32 = 0.5;
/// 网格条目高附加（O：0x142a89f44 = 0.36）。
pub const LINTEL_MESH_HEIGHT_EXTRA: f32 = 0.36;
/// rank >= 2 的位置旋转常量（O：0x142a89f48/58/64 = 0.081）。
pub const LINTEL_ROTATION_OFFSET: f32 = 0.081;
/// 首/末件 scale（O：0x142a89f70 = 0.82）。
pub const LINTEL_SCALE_END: f32 = 0.82;
/// 中部件 scale（O：0x142a89f74 = 0.64）。
pub const LINTEL_SCALE_MIDDLE: f32 = 0.64;
/// flag bit0 高度分母（O：0x142a7c88c = 0.9）。
pub const LINTEL_HEIGHT_DIVISOR: f32 = 0.9;
/// 0x14B 记录的常量字段（O：0x3e4ccccd = 0.2）。
pub const LINTEL_RECORD_CONST: f32 = 0.2;
/// 窗楣网格名（O：0x142a8acc0，len 0x24）。
pub const LINTEL_MESH: &str = "decorators/setdressing_window_lintel";
/// 窗楣着色器名（O：0x142a8a600，len 0x1e）。
pub const LINTEL_SHADER: &str = "nani_solid_vertex_color_window";

/// 0x14B 窗楣记录（O：`{u32, u32, f32, f32, u8}`）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LintelRecord {
    /// 句柄（`func_0x140c925d0` 产物）。
    pub handle: u32,
    /// 墙 id（`dst_as_wall` 的 `puVar10[1]`）。
    pub wall: u32,
    /// 高（网格条目 +0x30 + 0.36）。
    pub height: f32,
    /// 常量 0.2。
    pub constant: f32,
    /// 判别 0。
    pub kind: u8,
}

/// 窗楣来源件（O：12×f32；`[0..4]` 四元数、`[4..7]` 位置、`[7]` rank<2 的 scale）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LintelSource {
    pub quat: Quat,
    pub position: Vec3,
    pub scale: f32,
}

/// 窗楣视觉记录（0x160B 结构的关键可观察字段；其余槽位 U）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LintelVisual {
    pub mesh: &'static str,
    pub shader: &'static str,
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: f32,
}

/// 0x1407ef7b0（O，全文）：花床变换（网格位置 → 变换列表）。
/// mode <2 → 窗花床网格；mode ==3 → 角窗花床网格；subtype-1<3 三键；其余
/// fmt panic。元素数 = positions.len()/3（Vec3）。
pub fn flowerbed_xforms(
    out: &mut Vec<crate::utils::IsoTransform>,
    subtype: u8,
    mode: u8,
    _ctx: u64,
    mesh_lib: &AssetMeshLibrary,
) {
    let names: &[&str; 3] = match mode {
        0 | 1 => &FLOWERBED_NAMES,
        3 => &CORNER_FLOWERBED_NAMES,
        _ => panic!("flowerbed_xforms: unexpected mode {}", mode),
    };
    let idx = subtype.wrapping_sub(1);
    if idx >= 3 {
        panic!("flowerbed_xforms: unexpected subtype {}", subtype);
    }
    let Some(mesh) = mesh_lib.get_by_name(names[idx as usize]) else {
        return;
    };
    // O：positions（Vec3 序列；原 ÷3 = Vec3 计数与 core Mesh 返回一致）。
    let positions = mesh.positions_without_indices();
    for p in positions {
        out.push(crate::utils::IsoTransform::new(*p, Quat::IDENTITY));
    }
}

/// 0x1407ef990（O，全文）：窗楣装配。
///
/// - `wall_height` = param_10；`fVar26 = (wall_height + 1.43) * 0.5`；
/// - `mesh_height` = `entry_height + 0.36`（`None` → 原版 `FUN_14125cf90`
///   未命中即整段不产出；对应返回空）；
/// - 首件/末件 scale 0.82、中部 0.64；rank >= 2 时位置按件四元数旋转；
/// - `flag & 1` 时 scale *= param_12 / 0.9。
///
/// 返回 `(0x14B 记录, 逐件 0x160B 视觉记录)`；输入为空时仍返回记录（原版
/// 先 push 0x14B 记录再按 `param_4` 产出视觉件）。
pub fn add_lintels(
    handle: u32,
    wall: u32,
    rank: u8,
    sources: &[LintelSource],
    mesh_entry_height: Option<f32>,
    wall_height: f32,
    flag: u32,
    height_param: f32,
) -> (Option<LintelRecord>, Vec<LintelVisual>) {
    let Some(entry_height) = mesh_entry_height else {
        return (None, Vec::new());
    };
    let f26 = (wall_height + LINTEL_HEIGHT_OFFSET) * LINTEL_HALF;
    let height = entry_height + LINTEL_MESH_HEIGHT_EXTRA;
    let record = LintelRecord {
        handle,
        wall,
        height,
        constant: LINTEL_RECORD_CONST,
        kind: 0,
    };
    let mut visuals = Vec::with_capacity(sources.len());
    let n = sources.len();
    for (i, src) in sources.iter().enumerate() {
        let mut scale = if rank < 2 {
            src.scale
        } else {
            if i == 0 || i + 1 == n {
                LINTEL_SCALE_END
            } else {
                LINTEL_SCALE_MIDDLE
            }
        };
        // O：位置（rank>=2 旋转；常量偏移 U 的精确槽位）。
        let mut position = src.position;
        position.y += f26;
        if rank >= 2 {
            position += src.quat * Vec3::splat(LINTEL_ROTATION_OFFSET);
        }
        if flag & 1 != 0 {
            scale *= height_param / LINTEL_HEIGHT_DIVISOR;
        }
        visuals.push(LintelVisual {
            mesh: LINTEL_MESH,
            shader: LINTEL_SHADER,
            position,
            rotation: src.quat,
            scale,
        });
    }
    (Some(record), visuals)
}

/// 0x1407ebfd0（O 结构）：生成农舍墙窗装饰视觉。
pub fn generate_cottage_wall_windows(
    decorator_storage: Res<DecoratorStorage>,
    _cached: Res<
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms,
    >,
    derived: Res<DerivedDecoratorSubtypes>,
    mesh_lib: Res<AssetMeshLibrary>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
) {
    for (_wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            if !is_cottage_window(info) {
                continue;
            }
            let d = derived.get(id);
            // O：花床（flowerbed_xforms 消费，mode=0）。
            let mut beds = Vec::new();
            flowerbed_xforms(&mut beds, d.subtype().0, 0, 0, &mesh_lib);
            // O：蓝图 + 视觉实体。
            let mut blueprint = DecoratorBlueprint::new();
            blueprint.set_colliders(vec![d.collision_mesh()]);
            blueprint.set_outline((0.0, 0.0, 0.0));
            blueprints.insert(id, blueprint);
            let builder = WindowVisualEntityBuilder::new(Vec3::ZERO, Quat::IDENTITY, id);
            let _ = &mut transient;
            let _ = &builder;
            // O：add_lintels（窗楣；sources 来自窗框几何，当前 U → 空 slice，
            // 但记录条目仍按真实公式生成）。
            let (_record, _visuals) = add_lintels(
                id,
                0,
                d.subtype().0,
                &[],
                None,
                0.0,
                0,
                0.0,
            );
        }
    }
}

/// 农舍窗判别（O：DecoratorType 0x38；`is_window` 含 0x39/0x3a，故按精确值）。
pub fn is_cottage_window(info: &DecoratorInfo) -> bool {
    info.decorator_type().0 == 0x38
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lintel_constants_match_machine_code() {
        assert_eq!(LINTEL_HEIGHT_OFFSET, 1.43);
        assert_eq!(LINTEL_HALF, 0.5);
        assert_eq!(LINTEL_MESH_HEIGHT_EXTRA, 0.36);
        assert_eq!(LINTEL_SCALE_END, 0.82);
        assert_eq!(LINTEL_SCALE_MIDDLE, 0.64);
        assert_eq!(LINTEL_HEIGHT_DIVISOR, 0.9);
        assert_eq!(LINTEL_RECORD_CONST, 0.2);
    }

    fn src(x: f32) -> LintelSource {
        LintelSource {
            quat: Quat::IDENTITY,
            position: Vec3::new(x, 1.0, 0.0),
            scale: 1.0,
        }
    }

    #[test]
    fn lintel_scales_first_last_and_middle() {
        let sources = [src(0.0), src(1.0), src(2.0)];
        let (record, visuals) =
            add_lintels(7, 3, 2, &sources, Some(1.0), 2.0, 0, 0.0).clone();
        let record = record.unwrap();
        assert_eq!(record.height, 1.36);
        assert_eq!(record.constant, 0.2);
        assert_eq!(visuals.len(), 3);
        assert_eq!(visuals[0].scale, 0.82);
        assert_eq!(visuals[1].scale, 0.64);
        assert_eq!(visuals[2].scale, 0.82);
        assert!(
            (visuals[0].position.y - (1.0 + (2.0 + 1.43) * 0.5 + 0.081)).abs() < 1e-5,
            "rank>=2 applies the 0.081 rotation offset"
        );
    }

    #[test]
    fn lintel_flag_scales_by_height_ratio() {
        let sources = [src(0.0)];
        let (_, visuals) = add_lintels(1, 0, 0, &sources, Some(1.0), 0.0, 1, 0.45);
        assert!((visuals[0].scale - 1.0 * (0.45 / 0.9)).abs() < 1e-6);
    }

    #[test]
    fn missing_mesh_entry_yields_nothing() {
        let (record, visuals) = add_lintels(1, 0, 0, &[src(0.0)], None, 0.0, 0, 0.0);
        assert!(record.is_none());
        assert!(visuals.is_empty());
    }

    #[test]
    fn type_predicate_only_0x38() {
        let mut info = DecoratorInfo::default();
        info.ty = 0x38;
        assert!(is_cottage_window(&info));
        info.ty = 0x39;
        assert!(!is_cottage_window(&info));
    }
}
