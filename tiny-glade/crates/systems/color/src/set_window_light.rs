//! set_window_light.rs — 窗灯颜色写入（B 0x142298260，582B；restorer-state-color
//! 2026-10-17 fresh 反编译重建）。
//!
//! 真实控制流：事件循环（stride 0x18）。逐事件：
//! 1. `DecoratorStorage::lookup_decorator_info_mut(id)`（unwrap，反编译
//!    `extraout_RAX==0 → unwrap_failed`）。
//! 2. `DecoratorExtraInfo::as_window()` → 写 `WindowExtra.f = event+0xc`。
//! 3. `event+0x10==1`（custom）：`sub = type(+0x46)-0x38`，`sub>7 → 6`；
//!    `tag = 10`，`(0xcf >> sub & 1)==0` 时 `sub==4 → 0xd`，否则 `8`；
//!    `CachedDecoratorTransforms::position(id)` → 推 0x20B 记录（tag=uVar21，
//!    value=DecoratorInfo+0x28）。

use bevy_ecs::prelude::*;

use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::systems::audio::PlayWallUiSound;

use crate::SetWindowLightCmd;

/// `set_window_light` @0x142298260（582B，重建）。
#[allow(clippy::too_many_arguments)]
pub fn set_window_light(
    mut storage: ResMut<DecoratorStorage>,
    mut cmds: EventReader<SetWindowLightCmd>,
    mut sounds: EventWriter<PlayWallUiSound>,
    transforms: Res<CachedDecoratorTransforms>,
) {
    for cmd in cmds.read() {
        let info = storage
            .lookup_decorator_info_mut(DecoratorId(cmd.id))
            .unwrap();
        // O：`DecoratorExtraInfo::as_window()` 写 +0x0（WindowExtra 现为
        // `pub struct WindowExtra(pub u32)`；原生整 4 字节写事件 +0xc 原始位模式）。
        let mut extra = info.extra();
        let window = extra.as_window_mut();
        window.0 = cmd.value;
        info.set_extra(extra);
        // O：custom == 1（反编译 `event+0x10 == '\x01'`）→ 子类型 tag 判定
        // （`type-0x38`，`>7→6`；`0xcf>>sub&1==0` → sub==4→0xd 否则 8；默认 10）。
        if cmd.custom == 1 {
            let mut sub = info.ty.wrapping_sub(0x38);
            if sub > 7 {
                sub = 6;
            }
            let mut tag = 10u8;
            if (0xcfu8 >> (sub & 0x1f)) & 1 == 0 {
                tag = if sub == 4 { 0xd } else { 8 };
            }
            let position = transforms.position(cmd.id);
            // O：推 0x20B 记录（+0x08 kind=tag、+0x0c value=DecoratorInfo+0x28、
            // +0x10 position）。selector(+0x09) 字节未解出，登记 U。
            sounds.write(PlayWallUiSound {
                kind: tag,
                selector: 0,
                value: f32::from_bits(info.u28),
                position,
            });
        }
    }
}
