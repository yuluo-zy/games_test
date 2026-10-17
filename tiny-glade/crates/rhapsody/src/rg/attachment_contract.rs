//! Pure raster attachment semantics; no Vulkan object ownership or ABI layout.
use crate::Result;
use ash::vk;
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AttachmentKey {
    pub format: i32,
    pub samples: u32,
    pub load: i32,
    pub store: i32,
    pub stencil_load: i32,
    pub stencil_store: i32,
    pub initial_layout: i32,
    pub final_layout: i32,
    pub subpass_layout: i32,
    pub depth_stencil: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RenderPassKey {
    pub attachments: Vec<AttachmentKey>,
}

impl RenderPassKey {
    pub fn compatibility_key(&self) -> Vec<(i32, u32, bool)> {
        self.attachments
            .iter()
            .map(|a| (a.format, a.samples, a.depth_stencil))
            .collect()
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RenderPassAttachmentDesc {
    pub format: vk::Format,
    pub clear_input: u32,
    pub store_op: u32,
    pub sample_count: u8,
}
impl RenderPassAttachmentDesc {
    pub fn new(format: vk::Format) -> Self {
        Self {
            format,
            clear_input: 0,
            store_op: 0,
            sample_count: 1,
        }
    }
    pub fn clear_input(mut self) -> Self {
        self.clear_input = 1;
        self
    }
    pub fn sample_count(mut self, samples: u8) -> Self {
        self.sample_count = samples;
        self
    }
    pub fn is_depth_stencil(&self) -> bool {
        (124..=130).contains(&self.format.as_raw())
    }
    pub fn to_attachment_key(&self) -> AttachmentKey {
        let depth_stencil = self.is_depth_stencil();
        let layout = if depth_stencil {
            DepthAccess::ReadWrite.layout()
        } else {
            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL
        };
        AttachmentKey {
            format: self.format.as_raw(),
            samples: u32::from(self.sample_count),
            load: self.clear_input as i32,
            store: self.store_op as i32,
            stencil_load: vk::AttachmentLoadOp::LOAD.as_raw(),
            stencil_store: vk::AttachmentStoreOp::STORE.as_raw(),
            initial_layout: layout.as_raw(),
            final_layout: layout.as_raw(),
            subpass_layout: layout.as_raw(),
            depth_stencil,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DepthAccess {
    ReadOnly = 0,
    ReadWrite = 1,
    DepthWriteStencilRead = 2,
}
impl DepthAccess {
    pub fn layout(self) -> vk::ImageLayout {
        match self {
            Self::ReadOnly => vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
            Self::ReadWrite => vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            Self::DepthWriteStencilRead => {
                vk::ImageLayout::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL
            }
        }
    }
    pub fn from_tag(tag: u8) -> Option<Option<Self>> {
        match tag {
            0 => Some(Some(Self::ReadOnly)),
            1 => Some(Some(Self::ReadWrite)),
            2 => Some(Some(Self::DepthWriteStencilRead)),
            3 => Some(None),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct AttachmentArray {
    items: [RenderPassAttachmentDesc; 8],
    len: u32,
}
impl AttachmentArray {
    pub const CAPACITY: usize = 8;
    pub fn new() -> Self {
        Self {
            items: [RenderPassAttachmentDesc::new(vk::Format::UNDEFINED); 8],
            len: 0,
        }
    }
    pub fn push(&mut self, item: RenderPassAttachmentDesc) -> Result<()> {
        if self.len as usize == Self::CAPACITY {
            return Err(crate::Error::invalid_usage("渲染通道最多支持8个颜色附件"));
        }
        self.items[self.len as usize] = item;
        self.len += 1;
        Ok(())
    }
    pub fn try_from_iter(iter: impl IntoIterator<Item = RenderPassAttachmentDesc>) -> Result<Self> {
        let mut array = Self::new();
        for item in iter {
            array.push(item)?;
        }
        Ok(array)
    }
    pub fn len(&self) -> u32 {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn as_slice(&self) -> &[RenderPassAttachmentDesc] {
        &self.items[..self.len as usize]
    }
}
impl Default for AttachmentArray {
    fn default() -> Self {
        Self::new()
    }
}
impl PartialEq for AttachmentArray {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}
impl Eq for AttachmentArray {}
impl std::hash::Hash for AttachmentArray {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}
impl std::fmt::Debug for AttachmentArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.as_slice()).finish()
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct RenderPassDesc {
    pub colors: AttachmentArray,
    pub depth: Option<(RenderPassAttachmentDesc, DepthAccess)>,
}
impl RenderPassDesc {
    pub fn from_flat(attachments: &[RenderPassAttachmentDesc]) -> Result<Self> {
        let mut desc = Self::default();
        for attachment in attachments {
            if attachment.is_depth_stencil() {
                if desc.depth.is_some() {
                    return Err(crate::Error::invalid_usage("渲染通道只能有一个深度附件"));
                }
                desc.depth = Some((*attachment, DepthAccess::ReadWrite));
            } else {
                desc.colors.push(*attachment)?;
            }
        }
        Ok(desc)
    }
    pub fn pass_samples(&self) -> Result<u32> {
        if let Some((depth, _)) = &self.depth {
            return Ok(u32::from(depth.sample_count));
        }
        self.colors
            .as_slice()
            .first()
            .map(|a| u32::from(a.sample_count))
            .ok_or_else(|| crate::Error::invalid_usage("渲染通道必须至少有一个附件"))
    }
}
