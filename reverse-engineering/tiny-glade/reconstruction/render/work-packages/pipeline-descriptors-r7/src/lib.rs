use ash::vk;
use serde::{Deserialize, Serialize};

/// Caller-resolved private reflection contract. Tags are original CPU enum tags.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Binding {
    pub tag: u8,
    pub name: String,
    pub runtime: bool,
    pub count: u32,
    /// Supplied result of GraphicsDevice::get_sampler; its cache is outside this package.
    pub sampler: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutBinding {
    pub binding: u32,
    pub descriptor_type: i32,
    pub descriptor_count: u32,
    pub stage_flags: u32,
    pub immutable_sampler: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutPlan {
    pub flags: u32,
    pub binding_flags: Vec<u32>,
    pub bindings: Vec<LayoutBinding>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
    ZeroDescriptorCount,
    UnknownReflectionTag(u8),
    Vulkan(i32),
}

/// Original runtime count rule: leave 32 slots, cap variable descriptors at 8192.
pub fn runtime_descriptor_count(device_limit: u32) -> u32 {
    device_limit.saturating_sub(32).min(8192)
}

/// Recovered private create_descriptor_set_layout<Enumerate<SliceIter>> CPU path.
/// Source entry 0x1418c2150. Public Vulkan/ash definitions are reused.
pub fn descriptor_set_layout(
    bindings: &[Binding],
    first_binding: u64,
    stage_flags: vk::ShaderStageFlags,
    device_limit: u32,
) -> Result<LayoutPlan, Error> {
    let mut result = LayoutPlan {
        flags: 0,
        binding_flags: vec![0; bindings.len()],
        bindings: Vec::with_capacity(bindings.len()),
    };
    for (index, binding) in bindings.iter().enumerate() {
        let count = if binding.runtime {
            runtime_descriptor_count(device_limit)
        } else {
            binding.count
        };
        if count == 0 {
            return Err(Error::ZeroDescriptorCount);
        }
        if count != 1 {
            result.binding_flags[index] = vk::DescriptorBindingFlags::PARTIALLY_BOUND.as_raw();
        }
        let descriptor_type = match binding.tag {
            0 => {
                if binding.name.ends_with("_dyn") {
                    vk::DescriptorType::STORAGE_BUFFER_DYNAMIC
                } else {
                    vk::DescriptorType::STORAGE_BUFFER
                }
            }
            1 => {
                if binding.name.ends_with("_dyn") {
                    vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC
                } else {
                    vk::DescriptorType::UNIFORM_BUFFER
                }
            }
            2 => vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            3 => vk::DescriptorType::STORAGE_IMAGE,
            4 => {
                if binding.runtime {
                    result.binding_flags[index] = 15;
                    result.flags |=
                        vk::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL.as_raw();
                }
                vk::DescriptorType::SAMPLED_IMAGE
            }
            5 => vk::DescriptorType::SAMPLER,
            other => return Err(Error::UnknownReflectionTag(other)),
        };
        result.bindings.push(LayoutBinding {
            binding: first_binding.wrapping_add(index as u64) as u32,
            descriptor_type: descriptor_type.as_raw(),
            descriptor_count: if binding.tag == 5 { 1 } else { count },
            stage_flags: stage_flags.as_raw(),
            immutable_sampler: (binding.tag == 5).then_some(binding.sampler),
        });
    }
    Ok(result)
}

impl LayoutPlan {
    /// Pointers remain owned and stable through the callback, including immutable samplers.
    pub fn with_create_info<R>(
        &self,
        call: impl FnOnce(&vk::DescriptorSetLayoutCreateInfo<'_>) -> R,
    ) -> R {
        use ash::vk::Handle;
        let samplers: Vec<vk::Sampler> = self
            .bindings
            .iter()
            .map(|b| vk::Sampler::from_raw(b.immutable_sampler.unwrap_or(0)))
            .collect();
        let bindings: Vec<_> = self
            .bindings
            .iter()
            .enumerate()
            .map(|(i, b)| {
                let mut result = vk::DescriptorSetLayoutBinding::default()
                    .binding(b.binding)
                    .descriptor_type(vk::DescriptorType::from_raw(b.descriptor_type))
                    .descriptor_count(b.descriptor_count)
                    .stage_flags(vk::ShaderStageFlags::from_raw(b.stage_flags));
                if b.immutable_sampler.is_some() {
                    result.p_immutable_samplers = &samplers[i];
                }
                result
            })
            .collect();
        let flags: Vec<_> = self
            .binding_flags
            .iter()
            .map(|&x| vk::DescriptorBindingFlags::from_raw(x))
            .collect();
        let mut extension =
            vk::DescriptorSetLayoutBindingFlagsCreateInfo::default().binding_flags(&flags);
        let info = vk::DescriptorSetLayoutCreateInfo::default()
            .flags(vk::DescriptorSetLayoutCreateFlags::from_raw(self.flags))
            .bindings(&bindings)
            .push_next(&mut extension);
        call(&info)
    }
}

/// Pipeline-layout create contract observed at compute entry 0x1416a4b20.
pub fn with_compute_pipeline_layout<R>(
    layouts: &[vk::DescriptorSetLayout],
    push_constant_bytes: u32,
    call: impl FnOnce(&vk::PipelineLayoutCreateInfo<'_>) -> R,
) -> R {
    let range = vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::COMPUTE)
        .offset(0)
        .size(push_constant_bytes);
    let ranges = if push_constant_bytes == 0 {
        &[][..]
    } else {
        std::slice::from_ref(&range)
    };
    call(
        &vk::PipelineLayoutCreateInfo::default()
            .set_layouts(layouts)
            .push_constant_ranges(ranges),
    )
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistentPoolPlan {
    pub flags: u32,
    pub max_sets: u32,
    pub sizes: Vec<(i32, u32)>,
    pub variable_descriptor_count: u32,
}

/// Remaining create_persistent_descriptor_set CPU create-info contract, entry 0x140bfef70.
pub fn persistent_pool_plan(layout: &LayoutPlan, device_limit: u32) -> PersistentPoolPlan {
    let mut sizes: Vec<(i32, u32)> = Vec::new();
    for binding in &layout.bindings {
        if let Some((_, count)) = sizes
            .iter_mut()
            .find(|(ty, _)| *ty == binding.descriptor_type)
        {
            *count = count.wrapping_add(binding.descriptor_count);
        } else {
            sizes.push((binding.descriptor_type, binding.descriptor_count));
        }
    }
    PersistentPoolPlan {
        flags: vk::DescriptorPoolCreateFlags::UPDATE_AFTER_BIND.as_raw(),
        max_sets: 1,
        sizes,
        variable_descriptor_count: runtime_descriptor_count(device_limit),
    }
}

impl PersistentPoolPlan {
    pub fn with_pool_create_info<R>(
        &self,
        call: impl FnOnce(&vk::DescriptorPoolCreateInfo<'_>) -> R,
    ) -> R {
        let sizes: Vec<_> = self
            .sizes
            .iter()
            .map(|&(ty, count)| vk::DescriptorPoolSize {
                ty: vk::DescriptorType::from_raw(ty),
                descriptor_count: count,
            })
            .collect();
        call(
            &vk::DescriptorPoolCreateInfo::default()
                .flags(vk::DescriptorPoolCreateFlags::from_raw(self.flags))
                .max_sets(self.max_sets)
                .pool_sizes(&sizes),
        )
    }

    pub fn with_allocate_info<R>(
        &self,
        pool: vk::DescriptorPool,
        layout: vk::DescriptorSetLayout,
        call: impl FnOnce(&vk::DescriptorSetAllocateInfo<'_>) -> R,
    ) -> R {
        let counts = [self.variable_descriptor_count];
        let layouts = [layout];
        let mut extension = vk::DescriptorSetVariableDescriptorCountAllocateInfo::default()
            .descriptor_counts(&counts);
        call(
            &vk::DescriptorSetAllocateInfo::default()
                .descriptor_pool(pool)
                .set_layouts(&layouts)
                .push_next(&mut extension),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ash::vk::Handle;
    #[test]
    fn pointers_preserve_array_flags_and_immutable_sampler() {
        let input = vec![
            Binding {
                tag: 5,
                name: "s".into(),
                runtime: false,
                count: 1,
                sampler: 73,
            },
            Binding {
                tag: 4,
                name: "t".into(),
                runtime: true,
                count: 0,
                sampler: 0,
            },
        ];
        let plan = descriptor_set_layout(&input, 7, vk::ShaderStageFlags::ALL, 4096).unwrap();
        plan.with_create_info(|info| unsafe {
            assert_eq!(info.binding_count, 2);
            assert_eq!(
                *(*info.p_bindings).p_immutable_samplers,
                vk::Sampler::from_raw(73)
            );
            let ext = &*(info.p_next as *const vk::DescriptorSetLayoutBindingFlagsCreateInfo);
            assert_eq!(
                *ext.p_binding_flags.add(1),
                vk::DescriptorBindingFlags::from_raw(15)
            );
        });
    }
    #[test]
    fn compute_push_range_is_optional() {
        for size in [0, 4, 24] {
            with_compute_pipeline_layout(&[], size, |info| unsafe {
                assert_eq!(info.push_constant_range_count, u32::from(size != 0));
                if size != 0 {
                    assert_eq!((*info.p_push_constant_ranges).size, size);
                }
            });
        }
    }
}
