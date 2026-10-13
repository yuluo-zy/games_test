//! 只用于 l0-harness 离线编译；真实工程使用 crates.io 的 parking_lot =0.12.1。
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug, Default)]
pub struct RwLock<T>(std::sync::RwLock<T>);

impl<T> RwLock<T> {
    pub fn new(v: T) -> Self {
        RwLock(std::sync::RwLock::new(v))
    }
    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut().unwrap()
    }
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.0.read().unwrap_or_else(|e| e.into_inner())
    }
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write().unwrap_or_else(|e| e.into_inner())
    }
}

/// 与 parking_lot 0.12 同形的 `Mutex`：`lock` + `data_ptr`（后者 sparse_instance_buffer 不加锁读 chunk 表时用）。
pub struct Mutex<T> {
    lock: std::sync::Mutex<()>,
    data: std::cell::UnsafeCell<T>,
}
unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T: Default> Default for Mutex<T> {
    fn default() -> Self {
        Mutex::new(T::default())
    }
}
impl<T: core::fmt::Debug> core::fmt::Debug for Mutex<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Mutex")
    }
}

pub struct MutexGuard<'a, T> {
    _guard: std::sync::MutexGuard<'a, ()>,
    data: &'a mut T,
}
impl<T> core::ops::Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.data
    }
}
impl<T> core::ops::DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

impl<T> Mutex<T> {
    pub fn new(v: T) -> Self {
        Mutex { lock: std::sync::Mutex::new(()), data: std::cell::UnsafeCell::new(v) }
    }
    pub fn lock(&self) -> MutexGuard<'_, T> {
        let guard = self.lock.lock().unwrap_or_else(|e| e.into_inner());
        MutexGuard { _guard: guard, data: unsafe { &mut *self.data.get() } }
    }
    pub fn data_ptr(&self) -> *mut T {
        self.data.get()
    }
}
