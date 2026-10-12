//! 逻辑资源接口：依据 PDB 函数名及汇编重建，公开 Rust 签名仍是候选。
use super::graph::{Descriptor, OriginSource};

/// 图内逻辑索引；原作 raw 入口直接把 ECX 返回到 EAX。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ResourceHandle(u32);
impl ResourceHandle {
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }
    pub const fn raw(self) -> u32 {
        self.0
    }
}

/// 携带描述符的资源句柄；`desc()` 返回声明期描述符，`clone_unchecked()` 不做
/// 引用计数/有效性检查地复制（原 197 处内联实例的廉价克隆语义）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Handle {
    pub resource: ResourceHandle,
    pub descriptor: Descriptor,
}
impl Handle {
    pub fn new(resource: ResourceHandle, descriptor: Descriptor) -> Self {
        Self {
            resource,
            descriptor,
        }
    }
    pub fn desc(&self) -> &Descriptor {
        &self.descriptor
    }
    pub fn raw(&self) -> ResourceHandle {
        self.resource
    }
    pub fn clone_unchecked(&self) -> Self {
        Self {
            resource: self.resource,
            descriptor: self.descriptor,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceType {
    Image,
    Buffer,
    Other,
}

/// 候选公开表示；原作 ResourceOrigin 的 72 字节布局另有证据记录。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceOrigin {
    pub descriptor: Descriptor,
    pub source: OriginSource,
}
impl ResourceOrigin {
    pub fn ty(self) -> ResourceType {
        match self.descriptor {
            Descriptor::Image(_) => ResourceType::Image,
            Descriptor::Buffer(_) => ResourceType::Buffer,
        }
    }
}

/// 已观察到的两个 validate_read_access 实例均只有 RET；保留空操作。
pub fn validate_read_access(_access: u8) {}

use super::resource_registry::{Buffer, RegistryResource, Texture};
use std::sync::Arc;

pub type RuntimeResource<T, B, O> = RegistryResource<T, B, Arc<O>>;

pub enum BorrowedResource<'a, T, B, O> {
    Image(&'a Texture<T>),
    Buffer(&'a Buffer<B>),
    Other(&'a O),
}

impl<T, B, O> RegistryResource<T, B, Arc<O>> {
    /// 原入口标签 0/1、2/3 直接借用载荷；标签 4 跳过 Arc 的两个引用计数。
    pub fn borrow(&self) -> BorrowedResource<'_, T, B, O> {
        match self {
            Self::OwnedTexture(value) | Self::UnownedTexture(value) => {
                BorrowedResource::Image(value)
            }
            Self::OwnedBuffer(value) | Self::UnownedBuffer(value) => {
                BorrowedResource::Buffer(value)
            }
            Self::Other(value) => BorrowedResource::Other(value.as_ref()),
            Self::Empty => panic!("不能借用空资源"),
        }
    }
}

/// 原作 ResourceOrigin 的取证载体。只暴露已观察字段，不猜测其余字段名。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct ResourceOriginLayout {
    pub tag: u64,
    pub payload: [u8; 64],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OriginTypeError {
    DescriptorKind8,
    InvalidOrigin(u64),
}

impl ResourceOriginLayout {
    /// 标签 0 的描述符从对象 +24 开始，描述符判别字节位于 +54。
    /// 标签 1/2 的类型码直接取对象 +8；其他类型码保留原值。
    pub fn ty(&self) -> Result<u8, OriginTypeError> {
        match self.tag {
            0 => match self.payload[46] {
                7 => Ok(1),
                8 => Err(OriginTypeError::DescriptorKind8),
                _ => Ok(0),
            },
            1 | 2 => Ok(self.payload[0]),
            tag => Err(OriginTypeError::InvalidOrigin(tag)),
        }
    }
}

/// 对应原转换入口 +0xbd6ce0：逐字节复制 32 字节，包括填充字节。
pub fn descriptor_from_texture(input: &[u8; 32]) -> [u8; 32] {
    *input
}

/// 对应原转换入口 +0xbd6d00：只写前 16 字节及第 30 字节。
/// 未写入区保持原值，不能错误地补零或复制全部 32 字节。
pub fn descriptor_from_buffer(input: &[u8; 16], output: &mut [u8; 32]) {
    output[..16].copy_from_slice(input);
    output[30] = 7;
}

/// 只允许在所属 builder 中生产绑定；原 Ref 的完整泛型 ABI 尚未恢复。
pub struct Ref<'borrow, 'graph, Kind> {
    pub(crate) builder: &'borrow mut super::pass_builder::PassBuilder<'graph>,
    pub(crate) resource: ResourceHandle,
    pub(crate) access_code: u8,
    pub(crate) group: u32,
    pub(crate) kind: std::marker::PhantomData<Kind>,
}
pub enum BufferRef {}
pub enum TextureRef {}
impl<K> Ref<'_, '_, K> {
    pub fn raw(&self) -> ResourceHandle {
        self.resource
    }
}
impl Ref<'_, '_, BufferRef> {
    pub fn bind_uniform_buffer(
        self,
        offset: u64,
        size: u64,
    ) -> Result<super::pass_bindings::PassBinding, super::pass_bindings::PassBindingError> {
        self.builder
            .add_resource_access(self.resource, self.access_code, self.group)?;
        Ok(super::pass_bindings::PassBinding::buffer(
            self.resource,
            offset,
            size,
            true,
        ))
    }
    pub fn bind_storage_buffer(
        self,
        offset: u64,
        size: u64,
    ) -> Result<super::pass_bindings::PassBinding, super::pass_bindings::PassBindingError> {
        self.builder
            .add_resource_access(self.resource, self.access_code, self.group)?;
        Ok(super::pass_bindings::PassBinding::buffer(
            self.resource,
            offset,
            size,
            false,
        ))
    }
}
impl Ref<'_, '_, TextureRef> {
    /// view 与布局在 callback 解析真实纹理时验证，绑定自动登记本 pass 的资源访问。
    pub fn bind_view(
        self,
        view: crate::raw::texture::TextureViewDesc,
        layout: ash::vk::ImageLayout,
        storage: bool,
    ) -> Result<super::pass_bindings::PassBinding, super::pass_bindings::PassBindingError> {
        self.builder
            .add_resource_access(self.resource, self.access_code, self.group)?;
        Ok(super::pass_bindings::PassBinding::image(
            self.resource,
            view,
            layout,
            storage,
        ))
    }
    pub fn into_sampled_rg_texture(
        self,
        view: crate::raw::texture::TextureViewDesc,
        layout: ash::vk::ImageLayout,
    ) -> Result<super::pass_bindings::PassBinding, super::pass_bindings::PassBindingError> {
        self.bind_view(view, layout, false)
    }
}

/// 原 view carrier 的限定字节写入合同；不把未知 60 字节强行映射到候选 view 描述符。
pub fn original_ref_view_binding(
    view: &[u8; 60],
    logical: ResourceHandle,
    role: u8,
    output: &mut [u8; 68],
) {
    output[..60].copy_from_slice(view);
    output[60..64].copy_from_slice(&logical.raw().to_le_bytes());
    output[64] = role;
}
/// 原 buffer carrier：uniform=3，另一 bind 实例=4；未知尾部不参与恢复。
pub fn original_ref_buffer_binding(logical: ResourceHandle, uniform: bool) -> [u32; 2] {
    [if uniform { 3 } else { 4 }, logical.raw()]
}
