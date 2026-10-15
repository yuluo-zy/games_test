//! `toolbar.rs` — 工具栏资源与"按类别推进模式"逻辑。
//!
//! 证据（B 版 RNE 机器码，`evidence/country-core-deep/ui-input/decomp/`）：
//! - `Toolbar`：`categories`（cap-first Vec，`{cap@0, ptr@8, len@0x10}`）、`current: u8 @+0x18`（O，各类别闭包读写）。
//! - `ToolbarCategoryModes`（0x20 B）：`{cap@0, modes ptr@8, modes len@0x10, current@0x18}`（O，由闭包的
//!   `+0x148/+0x150/+0x158` 与 `+0xc8/+0xd0/+0xd8` 等偏移按 `N*0x20` 推出）。
//! - `ModeEntry`（0x10 B）：`{tag u32@0, arg u64@4, extra u32@0xc}`，闭包里整块按这三段拷贝（O）；
//!   `tag == 0x15` 是"无"哨兵（O，`Option` niche）。
//!
//! 未恢复（保持显式缺口，不猜）：`Toolbar::new` 的各类别模式表、`contains`/`switch_mode` 的逐变体比较
//! （`contains` 是 derive(PartialEq) 生成，文档里"区间匹配"的说法是对 niche 编码的误读）、`ModeEntry`
//! 各变体负载含义。`switch_mode` 由调用方通过 [`SwitchMode`] 注入。
use bevy_ecs::resource::Resource;

/// `ModeEntry.tag` 的"无"哨兵（O：各闭包 `if (iVar1 != 0x15)` 才拷贝入 pending）。
pub const MODE_ENTRY_NONE_TAG: u32 = 0x15;

/// 0x10 B 的模式条目（也是 `pending`/`AppMode` 请求的 16 字节载体）。
///
/// 字段名按偏移命名；逐变体的负载语义未恢复，只保证"整块拷贝"行为与机器码一致。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ModeEntry {
    /// `+0x00`，变体标签。`0x15` = 无。
    pub tag: u32,
    /// `+0x04`（8 字节，未对齐读写）。
    pub arg: u64,
    /// `+0x0c`。
    pub extra: u32,
}

impl ModeEntry {
    /// 是否为"无"哨兵。
    pub fn is_none_sentinel(&self) -> bool {
        self.tag == MODE_ENTRY_NONE_TAG
    }

    /// 机器码里 `check_if_can_switch<closure_env$3>` 对标签的归一化（O，0x140afa2c0）：
    /// `tag - 6 < 0xf` 时取 `tag - 5`，否则 0。`6..=0x14 -> 1..=15`，其余 -> 0。
    pub fn normalized_variant(&self) -> u32 {
        let t = self.tag;
        if t.wrapping_sub(6) < 0xf { t - 5 } else { 0 }
    }
}

/// 一个类别：模式列表加"上次选中的下标"。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ToolbarCategoryModes {
    pub modes: Vec<ModeEntry>,
    pub current: usize,
}

/// 原版里对应 `FUN_1428d9518` 的下标越界 panic（`toolbar.rs`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToolbarIndexOutOfBounds {
    pub index: usize,
    pub len: usize,
}

impl core::fmt::Display for ToolbarIndexOutOfBounds {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "toolbar mode index {} out of bounds (len {})",
            self.index, self.len
        )
    }
}

impl std::error::Error for ToolbarIndexOutOfBounds {}

/// `switch_mode`（0x140af9850）的注入点：入参是 `(&mut Toolbar, &16B 记录)`。
pub trait SwitchMode {
    fn switch_mode(&mut self, toolbar: &mut Toolbar, entry: &ModeEntry);
}

/// 工具栏资源。
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct Toolbar {
    pub categories: Vec<ToolbarCategoryModes>,
    /// `+0x18`：当前类别下标。
    pub current: u8,
}

impl Toolbar {
    /// 类别 N 推进的前四步（O，见 [`Self::advance_category`]）：返回选中的模式下标，并已写 `Toolbar.current = N`。
    /// 类别列表过短或 `modes` 为空时返回 `Ok(None)` 且不改任何状态。
    pub fn select_index(&mut self, n: usize) -> Result<Option<usize>, ToolbarIndexOutOfBounds> {
        if self.categories.len() < n + 1 {
            return Ok(None);
        }
        let selected = self.current as usize == n;
        let cat = &mut self.categories[n];
        let len = cat.modes.len();
        if len == 0 {
            return Ok(None);
        }
        let idx = if selected {
            cat.current = (cat.current + 1) % len;
            cat.current
        } else if cat.current < len {
            cat.current
        } else {
            return Err(ToolbarIndexOutOfBounds {
                index: cat.current,
                len,
            });
        };
        self.current = n as u8;
        Ok(Some(idx))
    }

    /// 各 `check_if_can_switch<closure_env$N>` 非 force 分支共用的"类别 N 推进"（O）。
    ///
    /// 机器码顺序（以 env_10 = 类别 6 为例，0x140afa190）：
    /// 1. `categories.len < N+1` 返回（`env_5` 为 N=0，等价于 `len == 0`）；
    /// 2. 类别 N 的 `modes` 为空则返回；
    /// 3. `Toolbar.current == N`：`cat.current = (cat.current + 1) % len` 并取该下标；
    ///    否则不推进，取 `cat.current`，越界则 panic（这里返回 [`ToolbarIndexOutOfBounds`]）；
    /// 4. `Toolbar.current = N`；
    /// 5. 取 `modes[idx]`，非"无"哨兵才返回（调用方写入 pending）。
    ///
    /// 到第 5 步条目为哨兵时返回 `Ok(None)`，但第 3、4 步的状态更新已经发生。
    pub fn advance_category(
        &mut self,
        n: usize,
    ) -> Result<Option<ModeEntry>, ToolbarIndexOutOfBounds> {
        let Some(idx) = self.select_index(n)? else {
            return Ok(None);
        };
        let entry = self.categories[n].modes[idx];
        Ok((!entry.is_none_sentinel()).then_some(entry))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(tag: u32, arg: u64, extra: u32) -> ModeEntry {
        ModeEntry { tag, arg, extra }
    }

    fn toolbar_with(
        n_categories: usize,
        cat: usize,
        modes: Vec<ModeEntry>,
        current: usize,
    ) -> Toolbar {
        let mut categories = vec![ToolbarCategoryModes::default(); n_categories];
        categories[cat] = ToolbarCategoryModes { modes, current };
        Toolbar {
            categories,
            current: 0,
        }
    }

    #[test]
    fn normalized_variant_matches_0x140afa2c0_formula() {
        let n = |tag| entry(tag, 0, 0).normalized_variant();
        assert_eq!(n(0), 0);
        assert_eq!(n(5), 0);
        assert_eq!(n(6), 1);
        assert_eq!(n(0x13), 14);
        assert_eq!(n(0x14), 15);
        assert_eq!(n(0x15), 0);
    }

    #[test]
    fn short_category_list_returns_without_touching_state() {
        let mut tb = toolbar_with(3, 1, vec![entry(7, 1, 2)], 0);
        tb.current = 2;
        assert_eq!(tb.advance_category(6), Ok(None));
        assert_eq!(tb.current, 2);
    }

    #[test]
    fn empty_category_returns_without_touching_state() {
        let mut tb = toolbar_with(8, 6, vec![], 0);
        tb.current = 3;
        assert_eq!(tb.advance_category(6), Ok(None));
        assert_eq!(tb.current, 3);
    }

    #[test]
    fn already_selected_category_advances_and_wraps() {
        let modes = vec![entry(7, 10, 1), entry(8, 20, 2), entry(9, 30, 3)];
        let mut tb = toolbar_with(8, 6, modes, 2);
        tb.current = 6;
        assert_eq!(tb.advance_category(6), Ok(Some(entry(7, 10, 1))));
        assert_eq!(tb.categories[6].current, 0);
        assert_eq!(tb.advance_category(6), Ok(Some(entry(8, 20, 2))));
        assert_eq!(tb.categories[6].current, 1);
    }

    #[test]
    fn switching_into_a_category_keeps_its_remembered_mode() {
        let modes = vec![entry(7, 10, 1), entry(8, 20, 2)];
        let mut tb = toolbar_with(8, 6, modes, 1);
        tb.current = 0;
        assert_eq!(tb.advance_category(6), Ok(Some(entry(8, 20, 2))));
        assert_eq!(tb.categories[6].current, 1);
        assert_eq!(tb.current, 6);
    }

    #[test]
    fn remembered_index_out_of_bounds_is_the_original_panic() {
        let mut tb = toolbar_with(8, 6, vec![entry(7, 0, 0)], 5);
        tb.current = 0;
        assert_eq!(
            tb.advance_category(6),
            Err(ToolbarIndexOutOfBounds { index: 5, len: 1 })
        );
        assert_eq!(tb.current, 0);
    }

    #[test]
    fn none_sentinel_updates_state_but_yields_nothing() {
        let modes = vec![entry(MODE_ENTRY_NONE_TAG, 0, 0)];
        let mut tb = toolbar_with(8, 6, modes, 0);
        tb.current = 0;
        assert_eq!(tb.advance_category(6), Ok(None));
        assert_eq!(tb.current, 6);
    }

    #[test]
    fn category_zero_uses_the_same_rule_as_env_5() {
        let modes = vec![entry(7, 1, 1), entry(8, 2, 2)];
        let mut tb = toolbar_with(1, 0, modes, 0);
        tb.current = 0;
        assert_eq!(tb.advance_category(0), Ok(Some(entry(8, 2, 2))));
        assert_eq!(tb.categories[0].current, 1);
    }
}
