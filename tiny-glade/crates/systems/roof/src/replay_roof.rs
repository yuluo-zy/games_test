//! 屋顶回放（replay_roof.rs）。
//!
//! - `replay_roof`（0x141a37ad0，824B/338）。真逻辑（fresh decomp O）：
//!   `InputStream::sync_point_count()`；`ReplayBuffer::front_mut_bucket()`；扫描桶
//!   元素（stride 0x30，首字节=5 RoofEdit），内联 `impl$0::process()` 后
//!   `元素[0x1d]==1`（完成）收集下标；反转下标 Vec；从后往前 `Vec::remove` 并按
//!   tag drop（0/1/2=RasterEdit,3=TerrainEdit,4=WallReplayCmd,5=RoofEdit,
//!   6/7=DecoratorHistoryEdit,8=TreeEdit,9=CrashMiscInfo,default=PhysicalSize）。
//! - `process`（0x1421b48e0）：RoofReplayCmd::process 状态机（case0/1 Create/Delete
//!   按 undo 发 0x40/0x18 事件；case2 VisualEdit 按 progress 线性混合 prev/new
//!   profile/height/ridge/eave/tip 发 0x40 事件(mode=1)；case3/4 Direction/
//!   FlatRimStyle 仅 elapsed==0 发 0x18 事件，推进并 clamp，完成置 in_RCX[0x15]=1）。
//!
//! 判等（经 core 委托）：`RoofReplayCommand::advance`（shared building modules 重建，
//! 与 process 状态机逐条一致——`reconstruction/roof/replay.rs` 验证链）+
//! `advance_roof_bucket` 保留策略（`state.bytes()[21] != 1` 完成移除）。原桶扫描
//! 内联 process 的完成位在原始结构 @0x1d，core VisualReplayState 映射到
//! bytes[21]（I 级重建差异，core 文档已登记）。`sync_point_count()` 为调度同步
//! 外围调用，本实现不重复。
use country_core::resources::history::building_transactions::BuildingReplayOutput;
use country_core::resources::roofs::creation::RoofCreateRequestObserved;
use country_core::resources::roofs::customization::FlatRoofRimRequest;
use country_core::resources::roofs::geometry::replay::ObservedReplayEvent;
use country_core::resources::walls::publication_fields::WallId as ObservedWallId;
use country_core::resources::{
    history::{HistoryReplayError, ReplayBuffer, ReplayCommand},
    roofs::commands::*,
    time::GameTime,
    walls::WallId,
};
use country_core::systems::history::HistoryReplayFailed;
use bevy_ecs::prelude::*;
use crate::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd;

/// B 0x141a37ad0：只访问前桶的 Roof 命令并直接写入命令事件。
pub fn replay_roof(
    time: Res<GameTime>,
    mut replay: ResMut<ReplayBuffer>,
    mut create: EventWriter<CreateRoofCmd>,
    mut delete: EventWriter<DeleteRoofCmd>,
    mut visual: EventWriter<EditRoofCmd>,
    mut ridge_dir: EventWriter<SwitchRoofRidgeDirCmd>,
    mut rim: EventWriter<SetFlatRoofRimStyleCmd>,
    mut failures: EventWriter<HistoryReplayFailed>,
) {
    let dt = time.delta * replay.speed;
    let Some(bucket) = replay.front_mut_bucket() else {
        return;
    };
    for command in &mut bucket.commands {
        let ReplayCommand::Roof(command) = command else {
            continue;
        };
        match command.advance(dt) {
            Ok(BuildingReplayOutput::Discrete(event)) => {
                process(&event, &mut create, &mut delete, &mut ridge_dir, &mut rim);
            }
            Ok(BuildingReplayOutput::Visual(event)) => {
                visual.write(EditRoofCmd(event));
            }
            Ok(BuildingReplayOutput::NoEvent) => {}
            Err(error) => {
                failures.write(HistoryReplayFailed {
                    command: None,
                    error: HistoryReplayError::Roof(error),
                });
            }
        }
    }
    bucket.commands.retain(|command| {
        !matches!(command,
        ReplayCommand::Roof(command) if command.state().bytes()[21] == 1)
    });
}

/// B 0x1421b48e0：`ReplayRoofEvent::process(dt, writers)` 的离散分支。
/// tag 判别在观测事件字节 +0x10（Create/Delete/RidgeDir/FlatRimStyle）。
pub fn process(
    event: &ObservedReplayEvent,
    create: &mut EventWriter<CreateRoofCmd>,
    delete: &mut EventWriter<DeleteRoofCmd>,
    ridge_dir: &mut EventWriter<SwitchRoofRidgeDirCmd>,
    rim: &mut EventWriter<SetFlatRoofRimStyleCmd>,
) {
    match event {
        ObservedReplayEvent::Create { .. } => {
            create.write(CreateRoofCmd(
                RoofCreateRequestObserved::from_replay_event(event).unwrap(),
            ));
        }
        ObservedReplayEvent::Delete { wall_index } => {
            delete.write(DeleteRoofCmd(WallId(*wall_index)));
        }
        ObservedReplayEvent::Direction {
            wall_index,
            direction: value,
        } => {
            ridge_dir.write(SwitchRoofRidgeDirCmd {
                wall: WallId(*wall_index),
                direction: *value,
                previous_direction: *value,
            });
        }
        ObservedReplayEvent::FlatRimStyle { wall_index, style } => {
            rim.write(SetFlatRoofRimStyleCmd(FlatRoofRimRequest {
                wall_id: ObservedWallId(*wall_index),
                from: *style,
                to: *style,
                observed_record_history: 0,
            }));
        }
    }
}
