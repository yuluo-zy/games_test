//! Binding a finished upload into a render graph.
//!
//! [`UploadTicket`] belongs to the transfer layer and knows nothing about graphs; importing
//! its buffer needs the graph and its Vulkan backend, so it lives with them.
use crate::transfer::UploadTicket;

impl UploadTicket {
    /// Bind through the normal graph import validation. The graph plans its
    /// accesses from the buffer's tracked range state, acquiring a range
    /// still released by a cross-family upload itself.
    pub fn import_buffer(
        &self,
        graph: &mut crate::rg::graph::RenderGraph,
        backend: &mut crate::rg::vulkan::VulkanGraphBackend,
        name: &str,
    ) -> crate::Result<crate::rg::resource::BufferId> {
        if backend.device.generation() != self.device.generation()
            || backend.selected_family() != self.device.queues.primary_family
        {
            return Err(crate::Error::invalid_usage(
                "upload ticket belongs to another device or queue-family domain",
            ));
        }
        let token =
            backend.bind_import(crate::rg::vulkan::GpuResource::Buffer(self.buffer.clone()))?;
        let handle = graph.import_unowned(
            name,
            crate::rg::vulkan::ResourceSpec::Buffer(self.buffer.desc).descriptor(),
            token,
            0x37,
        )?;
        graph.buffer_id(handle).map_err(Into::into)
    }
}
