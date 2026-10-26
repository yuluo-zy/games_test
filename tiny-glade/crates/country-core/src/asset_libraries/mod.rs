//! 游戏资产的类型化身份。句柄不拥有资源，Bevy 只提供组件存储与查询。
//! 原作来源见项目 evidence；当前接口、行为变化与验证范围见 README.md。

#![allow(clippy::new_without_default)]
use std::fmt;

use std::sync::atomic::{AtomicU64, Ordering};

static HANDLE_COUNT: AtomicU64 = AtomicU64::new(1);

/// 游戏自有的 64 位资产编号，零保留为无效值。
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HandleId(u64);

impl HandleId {
    /// 创建初始状态。
    pub fn new() -> Self {
        HandleId(
            HANDLE_COUNT
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |next_id| {
                    next_id.checked_add(1)
                })
                .expect("资产句柄编号已耗尽"),
        )
    }

    pub fn from_raw(raw: u64) -> Self {
        HandleId(raw)
    }

    pub fn raw(self) -> u64 {
        self.0
    }
}

impl fmt::Debug for HandleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HandleId({:#x})", self.0)
    }
}

/// 带资源类型的轻量句柄；标记不占空间，不持有 T，也不增加其引用计数。
#[derive(bevy_ecs::component::Component)]
#[component(immutable,
    on_insert = crate::render::extract::record_component_change::<Self>,
    on_remove = crate::render::extract::record_component_change::<Self>)]
pub struct Handle<T> {
    id: HandleId,
    _marker: core::marker::PhantomData<fn() -> T>,
}

impl<T: Send + Sync + 'static> crate::render::extract::JournaledComponent for Handle<T> {}

impl<T> Handle<T> {
    pub(crate) fn from_raw(raw: u64) -> Self {
        Self { id: HandleId::from_raw(raw), _marker: core::marker::PhantomData }
    }
    /// 创建初始状态。
    pub fn new() -> Self {
        Handle {
            id: HandleId::new(),
            _marker: core::marker::PhantomData,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.id.raw() != 0
    }

    pub fn id(&self) -> HandleId {
        self.id
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Handle<T> {}
impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T> Eq for Handle<T> {}
impl<T> PartialOrd for Handle<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for Handle<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
impl<T> core::hash::Hash for Handle<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Handle").field(&self.id).finish()
    }
}

/// 可命名资产的基础描述。
#[derive(Clone, Debug)]
pub struct Asset {
    pub name: String,
}

impl Asset {
    pub fn new(name: String) -> Self {
        Asset { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub mod audio_trigger_library;
pub mod color_library;
pub mod configwatch;
pub mod descriptor_table;
pub mod mesh_library;
pub mod meshwatch;
pub mod placement_data_library;
pub mod runtime;
pub mod shader_library;
pub mod ssbo_library;
pub mod texture_library;
pub mod texture_watch;
pub mod transient_mesh;
pub mod vao_library;
