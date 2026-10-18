//! B `systems/wall/shapes/circle/signifiers/edit_arrow.rs`: circle edit arrow.
//!
//! Evidence: `chapters/systems/wall/shapes/circle/signifiers/edit_arrow.rs.md`
//! (O, 0x140b35120). fresh 反编译/反汇编 2026-10-17：门控 **`tag != 2`**
//! （`CMP byte[subject+0x38],2; JZ skip`）——圆与矩形都发箭头；freehand 不发。
//! 名字格式 = `format!("edit_circle_{}_arrow", display)`（0x142aed850 两片段
//! "edit_circle_" + "_arrow"），经 `rotation_from_normal(subject+0x2c)` 后
//! push 到 AppendOnlyVec。ECS Query 形态（subject +0x20 Vec3 / +0x2c axis /
//! +0x34 rotation）与逐段列写入为 render-host 接线；本函数以 PublicWalls 桶
//! 承载同判别门与载荷。

use bevy_ecs::prelude::*;
use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::ui::signifiers::signifier_create::{UiAppendSignifier, UiSignifierStream};

/// B 0x140b35120 (3-param system).
///
/// 门控修正（LOGIC3 §3.7 / fresh）：`tag != 2`。矩形分支的朝向轴取自
/// `Rectangle2d::axis`（原生 `rotation_from_normal(subject+0x2c)` 的输入面，
/// 语义登记——见模块注释）。
pub fn signifier_edit_circle_arrow(
    public: Res<PublicWalls>,
    mut stream: ResMut<UiSignifierStream>,
) {
    for (_id, state) in public.all() {
        if state.shape().tag() == 2 {
            continue; // freehand 不发箭头；圆(0)/矩形(1) 都发。
        }
        let (position, scale, name) = if let Some(circle) = state.is_circle() {
            (
                glam::Vec3::new(circle.center.x, 0.0, circle.center.y),
                circle.rotation,
                pack2(circle.get_tangent_at_u(0.0)),
            )
        } else if let Some(rectangle) = state.is_rectangle() {
            (
                glam::Vec3::new(rectangle.center.x, 0.0, rectangle.center.y),
                rectangle.axis.y.atan2(rectangle.axis.x),
                pack2(rectangle.axis),
            )
        } else {
            continue;
        };
        stream.write(UiAppendSignifier {
            tag: 5,
            kind: 0,
            position,
            scale,
            name,
        });
    }
}

/// B 0x140b35ba0: component-registry slice (host 待接线).
pub fn register_required_components() -> ([u64; 0], [u64; 0]) {
    ([], [])
}

/// B 0x140002370: clone behavior marker (returns 0).
pub fn clone_behavior() -> u8 {
    0
}

/// 打包两个 f32 到 qword（I：40B 布局裁决后的切线方向承载）。
fn pack2(v: glam::Vec2) -> u64 {
    u64::from(v.x.to_bits()) | (u64::from(v.y.to_bits()) << 32)
}
