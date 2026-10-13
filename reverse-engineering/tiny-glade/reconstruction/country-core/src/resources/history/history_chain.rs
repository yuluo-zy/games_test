//! `resources::history::history_chain` — 历史链索引与编辑链（B 骨架对齐，WP13）。
//!
//! O 证据（合同见 `contracts/history.md`）：
//!   - `HistoryChainIndex` u32 恒等封装（`new`/`inner` 0x140833250；Display
//!     fmt 0x140b31c90；serde 数字；`next`/`prev` 内联进 insert_starting_edit /
//!     history_process）。
//!   - `HistoryEditChain` = Vec<MultiEdit> 封装（元素 0x28=40B，O：grow_one
//!     元素尺寸；`iter_mut` 0x140b31cf0）。

use crate::resources::history::MultiEdit;

/// 历史链索引（O：u32 恒等封装，不猜当前 cursor、不检查链长）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HistoryChainIndex(pub u32);

impl HistoryChainIndex {
    /// O：`new`/`inner` 0x140833250 恒等。
    pub fn new(index: u32) -> Self {
        Self(index)
    }
    /// O：方法存在（`inner`，与 new 同址折叠）。
    pub fn inner(self) -> u32 {
        self.0
    }
    /// O：`next`（内联进 insert_starting_edit；+1）。
    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
    /// O：`prev`（内联进 history_process；-1）。
    pub fn prev(self) -> Self {
        Self(self.0 - 1)
    }
}

impl std::fmt::Display for HistoryChainIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 历史编辑链（O：Vec<MultiEdit> 封装，元素 0x28=40B）。
#[derive(Clone, Debug, Default)]
pub struct HistoryEditChain {
    pub inner: Vec<MultiEdit>,
}

impl HistoryEditChain {
    /// O：`new_from_inner`。
    pub fn new_from_inner(inner: Vec<MultiEdit>) -> Self {
        Self { inner }
    }
    /// O：`is_empty`。
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// O：`iter` / `iter_mut` 0x140b31cf0。
    pub fn iter(&self) -> std::slice::Iter<'_, MultiEdit> {
        self.inner.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, MultiEdit> {
        self.inner.iter_mut()
    }
    /// O：`get`。
    pub fn get(&self, index: usize) -> Option<&MultiEdit> {
        self.inner.get(index)
    }
    /// O：`push`（元素 0x28）。
    pub fn push(&mut self, edit: MultiEdit) {
        self.inner.push(edit);
    }
    /// O：`insert_at_start`（首部插入，memmove 右移）。
    pub fn insert_at_start(&mut self, edit: MultiEdit) {
        self.inner.insert(0, edit);
    }
    /// O：`truncate`。
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }
}

impl std::ops::Index<usize> for HistoryEditChain {
    type Output = MultiEdit;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}
