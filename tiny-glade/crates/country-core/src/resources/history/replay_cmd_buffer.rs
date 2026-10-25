//! Mixed owner commands in a single front bucket (B replay_roof 0x141a37ad0).
//! This does not split each edit into an outer bucket. Auxiliary decorator and
//! stair owners must be recovered before accepting their nonempty buckets.
use super::EditType;
use crate::resources::history::building_transactions::BuildingHistoryError;
use crate::resources::history::building_transactions::RoofReplayCommand;
use crate::resources::history::transactions::HistoryEntry;
use crate::resources::history::transactions::HistoryTransactionError;
use crate::systems::history::wall_replay_interpreter::WallReplayCommand;
use crate::systems::history::wall_replay_interpreter::WallReplayError;
use crate::systems::history::wall_replay_interpreter::expand_wall_compound;
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
    /// 颜色编辑回放（fresh `generate_replay_cmds_from_edit` tag 9 → 命令 tag 6）。
    Color(ColorReplayCmd),
    /// 装饰编辑回放（fresh tag 10 DecoratorV5 → 命令 tag 7；tag 11 Decorator
    /// 在原版生成器里是 `unreachable!()`）。
    Decorator(DecoratorReplayCmd),
}

/// 颜色回放命令（原生 0x30B 记录，fresh 0x14143fcc0 tag-9 生成器逐字段）。
///
/// 布局：`tag u8 @+0x00 = 6`、`flags u32 @+0x08`（bit0 = timed = `!instant`）、
/// `duration f32 @+0x0c = 0.2`、`Arc<ColorEdit> @+0x10`、`elapsed f32 @+0x18 = 0`、
/// `undo bool @+0x1c`。
#[derive(Clone, Debug)]
pub struct ColorReplayCmd {
    pub edit: std::sync::Arc<super::color_edit::ColorEdit>,
    /// 原生 flags bit0：`true` = 动画路径（timed）。
    pub timed: bool,
    pub duration: f32,
    pub elapsed: f32,
    pub undo: bool,
}
impl ColorReplayCmd {
    /// 原生生成器常量 0x3e4ccccd。
    pub const DURATION: f32 = 0.2;
    pub fn expanded(
        edit: std::sync::Arc<super::color_edit::ColorEdit>,
        undo: bool,
        instant: bool,
    ) -> Self {
        Self {
            edit,
            timed: !instant,
            duration: Self::DURATION,
            elapsed: 0.0,
            undo,
        }
    }
    pub fn make_instant(&mut self) {
        self.timed = false;
    }
}

/// 装饰回放命令（原生 0x30B 记录，fresh 0x14143fed0 tag-10 生成器逐字段）。
///
/// 布局：`tag u8 @+0x00 = 7`、`flags @+0x08`（bit0 = timed）、
/// `duration f32 @+0x0c = 0.2`、`Arc<DecoratorHistoryEdit> @+0x10`、`elapsed @+0x18`、
/// `undo @+0x1c`。
#[derive(Clone, Debug)]
pub struct DecoratorReplayCmd {
    pub edit: std::sync::Arc<super::decorator_edit::DecoratorV5Edit>,
    pub timed: bool,
    pub duration: f32,
    pub elapsed: f32,
    pub undo: bool,
}
impl DecoratorReplayCmd {
    /// 原生生成器常量 0x3e4ccccd。
    pub const DURATION: f32 = 0.2;
    pub fn expanded(
        edit: std::sync::Arc<super::decorator_edit::DecoratorV5Edit>,
        undo: bool,
        instant: bool,
    ) -> Self {
        Self {
            edit,
            timed: !instant,
            duration: Self::DURATION,
            elapsed: 0.0,
            undo,
        }
    }
    pub fn make_instant(&mut self) {
        self.timed = false;
    }
}

/// 每个 ReplayBucket 的辅助 owner diff（原生 bucket 内联
/// `Arc<DecoratorHistoryDiff>` / `Arc<StairsHistoryDiff>` 两槽；`replay_pop_bucket`
/// 0x140b38250 弹桶时按它们恢复装饰/楼梯）。
#[derive(Clone, Debug, Default)]
pub struct ReplayBucketAux {
    pub decorator_diff: Option<std::sync::Arc<super::decorator_edit::DecoratorHistoryDiffMap>>,
    pub stairs_diff:
        Option<std::sync::Arc<crate::resources::stairs::stairs_state::StairsHistoryDiff>>,
    /// 原生 bucket `+0x18/+0x20` 的 `Arc<[DecoratorId]>` cull-allow 表；
    /// `replay_decorator` 以 take 语义注入 archivist 允许集合。Producer is
    /// HistoryEntryAux::for_replay on the selected committed entry's diff.
    pub cull_allow: Option<std::sync::Arc<[crate::resources::walls::decorator::DecoratorId]>>,
}
#[derive(Clone, Debug)]
pub struct ReplayBucket {
    pub entry_index: usize,
    pub undo: bool,
    pub commands: Vec<ReplayCommand>,
    /// O：每条命令的时序头（原 0x30B 命令记录内联——2026-10-17 coregap-1 fresh 反编译
    /// `replay_tree` 0x141bc34f0：`flags byte@+8`、`delay f32@+0xc`、
    /// `elapsed f32@+0x18`、`byte@+0x1c`）。`commands` 与 `timing` 长度一一对应；
    /// 时序推进经 [`Self::advance_timing`]。
    pub timing: Vec<CommandTiming>,
}

/// 原 0x30B 命令记录的时序头（O：replay_tree 0x141bc34f0 fresh）。
///
/// 布局：`flags byte@+8`（**bit0 置位 = 动画中/可推进，清零 = 待移除**）、
/// `delay f32@+0xc`（除以 `ReplayBuffer.speed` 后与 `elapsed` 比较）、
/// `elapsed f32@+0x18`（bit0 置位命令每帧 += 帧增量；`== 0 且非 NaN` 才执行动画）、
/// `byte@+0x1c`（0 = 未开始/瞬时门控）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CommandTiming {
    /// @+8：标志字节（O：bit0 置位 = 可推进；清零 = 待移除）。
    pub flags: u8,
    /// @+0xc：延迟。
    pub delay: f32,
    /// @+0x18：已流逝时间。
    pub elapsed: f32,
    /// @+0x1c：开始门控字节（0 = 未开始）。
    pub started: bool,
}
impl ReplayBucket {
    /// Current Rust projection stores native per-command timing separately.
    /// Every domain removal must compact both arrays in the same order.
    pub fn remove_command(&mut self, index: usize) -> ReplayCommand {
        if index < self.timing.len() {
            self.timing.remove(index);
        }
        self.commands.remove(index)
    }

    pub fn retain_commands_mut(&mut self, mut keep: impl FnMut(&mut ReplayCommand) -> bool) {
        let mut removed = Vec::new();
        let mut index = 0;
        self.commands.retain_mut(|command| {
            let retain = keep(command);
            if !retain {
                removed.push(index);
            }
            index += 1;
            retain
        });
        for index in removed.into_iter().rev() {
            if index < self.timing.len() {
                self.timing.remove(index);
            }
        }
    }

    pub fn expand(
        entry_index: usize,
        entry: &HistoryEntry<EditType>,
        undo: bool,
        instant: bool,
    ) -> Result<Self, HistoryReplayError> {
        Self::expand_with_aux(
            entry_index,
            entry,
            undo,
            instant,
            ReplayBucketAux::default(),
        )
    }

    /// B `replay_edit` 0x140af6170：逐 edit `generate_replay_cmds_from_edit`
    /// （跳表 0x142c5f284 按 edit tag 分派）→ undo 逆序 → 调度为 ReplayBucket；
    /// History 的两条 aux diff `Arc`（原生 +0x18/+0x20）随桶携带，供
    /// [`replay_pop_bucket`] 恢复装饰/楼梯。
    ///
    /// fresh tag→命令映射：0 Terrain→typed Terrain New/Edit/Move、
    /// 1..6→Raster、7 Wall、8 Roof（命令 tag 5）、9 Color（命令 tag 6，0.2s）、
    /// 10 DecoratorV5（命令 tag 7，0.2s）、11 Decorator→`unreachable!()`、
    /// 12 Tree（tag 8，0.1s）、13 AutoClutter（tag 9，0.1s）、
    /// 14 Stairs（tag 10，0.2s）。
    pub fn expand_with_aux(
        entry_index: usize,
        entry: &HistoryEntry<EditType>,
        undo: bool,
        instant: bool,
        aux: ReplayBucketAux,
    ) -> Result<Self, HistoryReplayError> {
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
                        edit.clone(),
                        undo,
                        instant,
                    ),
                ),
                EditType::Roof(edit) => ReplayCommand::Roof(RoofReplayCommand::expanded_shared(
                    edit.clone(),
                    undo,
                    instant,
                )),
                EditType::Wall(edit) => {
                    use crate::resources::history::wall_edit::WallHistoryEdit;
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
                EditType::Color(edit) => {
                    ReplayCommand::Color(ColorReplayCmd::expanded(edit.clone(), undo, instant))
                }
                EditType::DecoratorV5(edit) => ReplayCommand::Decorator(
                    DecoratorReplayCmd::expanded(edit.clone(), undo, instant),
                ),
                // fresh 0x141441b83：tag-11 生成器分支是
                // `unreachable!("internal error: entered unreachable code")`。
                EditType::Decorator(_) => {
                    unreachable!("internal error: entered unreachable code")
                }
                EditType::Terrain(edit) => ReplayCommand::Terrain(
                    crate::systems::terrain::replay_terrain::TerrainReplayCmd::expanded(
                        edit.clone(),
                        undo,
                        instant,
                    ),
                ),
                // 光栅族（tag 1..6）已在上方 `edit.raster()` 分支消费。
                EditType::Path(_)
                | EditType::Cobblestone1(_)
                | EditType::Cobblestone2(_)
                | EditType::Cobblestone3(_)
                | EditType::Water(_)
                | EditType::Garden(_) => unreachable!("raster edits expand via edit.raster()"),
            });
        }
        if undo {
            commands.reverse();
        }
        let timing = vec![CommandTiming::default(); commands.len()];
        Ok(Self {
            entry_index,
            undo,
            commands,
            timing,
        })
    }

    /// 时序推进（O：replay_tree 0x141bc34f0 只对 **`flags.bit0 置位`** 命令推进
    /// `elapsed += delta`；`flags.bit0 清零` 命令为待移除，**不推进**）。`elapsed == 0 且
    /// 非 NaN` 是动画执行门控；`delay / speed`（下限 0.05）`< elapsed` 是移除判据（消费端）。
    pub fn advance_timing(&mut self, delta: f32) {
        for timing in &mut self.timing {
            if timing.flags & 1 == 0 {
                // O：bit0 清零 = 待移除命令，不推进 elapsed。
                continue;
            }
            timing.elapsed += delta;
        }
    }
}

#[derive(Resource, Clone, Debug)]
pub struct ReplayBuffer {
    pub buckets: VecDeque<ReplayBucket>,
    /// 与 `buckets` 一一对应的 aux owner diff（原生 bucket 内联两 Arc 槽）。
    pub aux: VecDeque<ReplayBucketAux>,
    pub speed: f32,
    /// B pop_front increments this even before the empty-queue assertion.
    pop_counter: u64,
}
impl ReplayBuffer {
    /// Speed is supplied by the real replay-mode owner; no guessed startup
    /// value is silently installed by registration.
    pub fn new(speed: f32) -> Self {
        Self {
            buckets: VecDeque::new(),
            aux: VecDeque::new(),
            speed,
            pop_counter: 0,
        }
    }
    pub fn pop_counter(&self) -> u64 {
        self.pop_counter
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
        self.push_with_aux(bucket, ReplayBucketAux::default());
    }
    pub fn push_with_aux(&mut self, bucket: ReplayBucket, aux: ReplayBucketAux) {
        self.buckets.push_back(bucket);
        self.aux.push_back(aux);
    }
    /// 弹出前桶及其 aux（原 `ReplayBuffer::pop_front` 0x1409f18f0 之外的
    /// history 侧模型：aux 与 bucket 同步）。
    pub fn pop_front_bucket(&mut self) -> Option<(ReplayBucket, ReplayBucketAux)> {
        self.pop_counter = self.pop_counter.wrapping_add(1);
        let bucket = self.buckets.pop_front()?;
        let aux = self.aux.pop_front().unwrap_or_default();
        Some((bucket, aux))
    }
    pub fn front_aux(&self) -> Option<&ReplayBucketAux> {
        self.aux.front()
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
                    ReplayCommand::Color(command) => command.make_instant(),
                    ReplayCommand::Decorator(command) => command.make_instant(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::history::color_edit::{ColorEdit, ColorTarget};
    use crate::resources::history::decorator_edit::{DecoratorHistoryEdit, DecoratorV5Edit};
    use crate::resources::terrain_editing::StrokeId;
    use crate::resources::walls::WallId;
    use std::sync::Arc;

    fn color_edit() -> Arc<ColorEdit> {
        Arc::new(ColorEdit::Wall {
            wall_id: WallId(7),
            from: StrokeId(1),
            to: StrokeId(2),
        })
    }

    #[test]
    fn color_edit_expands_to_tag_six_command_with_point_two_duration() {
        // fresh 0x14143fcc0：命令 tag 6、flags=!instant、duration=0x3e4ccccd、undo@+0x1c。
        let entry = HistoryEntry::from_edits(vec![EditType::Color(color_edit())]);
        let bucket = ReplayBucket::expand(0, &entry, false, false).unwrap();
        let ReplayCommand::Color(command) = &bucket.commands[0] else {
            panic!("expected color command");
        };
        assert!(command.timed);
        assert_eq!(command.duration, 0.2);
        assert_eq!(command.elapsed, 0.0);
        assert!(!command.undo);
        assert_eq!(command.edit.target(), ColorTarget::Wall);

        let bucket = ReplayBucket::expand(0, &entry, true, true).unwrap();
        let ReplayCommand::Color(command) = &bucket.commands[0] else {
            panic!("expected color command");
        };
        assert!(!command.timed);
        assert!(command.undo);
    }

    #[test]
    fn decorator_v5_expands_to_tag_seven_command() {
        // fresh 0x14143fed0：DecoratorV5(tag10) → 命令 tag 7、duration 0.2。
        let entry = HistoryEntry::from_edits(vec![EditType::DecoratorV5(Arc::new(
            DecoratorV5Edit::default(),
        ))]);
        let bucket = ReplayBucket::expand(0, &entry, false, false).unwrap();
        let ReplayCommand::Decorator(command) = &bucket.commands[0] else {
            panic!("expected decorator command");
        };
        assert!(command.timed);
        assert_eq!(command.duration, 0.2);
    }

    #[test]
    #[should_panic(expected = "entered unreachable code")]
    fn deprecated_decorator_tag_is_unreachable_in_generator() {
        // fresh 0x141441b83：tag 11 生成器即 `unreachable!()`。
        let entry = HistoryEntry::from_edits(vec![EditType::Decorator(Arc::new(
            DecoratorHistoryEdit::default(),
        ))]);
        let _ = ReplayBucket::expand(0, &entry, false, false);
    }

    #[test]
    fn aux_diffs_are_carried_with_the_bucket() {
        let entry = HistoryEntry::from_edits(vec![EditType::Color(color_edit())]);
        let aux = ReplayBucketAux {
            decorator_diff: Some(Arc::new(Default::default())),
            stairs_diff: Some(Arc::new(
                crate::resources::stairs::stairs_state::StairsHistoryDiff {
                    before_checksum: 0,
                    after_checksum: 0,
                    removed: Vec::new(),
                    added: Vec::new(),
                },
            )),
            cull_allow: None,
        };
        let bucket = ReplayBucket::expand_with_aux(3, &entry, false, false, aux.clone()).unwrap();
        let mut buffer = ReplayBuffer::new(1.0);
        buffer.push_with_aux(bucket, aux);
        assert_eq!(buffer.aux.len(), 1);
        assert!(buffer.front_aux().unwrap().decorator_diff.is_some());
        let (bucket, aux) = buffer.pop_front_bucket().unwrap();
        assert_eq!(bucket.entry_index, 3);
        assert!(aux.stairs_diff.is_some());
        assert!(buffer.aux.is_empty());
    }

    #[test]
    fn scheduling_predicate_matches_fresh_replay_edit_switch() {
        // fresh replay_edit 0x140af6170 的相邻命令兼容谓词（switch case 6/7/8/10）。
        let color = HistoryEntry::from_edits(vec![EditType::Color(color_edit())]);
        let roof = HistoryEntry::from_edits(vec![EditType::Roof(Arc::new(
            crate::resources::history::building_transactions::RoofHistoryEdit::from_observed(
                [0; 64],
            )
            .unwrap(),
        ))]);
        let decorator = HistoryEntry::from_edits(vec![EditType::DecoratorV5(Arc::new(
            DecoratorV5Edit::default(),
        ))]);
        let tree = HistoryEntry::from_edits(vec![EditType::Tree(Arc::new(
            crate::resources::history::tree_edit::TreeEdit::default(),
        ))]);
        let color_cmd = &ReplayBucket::expand(0, &color, false, true)
            .unwrap()
            .commands[0];
        let other_color = &ReplayBucket::expand(1, &color, false, true)
            .unwrap()
            .commands[0];
        let roof_cmd = &ReplayBucket::expand(0, &roof, false, true)
            .unwrap()
            .commands[0];
        let decorator_cmd = &ReplayBucket::expand(0, &decorator, false, true)
            .unwrap()
            .commands[0];
        let tree_cmd = &ReplayBucket::expand(0, &tree, false, true)
            .unwrap()
            .commands[0];
        assert_eq!(color_cmd.native_tag(), 6);
        assert_eq!(roof_cmd.native_tag(), 5);
        assert_eq!(decorator_cmd.native_tag(), 7);
        assert_eq!(tree_cmd.native_tag(), 8);
        // case 6：next 6
        assert!(color_cmd.can_merge_with(other_color));
        assert!(!color_cmd.can_merge_with(roof_cmd));
        // case 7：next ∈ {7,10}
        assert!(decorator_cmd.can_merge_with(decorator_cmd));
        assert!(!decorator_cmd.can_merge_with(roof_cmd));
        // case 8：next & 7 == 0（即 next tag 0 或 8）
        assert!(tree_cmd.can_merge_with(tree_cmd));
        assert!(!tree_cmd.can_merge_with(other_color));
        assert!(!tree_cmd.can_merge_with(roof_cmd));
        // 5 → 4 才可并；4 → 0 才可并。
        assert!(!roof_cmd.can_merge_with(color_cmd));
    }

    #[test]
    fn undo_reverses_generated_commands() {
        let entry = HistoryEntry::from_edits(vec![
            EditType::Color(color_edit()),
            EditType::Tree(Arc::new(
                crate::resources::history::tree_edit::TreeEdit::default(),
            )),
        ]);
        let bucket = ReplayBucket::expand(0, &entry, true, true).unwrap();
        assert!(matches!(bucket.commands[0], ReplayCommand::Tree(_)));
        assert!(matches!(bucket.commands[1], ReplayCommand::Color(_)));
    }
}

impl ReplayCommand {
    /// 原生 `ReplayCmd` 判别 tag（fresh：命令记录 `+0x00`；生成器逐 tag 写入）。
    pub const fn native_tag(&self) -> u8 {
        match self {
            Self::Raster(_) => 0, // 0/1/2 三个光栅域共用 tag 面（+1 槽为域判别）
            Self::Terrain(_) => 3,
            Self::Wall(_) => 4,
            Self::Roof(_) => 5,
            Self::Color(_) => 6,
            Self::Decorator(_) => 7,
            Self::Tree(_) => 8,
            Self::AutoClutter(_) => 9,
            Self::Stairs(_) => 10,
        }
    }

    /// `replay_edit` 0x140af6170 调度谓词（decomp switch，逐 case）：
    /// 相邻命令能否并入同一 ReplayBucket。
    ///
    /// fresh 规则（prev tag → next tag）：
    /// - 0：next 0 要求 `next[1]==prev[1] && (next[1]==0 || next[2]==prev[2])`；
    ///   否则 next ∉ {4,8} 即可；
    /// - 4：next 0；5：next 4（wall 内组检查在 0x140af6525，登记）；
    /// - 6：next 6；7：next ∈ {7,10}；8：next & 7 == 0；10：next 7；
    /// - 其余 → 不可并。
    pub fn can_merge_with(&self, next: &Self) -> bool {
        let (prev_tag, next_tag) = (self.native_tag(), next.native_tag());
        match prev_tag {
            0 => {
                if next_tag == 0 {
                    // 光栅子判别：原生比较命令 byte[1]/byte[2]（域/undo 面）。
                    match (self, next) {
                        (Self::Raster(a), Self::Raster(b)) => {
                            a.domain == b.domain && a.undo == b.undo
                        }
                        _ => false,
                    }
                } else {
                    next_tag != 4 && next_tag != 8
                }
            }
            4 => next_tag == 0,
            5 => next_tag == 4,
            6 => next_tag == 6,
            7 => next_tag == 7 || next_tag == 10,
            8 => next_tag & 7 == 0,
            10 => next_tag == 7,
            _ => false,
        }
    }
}

#[cfg(test)]
mod pop_counter_tests {
    use super::*;
    #[test]
    fn counter_tracks_pop_attempts_and_is_preserved_by_clone() {
        let mut replay = ReplayBuffer::new(1.0);
        assert_eq!(replay.pop_counter(), 0);
        replay.push(ReplayBucket {
            entry_index: 0,
            undo: false,
            commands: Vec::new(),
            timing: Vec::new(),
        });
        assert_eq!(replay.pop_counter(), 0);
        assert!(replay.pop_front_bucket().is_some());
        assert_eq!(replay.pop_counter(), 1);
        assert_eq!(replay.clone().pop_counter(), 1);
        assert!(replay.pop_front_bucket().is_none());
        assert_eq!(replay.pop_counter(), 2);
        replay.pop_counter = u64::MAX;
        assert!(replay.pop_front_bucket().is_none());
        assert_eq!(replay.pop_counter(), 0);
    }
}
