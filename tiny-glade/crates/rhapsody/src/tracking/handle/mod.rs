//! 类型化的代数句柄与资源存储。
//!
//! # 设计
//!
//! [`Handle<T>`] 是资源在 [`ResourceStore<T>`] 里的 8 字节 `Copy` 键，内部是
//! `slotmap::DefaultKey`（“槽位下标 + 代际”）。它不持有资源本体，只负责定位，因此可以
//! 在图、命令录制与外部系统中长期引用；槽位删除后代际递增，旧句柄自然失效，不会访问
//! 到后来复用了同一槽位的新资源。
//!
//! `T` 只是编译期标记（`PhantomData<fn() -> T>`），让 `Handle<Buffer>` 与
//! `Handle<Texture>` 成为不同类型，从类型系统上杜绝把纹理句柄当缓冲句柄用。函数指针
//! 标记协变、且不改变 `Send`/`Sync`/析构语义——句柄不拥有 `T`，因此无论 `T` 是否线程
//! 安全，句柄都可跨线程传递。
//!
//! [`ResourceStore<T>`] 是对 `SlotMap<DefaultKey, T>` 的类型化薄封装：存储本身复用
//! slotmap 的稠密槽位与代际空闲链表，增删 O(1)、紧凑、缓存友好。存储不承担线程同步，
//! 并发访问由上层用锁或按执行线程拆分。资源本体以 `Arc<资源>` 形式存入，句柄只是指向
//! 它的轻量索引。
//!
//! 与图层的 `TextureId`/`BufferId`（见 `ARCHITECTURE.md` 的「资源身份与使用追踪」一节）不同，本模块是 raw 层
//! 的通用句柄基础设施，不绑定任何具体资源类型。

use slotmap::{DefaultKey, Key, SlotMap};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// 指向 `ResourceStore<T>` 中某个资源槽位的轻量键。
///
/// `Copy`/`Clone`/`Eq`/`Hash`/`Ord` 都只作用于内部的槽位键（零尺寸的 `T` 标记不参与），
/// 因此这些实现对任意 `T` 都成立，不要求 `T` 实现任何 trait。
pub struct Handle<T> {
    key: DefaultKey,
    marker: PhantomData<fn() -> T>,
}

impl<T> Handle<T> {
    /// 从 64 位不透明值恢复句柄。该值必须来自同一次会话里 [`Handle::as_ffi`] 的输出，
    /// 跨会话、跨设备复用都没有意义（slotmap 键不保证跨存储稳定）。
    pub fn from_ffi(value: u64) -> Self {
        Self {
            key: if value == 0 { DefaultKey::null() } else { DefaultKey::from(slotmap::KeyData::from_ffi(value)) },
            marker: PhantomData,
        }
    }

    /// 句柄的 64 位不透明表示，便于作为无类型句柄传给外部代码。
    pub fn as_ffi(self) -> u64 {
        self.key.data().as_ffi()
    }

    /// 槽位下标。下标在槽位复用后保持不变，可用来映射 bindless 描述符数组下标等外部
    /// 索引；判定句柄是否仍有效必须同时看代际（`get` 会一起校验），不能只看下标。
    pub fn index(self) -> u32 {
        (self.as_ffi() & 0xffff_ffff) as u32
    }

    /// 是否为“空句柄”。空句柄在任何存储上查询都返回 `None`，是 `from_ffi` 可能得到的
    /// 哨兵值之一；正常插入得到的句柄不会是空句柄。
    pub fn is_null(self) -> bool {
        self.key.is_null()
    }
}

impl<T> Copy for Handle<T> {}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> Eq for Handle<T> {}

impl<T> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl<T> PartialOrd for Handle<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Handle<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let short = std::any::type_name::<T>()
            .rsplit("::")
            .next()
            .unwrap_or("T");
        write!(f, "Handle<{short}>({:?})", self.key)
    }
}

/// 按 [`Handle<T>`] 索引的稠密资源存储；实际存储交给 slotmap，本类型只负责类型安全。
pub struct ResourceStore<T> {
    slots: SlotMap<DefaultKey, T>,
}

impl<T> ResourceStore<T> {
    pub fn new() -> Self {
        Self {
            slots: SlotMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            slots: SlotMap::with_capacity(capacity),
        }
    }

    /// 存活资源数。
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    /// 当前为存储分配的槽位容量（含空闲槽）。
    pub fn capacity(&self) -> usize {
        self.slots.capacity()
    }

    /// Fallible capacity reservation used before publishing managed resources.
    /// Insertion after a successful reservation does not grow the slot table.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), std::collections::TryReserveError> {
        self.slots.try_reserve(additional)
    }

    /// 存入资源，返回指向它的句柄。
    pub fn insert(&mut self, value: T) -> Handle<T> {
        Handle {
            key: self.slots.insert(value),
            marker: PhantomData,
        }
    }

    /// 按句柄取只读引用；句柄过期（已删除/代际不符/空句柄）返回 `None`。
    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        self.slots.get(handle.key)
    }

    /// 按句柄取可变引用；同 [`ResourceStore::get`]。
    pub fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        self.slots.get_mut(handle.key)
    }

    /// 句柄当前是否仍有效。
    pub fn contains(&self, handle: Handle<T>) -> bool {
        self.slots.contains_key(handle.key)
    }

    /// 删除并取回资源，旧句柄立即失效；空出的槽位进入空闲链表供后续复用。
    pub fn remove(&mut self, handle: Handle<T>) -> Option<T> {
        self.slots.remove(handle.key)
    }

    /// 清空全部资源；已有句柄全部失效。
    pub fn clear(&mut self) {
        self.slots.clear();
    }

    /// 只保留满足条件的资源；被移除项按正常删除流程失效。
    pub fn retain(&mut self, mut keep: impl FnMut(Handle<T>, &mut T) -> bool) {
        self.slots.retain(|key, value| {
            keep(
                Handle {
                    key,
                    marker: PhantomData,
                },
                value,
            )
        });
    }

    pub fn iter(&self) -> impl Iterator<Item = (Handle<T>, &T)> {
        self.slots.iter().map(|(key, value)| {
            (
                Handle {
                    key,
                    marker: PhantomData,
                },
                value,
            )
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Handle<T>, &mut T)> {
        self.slots.iter_mut().map(|(key, value)| {
            (
                Handle {
                    key,
                    marker: PhantomData,
                },
                value,
            )
        })
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.slots.values()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.slots.values_mut()
    }
}

impl<T> Default for ResourceStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Debug> fmt::Debug for ResourceStore<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

#[cfg(test)]
mod tests;
