//! Mixed owner commands in a single front bucket (B replay_roof 0x141a37ad0).
//! This does not split each edit into an outer bucket. Auxiliary decorator and
//! stair owners must be recovered before accepting their nonempty buckets.
use super::EditType;
use crate::recovered_building::{
    building_history_transactions::{BuildingHistoryError, RoofReplayCommand},
    history_transactions::{HistoryEntry, HistoryTransactionError},
    wall_history_replay::{WallReplayCommand, WallReplayError, expand_wall_compound},
};
use bevy_ecs::prelude::Resource;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub enum HistoryReplayError {
    Transaction(HistoryTransactionError),
    Roof(BuildingHistoryError),
    Wall(WallReplayError),
    UnrestoredReplayOwner(u64),
    UnrestoredCommand([u8; 4]),
}
impl From<HistoryTransactionError> for HistoryReplayError {
    fn from(value: HistoryTransactionError) -> Self {
        Self::Transaction(value)
    }
}
#[derive(Clone, Debug)]
pub enum ReplayCommand {
    Roof(RoofReplayCommand),
    Wall(VecDeque<Vec<WallReplayCommand>>),
    Raster(crate::systems::history::replay_raster::RasterReplayCmd),
    AutoClutter(super::auto_clutter_edit::AutoClutterReplayCmd),
    /// 楼梯支撑移除/恢复（B `replay_stairs` tag 10）。
    Stairs(super::stairs_history_edit::StairsReplayCmd),
    /// 地形回放（B `replay_terrain` 0x140995150 tag 3；CPU 侧模型，经中央仲裁接入）。
    Terrain(crate::systems::terrain::replay_terrain::TerrainReplayCmd),
    /// 树回放（O：`replay_tree` 0x141bc34f0 消费；组 A restorer 接线——
    /// `system_tree::replay_tree` 的 `ResMut<ReplayBuffer>` 面）。
    Tree(super::tree_edit::TreeReplayCmd),
}
#[derive(Clone, Debug)]
pub struct ReplayBucket {
    pub entry_index: usize,
    pub undo: bool,
    pub commands: Vec<ReplayCommand>,
}
impl ReplayBucket {
    pub fn expand(
        entry_index: usize,
        entry: &HistoryEntry<EditType>,
        undo: bool,
        instant: bool,
    ) -> Result<Self, HistoryReplayError> {
        if entry.observed_auxiliary_counts != [0; 3] {
            return Err(HistoryTransactionError::AuxiliaryReplayOwnerRequired.into());
        }
        let mut commands = Vec::new();
        for edit in &entry.edits {
            if let Some((domain, stroke)) = edit.raster() {
                commands.push(ReplayCommand::Raster(
                    crate::systems::history::replay_raster::RasterReplayCmd::new(
                        domain, stroke, undo, instant, true,
                    ),
                ));
                continue;
            }
            commands.push(match edit {
                EditType::AutoClutter(edit) => ReplayCommand::AutoClutter(
                    super::auto_clutter_edit::AutoClutterReplayCmd::expanded_shared(
                        edit.clone(), undo, instant,
                    ),
                ),
                EditType::Roof(edit) => ReplayCommand::Roof(RoofReplayCommand::expanded_shared(
                    edit.clone(),
                    undo,
                    instant,
                )),
                EditType::Wall(edit) => {
                    use crate::recovered_building::wall_history_edit::WallHistoryEdit;
                    // Restoring a deleted/new wall needs its original +66.
                    // Fail before moving the history cursor when an older
                    // portable snapshot did not preserve that byte.
                    for operation in edit.groups.iter().flatten() {
                        let source = match operation {
                            WallHistoryEdit::Create { wall, source, .. } if !undo => {
                                Some((wall, source))
                            }
                            WallHistoryEdit::Delete { wall, source, .. } if undo => {
                                Some((wall, source))
                            }
                            _ => None,
                        };
                        if let Some((wall, source)) = source {
                            if source.metadata.source_66.is_none() {
                                return Err(HistoryReplayError::Wall(
                                    WallReplayError::MissingSource66(*wall),
                                ));
                            }
                        }
                    }
                    ReplayCommand::Wall(
                        expand_wall_compound(edit, undo, instant)
                            .map_err(HistoryReplayError::Wall)?,
                    )
                }
                EditType::Stairs(edit) => {
                    commands.extend(
                        super::stairs_history_edit::StairsReplayCmd::expanded(edit, undo, instant)
                            .into_iter()
                            .map(ReplayCommand::Stairs),
                    );
                    continue;
                }
                EditType::Tree(edit) => ReplayCommand::Tree(
                    super::tree_edit::TreeReplayCmd::expanded(edit, undo, instant),
                ),
                edit @ EditType::Terrain(_) => {
                    // 共享 TerrainEdit（{pending, previous_state}）无点集载荷；桶展开
                    // 暂不支持（revert/reapply 0x140858d30/0x1408592d0 属共享 owner）。
                    // 真实点集 TerrainReplayCmd 由地形回放 producer 直接入桶
                    // （CPU 侧模型，中央仲裁接入 ReplayCommand::Terrain）。
                    return Err(HistoryReplayError::UnrestoredReplayOwner(edit.native_tag()));
                }
                edit => return Err(HistoryReplayError::UnrestoredReplayOwner(edit.native_tag())),
            });
        }
        if undo {
            commands.reverse();
        }
        Ok(Self {
            entry_index,
            undo,
            commands,
        })
    }
}

#[derive(Resource, Clone, Debug)]
pub struct ReplayBuffer {
    pub buckets: VecDeque<ReplayBucket>,
    pub speed: f32,
}
impl ReplayBuffer {
    /// Speed is supplied by the real replay-mode owner; no guessed startup
    /// value is silently installed by registration.
    pub fn new(speed: f32) -> Self {
        Self {
            buckets: VecDeque::new(),
            speed,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.buckets.is_empty()
    }
    pub fn len(&self) -> usize {
        self.buckets.len()
    }
    pub fn front_mut_bucket(&mut self) -> Option<&mut ReplayBucket> {
        self.buckets.front_mut()
    }
    pub fn push(&mut self, bucket: ReplayBucket) {
        self.buckets.push_back(bucket);
    }
    /// B 0x1409f1ab0 visits all buckets. Clearing animation modes does not
    /// consume commands: their normal consumers still apply their end values.
    pub fn make_animations_instant(&mut self) {
        for bucket in &mut self.buckets {
            for command in &mut bucket.commands {
                match command {
                    ReplayCommand::Roof(command) => command.make_instant(),
                    ReplayCommand::Wall(groups) => {
                        for command in groups.iter_mut().flatten() {
                            command.timed = false;
                        }
                    }
                    ReplayCommand::Raster(command) => command.make_instant(),
                    ReplayCommand::AutoClutter(command) => command.make_instant(),
                    ReplayCommand::Stairs(command) => command.instant = true,
                    ReplayCommand::Terrain(command) => command.make_instant(),
                    ReplayCommand::Tree(command) => command.make_instant(),
                }
            }
        }
    }
}
