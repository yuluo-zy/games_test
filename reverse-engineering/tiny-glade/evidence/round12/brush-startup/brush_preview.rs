//! `brush_preview.rs` — 笔刷预览（B 0x140865050，O）。
//!
//! 证据（2026-10-17 fresh 反编译/机器码 0x140865050）：
//! - 系统签名（PDB）：`Query<(Entity, &BrushPreview, Option<&mut Transform>)>`,
//!   `EventReader<OverwriteBrushPreview>`, `Res<CursorTerrainRaycast>`, `Commands`,
//!   `Res<DebugGui>`, `Res<EraserSizeMultiplier>`, `Res<BrushSizes>`；
//! - `DebugGui.hide_brush_previews`（+0xfa）置位 → 对全部 BrushPreview 实体
//!   `Commands::remove::<Transform>` 后返回；
//! - 最后一条 `OverwriteBrushPreview`（0x20 B：`flag@+8` bit0、世界位置 Vec3@+0xc、
//!   `kind@+0x18`）：kind 不匹配的实体移除 Transform；匹配时按 flag 位
//!   写/插入 Transform.translation（flag bit0=0 且无 Transform 时用光标位置）；
//! - 逐实体按 `BrushPreview` 的 kind 字节（4..=10）算 preview scale：
//!   4 → `BrushSizes.water@+4 × 0.5`；5 → `garden@+8 × 0.5 × 0.75`；
//!   6 → `(m×1.68×0.5, clamp(m), m×1.68×0.5)`；7|8 → `m`；
//!   9 → `((inverse_lerp(0.3,3.0,m)×3+1)×0.5, clamp(m), 同上)`；
//!   10 → `m×3.0×0.5`；其它 → 1.0；`clamp(m)=min(1.0,max(0.5,m))`，
//!   `m = EraserSizeMultiplier.0`；写 `Transform.scale`（+0x1c/+0x20/+0x24）。
//! - `clone_behavior` @0x140867080：判别 2（已判等）。
//!
//! Native32B EventInstance includes the8B Bevy EventId. Business payload is
//! flag/Vec3/kind; the104B deferred record is insert<Transform> plus Bevy
//! command ownership/error-handler metadata, not an additional game component.
use bevy_ecs::prelude::*;
use glam::Vec3;

use crate::{
    resources::{
        brush_sizes::BrushSizes,
        cursor_terrain_raycast::CursorTerrainRaycast,
        gui::DebugGui,
        walls::EraserSizeMultiplier,
    },
    utils::components::transform::Transform,
};

/// `BrushPreview` 组件标记（O：组件 1 B = 笔刷种类判别；`clone_behavior` 判别 2）。
///
/// 取值 4..=10（fresh switch：`kind-4 <= 6`）；种类名未取证，保留原始字节。
#[repr(transparent)]
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BrushPreview(pub u8);

/// Original EventInstance: EventId@0, payload@8. Keep EventId in Bevy.
#[repr(C)]
#[derive(Event, Clone, Copy, Debug, Default, PartialEq)]
pub struct OverwriteBrushPreview {
    pub flag: u8,
    pub pad: [u8; 3],
    pub position: Vec3,
    pub kind: u8,
    pub pad2: [u8; 3],
}

/// 克隆行为判别（O：0x140867080 `mov eax,2`）。
pub const CLONE_BEHAVIOR_DISC: u8 = 2;

/// `maxss`：任一操作数 NaN 时返回 src（与 EXE 位语义一致）。
fn maxss(dst: f32, src: f32) -> f32 {
    if dst > src {
        dst
    } else {
        src
    }
}

/// `minss`：任一操作数 NaN 时返回 src。
fn minss(dst: f32, src: f32) -> f32 {
    if dst < src {
        dst
    } else {
        src
    }
}

/// kind 6 的乘数（O 位型 0x3fd70a3d，读自 0x142a95dd0）。
const KIND6_FACTOR: f32 = f32::from_bits(0x3fd7_0a3d);

/// `brush_preview` 行 94 的内联 scale 映射（O 0x140866040 的 7 路跳表）。
///
/// `kind` = `BrushPreview.0`；`multiplier` = `EraserSizeMultiplier.0`。
/// 返回写入 `Transform.scale` 的三分量。
pub fn brush_size(kind: u8, brush_sizes: &BrushSizes, multiplier: f32) -> Vec3 {
    match kind {
        // case 4：water(+4) × 0.5。
        4 => Vec3::splat(brush_sizes.water * 0.5),
        // case 5：garden(+8) × 0.5 × 0.75。
        5 => Vec3::splat(brush_sizes.garden * 0.5 * 0.75),
        // case 6：a = m×1.68×0.5；xvy((a,a), clamp(m))。
        6 => {
            let a = multiplier * KIND6_FACTOR * 0.5;
            let y = minss(1.0, maxss(0.5, multiplier));
            Vec3::new(a, y, a)
        }
        // case 7 / 8：m。
        7 | 8 => Vec3::splat(multiplier),
        // case 9：v = inverse_lerp(0.3, 3.0, m)；s = (v×3+1)×0.5；
        // xvy((s,s), clamp(m))。
        9 => {
            let v = (multiplier - 0.3) / (3.0 - 0.3);
            let s = (v * 3.0 + 1.0) * 0.5;
            let y = minss(1.0, maxss(0.5, multiplier));
            Vec3::new(s, y, s)
        }
        // case 10：m × 3.0 × 0.5。
        10 => Vec3::splat(multiplier * 3.0 * 0.5),
        // default（含 kind-4 > 6）：1.0。
        _ => Vec3::ONE,
    }
}

/// 预览网格跟随（O 0x140865050）。
pub fn brush_preview(
    mut query: Query<(Entity, &BrushPreview, Option<&mut Transform>)>,
    mut overwrite: EventReader<OverwriteBrushPreview>,
    cursor: Res<CursorTerrainRaycast>,
    mut commands: Commands,
    debug_gui: Res<DebugGui>,
    eraser_multiplier: Res<EraserSizeMultiplier>,
    brush_sizes: Res<BrushSizes>,
) {
    // O：`DebugGui.hide_brush_previews`（+0xfa）→ 清掉所有预览 Transform 后返回。
    if debug_gui.hide_brush_previews {
        for (entity, _preview, _transform) in &mut query {
            commands.entity(entity).remove::<Transform>();
        }
        return;
    }

    // O：OverwriteBrushPreview → 逐事件按 kind 写/插入 translation。
    if let Some(event) = overwrite.read().last() {
        for (entity, preview, transform) in &mut query {
            if preview.0 != event.kind {
                commands.entity(entity).remove::<Transform>();
                continue;
            }
            if event.flag & 1 != 0 {
                match transform {
                    Some(mut transform) => transform.translation = event.position,
                    None => {
                        commands.entity(entity).insert(Transform {
                            translation: event.position,
                            ..Transform::identity()
                        });
                    }
                }
            } else if transform.is_none() {
                // O：光标命中（tag bit0）→ pos，否则 0。
                let translation = cursor.position();
                commands.entity(entity).insert(Transform {
                    translation,
                    ..Transform::identity()
                });
            }
        }
    }

    // O：逐实体按 kind 映射 scale。
    for (_entity, preview, transform) in &mut query {
        if let Some(mut transform) = transform {
            transform.scale = brush_size(preview.0, &brush_sizes, eraser_multiplier.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_switch_matches_native_jump_table() {
        let sizes = BrushSizes {
            path: 0.8,
            water: 2.0,
            garden: 3.0,
        };
        assert_eq!(brush_size(4, &sizes, 1.0), Vec3::splat(1.0));
        assert_eq!(brush_size(5, &sizes, 1.0), Vec3::splat(1.125));
        assert_eq!(brush_size(7, &sizes, 1.0), Vec3::splat(1.0));
        assert_eq!(brush_size(8, &sizes, 0.5), Vec3::splat(0.5));
        assert_eq!(brush_size(10, &sizes, 1.0), Vec3::splat(1.5));
        assert_eq!(brush_size(0, &sizes, 0.5), Vec3::ONE);
        assert_eq!(brush_size(11, &sizes, 0.5), Vec3::ONE);
    }

    #[test]
    fn kind_6_and_9_use_the_clamped_middle_component() {
        let sizes = BrushSizes {
            path: 0.8,
            water: 2.0,
            garden: 3.0,
        };
        let v6 = brush_size(6, &sizes, 2.0);
        assert_eq!(v6.y, 1.0);
        assert_eq!(v6.x, 2.0 * KIND6_FACTOR * 0.5);
        assert_eq!(v6.z, v6.x);

        let v9 = brush_size(9, &sizes, 0.3);
        // inverse_lerp(0.3,3.0,0.3)=0 → (0*3+1)*0.5 = 0.5
        assert_eq!(v9.x, 0.5);
        assert_eq!(v9.y, 0.5);
        assert_eq!(v9.z, 0.5);
    }

    #[test]
    fn event_layout_matches_native_offsets() {
        let e = OverwriteBrushPreview::default();
        let base = &e as *const _ as usize;
        assert_eq!(&e.flag as *const _ as usize - base, 0x00);
        assert_eq!(&e.position as *const _ as usize - base, 0x04);
        assert_eq!(&e.kind as *const _ as usize - base, 0x10);
        assert_eq!(std::mem::size_of::<OverwriteBrushPreview>(), 0x14);
    }
}
