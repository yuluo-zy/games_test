//! `animation.rs` — 指示符动画（B 0x140a69340 `signifier_animation_t` / 0x140a69c30
//! `signifier_animation`，O）。
//!
//! 证据（2026-10-17 fresh 反编译/机器码）：
//! - `SignifierAnimation` 组件元素 0x1c B：`byte@+8`、`byte@+9`、`f32@+0xc`（value）、
//!   `f32@+0x10`（duration）、`f32@+0x14`（t）、`f32@+0x18`（eased）。
//! - `signifier_animation_t`：`duration != 0 || NaN` 时
//!   `t' = clamp(t + (1/duration)·tick, 0, 1)`，否则 `t' = 反向?0:1`；
//!   `eased = EasingFunction::apply(t)`；tick 累加到查询上的可选组件（非本组件）。
//! - `signifier_animation`：Query<(&SignifierAnimation, &mut Transform,
//!   Option<&SignifierId>, Option<&SignifierParentToVolumeOrigin>)> + `Res<UiState>`。
//!   门 `byte@+8 == 0` 时：`fVar32 = eased·value`、`fVar33 = byte@+9==0 ? value : fVar32`，
//!   写 `Transform.scale = (fVar33, fVar32, fVar33)`（+0x1c/+0x20/+0x24）。随后按
//!   UiState 配置的 kind（`*(*[UiState+0x20]+0x98)`）switch 写 `Transform.translation`
//!   （+0x10..+0x18）：Sphere center、PlaneDisc point、SphereCloud indexed center。
//!   原有界跳表仅八项；旧反编译 8..16 是损坏输出，不能作为算法分支。
use bevy_ecs::prelude::*;
use glam::Vec3;

use crate::systems::ui::resources::ui_state::UiState;
use crate::utils::components::transform::Transform;

use super::signifier_create::SignifierId;

/// `SignifierAnimation` 组件（O 布局 0x1c B）。
#[repr(C)]
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct SignifierAnimation {
    /// +0x00..+0x08: canonical EasingFunction tag and optional f32 payload.
    pub head: [u8; 8],
    /// +0x08：写入门（==0 时 `signifier_animation` 更新 scale）。
    pub flag_8: u8,
    /// +0x09：scale.x/z 选取门（0 → value，1 → eased·value）。
    pub flag_9: u8,
    /// +0x0a..+0x0c：填充。
    pub pad_a: [u8; 2],
    /// +0x0c：value（动画输入值）。
    pub value: f32,
    /// +0x10：动画时长（秒）。
    pub duration: f32,
    /// +0x14：当前进度 t。
    pub t: f32,
    /// +0x18：缓动派生值（`EasingFunction::apply(t)`）。
    pub eased: f32,
}

impl Default for SignifierAnimation {
    fn default() -> Self {
        Self {
            head: [0; 8],
            flag_8: 0,
            flag_9: 0,
            pad_a: [0; 2],
            value: 0.0,
            duration: 1.0,
            t: 0.0,
            eased: 0.0,
        }
    }
}

/// 指示符挂到 volume 原点（O：查询 `Option<&SignifierParentToVolumeOrigin>`；
/// 组件布局未取证，仅查询存在性/承载需要）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SignifierParentToVolumeOrigin;

/// `signifier_animation_t` @0x140a69340（O）：每帧推进所有指示符动画时间。
///
/// 原版对每个组件：`duration != 0.0 || duration.is_nan()` 时
/// `t' = clamp(t + (1/duration)·tick, 0, 1)`；否则 `t' = 反向?0:1`，
/// 再 `eased = EasingFunction::apply(t')`，tick 累加到可选组件。
/// （该函数此前已判等[结构]；本轮只按 fresh 修正组件布局，不改其控制流。）
impl SignifierAnimation {
    pub fn easing(&self) -> ::utils::easing_functions::EasingFunction {
        use ::utils::easing_functions::EasingFunction as E;
        let tag = u32::from_le_bytes(self.head[..4].try_into().unwrap());
        let value = f32::from_le_bytes(self.head[4..].try_into().unwrap());
        match tag {
            0 => E::CircularInOut,
            1 => E::CubicInOut,
            2 => E::SineInOut,
            3 => E::Bias(value),
            4 => E::ElasticInOut,
            5 => E::BackOut,
            6 => E::CubicOut,
            _ => panic!("invalid SignifierAnimation easing tag {tag}"),
        }
    }
    /// Original140a69998..140a69a1a, independently checked against RNE.
    pub fn advance(&mut self, delta: f32, marked: bool) {
        self.t = if self.duration != 0.0 {
            let signed_delta = if marked { -delta } else { delta };
            let raw = (1.0 / self.duration) * signed_delta + self.t;
            let lower = if 0.0 > raw { 0.0 } else { raw };
            if 1.0 < lower { 1.0 } else { lower }
        } else if marked {
            0.0
        } else {
            1.0
        };
        self.eased = self.easing().apply(self.t);
    }
}

pub fn signifier_animation_t(
    time: Res<crate::resources::time::GameTime>,
    mut query: Query<(
        &mut SignifierAnimation,
        Option<&super::signifier_mark_for_delete::SignifierMarkedForDeletion>,
        Option<&mut super::signifier_mark_for_delete::SignifierMinimumLifespan>,
    )>,
) {
    for (mut anim, marked, lifespan) in &mut query {
        anim.advance(time.delta, marked.is_some());
        if let Some(mut lifespan) = lifespan {
            lifespan.t += time.delta;
        }
    }
}

/// Original140a69c30: scale and parent-to-volume-origin are independent gates.
/// The bounded geometry jump table142ad55ac contains only eight variants:
/// Sphere/Cloud/PlaneDisc update translation; the other five preserve it.
pub fn signifier_animation(
    mut query: Query<(
        &SignifierAnimation,
        ComponentWrite<Transform>,
        Option<&SignifierId>,
        Option<&SignifierParentToVolumeOrigin>,
    )>,
    ui_state: Res<UiState>,
) {
    use crate::systems::ui::resources::widget_params::WidgetGeometry;
    for (anim, mut transform, id, parent_to_origin) in &mut query {
        if anim.flag_8 == 0 {
            let value = anim.value;
            let scaled = anim.eased * value;
            let x = if anim.flag_9 == 0 { value } else { scaled };
            transform.scale = Vec3::new(x, scaled, x);
        }
        let Some(id) = id else {
            continue;
        };
        if parent_to_origin.is_none()
            || ui_state.hover_byte() == 0
            || id.1 != ui_state.active_widget()
        {
            continue;
        }
        let params = ui_state
            .active_params()
            .expect("native active signifier WidgetParams unwrap");
        let position = match params.resolved_geometry().as_ref() {
            WidgetGeometry::Sphere { center, .. } => Some(*center),
            WidgetGeometry::SphereCloud { centers, .. } => {
                let hit = ui_state
                    .widget_context(ui_state.active_widget())
                    .intersecting();
                let index = u64::from_le_bytes(hit[24..32].try_into().unwrap()) as usize;
                Some(centers[index]) // Original bounds panic is part of the contract.
            }
            WidgetGeometry::PlaneDisc { point, .. } => Some(*point),
            _ => None,
        };
        if let Some(position) = position {
            transform.translation = position;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_layout_matches_native_offsets() {
        assert_eq!(std::mem::size_of::<SignifierAnimation>(), 0x1c);
        let a = SignifierAnimation {
            flag_8: 1,
            flag_9: 2,
            value: 3.0,
            duration: 4.0,
            t: 5.0,
            eased: 6.0,
            ..Default::default()
        };
        let base = &a as *const _ as usize;
        assert_eq!(&a.value as *const _ as usize - base, 0x0c);
        assert_eq!(&a.duration as *const _ as usize - base, 0x10);
        assert_eq!(&a.t as *const _ as usize - base, 0x14);
        assert_eq!(&a.eased as *const _ as usize - base, 0x18);
    }

    #[test]
    fn scale_mapping_uses_flag_9() {
        let anim = SignifierAnimation {
            value: 2.0,
            eased: 0.25,
            ..Default::default()
        };
        let mut t = Transform::identity();
        let scaled = anim.eased * anim.value;
        let x = if anim.flag_9 == 0 { anim.value } else { scaled };
        t.scale = Vec3::new(x, scaled, x);
        assert_eq!(t.scale, Vec3::new(2.0, 0.5, 2.0));
    }

    #[test]
    fn parent_origin_is_independent_of_scale_gate_and_matches_full_widget_identity() {
        use crate::systems::ui::resources::{
            ui_state::{UiInteraction, WidgetId},
            widget_params::{WidgetGeometry, WidgetParams},
        };
        let target = Vec3::new(3.0, 4.0, 5.0);
        for geometry in [
            WidgetGeometry::Sphere {
                center: target,
                radius: 2.0,
            },
            WidgetGeometry::SphereCloud {
                centers: vec![Vec3::ZERO, target],
                radius: 1.0,
            },
            WidgetGeometry::PlaneDisc {
                point: target,
                normal: Vec3::Y,
                radius: 1.0,
            },
        ] {
            let mut world = World::new();
            let mut ui = UiState::at_startup();
            let mut hit = [0; 56];
            hit[24..32].copy_from_slice(&1u64.to_le_bytes());
            ui.replace_interaction_full(
                UiInteraction {
                    native_tag: 1,
                    native_phase: 1,
                    widget: WidgetId(11),
                },
                Some(std::sync::Arc::new(
                    WidgetParams::default().geometry(geometry),
                )),
                hit,
            );
            world.insert_resource(ui);
            let animation = SignifierAnimation {
                flag_8: 1,
                ..Default::default()
            };
            let matching = world
                .spawn((
                    animation,
                    Transform::identity(),
                    SignifierId(7, WidgetId(11)),
                    SignifierParentToVolumeOrigin,
                ))
                .id();
            let other_widget = world
                .spawn((
                    animation,
                    Transform::identity(),
                    SignifierId(7, WidgetId(12)),
                    SignifierParentToVolumeOrigin,
                ))
                .id();
            let mut schedule = Schedule::default();
            schedule.add_systems(signifier_animation);
            schedule.run(&mut world);
            let transform = world.get::<Transform>(matching).unwrap();
            assert_eq!(transform.translation, target);
            assert_eq!(
                transform.scale,
                Vec3::ONE,
                "scale-disabled animation still follows volume origin"
            );
            assert_eq!(
                world.get::<Transform>(other_widget).unwrap().translation,
                Vec3::ZERO
            );
        }
    }
}

use crate::render::extract::ComponentWrite;
