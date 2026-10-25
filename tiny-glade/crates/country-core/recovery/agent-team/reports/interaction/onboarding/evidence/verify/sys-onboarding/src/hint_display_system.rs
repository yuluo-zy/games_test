//! hint_display_system（O：动画/冷却/音效/删除/位置/高亮/绘制）。
//!
//! 恢复（evidence/batch2-display.txt）：
//! - `hint_animation_system` 0x141c4f400：逐提示逐属性 `value=clamp(value+dt/0.3*dir,
//!   0,1)`（dir=±1，删除中 -1）、`eased=EasingFunction::apply(value)`、`time+=dt`。
//! - `hint_deletion_cooldown_system` 0x141c4f790：`timer-=dt`，`<=0` →
//!   remove<HintDeleteAfterCooldown> + insert<HintMarkedForDeletion>（warn）。
//! - `hint_audio` 0x141c4fb00：动画 `+4`(eased)==1.0 → PlayUiSound tag 0x15；
//!   `+8`(time)==0.0 → tag 0x14。
//! - `hint_deletion_system` 0x141c4fea0：动画 `+4`(eased)==0.0 → despawn。
//! - `progress_bar_update_system` 0x141c530f0：按 id 0xa/0xb/0xc 取 CameraHintProgress
//!   量化档位（0 / 1 / 2 / 3，阈值 0.4/0.99），档位变 → PlayUiSound tag 0x16。
//! 未实现（分歧点见各函数）：`hint_highlight_shape`（HighlightRoofCmd/HighlightWallCmd
//! 未落库）、`hint_position_update`（HintPlacement 跳转表 7/8 依赖相机矩阵/几何）、
//! `hint_draw`/`render_hints`（mei/toolbar 批次 3）。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;
use country_core::systems::ui::resources::widget_params::PlayUiSound;
use utils::easing_functions::EasingFunction;

use crate::{BindingHintDisplay, HintPlacement, HintProgressBar, HintType};

/// 提示动画（O：HintAnimation，每属性 {value f32@+0, eased f32@+4, time f32@+8}，
/// 步长 0xc；属性数组结构未恢复，本重建取单属性）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintAnimation {
    /// 归一化进度 [0,1]
    pub value: f32,
    /// 缓动值
    pub eased: f32,
    /// 存活时间
    pub time: f32,
}

/// 提示弹簧（O：HintSpring{pos, target, initialized}；position_update 写，
/// initialized@+8）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintSpring {
    pub pos: glam::Vec2,
    pub target: glam::Vec2,
    pub initialized: bool,
}

/// 提示文本（O：HintText；lang_lib lookup 载荷）。
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct HintText {
    pub text: String,
}

/// 删除冷却标记（O：HintDeleteAfterCooldown，f32 计时）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintDeleteAfterCooldown {
    pub timer: f32,
}

/// 待删除标记（O：HintMarkedForDeletion，unit）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintMarkedForDeletion;

/// 冷却表 DAT_143038ae8（按提示 id 索引，O 读 80B）：[0]=0 [1]=0.3 [2]=0 [3]=0
/// [4]=0.3 [5]=0.3 [6]=0.3 [7]=0.3 [8]=0 [9]=0 [10]=0.3 [11]=0.3 [12]=0.3
/// [13]=0 [14..]=0 → 提示 1/4/5/6/7/10/11/12 删除冷却 0.3s。
pub const DELETION_COOLDOWN_TABLE: [f32; 18] = [
    0.0, 0.3, 0.0, 0.0, 0.3, 0.3, 0.3, 0.3, 0.0, 0.0, 0.3, 0.3, 0.3, 0.0, 0.0, 0.0, 0.0, 0.0,
];

/// hint_animation_system（O 0x141c4f400）：`value = clamp(value + dt/0.3*dir, 0, 1)`，
/// `eased = EasingFunction::apply(value)`，`time += dt`；dir 删除中 -1（有
/// HintMarkedForDeletion）否则 +1。EasingFunction 变体未从反编译解析（I：取
/// CubicInOut，与 utils 恢复一致）。
pub fn hint_animation_system(
    time: Res<GameTime>,
    mut query: Query<(&mut HintAnimation, Option<&HintMarkedForDeletion>)>,
) {
    let dt = time.delta;
    for (mut anim, deleting) in &mut query {
        let dir = if deleting.is_some() { -1.0 } else { 1.0 };
        let mut value = anim.value + dt / 0.3 * dir;
        value = value.clamp(0.0, 1.0);
        anim.value = value;
        anim.eased = EasingFunction::CubicInOut.apply(value);
        anim.time += dt;
    }
}

/// hint_deletion_cooldown_system（O 0x141c4f790）：timer -= dt；<=0 →
/// remove<HintDeleteAfterCooldown> + insert<HintMarkedForDeletion>（warn）。
pub fn hint_deletion_cooldown_system(
    time: Res<GameTime>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut HintDeleteAfterCooldown)>,
) {
    let dt = time.delta;
    for (entity, mut cooldown) in &mut query {
        cooldown.timer -= dt;
        if cooldown.timer <= 0.0 {
            let mut cmd = commands.entity(entity);
            cmd.remove::<HintDeleteAfterCooldown>();
            cmd.insert(HintMarkedForDeletion);
        }
    }
}

/// hint_audio（O 0x141c4fb00）：**有 HintMarkedForDeletion 且** eased(+4)==1.0 →
/// PlayUiSound tag 0x15(21)；time(+8)==0.0 → tag 0x14(20)（无门控）。
/// PlayUiSound 判别值（tag）由 country-core 承载（当前 widget_params::PlayUiSound
/// 为 unit，数值边界登记）。
pub fn hint_audio(
    mut sounds: EventWriter<PlayUiSound>,
    query: Query<(&HintAnimation, Option<&HintMarkedForDeletion>)>,
) {
    for (anim, deleting) in &query {
        if deleting.is_some() && anim.eased == 1.0 {
            // O：PlayUiSound tag 0x15（21），仅在满显后被标记删除（播放"关闭"音）。
            let _ = sounds.write(PlayUiSound);
        }
        if anim.time == 0.0 {
            // O：PlayUiSound tag 0x14（20）。
            let _ = sounds.write(PlayUiSound);
        }
    }
}

/// hint_deletion_system（O 0x141c4fea0）：有 HintMarkedForDeletion 且 eased==0.0 →
/// despawn。
pub fn hint_deletion_system(
    mut commands: Commands,
    mut query: Query<(Entity, &HintAnimation), With<HintMarkedForDeletion>>,
) {
    for (entity, anim) in &mut query {
        if anim.eased == 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// hint_highlight_shape（O 0x141c50120）：动画>0.9 且 HintType∈{1,2,6,7,8,9}
/// （`*piVar1-6<5 || ==3` 截断）且 +0x11!=0 → PublicWalls::roof_entity → 向
/// HighlightRoofCmd/HighlightWallCmd 各写 tag 2 命令。
/// **未实现**：HighlightRoofCmd/HighlightWallCmd 未落库；本函数标未实现，不伪造
/// 高亮事件。
pub fn hint_highlight_shape(
    _walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    _query: Query<(Entity, &crate::HintType, &HintAnimation, &crate::HintPlacement)>,
) {
    // 未实现：见模块注释。
}

/// hint_position_update（O 0x141c50510）：边距 = WindowSize/1000 ×{20,30}；按
/// HintPlacement：6=光标+2×边距；7=实体世界点（CameraMatrices::eye_right +
/// world_space_to_screen_space）；8=Rectangle2d::as_points3 四角→屏幕→min/max；
/// 10=屏幕中心 window*0.5；其余跳转表未恢复。maxps/minps 夹取，time_continuous_lerp
/// 平滑写 HintSpring{pos,target,initialized}。
/// **未实现**：HintPlacement 跳转表未恢复 + 相机/几何依赖未接线。
pub fn hint_position_update(
    _window: Res<country_core::resources::window::WindowSize>,
    _query: Query<(&mut HintSpring, &HintPlacement, &crate::HintType, &crate::HintLayoutMemo)>,
) {
    // 未实现：见模块注释。
}

/// hint_draw（O 0x141c51a60，mei IMGUI）：动画>0 → new_at_pos + 模板 + 尺寸 +
/// with_slot 分槽渲染绑定/文本/进度条/交互。
/// **未实现**：mei/toolbar（批次 3）未落库；不伪造 UI 渲染。
pub fn hint_draw(
    _query: Query<(Entity, &HintSpring, &HintAnimation, &HintPlacement, &HintType)>,
) {
    // 未实现：见模块注释。
}

/// render_hints（O 0x141c52700）：按判别 switch：Empty 无操作；Graphic →
/// me_image(floorf(t/frames))；Key → keycode_to_str（0xc3 未绑→占位
/// DAT_142f147af）+ 模板；Mouse → mouse_button_to_string 闭包；0xc8 → 4 键
/// keycode_to_str + 4×with_slot；Steam → panic_fmt("Call resolve_steam() and
/// cleanup() on BindingHintDisplay before sending it in.")；列表 → child_with_global_id
/// + 递归 render_hints。
/// **未实现**：mei/toolbar（批次 3）未落库；switch/panic 串已核对，渲染 DSL 不伪造。
pub fn render_hints(_display: &BindingHintDisplay) {
    // 未实现：见模块注释。
}

/// progress_bar_update_system（O 0x141c530f0）：Query<(HintType,HintProgressBar)> +
/// Res<CameraHintProgress> + EventWriter<PlayUiSound>。取 id 0xa/0xb/0xc 的进度，
/// 量化：p==0→0；0<p≤0.4→1；0.4<p≤0.99→2；p<0 或 p>0.99→3。档位变化 →
/// PlayUiSound tag 0x16(22)。`core::panicking::panic()` 对非相机 id。
pub fn progress_bar_update_system(
    mut query: Query<(&crate::HintType, &mut HintProgressBar)>,
    progress: Res<crate::hint_camera_progress::CameraHintProgress>,
    mut sounds: EventWriter<PlayUiSound>,
) {
    for (hint_type, mut bar) in &mut query {
        // 非 0xa/0xb/0xc 的提示：原 panic。
        let value = match hint_type {
            crate::HintType::CameraPan => progress.progress_pan(),
            crate::HintType::CameraRotate => progress.progress_rotate(),
            crate::HintType::CameraZoom => progress.progress_zoom(),
            _ => panic!("progress_bar_update_system: non-camera hint"),
        };
        let level = quantize(value);
        if level != bar.level {
            let _ = sounds.write(PlayUiSound); // O：tag 0x16(22)
            bar.level = level;
        }
    }
}

/// 进度量化（O 0x141c530f0）：p==0→0；0<p≤0.4→1；0.4<p≤0.99→2；否则→3。
pub fn quantize(p: f32) -> u8 {
    if p == 0.0 {
        0
    } else if p < 0.0 || p > 0.99 {
        3
    } else if p > 0.4 {
        2
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::quantize;

    #[test]
    fn quantize_thresholds() {
        // O 0x141c530f0：0→0；(0,0.4]→1；(0.4,0.99]→2；<0 或 >0.99→3。
        assert_eq!(quantize(0.0), 0);
        assert_eq!(quantize(0.2), 1);
        assert_eq!(quantize(0.4), 1);
        assert_eq!(quantize(0.5), 2);
        assert_eq!(quantize(0.99), 2);
        assert_eq!(quantize(1.2), 3);
        assert_eq!(quantize(-1.0), 3);
    }
}
