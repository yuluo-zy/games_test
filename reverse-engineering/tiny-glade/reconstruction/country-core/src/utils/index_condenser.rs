//! utils/index_condenser.rs — `utils::index_condenser::IndexCondenser<T>`（游戏自有 `utils` crate；
//! 唯一实例 `IndexCondenser<gpu_world::InstanceKey>`）。
//!
//! 机器码（B patched EXE）：`IndexCondenser::remove<InstanceKey>` 0x140a19f50（561B，O）；
//! `Condex<InstanceKey>` Debug 0x140a19e90；`drop_in_place` 0x1405d5a80。
//! `extend_no_data` 0x140992280 / `create_instances` 0x14156d0d0 / `update_instance_data` 0x14156db40 /
//! `update_instance_mesh` 0x140991d60 把 push / resolve 内联展开，这里按同一语义收拢成方法。
//!
//! 布局（O，相对 `OwnedGpuWorldSubset` 起点）：
//! * `dense_to_id: Vec<u32>`（+0x00 cap、+0x08 ptr、+0x10 len）：稠密下标 → 稳定 id；
//! * `id_to_dense: HashMap<u32, u32>`（hashbrown RawTable 在 +0x18..+0x38，`RandomState` 在 +0x38）。
//!
//! 语义（O）：稳定 id 在实例被 swap_remove 搬动时保持不变；调用方持有的 key 里另带一个
//! “稠密下标提示”（key+4）：提示命中（`hint < len && dense_to_id[hint] == id`）走快路径，
//! 否则查 `id_to_dense`（空表直接失败）并把查到的稠密下标回写进提示。

use core::marker::PhantomData;
use std::collections::HashMap;

/// 稳定 id ↔ 稠密下标压缩器。`T` 仅用于类型区分（原版为 `IndexCondenser<InstanceKey>`）。
pub struct IndexCondenser<T> {
    dense_to_id: Vec<u32>,
    id_to_dense: HashMap<u32, u32>,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Default for IndexCondenser<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IndexCondenser<T> {
    pub fn new() -> Self {
        Self {
            dense_to_id: Vec::new(),
            id_to_dense: HashMap::new(),
            _marker: PhantomData,
        }
    }

    /// 当前稠密元素数（O：`[+0x10]`）。
    pub fn len(&self) -> usize {
        self.dense_to_id.len()
    }

    pub fn is_empty(&self) -> bool {
        self.dense_to_id.is_empty()
    }

    /// id 是否已被占用（O：`extend_no_data` / `create_instances` 分配 id 时的碰撞检查；
    /// `id_to_dense` 为空表时跳过查找）。
    pub fn contains_id(&self, id: u32) -> bool {
        !self.id_to_dense.is_empty() && self.id_to_dense.contains_key(&id)
    }

    /// 追加一个新 id，返回其稠密下标（O：`Vec::push(id)` + `HashMap::insert(id, dense)`）。
    pub fn push(&mut self, id: u32) -> usize {
        let dense = self.dense_to_id.len();
        self.dense_to_id.push(id);
        self.id_to_dense.insert(id, dense as u32);
        dense
    }

    /// 稠密下标 → id。
    pub fn id_at(&self, dense: usize) -> Option<u32> {
        self.dense_to_id.get(dense).copied()
    }

    /// 解析 `(id, hint)`：快路径 + 哈希回退（O：`update_instance_data` / `update_instance_mesh` 头部）。
    /// 命中哈希表时把稠密下标写回 `hint`。失败返回 `None`（原版调用方 `unwrap_failed`）。
    pub fn resolve(&self, id: u32, hint: &mut u32) -> Option<usize> {
        if let Some(&stored) = self.dense_to_id.get(*hint as usize) {
            if stored == id {
                return Some(*hint as usize);
            }
        }
        if self.id_to_dense.is_empty() {
            return None;
        }
        let dense = *self.id_to_dense.get(&id)?;
        *hint = dense;
        Some(dense as usize)
    }

    /// `IndexCondenser::remove`（O 0x140a19f50）：解析（同 `resolve`）→ `swap_remove` 稠密数组 →
    /// 从 `id_to_dense` 删除被移除 id → 若有元素被换到该位置，把它的映射改写为新下标。
    /// 返回被移除元素原先的稠密下标（原版 `rax = 1, rdx = index`；未命中 `rax = 0`）。
    pub fn remove(&mut self, id: u32, hint: &mut u32) -> Option<usize> {
        let dense = self.resolve(id, hint)?;
        let len = self.dense_to_id.len();
        assert!(
            dense < len,
            "swap_remove index (is {dense}) should be < len (is {len})"
        );
        let removed_id = self.dense_to_id.swap_remove(dense);
        self.id_to_dense.remove(&removed_id);
        if dense < self.dense_to_id.len() {
            // 原版：`[rdi+0x30] == 0` → `Option::unwrap` 失败；非空表必然含被换位 id。
            let moved_id = self.dense_to_id[dense];
            *self
                .id_to_dense
                .get_mut(&moved_id)
                .expect("called `Option::unwrap()` on a `None` value") = dense as u32;
        }
        Some(dense)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct K;

    #[test]
    fn push_resolve_and_fast_path() {
        let mut c = IndexCondenser::<K>::new();
        assert_eq!(c.push(10), 0);
        assert_eq!(c.push(11), 1);
        let mut hint = 1;
        assert_eq!(c.resolve(11, &mut hint), Some(1));
        // 提示错误 → 走哈希并回写提示。
        let mut bad = 0;
        assert_eq!(c.resolve(11, &mut bad), Some(1));
        assert_eq!(bad, 1);
        // 提示越界、id 不存在。
        let mut oob = 99;
        assert_eq!(c.resolve(77, &mut oob), None);
        assert!(c.contains_id(10) && !c.contains_id(77));
    }

    #[test]
    fn remove_swaps_last_into_hole_and_fixes_map() {
        let mut c = IndexCondenser::<K>::new();
        for id in [5, 6, 7] {
            c.push(id);
        }
        let mut h = 0;
        assert_eq!(c.remove(5, &mut h), Some(0));
        assert_eq!(c.len(), 2);
        assert_eq!(c.id_at(0), Some(7)); // 末元素补位
        let mut h7 = 99;
        assert_eq!(c.resolve(7, &mut h7), Some(0)); // 映射已改写
        assert!(!c.contains_id(5));
        // 移除末元素：不需要补位。
        let mut h6 = 1;
        assert_eq!(c.remove(6, &mut h6), Some(1));
        assert_eq!(c.len(), 1);
        // 未命中。
        let mut hm = 0;
        assert_eq!(c.remove(6, &mut hm), None);
    }

    #[test]
    fn empty_map_fails_fast() {
        let c = IndexCondenser::<K>::new();
        let mut h = 0;
        assert_eq!(c.resolve(1, &mut h), None);
    }
}
