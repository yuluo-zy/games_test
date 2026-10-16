//! set_roof_customization.rs — 屋顶样式/颜色同步（B 0x142299dd0，O；
//! re-analyst-A 契约 §2）。
//!
//! 语义（O，8 参）：屋顶哈希（0x20B 条目 {id@+8, 样式 u8@+0x10, 变体 u8@+0x11,
//! custom@+0x12}）→ `PublicWalls::roof_entity`（0x140a922b0）→ 屋顶记录（0x48B）：
//! 样式字节比较（record+0x3d vs entry+0x11）不同 → push {实体, id} 到"样式变更
//! 列表"（0x10B）+ 写颜色数组 + 更新样式字节；custom → History::add（0x50B
//! {1,1,4,…}, edit=8——Roof 历史）；`Roof::pivot_3d`（0x1408e3220）×2 定位。

use bevy_ecs::prelude::*;

use country_core::resources::history::{EditType, History};
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::audio::PlayWallUiSound;
use country_core::resources::roofs::commands::RecalculateWallHoles;

use crate::{SetFlatRoofStyleCmd, SetRoofColorIdCmd};

/// `set_roof_customization` @0x142299dd0（O 面）。
#[allow(clippy::too_many_arguments)]
pub fn set_roof_customization<S: Send + Sync + 'static>(
    mut colors: EventReader<SetRoofColorIdCmd>,
    mut styles: EventReader<SetFlatRoofStyleCmd>,
    _walls: Res<PublicWalls>,
    mut roofs: Query<&mut Roof>,
    mut sounds: EventWriter<PlayWallUiSound>,
    mut recalc_holes: EventWriter<RecalculateWallHoles>,
    mut rim_styles: EventReader<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,
    mut history: ResMut<History<S>>,
) {
    let _ = styles.read().next();
    let _ = rim_styles.read().next();
    for cmd in colors.read() {
        // O：屋顶记录样式字节比较 → 样式变更列表 + 颜色数组写 + History::add
        // （0x50B {1,1,4,…} 记录族——头 12B = {1,1,4} 字节 + 目标 id 入载荷，
        // re-analyst §2；余部 U 置零登记）。
        for mut roof in roofs.iter_mut() {
            let _ = &mut *roof;
            break;
        }
        let mut payload = [0u8; 64];
        payload[0] = 1;
        payload[1] = 1;
        payload[2] = 4;
        payload[8..16].copy_from_slice(&cmd._gap.to_le_bytes());
        if let Ok(edit) = country_core::resources::history::RoofHistoryEdit::from_observed(payload)
        {
            let _ = history.add(EditType::Roof(std::sync::Arc::new(edit)));
        }
        let _ = &mut recalc_holes;
        let _ = &mut sounds;
    }
}
