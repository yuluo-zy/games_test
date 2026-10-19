//! set_flag_connection_n_chimney_smoke.rs — 旗连接/烟囱烟开关（B 0x142298970，
//! 929B；restorer-state-color 2026-10-17 fresh 反编译重建）。
//!
//! 真实控制流（双事件源，均 stride 0x10 = E={id u32, value u8, custom u8}）：
//! - 旗（in_RDX）：`lookup_decorator_info_mut`（unwrap）→ `as_flag()` 写
//!   `FlagExtra.0 = event+0xc`；`event+0xd != 0` → `position(id)` 推 0x20B 记录
//!   tag=**9**。
//! - 烟（in_R8）：`as_chimney()` 写 `payload+4 = event+0xc`（core `ChimneyExtra`
//!   未建模 +4 字节，分歧点）；`event+0xd != 0` → 推 0x20B 记录 tag=**0xf**。
//! 注意：本模块的 `SetFlagConnectionCmd`/`SetChimneySmokeCmd` 按消费面重建（core
//! `walls::decorator` 同名占位 `_gap: u64` 字段未恢复，登记分歧）。

use bevy_ecs::prelude::*;

use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::systems::audio::PlayWallUiSound;

use crate::{SetChimneySmokeCmd, SetFlagConnectionCmd};

/// `set_flag_connection_and_chimney_smoke` @0x142298970（929B，重建）。
#[allow(clippy::too_many_arguments)]
pub fn set_flag_connection_and_chimney_smoke(
    mut storage: ResMut<DecoratorStorage>,
    mut flags: EventReader<SetFlagConnectionCmd>,
    mut smokes: EventReader<SetChimneySmokeCmd>,
    mut sounds: EventWriter<PlayWallUiSound>,
    transforms: Res<CachedDecoratorTransforms>,
) {
    for cmd in flags.read() {
        let _info = storage
            .lookup_decorator_info_mut(DecoratorId(cmd.id))
            .unwrap();
        let mut extra = _info.extra();
        let flag = extra.as_flag_mut();
        flag.0 = cmd.value;
        _info.set_extra(extra);
        if cmd.custom != 0 {
            let position = transforms.position(cmd.id);
            sounds.write(PlayWallUiSound {
                kind: 9,
                selector: 0,
                value: 0.0,
                position,
            });
        }
    }
    for cmd in smokes.read() {
        let info = storage
            .lookup_decorator_info_mut(DecoratorId(cmd.id))
            .unwrap();
        // The canonical core payload now includes the original +4 smoke byte.
        // Preserve the other union bytes while applying the actual command.
        let mut extra = info.extra();
        extra.as_chimney_mut().smoke = cmd.value;
        info.set_extra(extra);
        // O：custom == 1（反编译 `event+0xd == '\x01'`）→ tag 0xf 记录。
        if cmd.custom == 1 {
            let position = transforms.position(cmd.id);
            sounds.write(PlayWallUiSound {
                kind: 0xf,
                selector: 0,
                value: 0.0,
                position,
            });
        }
    }
}
