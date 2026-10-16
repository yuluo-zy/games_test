//! 玩家楼梯支柱提议处理（construct_playermade_stairs/pillars_process_proposals.rs）。
//!
//! 候选参数面（skeleton 富化，原 6 参）：`(ResMut<StairsPillarProposals>,
//!  Res<UiSignifierStream>, ResMut<RaycastWorld>, ResMut<StairColliderRegistry>,
//!  ResMut<RtWorld>, …)` —— 支柱提议 → 碰撞/rt 注册。RtWorld 与
//! StairColliderRegistry 属 country-core resources/rt 与 stairs/collider
//! （PDB 路径 `crates/country-core/src/resources/rt/mod.rs` 等），尚未落
//! workspace —— 参数面按证据登记、接线待该域落地。
use country_core::resources::stairs_walls::StairsPillarProposals;
use country_core::systems::collision::world::RaycastWorld;
use bevy_ecs::prelude::*;

/// B 支柱提议处理（0x14077xx 族；building-production）。
pub fn process_pillar_proposals(
    mut proposals: ResMut<StairsPillarProposals>,
    mut world: ResMut<RaycastWorld>,
    mut commands: Commands,
) {
    // 44B 提议记录 → 支柱实体 + 地形高度采样（collider/rt 注册参数面
    // 按原候选登记，接线待 resources/rt 域落地）。
    let mut spawned = Vec::new();
    proposals.drain_all(&mut spawned);
    for _proposal in spawned {
        let _ = &mut *world;
        let _ = commands.spawn_empty();
    }
}
