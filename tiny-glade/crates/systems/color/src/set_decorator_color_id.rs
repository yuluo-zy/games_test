//! set_decorator_color_id.rs — 装饰物颜色写入（B 0x1422af0c0，O；re-analyst-A
//! 契约 §2）。
//!
//! 语义（O）：DecoratorStorage 遍历（lookup_decorator_info_mut 0x140a1cbf0 ×9）；
//! `DecoratorExtraInfo::as_stair`（0x1408394b0）→ 命中写 `stair.color@+0xc`；
//! custom 时 CachedDecoratorTransform（0x140a1a9f0）→ 推 0x20B 记录 tag 10。

use bevy_ecs::prelude::*;

use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::systems::audio::PlayWallUiSound;

use crate::{SetDecoratorBrightnessCmd, SetDecoratorColorIdCmd, SetLadderColorIdCmd, SetMagicColorIdCmd, SetStairRailingCmd};

/// `set_decorator_color_id` @0x1422af0c0（O 面）。
#[allow(clippy::too_many_arguments)]
pub fn set_decorator_color_id(
    mut storage: ResMut<DecoratorStorage>,
    mut colors: EventReader<SetDecoratorColorIdCmd>,
    mut brightness: EventReader<SetDecoratorBrightnessCmd>,
    mut ladders: EventReader<SetLadderColorIdCmd>,
    mut railings: EventReader<SetStairRailingCmd>,
    mut magic: EventReader<SetMagicColorIdCmd>,
    mut sounds: EventWriter<PlayWallUiSound>,
    _stairs: Res<country_core::resources::stairs::stairs_state::StairsState>,
    _transforms: Res<CachedDecoratorTransforms>,
) {
    let _ = brightness.read().next();
    let _ = ladders.read().next();
    let _ = railings.read().next();
    let _ = magic.read().next();
    for cmd in colors.read() {
        // O：DecoratorStorage lookup → as_stair 命中 → stair.color 写 + 0x20B
        // 记录（tag 10）。载荷字段 U（typed gap）。
        // O：DecoratorStorage lookup → as_stair 命中 → stair.color 写 + 0x20B
        // 记录（tag 10；AABB/max_y 面 U——core 内表字段未恢复，登记 GAP）。
        if let Some(info) = storage.lookup_decorator_info_mut(DecoratorId(cmd._gap as u32)) {
            let _ = info;
        }
        let _ = &mut sounds;
    }
}
