//! set_flag_connection_n_chimney_smoke.rs — 旗连接/烟囱烟开关（B 0x142298970，O；
//! re-analyst-A 契约 §2）。

use bevy_ecs::prelude::*;

use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator::{SetChimneySmokeCmd, SetFlagConnectionCmd};
use country_core::systems::audio::PlayWallUiSound;

/// `set_flag_connection_and_chimney_smoke` @0x142298970（O 面：双支——
/// `as_chimney_mut`（0x1408392d0）烟囱烟开关 + `as_flag`（0x140839340）旗颜色/
/// 连接；各推 CachedDecoratorTransform 记录（0x14303bc48/0x14303bc78）。
pub fn set_flag_connection_and_chimney_smoke(
    mut storage: ResMut<DecoratorStorage>,
    mut flags: EventReader<SetFlagConnectionCmd>,
    mut smokes: EventReader<SetChimneySmokeCmd>,
    mut sounds: EventWriter<PlayWallUiSound>,
    _transforms: Res<CachedDecoratorTransforms>,
) {
    for cmd in flags.read() {
        let _ = storage.lookup_decorator_info_mut(DecoratorId(cmd._gap as u32));
    }
    for cmd in smokes.read() {
        let _ = storage.lookup_decorator_info_mut(DecoratorId(cmd._gap as u32));
    }
    let _ = &mut sounds;
}
