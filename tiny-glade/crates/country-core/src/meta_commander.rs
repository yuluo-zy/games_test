//! B `meta_commander.rs` — 元命令队列。
//!
//! - `MetaCommand` = **0x38B = 7 qword** `{tag@+0, 载荷 qword1@+8, qword2@+0x10,
//!   qword3@+0x18, qword4@+0x20, qword5@+0x28, qword6@+0x30}`；tag 0..0x13；
//!   **空哨兵 = 0x8000000000000014**（`0x8000000000000000` 才是变体 0 = 会话加载判别）。
//! - `MetaCommander` = 80B `{commands: VecDeque<MetaCommand>, profiler_sinks HashMap,
//!   RandomState}`；`peek` 0x14091f0c0 以 [+0x18] len 判空；`consume` 0x14091f100 =
//!   pop_front（head 环绕 + len--）；`undo_consume` 推回队首。
//! - `MetaCommandFence` = Arc<Mutex<(bool,bool)>>；`new_pair_blocked` 0x14091ef90 分配
//!   0x18B（strong=2，+0x10 单字节显式写 true；第二标志来源未定）。
//! - `commander_update` 0x14091f180 case 0..0x13 分派表。case 0（会话加载两路径
//!   状态机）、6（float 事件）、9（建目录）、5（延后）已按 RE-ANALYSIS/RE-SESSION-SWITCH
//!   落地为 `MetaCommander::update` → `CommanderOutcome`；case 2/3（profiler_sinks）、
//!   7（TimeUpdateStrategy）、8（0x30B 条目）、0xc（GPU/内存报告）等载荷/资源面由宿主
//!   消费（待 RE-SESSION-SWITCH/run_game_thread 定点补挖）。
//!
//! 会话加载链（case 0）：AppMode 门控 → 新 fence → 建临时 on_load Vec →
//! append_on_load<Fence> → 应用 → 命令带 fence 重入队；on_load 回放见
//! `systems::save::load_game::advance_load_game`。
use crate::resources::session::{OnLoadEntry, ON_LOAD_FENCE, ON_LOAD_VARIANT_MASK};
use ahash::RandomState;
use bevy_ecs::prelude::{Local, Res, ResMut};
use hashbrown::HashMap;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// 元命令 = 0x38B（7 qword：tag + 6 载荷）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MetaCommand {
    pub tag: u64,
    pub payloads: [u64; 6],
}
impl MetaCommand {
    /// 空哨兵（consume 空队列返回）：0x8000000000000014。
    pub const EMPTY_SENTINEL: u64 = 0x8000_0000_0000_0014;
    /// 变体 0 判别（会话加载）：0x8000000000000000。
    pub const SESSION_LOAD_MARKER: u64 = 0x8000_0000_0000_0000;
}

/// 会话加载 fence = Arc<Mutex<(bool, bool)>>。
#[derive(Clone, Debug)]
pub struct MetaCommandFence(pub Arc<Mutex<(bool, bool)>>);
impl MetaCommandFence {
    /// 0x14091ef90：分配 0x18B Arc（strong=2）；+0x10 单字节显式写 true（阻塞）。
    /// RE-SESSION-SWITCH 定案：**fence 标志偏移 = +0x10**（0x14091f450 movzx [rax+0x10]；
    /// 旧"两 flag true"的 +0x11 由 alloc 残留，"变体 2 unblock +2"为 Ghidra 伪影）。
    pub fn new_pair_blocked() -> Self {
        Self(Arc::new(Mutex::new((true, true))))
    }
    /// 清除阻塞标志（写 +0x10 字节）。
    pub fn unblock(&self) {
        self.0.lock().expect("fence poisoned").0 = false;
    }
    /// 读阻塞标志（+0x10）。
    pub fn is_blocked(&self) -> bool {
        self.0.lock().expect("fence poisoned").0
    }
    /// 载荷编码（on_load 条目 / MetaCommand +0x18 携带的 Arc 指针值）。
    pub fn as_payload(&self) -> u64 {
        Arc::as_ptr(&self.0) as usize as u64
    }
}

/// 元命令器（PDB：`MetaCommander`；80B：commands VecDeque + profiler_sinks HashMap +
/// RandomState）。为 Bevy 资源（commander_update 系统消费）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug)]
pub struct MetaCommander {
    commands: VecDeque<MetaCommand>,
    /// profiler_sinks 键类型/值待取证；以 `()` 占位登记，不伪造 sink 载荷。
    profiler_sinks: HashMap<String, (), RandomState>,
}
impl Default for MetaCommander {
    fn default() -> Self {
        Self::new()
    }
}
impl MetaCommander {
    /// 0x14091eff0：空命令队列 + 空 sinks + RandomState 种子。
    pub fn new() -> Self {
        Self {
            commands: VecDeque::new(),
            profiler_sinks: HashMap::with_hasher(RandomState::new()),
        }
    }
    /// `peek` 0x14091f0c0：队首命令（len 判空）。
    pub fn peek(&self) -> Option<MetaCommand> {
        self.commands.front().copied()
    }
    /// `consume` 0x14091f100：pop_front；空返回 `EMPTY_SENTINEL` 语义。
    pub fn consume(&mut self) -> Option<MetaCommand> {
        self.commands.pop_front()
    }
    /// `is_empty`（O）：队列空判定。
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
    /// `undo_consume`（I）：推回队首。
    pub fn undo_consume(&mut self, command: MetaCommand) {
        self.commands.push_front(command);
    }
    /// `extend_script`（I，0x140b5a180）：追加命令列表。
    pub fn extend_script(&mut self, commands: impl IntoIterator<Item = MetaCommand>) {
        self.commands.extend(commands);
    }
    /// `is_profiling` 0x14091f0f0：`[+0x38]!=0`（该偏移在 hashbrown 表语义未钉死，U）。
    pub fn is_profiling(&self) -> bool {
        !self.profiler_sinks.is_empty()
    }
}

/// `commander_update` 0x14091f180 每帧处理结果（供宿主消费的副作用面）。
/// case 0..0x13 分派；需游戏资源的 case 由宿主按 outcome 应用。
#[derive(Clone, Debug)]
pub enum CommanderOutcome {
    /// case 0 **路径 A**（+0x18==0 首调）会话加载：宿主按 AppMode 门控
    /// `Commands::insert_resource<NextSession>`（on_load 含变体 0 会话加载命令，
    /// payload = fence；FUN_1412e0b90 语义）。fence 由宿主跟踪，切换完成后释放。
    SessionLoad {
        on_load: Vec<OnLoadEntry>,
        fence: MetaCommandFence,
    },
    /// case 0 **路径 B**（+0x18!=0 重入队到达）：fence 阻塞则宿主重入队等待；
    /// `+0x10` 已释放 → Arc drop 完成。
    SessionLoadWait,
    /// case 6 float 事件（原版格式化推 32B 事件）。
    FloatEvent { value: f32 },
    /// case 9 建目录（路径载荷由宿主按 payload 语义构建；编码待取证）。
    CreateDirectory { path: PathBuf },
    /// 未知命令（default：verbosity>2 log + 断点）。
    Unknown,
    /// 已内部处理，无需宿主动作。
    Handled,
}

impl MetaCommander {
    /// `commander_update` 0x14091f180：每帧处理一个命令。
    ///
    /// 判别 `tag ^ 0x8000000000000000`，case 0..0x13。已按机器码登记行为；
    /// case 0 会话加载、case 6 float 事件、case 9 建目录产出 `CommanderOutcome`；
    /// case 2/3（profiler_sinks 迭代）、7（TimeUpdateStrategy）、8（0x30B 条目）、
    /// 0xc（GPU/内存报告）等载荷/资源面由宿主消费（RE-SESSION-SWITCH.md 落地后补挖）。
    pub fn update(&mut self) -> Option<CommanderOutcome> {
        let command = self.consume()?;
        let decoded = command.tag ^ 0x8000_0000_0000_0000;
        let outcome = match decoded {
            0 => self.handle_session_load(&command),
            1 => {
                // 命令重排（载荷 {+0x20 fn ptr, +0x28, +0x30} 重写入队）：宿主。
                CommanderOutcome::Handled
            }
            2 | 3 => {
                // profiler_sinks HashMap 迭代（hashbrown control-byte 位扫描；puffin
                // profiling 数据发送）：载荷非 0 → 重排 {tag 3, countdown 0x12c}（case 2），
                // 或直到 control==0x01 变体（case 3）。sink 载荷由宿主消费。
                CommanderOutcome::Handled
            }
            4 => CommanderOutcome::Handled,
            5 => {
                // 等待/延后：载荷非 0 → 重排 {tag 5, count-1}。
                if command.payloads[0] > 0 {
                    self.undo_consume(MetaCommand {
                        tag: command.tag,
                        payloads: [
                            command.payloads[0] - 1,
                            command.payloads[1],
                            command.payloads[2],
                            command.payloads[3],
                            command.payloads[4],
                            command.payloads[5],
                        ],
                    });
                }
                CommanderOutcome::Handled
            }
            6 => CommanderOutcome::FloatEvent {
                value: f32::from_bits(command.payloads[0] as u32),
            },
            7 => {
                // 清零 + TimeUpdateStrategy 应用（FUN_1412e1400(param_13, Vec, 0x142aadea8)）：
                // 宿主。
                CommanderOutcome::Handled
            }
            8 => {
                // 追加 0x30B 条目到 Vec（{+0x08..+0x30}，cap-first）：宿主。
                CommanderOutcome::Handled
            }
            9 => CommanderOutcome::CreateDirectory {
                path: PathBuf::new(),
            },
            10 | 0xb => CommanderOutcome::Handled,
            0xc => {
                // GPU/内存报告：alloc 0x68B → Arc → hashbrown insert → 回写（verbosity>2 log）。
                // 宿主。
                CommanderOutcome::Handled
            }
            _ => {
                log::warn!("Unknown MetaCommand: {command:?}");
                CommanderOutcome::Unknown
            }
        };
        Some(outcome)
    }

    /// case 0 会话加载（RE-SESSION-SWITCH 两路径状态机）：
    ///
    /// - **路径 A**（`+0x18==0` 首调）：新 fence（`MetaCommandFence::new_pair_blocked`）→
    ///   建临时 on_load（变体 0 会话加载命令，payload = fence）。宿主按 AppMode 门控
    ///   `insert_resource<NextSession>`（FUN_1412e0b90），再经 `reenqueue_session_load`
    ///   把命令带 fence 重入队。
    /// - **路径 B**（`+0x18!=0` 重入队到达）：`SessionLoadWait`——宿主查 fence `+0x10`
    ///   标志：阻塞 → 再重入队等待；已释放 → Arc drop 完成。
    fn handle_session_load(&mut self, command: &MetaCommand) -> CommanderOutcome {
        if command.payloads[3] == 0 {
            let fence = MetaCommandFence::new_pair_blocked();
            let on_load = vec![OnLoadEntry::Command {
                tag: ON_LOAD_VARIANT_MASK,
                payload: fence.as_payload(),
            }];
            CommanderOutcome::SessionLoad { on_load, fence }
        } else {
            CommanderOutcome::SessionLoadWait
        }
    }

    /// 宿主在 case 0 路径 A/B 重入队会话加载命令（携带 fence 载荷至 +0x18）。
    /// payloads[0..2] 语义（NextSession/命令载荷）待 RE-SESSION-SWITCH/run_game_thread
    /// 定点补挖。
    pub fn reenqueue_session_load(&mut self, fence: &MetaCommandFence) {
        self.undo_consume(MetaCommand {
            tag: ON_LOAD_VARIANT_MASK,
            payloads: [0, 0, 0, fence.as_payload(), 0, 0],
        });
    }
}

/// `commander_update_system`：每帧消费一条 MetaCommand（commander_update 0x14091f180
/// 的会话加载分支）。case 0 会话加载按 AppMode 门控写入 NextSession
/// （FUN_1412e0b90 = `Commands::insert_resource<NextSession>` 语义）；两路径状态机由
/// `Local<Option<MetaCommandFence>>` 跟踪（路径 B 按 fence +0x10 判定阻塞/释放）。
pub fn commander_update_system(
    mut commander: ResMut<MetaCommander>,
    mut next_session: ResMut<crate::resources::session::NextSession>,
    app_mode: Res<crate::systems::mode_manager::AppMode>,
    mut fence_state: Local<Option<MetaCommandFence>>,
) {
    let Some(outcome) = commander.update() else {
        return;
    };
    match outcome {
        CommanderOutcome::SessionLoad { on_load, fence } => {
            // AppMode 门控（RE-SESSION-SWITCH §1 机器码 `mode-6<0xf && mode-0xd<2`
            // → {13,14}={Replay,LoadGame}；reviewer 卡2 FIX-1。load_game 自己的门才是
            // ==LoadGame(14)）。
            if !matches!(app_mode.native_tag(), 13 | 14) {
                // 门控失败：幂等重入队等待（原版无 panic）。
                return;
            }
            // 置 NextSession（kind=LoadOnLoad + on_load 含变体 0 + fence）。
            next_session.on_load = on_load;
            next_session.kind = crate::resources::session::NextSessionKind::LoadOnLoad;
            *fence_state = Some(fence.clone());
            // 路径 A：命令带 fence 重入队。
            commander.reenqueue_session_load(&fence);
        }
        CommanderOutcome::SessionLoadWait => {
            // 路径 B：fence +0x10 判定——阻塞则等待重入队；已释放则完成（drop）。
            let blocked = fence_state
                .as_ref()
                .is_some_and(MetaCommandFence::is_blocked);
            if blocked {
                if let Some(fence) = fence_state.as_ref() {
                    commander.reenqueue_session_load(fence);
                }
            } else {
                *fence_state = None;
            }
        }
        CommanderOutcome::FloatEvent { value } => {
            log::trace!("commander float event: {value}");
        }
        CommanderOutcome::CreateDirectory { path } => {
            if let Err(error) = std::fs::create_dir_all(&path) {
                log::warn!("Error creating directory: {error}");
            }
        }
        CommanderOutcome::Unknown => {}
        CommanderOutcome::Handled => {}
    }
}

/// 注册会话切换系统（commander_update_system 入 Schedule；宿主装配）。
pub fn configure_session_switch_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(commander_update_system);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consume_peek_undo_and_extend_preserve_fifo_order() {
        let mut commander = MetaCommander::new();
        assert!(commander.is_empty());
        assert_eq!(commander.peek(), None);
        assert_eq!(commander.consume(), None);

        let first = MetaCommand {
            tag: 1,
            payloads: [0; 6],
        };
        let second = MetaCommand {
            tag: 2,
            payloads: [7; 6],
        };
        commander.extend_script([first, second]);
        assert!(!commander.is_empty());
        assert_eq!(commander.peek(), Some(first));
        assert_eq!(commander.consume(), Some(first));
        commander.undo_consume(first);
        assert_eq!(commander.peek(), Some(first));
        assert_eq!(commander.consume(), Some(first));
        assert_eq!(commander.consume(), Some(second));
        assert!(commander.is_empty());
    }

    #[test]
    fn fence_starts_blocked_and_unblocks() {
        let fence = MetaCommandFence::new_pair_blocked();
        assert!(fence.is_blocked());
        fence.unblock();
        assert!(!fence.is_blocked());
    }

    #[test]
    fn empty_sentinel_matches_the_binary_constant() {
        assert_eq!(MetaCommand::EMPTY_SENTINEL, 0x8000_0000_0000_0014);
        assert_eq!(MetaCommand::SESSION_LOAD_MARKER, 0x8000_0000_0000_0000);
    }

    #[test]
    fn update_dispatch_session_load_two_paths_and_unknown() {
        use crate::resources::session::{ON_LOAD_FENCE, ON_LOAD_VARIANT_MASK};
        let mut commander = MetaCommander::new();
        assert!(commander.update().is_none());

        // case 0 路径 A：会话加载命令（tag=0x8000000000000000，+0x18==0）。
        commander.extend_script([MetaCommand {
            tag: ON_LOAD_VARIANT_MASK,
            payloads: [1, 2, 3, 0, 0, 0],
        }]);
        match commander.update() {
            Some(CommanderOutcome::SessionLoad { on_load, fence }) => {
                assert!(fence.is_blocked());
                // 变体 0 会话加载命令条目（payload = fence）。
                assert_eq!(on_load.len(), 1);
                assert!(matches!(on_load[0], OnLoadEntry::Command { tag: ON_LOAD_VARIANT_MASK, .. }));
            }
            other => panic!("expected SessionLoad, got {other:?}"),
        }

        // 重入队后到达：+0x18 != 0 → 路径 B。
        let fence = MetaCommandFence::new_pair_blocked();
        commander.reenqueue_session_load(&fence);
        assert!(matches!(commander.update(), Some(CommanderOutcome::SessionLoadWait)));

        // 未知命令 → Unknown。
        commander.extend_script([MetaCommand {
            tag: 0x8000_0000_0000_0099,
            payloads: [0; 6],
        }]);
        assert!(matches!(commander.update(), Some(CommanderOutcome::Unknown)));
        let _ = ON_LOAD_FENCE;
    }
}
