//! `frozen::FrozenMap` 的调用面占位（外部库 `frozen`，离线缓存缺失，见 `Cargo.toml` 注释）。
//!
//! 证据（O，`FrozenMap::insert` 0x140969bf0 / `get` 0x1405ca870 的调用点）：
//!  * `get(&self, &K) -> Option<&V::Target>`：键相等才返回；返回的是 `Box` 内部的值，故值一旦插入就不会移动；
//!  * `insert(&self, K, V) -> Result<(), &V::Target>`：键已存在 → 不替换，返回**已有值**的引用（调用方据此
//!    拼 panic 消息）；
//!  * 表带一个“借用中”字节（`[map+8]`），`export_from_rg` 等遍历入口对它做 `try_borrow().unwrap()`。
//!
//! 这里用 `RwLock<HashMap<K, Box<V>>>` 提供同样的契约：值装箱、永不删除，因此把引用的生命周期延长到
//! `&self` 是安全的。库本体的实现不重写。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::doc_lazy_continuation)]
use std::collections::HashMap;
use std::hash::Hash;

use parking_lot::RwLock;

pub struct FrozenMap<K, V> {
    map: RwLock<HashMap<K, Box<V>>>,
}

impl<K, V> Default for FrozenMap<K, V> {
    fn default() -> Self {
        FrozenMap {
            map: RwLock::new(HashMap::new()),
        }
    }
}

impl<K: Eq + Hash, V> FrozenMap<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let guard = self.map.read();
        let ptr: *const V = &**guard.get(key)?;
        // SAFETY: 值装箱且永不移除 / 替换，`&self` 存活期间指针有效。
        Some(unsafe { &*ptr })
    }

    /// 独占访问（O：`get_mut` 内联了表探测，调用方持 `&mut` 库）。
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.map.get_mut().get_mut(key).map(|b| &mut **b)
    }

    /// 独占移除（O：`AssetMeshLibrary::remove` 直接对内部 hashbrown 表 `remove_entry`，值随后被 drop）。
    pub fn remove(&mut self, key: &K) -> Option<Box<V>> {
        self.map.get_mut().remove(key)
    }

    pub fn insert(&self, key: K, value: Box<V>) -> Result<(), &V> {
        let mut guard = self.map.write();
        if let Some(existing) = guard.get(&key) {
            let ptr: *const V = &**existing;
            // SAFETY: 同 `get`。
            return Err(unsafe { &*ptr });
        }
        guard.insert(key, value);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.map.read().len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.read().is_empty()
    }

    /// 遍历全部值（O：`export_from_rg` 对 hashbrown 表逐桶迭代）。
    pub fn for_each(&self, mut f: impl FnMut(&V)) {
        let guard = self.map.read();
        for v in guard.values() {
            let ptr: *const V = &**v;
            // SAFETY: 同 `get`。
            f(unsafe { &*ptr });
        }
    }
}
