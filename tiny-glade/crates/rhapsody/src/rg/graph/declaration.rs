//! Declaration types the graph is built from: resources, passes and their access dependencies.
use crate::rg::resource_info::AccessDeclaration;
use crate::rg::resource_info::ExportDeclaration;
use crate::rg::resource_registry::BufferDesc;
use crate::rg::resource_registry::TextureDesc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Descriptor {
    Image(TextureDesc),
    Buffer(BufferDesc),
}

impl Descriptor {
    pub fn kind_code(self) -> u8 {
        match self {
            Self::Image(_) => 0,
            Self::Buffer(_) => 1,
        }
    }
    pub fn usage(self) -> u32 {
        match self {
            Self::Image(desc) => desc.words_at_00_through_18[0],
            Self::Buffer(desc) => desc.word_at_08,
        }
    }
    pub fn with_usage(self, usage: u32) -> Self {
        match self {
            Self::Image(mut desc) => {
                desc.words_at_00_through_18[0] = usage;
                Self::Image(desc)
            }
            Self::Buffer(mut desc) => {
                desc.word_at_08 = usage;
                Self::Buffer(desc)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OriginSource {
    Create,
    ImportOwned { token: u64, initial_access: u8 },
    ImportUnowned { token: u64, initial_access: u8 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceDeclaration {
    pub sharing: crate::tracking::ResourceSharing,
    pub name: crate::resource::ResourceName,
    pub descriptor: Descriptor,
    pub source: OriginSource,
}

/// 可导出资源载体；`raw()` 返回图内逻辑句柄，`index()` 返回逻辑索引。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExportableResource {
    handle: super::super::resource::ResourceHandle,
}

impl ExportableResource {
    pub fn new(handle: super::super::resource::ResourceHandle) -> Self {
        Self { handle }
    }
    pub fn raw(&self) -> super::super::resource::ResourceHandle {
        self.handle
    }
    pub fn index(&self) -> u32 {
        self.handle.raw()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PassDeclaration {
    pub queue_class: crate::device::QueueClass,
    pub graph_id: Option<super::super::resource::GraphId>,
    pub name: crate::resource::ResourceName,
    pub accesses: Vec<AccessDeclaration>,
    /// Keeps externally observable work even when none of its outputs is exported.
    pub side_effect: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GraphDeclaration {
    pub subgraphs: Vec<super::super::subgraph::SubgraphDeclaration>,
    pub resources: Vec<ResourceDeclaration>,
    pub passes: Vec<PassDeclaration>,
    pub exports: Vec<ExportDeclaration>,
    pub batch_per_pass: bool,
    /// Logical resources whose first-using pass begins a new batch, sorted and
    /// unique. Used for imports made available by an external wait (the
    /// swapchain acquire): the wait is attached to a batch, so earlier
    /// independent passes stay out of it.
    pub gated_resources: Vec<u32>,
}

/// Resource count up to which duplicate names are found by pairwise comparison.
pub(super) const INLINE_NAME_LIMIT: usize = 48;

/// Sizes of a finished declaration. The next frame reserves its vectors from this in
/// one step instead of growing them through a chain of reallocations.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DeclarationShape {
    pub(super) passes: u32,
    pub(super) resources: u32,
    pub(super) exports: u32,
}

impl DeclarationShape {
    /// Upper bounds of what is reserved ahead: a pathological frame must not pin memory.
    const MAX_PASSES: u32 = 1024;
    const MAX_RESOURCES: u32 = 4096;
    pub fn of(declaration: &GraphDeclaration) -> Self {
        let clamp = |len: usize, limit: u32| len.min(limit as usize) as u32;
        Self {
            passes: clamp(declaration.passes.len(), Self::MAX_PASSES),
            resources: clamp(declaration.resources.len(), Self::MAX_RESOURCES),
            exports: clamp(declaration.exports.len(), Self::MAX_RESOURCES),
        }
    }
    /// The element-wise maximum, so alternating graphs (main and presentation) both fit.
    #[must_use]
    pub fn union(self, other: Self) -> Self {
        Self {
            passes: self.passes.max(other.passes),
            resources: self.resources.max(other.resources),
            exports: self.exports.max(other.exports),
        }
    }
}

pub trait AccessDependency {
    /// The declared access of `code`; resource usage flags derive from it.
    fn access_mask(&self, code: u8) -> ash::vk::AccessFlags;
    fn is_write(&self, code: u8) -> bool;
    /// Custom CPU policies may omit native details. Their conservative scope
    /// cannot be reused for a policy with different Vulkan stages or layouts.
    fn native_semantics(
        &self,
        code: u8,
    ) -> (
        crate::native::AccessScope,
        ash::vk::ImageLayout,
    ) {
        let write = self.is_write(code);
        (
            crate::native::AccessScope {
                stages: ash::vk::PipelineStageFlags::ALL_COMMANDS,
                access: if write {
                    ash::vk::AccessFlags::MEMORY_READ | ash::vk::AccessFlags::MEMORY_WRITE
                } else {
                    ash::vk::AccessFlags::MEMORY_READ
                },
                write,
            },
            ash::vk::ImageLayout::GENERAL,
        )
    }
}

/// 渲染回调须为 `Send`：并行录制时不同块的回调在任务池线程上执行。
pub type RenderCallback = Box<dyn for<'a> FnMut(super::super::executor::PassContext<'a>) -> bool + Send>;

pub type RetireCallback = Box<dyn FnOnce() + Send>;

/// 单图最多 `1 << 15` 个 pass；构图与编译入口均返回 [`BuildError::PassLimit`]。
pub const MAX_PASSES: usize = 1 << 15;

/// A shared policy (e.g. the backend's `Arc<AccessTable>`) is used as-is.
impl<T: AccessDependency + ?Sized> AccessDependency for std::sync::Arc<T> {
    fn access_mask(&self, code: u8) -> ash::vk::AccessFlags {
        (**self).access_mask(code)
    }
    fn is_write(&self, code: u8) -> bool {
        (**self).is_write(code)
    }
    fn native_semantics(&self, code: u8) -> (crate::native::AccessScope, ash::vk::ImageLayout) {
        (**self).native_semantics(code)
    }
}
