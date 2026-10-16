//! set_window_light.rs — 窗灯颜色写入（B 0x142298260，O；re-analyst-A 契约 §2）。

use bevy_ecs::prelude::*;

use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::systems::audio::PlayWallUiSound;

use crate::SetWindowLightCmd;

/// `set_window_light` @0x142298260（O 面：`as_window_mut`（0x1408393b0）→ 窗灯色
/// + CachedDecoratorTransform（0x140a1a9f0）+ push（tag 候选 9，I）。
pub fn set_window_light(
    mut storage: ResMut<DecoratorStorage>,
    mut cmds: EventReader<SetWindowLightCmd>,
    mut sounds: EventWriter<PlayWallUiSound>,
    _transforms: Res<CachedDecoratorTransforms>,
) {
    for cmd in cmds.read() {
        // O：`as_window_mut`（0x1408393b0）→ 窗灯色写（DecoratorExtraInfo 载荷
        // 面）；0x20B 记录（tag 候选 9，I——core 内表字段未恢复，登记 GAP）。
        let _ = storage.lookup_decorator_info_mut(country_core::resources::walls::decorator::DecoratorId(
            cmd._gap as u32,
        ));
        let _ = &mut sounds;
    }
}
