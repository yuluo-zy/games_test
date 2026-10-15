//! `mode_manager.rs` — AppMode 切换中枢（B 版 `systems::mode_manager`）。
//!
//! 证据（RNE 机器码 + `evidence/country-core-deep/ui-input/decomp/`）：
//! - `mode_manager` 0x140a86b60：先做 `esc_menu_canvas` 悬停早退，再按 bevy `Events` 读游标语义排空
//!   `RequestAppModeChange`，有未读事件时取**最后一条**调用 `check_if_can_switch`，最后按模式 id 走
//!   跳表（0x142ad890c，逐模式块**未恢复**，由调用方接）。
//! - `check_if_can_switch` 分发器 0x140afa8a0。
//! - `send_mode_just_changed_event` 0x140a875e0。
//!
//! 纠错（相对此前文档）：`mode_manager` 调分发器时第一个参数是 `cmp 0,[len+0x30]; setno cl`（0x140a86c94），
//! `0 - len` 对任何 `usize` 长度都不会有符号溢出，所以 `force` **恒为真**；文档里"force = 双队列读数差为 0"
//! 不成立。因此从 `mode_manager` 进入时总走 force 分支，非 force 分支只在其他调用点（如 timelapse 闭包）出现。
//!
//! 未恢复：各 `Events<T>` 的原类型名（只有负载布局）、跳表逐模式块、`AppMode` 完整变体表、`hash("esc_menu_canvas")`
//! 的哈希函数（0x1405bdda0）。
use bevy_ecs::event::{Event, EventCursor, Events};

use crate::resources::toolbar::{ModeEntry, SwitchMode, Toolbar, ToolbarIndexOutOfBounds};

/// 非 force 分支写入模式状态的值（O：`mov dword [rax],0xd`，注释记为 Replay）。
pub const PENDING_REPLAY_MODE: u32 = 0xd;

/// 请求切换模式（O：`EventInstance` 载荷 = `u32` 请求模式，0x10 B 含 id）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RequestAppModeChange {
    pub mode: u32,
}

impl RequestAppModeChange {
    /// 请求模式 u32 的第 1 字节（O：`byte [req+1]`，分发器要求 `<= 1`）。
    pub fn byte1(&self) -> u8 {
        (self.mode >> 8) as u8
    }
}

/// force 分支推入的事件（O：0x20 B `EventInstance`，载荷首 u32 = 4，其余载荷未恢复）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModeSwitchForced;

/// 非 force 分支第一个事件（O：载荷 = 请求模式 u32）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModeSwitchRequested(pub u32);

/// 非 force 分支第二个事件（O：载荷 = 请求模式的第 1 字节）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModeSwitchByte(pub u8);

/// 分发器写入的三条事件队列。
#[derive(Default)]
pub struct SwitchEvents {
    pub forced: Events<ModeSwitchForced>,
    pub requested: Events<ModeSwitchRequested>,
    pub byte: Events<ModeSwitchByte>,
}

/// 原版 `unreachable!()`（`toolbar.rs:40`，串 0x142ae5650，长 0x28）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnreachableRequestByte {
    pub byte1: u8,
}

impl core::fmt::Display for UnreachableRequestByte {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "internal error: entered unreachable code (request byte1 = {})",
            self.byte1
        )
    }
}

impl std::error::Error for UnreachableRequestByte {}

/// `check_if_can_switch<closure_env$0>` 分发器（0x140afa8a0）。
///
/// - `force`：向 `forced` 推 `ModeSwitchForced`，不碰状态；
/// - 非 force：`*state_mode = 0xd` → 推 `ModeSwitchRequested(mode)` → 若 `byte1 > 1` 则 `unreachable!()`
///   （此时第一个事件已推入）→ 推 `ModeSwitchByte(byte1)`。
pub fn check_if_can_switch(
    force: bool,
    state_mode: &mut u32,
    request: &RequestAppModeChange,
    out: &mut SwitchEvents,
) -> Result<(), UnreachableRequestByte> {
    if force {
        out.forced.send(ModeSwitchForced);
        return Ok(());
    }
    *state_mode = PENDING_REPLAY_MODE;
    out.requested.send(ModeSwitchRequested(request.mode));
    let byte1 = request.byte1();
    if byte1 > 1 {
        return Err(UnreachableRequestByte { byte1 });
    }
    out.byte.send(ModeSwitchByte(byte1));
    Ok(())
}

/// `esc_menu_canvas` 早退条件的输入（O：`[ui+0x18]`、`[ui+0x38]`、`[ui+0x28]`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EscMenuProbe {
    /// `+0x18`：悬停标志字节。
    pub hovered: bool,
    /// `+0x38`：悬停 widget 的 id。
    pub widget_id: u64,
    /// `+0x28`：交点载荷标签；`2` = 无。
    pub hit_tag: u32,
}

/// 早退判定（O：0x140a86b9f..0x140a86baf）：悬停 && id 等于 `hash("esc_menu_canvas")` && 标签 != 2 时
/// 整个系统直接返回。哈希函数（0x1405bdda0）未恢复，由调用方传入结果。
pub fn esc_menu_blocks(probe: EscMenuProbe, esc_menu_canvas_id: u64) -> bool {
    probe.hovered && probe.widget_id == esc_menu_canvas_id && probe.hit_tag != 2
}

/// `mode_manager` 单次运行的结果。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModeManagerStep {
    /// 被 ESC 菜单画布悬停挡住，未读事件，也不走跳表。
    Blocked,
    /// 已排空请求队列；`handled` 是被取走的最后一条请求。调用方随后按模式 id 走跳表（未恢复）。
    Drained {
        handled: Option<RequestAppModeChange>,
    },
}

/// `mode_manager` 的已恢复前半段：早退 + 排空 + 分发。
///
/// 排空与机器码一致：读游标推进到事件总数，有未读事件时只处理**最后一条**，且以 `force = true` 调分发器。
pub fn mode_manager_step(
    probe: EscMenuProbe,
    esc_menu_canvas_id: u64,
    cursor: &mut EventCursor<RequestAppModeChange>,
    requests: &Events<RequestAppModeChange>,
    state_mode: &mut u32,
    out: &mut SwitchEvents,
) -> Result<ModeManagerStep, UnreachableRequestByte> {
    if esc_menu_blocks(probe, esc_menu_canvas_id) {
        return Ok(ModeManagerStep::Blocked);
    }
    let last = cursor.read(requests).last().copied();
    if let Some(request) = last {
        check_if_can_switch(true, state_mode, &request, out)?;
    }
    Ok(ModeManagerStep::Drained { handled: last })
}

/// 模式变化事件（O：0x28 B `EventInstance`，`{first 16B @0, second 16B @0x10, id}`）。
///
/// 字段名为推断（I）：机器码把第 1 个参数写在 `+0`、第 2 个参数写在 `+0x10`，随后 `*second = *first`，
/// 所以 `second` 是"上次值"，`first` 是"当前值"。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppModeChanged<M: Copy + Send + Sync + 'static> {
    pub current: M,
    pub previous: M,
}

/// `send_mode_just_changed_event`（0x140a875e0）：两个 `AppMode`（16 B）不等时发事件并把 `previous` 同步为 `current`。
/// 不等判定是原版 `PartialEq::ne`（0x140a825e0），这里用 `M: PartialEq`。
pub fn send_mode_just_changed_event<M>(
    current: &M,
    previous: &mut M,
    out: &mut Events<AppModeChanged<M>>,
) where
    M: Copy + PartialEq + Send + Sync + 'static,
{
    if *current != *previous {
        out.send(AppModeChanged {
            current: *current,
            previous: *previous,
        });
        *previous = *current;
    }
}

/// 工具栏类别下标：`check_if_can_switch<closure_env$N>` 与类别的对应（O，由各闭包的 `len < N+1` 与偏移得出）。
pub mod category_closure {
    pub const ENV_5: usize = 0;
    pub const ENV_6: usize = 1;
    pub const ENV_7: usize = 2;
    pub const ENV_8: usize = 3;
    pub const ENV_9: usize = 4;
    pub const ENV_12: usize = 5;
    pub const ENV_10: usize = 6;
    /// env_3：特殊闭包，见 [`super::category_env3`]。
    pub const ENV_3: usize = 7;
    pub const ENV_13: usize = 8;
    pub const ENV_11: usize = 9;
    /// env_14：特殊闭包，见 [`super::category_env14`]。
    pub const ENV_14: usize = 10;
}

/// 通用类别闭包（env_5..13，非 force 分支）：推进类别 `n`，条目非哨兵则写入 `pending`。
pub fn category_advance(
    n: usize,
    toolbar: &mut Toolbar,
    pending: &mut ModeEntry,
) -> Result<(), ToolbarIndexOutOfBounds> {
    if let Some(entry) = toolbar.advance_category(n)? {
        *pending = entry;
    }
    Ok(())
}

/// env_14（Terrain，类别 10，0x140afa9f0）：`pending.tag == 0x13` 时先清零 tag，再以 `pending` 副本直调
/// `switch_mode` 并返回；否则同通用闭包。
pub fn category_env14<S: SwitchMode>(
    toolbar: &mut Toolbar,
    pending: &mut ModeEntry,
    switch: &mut S,
) -> Result<(), ToolbarIndexOutOfBounds> {
    if pending.tag == 0x13 {
        pending.tag = 0;
        let copy = *pending;
        switch.switch_mode(toolbar, &copy);
        return Ok(());
    }
    category_advance(category_closure::ENV_14, toolbar, pending)
}

/// env_3（类别 7，0x140afa2c0）：先把 `request` 整块拷入 `pending`；请求标签归一化后不是 0 且不是 15
/// （即标签 `6..=0x13`）时直调 `switch_mode(toolbar, request)` 返回；否则只推进类别 7 的选中下标，
/// **不**把条目写回 `pending`。
pub fn category_env3<S: SwitchMode>(
    request: &ModeEntry,
    toolbar: &mut Toolbar,
    pending: &mut ModeEntry,
    switch: &mut S,
) -> Result<(), ToolbarIndexOutOfBounds> {
    *pending = *request;
    let variant = request.normalized_variant();
    if variant != 0 && variant != 0xf {
        switch.switch_mode(toolbar, request);
        return Ok(());
    }
    toolbar.select_index(category_closure::ENV_3).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn probe(hovered: bool, widget_id: u64, hit_tag: u32) -> EscMenuProbe {
        EscMenuProbe {
            hovered,
            widget_id,
            hit_tag,
        }
    }

    fn count<T: Event>(events: &Events<T>) -> usize {
        events.get_cursor().read(events).count()
    }

    #[test]
    fn force_branch_only_pushes_the_forced_event() {
        let mut state = 7;
        let mut out = SwitchEvents::default();
        check_if_can_switch(
            true,
            &mut state,
            &RequestAppModeChange { mode: 0x1234 },
            &mut out,
        )
        .unwrap();
        assert_eq!(state, 7);
        assert_eq!(
            (count(&out.forced), count(&out.requested), count(&out.byte)),
            (1, 0, 0)
        );
    }

    #[test]
    fn non_force_branch_sets_replay_and_pushes_both_events() {
        let mut state = 0;
        let mut out = SwitchEvents::default();
        let req = RequestAppModeChange { mode: 0x0100 | 9 };
        check_if_can_switch(false, &mut state, &req, &mut out).unwrap();
        assert_eq!(state, PENDING_REPLAY_MODE);
        assert_eq!(
            out.requested.get_cursor().read(&out.requested).next(),
            Some(&ModeSwitchRequested(0x0109))
        );
        assert_eq!(
            out.byte.get_cursor().read(&out.byte).next(),
            Some(&ModeSwitchByte(1))
        );
    }

    #[test]
    fn non_force_byte1_above_one_is_the_original_unreachable_after_first_event() {
        let mut state = 0;
        let mut out = SwitchEvents::default();
        let err = check_if_can_switch(
            false,
            &mut state,
            &RequestAppModeChange { mode: 0x0200 },
            &mut out,
        )
        .unwrap_err();
        assert_eq!(err, UnreachableRequestByte { byte1: 2 });
        assert_eq!(state, PENDING_REPLAY_MODE);
        assert_eq!((count(&out.requested), count(&out.byte)), (1, 0));
    }

    #[test]
    fn esc_menu_blocks_only_when_hovered_matching_id_and_tag_not_two() {
        let id = 0xabcd;
        assert!(esc_menu_blocks(probe(true, id, 0), id));
        assert!(!esc_menu_blocks(probe(false, id, 0), id));
        assert!(!esc_menu_blocks(probe(true, id + 1, 0), id));
        assert!(!esc_menu_blocks(probe(true, id, 2), id));
    }

    #[test]
    fn blocked_step_leaves_unread_requests_in_the_queue() {
        let mut requests = Events::<RequestAppModeChange>::default();
        requests.send(RequestAppModeChange { mode: 1 });
        let mut cursor = requests.get_cursor();
        let (mut state, mut out) = (0, SwitchEvents::default());
        let step = mode_manager_step(
            probe(true, 5, 0),
            5,
            &mut cursor,
            &requests,
            &mut state,
            &mut out,
        )
        .unwrap();
        assert_eq!(step, ModeManagerStep::Blocked);
        assert_eq!(cursor.len(&requests), 1);
        assert_eq!(count(&out.forced), 0);
    }

    #[test]
    fn drain_handles_only_the_last_unread_request_with_force() {
        let mut requests = Events::<RequestAppModeChange>::default();
        requests.send(RequestAppModeChange { mode: 1 });
        requests.send(RequestAppModeChange { mode: 2 });
        requests.send(RequestAppModeChange { mode: 3 });
        let mut cursor = requests.get_cursor();
        let (mut state, mut out) = (0, SwitchEvents::default());
        let step = mode_manager_step(
            probe(false, 0, 2),
            9,
            &mut cursor,
            &requests,
            &mut state,
            &mut out,
        )
        .unwrap();
        assert_eq!(
            step,
            ModeManagerStep::Drained {
                handled: Some(RequestAppModeChange { mode: 3 })
            }
        );
        assert_eq!(count(&out.forced), 1);
        assert_eq!(cursor.len(&requests), 0);
        let step = mode_manager_step(
            probe(false, 0, 2),
            9,
            &mut cursor,
            &requests,
            &mut state,
            &mut out,
        )
        .unwrap();
        assert_eq!(step, ModeManagerStep::Drained { handled: None });
        assert_eq!(count(&out.forced), 1);
    }

    #[test]
    fn mode_changed_event_fires_once_and_syncs_previous() {
        let mut out = Events::<AppModeChanged<[u64; 2]>>::default();
        let current = [6, 1];
        let mut previous = [2, 0];
        send_mode_just_changed_event(&current, &mut previous, &mut out);
        assert_eq!(previous, current);
        let got: Vec<_> = out.get_cursor().read(&out).copied().collect();
        assert_eq!(
            got,
            vec![AppModeChanged {
                current: [6, 1],
                previous: [2, 0]
            }]
        );
        send_mode_just_changed_event(&current, &mut previous, &mut out);
        assert_eq!(out.get_cursor().read(&out).count(), 1);
    }

    #[derive(Default)]
    struct RecordingSwitch {
        calls: Vec<ModeEntry>,
    }

    impl SwitchMode for RecordingSwitch {
        fn switch_mode(&mut self, _toolbar: &mut Toolbar, entry: &ModeEntry) {
            self.calls.push(*entry);
        }
    }

    fn toolbar_11(cat: usize, modes: Vec<ModeEntry>, remembered: usize, current: u8) -> Toolbar {
        use crate::resources::toolbar::ToolbarCategoryModes;
        let mut categories = vec![ToolbarCategoryModes::default(); 11];
        categories[cat] = ToolbarCategoryModes {
            modes,
            current: remembered,
        };
        Toolbar {
            categories,
            current,
        }
    }

    fn me(tag: u32, arg: u64, extra: u32) -> ModeEntry {
        ModeEntry { tag, arg, extra }
    }

    #[test]
    fn generic_closure_writes_the_entry_into_pending() {
        let mut tb = toolbar_11(
            category_closure::ENV_10,
            vec![me(7, 1, 2), me(8, 3, 4)],
            0,
            category_closure::ENV_10 as u8,
        );
        let mut pending = ModeEntry::default();
        category_advance(category_closure::ENV_10, &mut tb, &mut pending).unwrap();
        assert_eq!(pending, me(8, 3, 4));
    }

    #[test]
    fn env14_with_tag_0x13_clears_tag_and_calls_switch_mode_only() {
        let mut tb = toolbar_11(category_closure::ENV_14, vec![me(9, 0, 0)], 0, 0);
        let mut pending = me(0x13, 77, 5);
        let mut sw = RecordingSwitch::default();
        category_env14(&mut tb, &mut pending, &mut sw).unwrap();
        assert_eq!(pending, me(0, 77, 5));
        assert_eq!(sw.calls, vec![me(0, 77, 5)]);
        assert_eq!(tb.current, 0);
    }

    #[test]
    fn env14_otherwise_behaves_like_the_generic_closure() {
        let mut tb = toolbar_11(category_closure::ENV_14, vec![me(9, 6, 7)], 0, 3);
        let mut pending = me(1, 0, 0);
        let mut sw = RecordingSwitch::default();
        category_env14(&mut tb, &mut pending, &mut sw).unwrap();
        assert_eq!(pending, me(9, 6, 7));
        assert!(sw.calls.is_empty());
        assert_eq!(tb.current, 10);
    }

    #[test]
    fn env3_copies_request_and_switches_for_tags_6_to_0x13() {
        let mut tb = toolbar_11(category_closure::ENV_3, vec![me(1, 0, 0)], 0, 0);
        let mut pending = ModeEntry::default();
        let mut sw = RecordingSwitch::default();
        let request = me(0x10, 9, 9);
        category_env3(&request, &mut tb, &mut pending, &mut sw).unwrap();
        assert_eq!(pending, request);
        assert_eq!(sw.calls, vec![request]);
        assert_eq!(tb.current, 0);
    }

    #[test]
    fn env3_for_tags_0_to_5_and_0x14_only_advances_category_7() {
        for tag in [0, 5, 0x14] {
            let mut tb = toolbar_11(
                category_closure::ENV_3,
                vec![me(1, 0, 0), me(2, 0, 0)],
                0,
                category_closure::ENV_3 as u8,
            );
            let mut pending = ModeEntry::default();
            let mut sw = RecordingSwitch::default();
            let request = me(tag, 1, 2);
            category_env3(&request, &mut tb, &mut pending, &mut sw).unwrap();
            assert_eq!(pending, request, "tag {tag}");
            assert!(sw.calls.is_empty(), "tag {tag}");
            assert_eq!(
                tb.categories[category_closure::ENV_3].current,
                1,
                "tag {tag}"
            );
            assert_eq!(tb.current, 7, "tag {tag}");
        }
    }
}
