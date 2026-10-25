//! indirect.rs — Vulkan indirect command wrappers.
//! O/I/U tags per `contracts/indirect.md`. All methods inline (0 standalone procs).

/// `VkDrawIndexedIndirectCommand` (20 bytes). Field order per Vulkan ABI (I).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}

impl DrawIndexedIndirectCommand {
    /// O name — zero command.
    pub fn empty() -> Self {
        Self::default()
    }
    /// O name — write helper (exact semantics I).
    pub fn w_vertex_count(&mut self, _index_count: u32) {
        // inlined; write the named field (I)
        self.index_count = _index_count;
    }
}

/// `VkDrawIndirectCommand` (16 bytes). Field order per Vulkan ABI (I).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DrawIndirectCommand {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}

impl DrawIndirectCommand {
    /// O name — zero command.
    pub fn empty() -> Self {
        Self::default()
    }
    /// O name — write helper (exact semantics I).
    pub fn w_instance_count(&mut self, _instance_count: u32) {
        self.instance_count = _instance_count;
    }
}

/// `VkDispatchIndirectCommand` (12 bytes). Field order per Vulkan ABI (I).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl DispatchIndirectCommand {
    /// O name — zero command.
    pub fn empty() -> Self {
        Self::default()
    }
}
