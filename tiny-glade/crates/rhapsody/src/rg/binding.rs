//! 图资源到描述符的候选转换。资源解析和生命周期持有由真实宿主对象供给。
use crate::raw::binding::{DescriptorSetBinding, ResourceOwner};
use crate::rg::resource::ResourceHandle;
use ash::vk;
#[derive(Clone, Copy, Debug)]
pub enum PassResourceSlot {
    UniformBuffer {
        resource: ResourceHandle,
        offset: u64,
        size: u64,
    },
    StorageBuffer {
        resource: ResourceHandle,
        offset: u64,
        size: u64,
    },
    SampledImage {
        resource: ResourceHandle,
        layout: vk::ImageLayout,
    },
    StorageImage {
        resource: ResourceHandle,
        layout: vk::ImageLayout,
    },
}
impl PassResourceSlot {
    /// 构造 uniform 缓冲槽；uniform=1 语义由 `rg_binding_to_descriptor_binding` 判定。
    pub fn uniform_buffer(resource: ResourceHandle, offset: u64, size: u64) -> Self {
        Self::UniformBuffer {
            resource,
            offset,
            size,
        }
    }
    pub fn storage_buffer(resource: ResourceHandle, offset: u64, size: u64) -> Self {
        Self::StorageBuffer {
            resource,
            offset,
            size,
        }
    }
    pub fn sampled_image(resource: ResourceHandle, layout: vk::ImageLayout) -> Self {
        Self::SampledImage { resource, layout }
    }
    pub fn storage_image(resource: ResourceHandle, layout: vk::ImageLayout) -> Self {
        Self::StorageImage { resource, layout }
    }
}
pub enum ResolvedBinding {
    Buffer {
        buffer: vk::Buffer,
        owner: ResourceOwner,
    },
    Image {
        view: vk::ImageView,
        owner: ResourceOwner,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BindingError {
    MissingResource,
    WrongKind,
}
pub fn rg_binding_to_descriptor_binding(
    slot: PassResourceSlot,
    mut resolve: impl FnMut(ResourceHandle) -> Option<ResolvedBinding>,
) -> Result<DescriptorSetBinding, BindingError> {
    match slot {
        PassResourceSlot::UniformBuffer {
            resource,
            offset,
            size,
        }
        | PassResourceSlot::StorageBuffer {
            resource,
            offset,
            size,
        } => {
            let ty = if matches!(slot, PassResourceSlot::UniformBuffer { .. }) {
                vk::DescriptorType::UNIFORM_BUFFER
            } else {
                vk::DescriptorType::STORAGE_BUFFER
            };
            match resolve(resource).ok_or(BindingError::MissingResource)? {
                ResolvedBinding::Buffer { buffer, owner } => Ok(DescriptorSetBinding::Buffer {
                    ty,
                    info: vk::DescriptorBufferInfo {
                        buffer,
                        offset,
                        range: size,
                    },
                    owner,
                }),
                _ => Err(BindingError::WrongKind),
            }
        }
        PassResourceSlot::SampledImage { resource, layout }
        | PassResourceSlot::StorageImage { resource, layout } => {
            let ty = if matches!(slot, PassResourceSlot::SampledImage { .. }) {
                vk::DescriptorType::SAMPLED_IMAGE
            } else {
                vk::DescriptorType::STORAGE_IMAGE
            };
            match resolve(resource).ok_or(BindingError::MissingResource)? {
                ResolvedBinding::Image { view, owner } => Ok(DescriptorSetBinding::Image {
                    ty,
                    info: vk::DescriptorImageInfo {
                        sampler: vk::Sampler::null(),
                        image_view: view,
                        image_layout: layout,
                    },
                    owners: vec![owner],
                }),
                _ => Err(BindingError::WrongKind),
            }
        }
    }
}

/// 使用原反射变量的名称和 sparse binding 连接 pass 资源；缺少声明访问立即报错。
pub fn create_descriptor_writes(
    entry: &crate::raw::shader::ReflectedEntry,
    set_index: u32,
    pass: &super::graph::PassDeclaration,
    named_slots: &std::collections::BTreeMap<String, Vec<PassResourceSlot>>,
    mut resolve: impl FnMut(ResourceHandle) -> Option<ResolvedBinding>,
) -> Result<Vec<crate::raw::descriptor_set::DescriptorWrite>, BindingError> {
    let mut writes = Vec::new();
    for descriptor in entry
        .descriptors
        .iter()
        .filter(|d| d.set == set_index && d.descriptor_type != "Sampler")
    {
        let slots = named_slots
            .get(&descriptor.name)
            .ok_or(BindingError::MissingResource)?;
        if descriptor.count != 0 && slots.len() != descriptor.count as usize {
            return Err(BindingError::WrongKind);
        }
        for (element, slot) in slots.iter().copied().enumerate() {
            let resource = match slot {
                PassResourceSlot::UniformBuffer { resource, .. }
                | PassResourceSlot::StorageBuffer { resource, .. }
                | PassResourceSlot::SampledImage { resource, .. }
                | PassResourceSlot::StorageImage { resource, .. } => resource,
            };
            if !pass
                .accesses
                .iter()
                .any(|access| access.resource_index == resource.raw())
            {
                return Err(BindingError::MissingResource);
            }
            let mut value = rg_binding_to_descriptor_binding(slot, &mut resolve)?;
            if let DescriptorSetBinding::Buffer { info, .. } = &value {
                // spirq 的 runtime buffer minimum_bytes=0 不能当实际大小；固定最小 extent 则可检查。
                if descriptor.reflected_type["minimum_bytes"]
                    .as_u64()
                    .is_some_and(|minimum| minimum > info.range)
                {
                    return Err(BindingError::WrongKind);
                }
            }
            // 原 _dyn 名称影响布局，写入的 descriptor type 也必须同步改变。
            if descriptor.name.ends_with("_dyn")
                && let DescriptorSetBinding::Buffer { ty, .. } = &mut value
            {
                *ty = match *ty {
                    vk::DescriptorType::UNIFORM_BUFFER => {
                        vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC
                    }
                    vk::DescriptorType::STORAGE_BUFFER => {
                        vk::DescriptorType::STORAGE_BUFFER_DYNAMIC
                    }
                    _ => *ty,
                };
            }
            writes.push(crate::raw::descriptor_set::DescriptorWrite {
                binding: descriptor.binding,
                array_element: element as u32,
                value,
            });
        }
    }
    Ok(writes)
}

/// # Safety
/// command 正在合法录制；调用者须持有 pipeline/set 直到此提交完成。
pub unsafe fn bind_descriptor_set(
    command: vk::CommandBuffer,
    pipeline: &crate::raw::shader::VulkanPipeline,
    set_index: u32,
    set: &crate::raw::descriptor_set::VulkanDescriptorSet,
    dynamic_offsets: &[u32],
) -> Result<(), vk::Result> {
    let layout = pipeline
        .layout
        .sets
        .get(set_index as usize)
        .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?;
    if !std::sync::Arc::ptr_eq(&pipeline.device, &set.pool.device) || layout.plan != set.layout.plan
    {
        return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
    }
    let dynamic_count = layout
        .plan
        .bindings
        .iter()
        .filter(|slot| {
            matches!(
                vk::DescriptorType::from_raw(slot.descriptor_type),
                vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC
                    | vk::DescriptorType::STORAGE_BUFFER_DYNAMIC
            )
        })
        .map(|slot| slot.descriptor_count as usize)
        .sum::<usize>();
    if dynamic_offsets.len() != dynamic_count {
        return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
    }
    unsafe {
        crate::raw::binding::bind_persistent_descriptor_set(
            &pipeline.device,
            command,
            pipeline.point,
            pipeline.layout.handle,
            set_index,
            &[set.handle],
            dynamic_offsets,
        );
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// 原版 `RenderPassBinding`（= PassResourceSlot 的内存形态）0x44B 布局。
// 证据：`rg_binding_to_descriptor_binding`（0x14205eff0）对 `r9`（槽指针）的全部访问 + bind_view。
// ---------------------------------------------------------------------------------------------

/// 原槽判别式（`[slot+0]` u32，0x14205f016..0x14205f03c 的跳表 0x142fcf75c）：
/// 0/1/2 = 纹理视图槽（该字段同时是 `TextureViewDesc` 的首字段）；3 = 整缓冲（输出描述符 tag 6，
/// 机器码 0x14205f18d）；4 = 整缓冲（tag 5，0x14205f0e9）；5 = `todo!()`（0x14205f4c0，
/// 消息 "not yet implemented"，位置 `crates/rhapsody/src/rg/binding.rs:134:57`）；≥6 同样落到跳表索引 0
/// 的处理器（0x14205f03f：`cmp eax,2; jne` 视图子范围路径）——由于原代码从不构造 ≥6，视为不可达。
pub mod pass_slot_disc {
    pub const VIEW_0: u32 = 0;
    pub const VIEW_1: u32 = 1;
    pub const VIEW_2: u32 = 2;
    /// 整缓冲 → 描述符 tag 6（原 `into_uniform_buffer_binding` 同 tag；推断为 uniform）。
    pub const BUFFER_TAG6: u32 = 3;
    /// 整缓冲 → 描述符 tag 5（推断为 storage）。
    pub const BUFFER_TAG5: u32 = 4;
    /// `todo!()`。
    pub const TODO: u32 = 5;
}

/// `todo!()` 的原文案（0x142fcf5c0，len 0x13）。
pub const TODO_SLOT_MESSAGE: &str = "not yet implemented";
/// 槽 `todo!()` 与 `unreachable!()` 的源位置。
pub const TODO_SLOT_LOCATION: (&str, u32, u32) = ("crates/rhapsody/src/rg/binding.rs", 134, 57);
/// 资源注册表项种类字节不在 {2,3} 时 `unreachable!()`（0x142b101c8 len 0x28，位置 0x142b13d38）。
pub const REGISTRY_UNREACHABLE_LOCATION: (&str, u32, u32) =
    ("crates/rhapsody/src/rg/resource_registry.rs", 100, 59);
/// 视图 mip 范围校验 panic 的源位置（格式串片段在 0x142fcf520，Location 0x142fcf560）。
pub const MIP_RANGE_PANIC_LOCATION: (&str, u32, u32) =
    ("crates/rhapsody/src/rg/binding.rs", 54, 25);

/// 原 `TextureViewDesc`（0x3cB）。已读到的字段（其余字节按原样保留，**语义未证实**）：
/// `+0x00` u32 种类（= 槽判别式 0/1/2）；`+0x20` u8 的 bit0 = "显式 mip 数" Option 标志，
/// `+0x24` u32 = 显式 mip 数；`+0x30` u32 = base mip；`+0x38` u8 标志（bit0=1 走子范围路径）；
/// `+0x39` u16 / `+0x3b` u8 原样拷贝。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextureViewDescRaw {
    pub bytes: [u8; 0x3c],
}

impl TextureViewDescRaw {
    pub const SIZE: usize = 0x3c;
    pub fn kind(&self) -> u32 {
        u32::from_le_bytes(self.bytes[0..4].try_into().unwrap())
    }
    pub fn has_explicit_mip_count(&self) -> bool {
        self.bytes[0x20] & 1 != 0
    }
    pub fn explicit_mip_count(&self) -> u32 {
        u32::from_le_bytes(self.bytes[0x24..0x28].try_into().unwrap())
    }
    pub fn base_mip(&self) -> u32 {
        u32::from_le_bytes(self.bytes[0x30..0x34].try_into().unwrap())
    }
    /// `+0x38` bit0：1 = 带子范围（0x14205f1a0 `test r10b,1`），0 = 整视图（直接走 0x14205f2de）。
    pub fn has_subrange(&self) -> bool {
        self.bytes[0x38] & 1 != 0
    }
}

/// 原 `RenderPassBinding`（0x44B）：`[0..0x3c)` = `TextureViewDesc`；`+0x3c` u32 = 资源号
/// （`ResourceRegistry` 间接表索引）；`+0x40` u8 = 写标志（0 → 输出 tag 0/2，1 → tag 1/3，
/// 0x14205f329/0x14205f2d3）。缓冲槽（判别式 3/4）只用 `+0` 判别式和 `+4` u32 资源号，其余字节不读。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderPassBindingRaw {
    pub view: TextureViewDescRaw,
    pub resource: u32,
    pub write: u8,
    pub _pad: [u8; 3],
}

impl RenderPassBindingRaw {
    pub const SIZE: usize = 0x44;
    pub const OFFSET_RESOURCE: usize = 0x3c;
    pub const OFFSET_WRITE: usize = 0x40;
    /// 槽判别式（`[slot+0]`）。
    pub fn discriminant(&self) -> u32 {
        self.view.kind()
    }
    /// 缓冲槽（判别式 3/4）的资源号位于 `+4`，而非 `+0x3c`。
    pub fn buffer_resource(&self) -> u32 {
        u32::from_le_bytes(self.view.bytes[4..8].try_into().unwrap())
    }
    pub fn is_write(&self) -> bool {
        self.write != 0
    }
}

/// 视图子范围解析的失败（对应原 panic）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MipRangeError {
    /// `sub bl, al` 借位（0x14205f275 `jb`）：图像 mip 数（u8）小于 base（低 8 位）。
    BaseBeyondImage {
        image_mips: u8,
        base: u32,
    },
    EmptyRange,
    /// 0x14205f28d `cmp edi,esi; jae`：请求的 mip 数 ≥ 图像 mip 数 − base。
    RequestedTooLarge {
        requested: u32,
        image_mips: u8,
        base: u32,
    },
}

impl std::fmt::Display for MipRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BaseBeyondImage { image_mips, base } => {
                write!(f, "base mip {base} is outside {image_mips} image levels")
            }
            Self::EmptyRange => write!(f, "a mip view must contain at least one level"),
            // 片段 0x142fcf520：4 段 + 3 个 Display 参数（请求 mip 数 / 图像 mip 数 / base）。
            Self::RequestedTooLarge {
                requested,
                image_mips,
                base,
            } => write!(
                f,
                "The requested mip count ({requested}) is larger than the image mip count ({image_mips}) minus base level ({base})"
            ),
        }
    }
}
impl std::error::Error for MipRangeError {}

/// 校验非空子范围并返回 `[base, end)`；在完整 u32 域内检查，避免截断。
pub fn resolve_view_mip_range(
    image_mips: u8,
    base: u32,
    explicit: Option<u32>,
) -> Result<(u32, u32), MipRangeError> {
    let avail = u32::from(image_mips)
        .checked_sub(base)
        .filter(|&remaining| remaining > 0)
        .ok_or(MipRangeError::BaseBeyondImage { image_mips, base })?;
    let count = match explicit {
        Some(n) => {
            if n == 0 {
                return Err(MipRangeError::EmptyRange);
            }
            if n > avail {
                return Err(MipRangeError::RequestedTooLarge {
                    requested: n,
                    image_mips,
                    base,
                });
            }
            n
        }
        None => avail,
    };
    Ok((base, count + base))
}

#[cfg(test)]
mod raw_layout_tests {
    use super::*;
    use std::mem::{offset_of, size_of};

    #[test]
    fn render_pass_binding_is_0x44_bytes() {
        assert_eq!(size_of::<TextureViewDescRaw>(), 0x3c);
        assert_eq!(size_of::<RenderPassBindingRaw>(), 0x44);
        assert_eq!(offset_of!(RenderPassBindingRaw, resource), 0x3c);
        assert_eq!(offset_of!(RenderPassBindingRaw, write), 0x40);
    }

    #[test]
    fn mip_range_accepts_exact_remainder_and_rejects_empty_or_truncated_ranges() {
        assert_eq!(resolve_view_mip_range(10, 2, None), Ok((2, 10)));
        assert_eq!(resolve_view_mip_range(10, 2, Some(3)), Ok((2, 5)));
        assert_eq!(resolve_view_mip_range(10, 2, Some(8)), Ok((2, 10)));
        assert!(matches!(
            resolve_view_mip_range(10, 2, Some(9)),
            Err(MipRangeError::RequestedTooLarge { .. })
        ));
        assert_eq!(
            resolve_view_mip_range(10, 2, Some(0)),
            Err(MipRangeError::EmptyRange)
        );
        for base in [10, 256, u32::MAX] {
            assert!(matches!(
                resolve_view_mip_range(10, base, None),
                Err(MipRangeError::BaseBeyondImage { .. })
            ));
        }
        assert!(resolve_view_mip_range(0, 0, None).is_err());
        assert!(matches!(
            resolve_view_mip_range(3, 5, None),
            Err(MipRangeError::BaseBeyondImage { .. })
        ));
    }

    #[test]
    fn slot_disc_constants_match_jump_table() {
        assert_eq!(pass_slot_disc::TODO, 5);
        assert_eq!(TODO_SLOT_MESSAGE.len(), 0x13);
    }
}
