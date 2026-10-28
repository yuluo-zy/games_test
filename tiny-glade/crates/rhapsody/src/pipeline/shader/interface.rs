//! Reflected shader interfaces and stored pipeline entries.
use ash::vk;
use crate::Result;
use crate::command::descriptor_set::Binding;
use crate::command::descriptor_set::LayoutPlan;
use crate::command::descriptor_set::VulkanDescriptorSetLayout;
use crate::command::descriptor_set::descriptor_set_layout;
use crate::device::Device;
use std::collections::BTreeMap;
use std::sync::Arc;
use super::layout::PipelineLayout;
use super::layout::ShaderModule;
use super::layout::invalid_shader;
use super::reflection::InterfaceError;
use super::reflection::ReflectedDescriptor;
use super::reflection::ReflectedEntry;
use super::reflection::ReflectedLocation;
use super::reflection::ReflectedPushConstant;
use super::reflection::reflected_type_json;

pub struct ShaderInterfacePlan {
    /// 保留原反射 set/binding 编号；缺失的 set 用空布局填充，绝不重新枚举 sparse binding。
    pub sets: Vec<LayoutPlan>,
    pub push_constants: Vec<vk::PushConstantRange>,
}

pub(crate) fn reflected_stage(stage: &str) -> Result<vk::ShaderStageFlags> {
    match stage {
        "GLCompute" => Ok(vk::ShaderStageFlags::COMPUTE),
        "Vertex" => Ok(vk::ShaderStageFlags::VERTEX),
        "Fragment" => Ok(vk::ShaderStageFlags::FRAGMENT),
        other => Err(InterfaceError::Stage(other.to_owned()).into()),
    }
}

impl ShaderInterfacePlan {
    pub fn from_reflection(
        entries: &[ReflectedEntry],
        immutable_samplers: &BTreeMap<String, u64>,
        runtime_device_scalar: u32,
    ) -> Result<Self> {
        if let (Some(vertex), Some(fragment)) = (
            entries.iter().find(|entry| entry.stage == "Vertex"),
            entries.iter().find(|entry| entry.stage == "Fragment"),
        ) {
            for input in &fragment.inputs {
                let output = vertex
                    .outputs
                    .iter()
                    .find(|output| {
                        output.location == input.location && output.component == input.component
                    })
                    .ok_or(InterfaceError::IncompatibleLocation(
                        input.location,
                        input.component,
                    ))?;
                if canonical_type(&input.reflected_type) != canonical_type(&output.reflected_type) {
                    return Err(InterfaceError::IncompatibleLocation(
                        input.location,
                        input.component,
                    )
                    .into());
                }
            }
        }
        let mut merged = BTreeMap::<(u32, u32), (Binding, vk::ShaderStageFlags)>::new();
        let mut pushes = BTreeMap::<(u32, u32), vk::ShaderStageFlags>::new();
        for entry in entries {
            let stage = reflected_stage(&entry.stage)?;
            for descriptor in &entry.descriptors {
                let tag = match descriptor.descriptor_type.as_str() {
                    "StorageBuffer(ReadWrite)" | "StorageBuffer(ReadOnly)" => 0,
                    "UniformBuffer" => 1,
                    "CombinedImageSampler" => 2,
                    "StorageImage(ReadWrite)" | "StorageImage(ReadOnly)" => 3,
                    "SampledImage" => 4,
                    "Sampler" => 5,
                    other => return Err(InterfaceError::Descriptor(other.to_owned()).into()),
                };
                let sampler = if tag == 5 {
                    *immutable_samplers
                        .get(&descriptor.name)
                        .filter(|v| **v != 0)
                        .ok_or_else(|| InterfaceError::MissingSampler(descriptor.name.clone()))?
                } else {
                    0
                };
                let value = Binding {
                    tag,
                    name: descriptor.name.clone(),
                    count: descriptor.count,
                    runtime: descriptor.count == 0,
                    sampler,
                };
                match merged.entry((descriptor.set, descriptor.binding)) {
                    std::collections::btree_map::Entry::Vacant(slot) => {
                        slot.insert((value, stage));
                    }
                    std::collections::btree_map::Entry::Occupied(mut slot) => {
                        let (prior, stages) = slot.get_mut();
                        // 名称仅在 _dyn 分类与 immutable sampler 策略中影响布局；跨 stage 别名允许合并。
                        if prior.tag != value.tag
                            || prior.count != value.count
                            || prior.sampler != value.sampler
                            || prior.name.ends_with("_dyn") != value.name.ends_with("_dyn")
                        {
                            return Err(InterfaceError::IncompatibleBinding(
                                descriptor.set,
                                descriptor.binding,
                            )
                            .into());
                        }
                        *stages |= stage;
                    }
                }
            }
            for push in &entry.push_constants {
                let size = push.reflected_type["minimum_bytes"]
                    .as_u64()
                    .and_then(|size| u32::try_from(size).ok())
                    .filter(|size| *size > 0 && *size % 4 == 0)
                    .ok_or(InterfaceError::InvalidPushConstant)?;
                // 当前原 catalog 的 push block 始于 0；大小来自原 fork 反射，而非猜测成员数量。
                *pushes.entry((0, size)).or_default() |= stage;
            }
        }
        let set_count = merged
            .keys()
            .map(|(set, _)| *set)
            .max()
            .map_or(0, |set| set as usize + 1);
        if set_count > 32 {
            return Err(InterfaceError::InvalidLayout.into());
        }
        let mut sets = (0..set_count)
            .map(|_| LayoutPlan {
                flags: 0,
                bindings: Vec::new(),
                binding_flags: Vec::new(),
            })
            .collect::<Vec<_>>();
        for ((set, binding), (value, stages)) in merged {
            let plan =
                descriptor_set_layout(&[value], u64::from(binding), stages, runtime_device_scalar)
                    .map_err(|_| InterfaceError::InvalidLayout)?;
            let target = &mut sets[set as usize];
            target.flags |= plan.flags;
            target.bindings.extend(plan.bindings);
            target.binding_flags.extend(plan.binding_flags);
        }
        for (set, plan) in sets.iter().enumerate() {
            for (index, flags) in plan.binding_flags.iter().enumerate() {
                if flags & vk::DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT.as_raw() != 0
                    && index + 1 != plan.bindings.len()
                {
                    return Err(InterfaceError::RuntimeBindingNotLast(
                        set as u32,
                        plan.bindings[index].binding,
                    )
                    .into());
                }
            }
        }
        // Vulkan 不允许同一 stage 在多个 push 范围中出现；合并该 stage 的最大范围。
        let mut by_stage = BTreeMap::<u32, u32>::new();
        for ((_, size), flags) in pushes {
            for stage in [
                vk::ShaderStageFlags::VERTEX,
                vk::ShaderStageFlags::FRAGMENT,
                vk::ShaderStageFlags::COMPUTE,
            ] {
                if flags.contains(stage) {
                    by_stage
                        .entry(stage.as_raw())
                        .and_modify(|s| *s = (*s).max(size))
                        .or_insert(size);
                }
            }
        }
        let mut grouped = BTreeMap::<u32, vk::ShaderStageFlags>::new();
        for (stage, size) in by_stage {
            *grouped.entry(size).or_default() |= vk::ShaderStageFlags::from_raw(stage);
        }
        Ok(Self {
            sets,
            push_constants: grouped
                .into_iter()
                .map(|(size, stage_flags)| vk::PushConstantRange {
                    stage_flags,
                    offset: 0,
                    size,
                })
                .collect(),
        })
    }
    pub fn create_descriptor_set_layouts(
        &self,
        device: &Arc<Device>,
        samplers: &[Arc<crate::resource::sampler::Sampler>],
    ) -> Result<Vec<Arc<VulkanDescriptorSetLayout>>> {
        if self.sets.len() > crate::command::descriptor_set::MAX_DESCRIPTOR_SETS {
            return Err(crate::Error::invalid_usage("描述符集合数超过模块上限"));
        }
        self.sets
            .iter()
            .map(|plan| {
                VulkanDescriptorSetLayout::new_with_samplers(
                    device.clone(),
                    plan,
                    samplers.to_vec(),
                )
            })
            .collect()
    }
    pub fn create_pipeline_layout(
        &self,
        device: Arc<Device>,
        samplers: &[Arc<crate::resource::sampler::Sampler>],
    ) -> Result<Arc<PipelineLayout>> {
        let layouts = self.create_descriptor_set_layouts(&device, samplers)?;
        PipelineLayout::new(device, layouts, self.push_constants.clone())
    }
}

/// 去掉反射调试名字后比较结构；仅处理已有 schema，不解析 SPIR-V 或重写公开反射库。
pub(crate) fn canonical_type(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(fields) => serde_json::Value::Object(
            fields
                .iter()
                .filter(|(key, _)| key.as_str() != "name")
                .map(|(key, child)| (key.clone(), canonical_type(child)))
                .collect(),
        ),
        serde_json::Value::Array(values) => {
            serde_json::Value::Array(values.iter().map(canonical_type).collect())
        }
        other => other.clone(),
    }
}

impl ShaderInterfacePlan {
    /// 直接消费发行容器的stored reflection；SPIR-Q重反射仅用于独立核对。
    pub fn from_compiled_pipeline(
        compiled: &shader_pipeline::compiled::CompiledShaderPipeline,
        samplers: &BTreeMap<String, u64>,
        runtime_device_scalar: u32,
    ) -> Result<Self> {
        use shader_pipeline::{
            compiled::ShaderStage,
            reflection::{BindingArray, BindingKind},
        };
        let compute =
            compiled.stages.len() == 1 && compiled.stages[0].stage == ShaderStage::Compute;
        let raster = compiled.stages.len() == 2
            && compiled
                .stages
                .iter()
                .any(|stage| stage.stage == ShaderStage::Vertex)
            && compiled
                .stages
                .iter()
                .any(|stage| stage.stage == ShaderStage::Fragment);
        if !compute && !raster {
            return Err(invalid_shader());
        }
        for index in 0..compiled.stages.len() {
            stored_pipeline_entry(compiled, index)?;
        }
        let visibility = if compute {
            vk::ShaderStageFlags::COMPUTE
        } else {
            vk::ShaderStageFlags::ALL_GRAPHICS
        };
        let mut sets = Vec::new();
        for stored_set in compiled.reflection.effective_descriptor_sets() {
            let mut set = LayoutPlan {
                flags: 0,
                bindings: Vec::new(),
                binding_flags: Vec::new(),
            };
            for (index, binding) in stored_set {
                let tag = match binding.kind {
                    BindingKind::StorageBuffer => 0,
                    BindingKind::UniformBuffer => 1,
                    BindingKind::CombinedImageSampler => 2,
                    BindingKind::StorageImage => 3,
                    BindingKind::SampledImage => 4,
                    BindingKind::Sampler => 5,
                };
                let (count, runtime) = match binding.array {
                    BindingArray::Fixed(count) => (count, false),
                    BindingArray::Runtime => (0, true),
                };
                let sampler = if tag == 5 {
                    samplers
                        .get(&binding.name)
                        .copied()
                        .filter(|handle| *handle != 0)
                        .ok_or_else(|| {
                            crate::Error::invalid_usage("着色器资源、接口或设备不匹配")
                        })?
                } else {
                    0
                };
                let plan = descriptor_set_layout(
                    &[Binding {
                        tag,
                        name: binding.name.clone(),
                        count,
                        runtime,
                        sampler,
                    }],
                    u64::from(index),
                    visibility,
                    runtime_device_scalar,
                )
                .map_err(|_| crate::Error::invalid_usage("着色器资源、接口或设备不匹配"))?;
                set.flags |= plan.flags;
                set.bindings.extend(plan.bindings);
                set.binding_flags.extend(plan.binding_flags);
            }
            for (index, flags) in set.binding_flags.iter().enumerate() {
                if flags & vk::DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT.as_raw() != 0
                    && index + 1 != set.bindings.len()
                {
                    return Err(crate::Error::invalid_usage("着色器资源、接口或设备不匹配"));
                }
            }
            sets.push(set);
        }
        let size = compiled.reflection.push_constant_size;
        if !size.is_multiple_of(4) {
            return Err(crate::Error::invalid_usage("着色器资源、接口或设备不匹配"));
        }
        let push_constants = if size == 0 {
            Vec::new()
        } else {
            vec![vk::PushConstantRange {
                stage_flags: visibility,
                offset: 0,
                size,
            }]
        };
        Ok(Self {
            sets,
            push_constants,
        })
    }
}

/// 已解析发行容器中的一个模块。容器 offset/word_count 来自原 spirq catalog。
#[derive(Clone)]
pub struct ReflectedShader {
    pub module: Arc<ShaderModule>,
    pub entry: ReflectedEntry,
}

impl ReflectedShader {
    pub fn from_compiled_container(
        device: Arc<Device>,
        container: &[u8],
        metadata: &serde_json::Value,
        entry_name: &str,
    ) -> Result<Self> {
        let offset = metadata["offset"]
            .as_u64()
            .and_then(|v| usize::try_from(v).ok())
            .ok_or_else(invalid_shader)?;
        let len = metadata["word_count"]
            .as_u64()
            .and_then(|v| usize::try_from(v).ok())
            .and_then(|v| v.checked_mul(4))
            .ok_or_else(invalid_shader)?;
        let payload = container
            .get(offset..offset.checked_add(len).ok_or_else(invalid_shader)?)
            .ok_or_else(invalid_shader)?;
        let entry: ReflectedEntry = serde_json::from_value(
            metadata["entries"]
                .as_array()
                .and_then(|entries| {
                    entries
                        .iter()
                        .find(|entry| entry["entry_point"] == entry_name)
                })
                .cloned()
                .ok_or_else(invalid_shader)?,
        )
        .map_err(|_| invalid_shader())?;
        let parsed = rspirv::dr::load_bytes(payload).map_err(|_| invalid_shader())?;
        let expected = match entry.stage.as_str() {
            "GLCompute" => rspirv::spirv::ExecutionModel::GLCompute,
            "Vertex" => rspirv::spirv::ExecutionModel::Vertex,
            "Fragment" => rspirv::spirv::ExecutionModel::Fragment,
            _ => return Err(invalid_shader()),
        };
        let present = parsed.entry_points.iter().any(|point| matches!(
            (point.operands.first(), point.operands.get(2)),
            (Some(rspirv::dr::Operand::ExecutionModel(stage)), Some(rspirv::dr::Operand::LiteralString(name)))
            if *stage == expected && name == entry_name
        ));
        if !present {
            return Err(invalid_shader());
        }
        Ok(Self {
            module: ShaderModule::from_spirv(device, payload)?,
            entry,
        })
    }
}

impl ReflectedShader {
    /// 运行时完整容器入口：消费真实stored reflection，不构造空尾段替身。
    pub fn from_compiled_pipeline_stage(
        device: Arc<Device>,
        compiled: &shader_pipeline::compiled::CompiledShaderPipeline,
        index: usize,
    ) -> Result<Self> {
        let entry = stored_pipeline_entry(compiled, index)?;
        let bytes = compiled.stages[index].spirv_bytes();
        Ok(Self {
            module: ShaderModule::from_spirv(device, &bytes)?,
            entry,
        })
    }
}

pub(super) fn serialized_stage_name(stage: shader_pipeline::compiled::ShaderStage) -> &'static str {
    use shader_pipeline::compiled::ShaderStage;
    match stage {
        ShaderStage::Compute => "GLCompute",
        ShaderStage::Vertex => "Vertex",
        ShaderStage::Fragment => "Fragment",
    }
}

/// 原存储反射直接生产资源名字/编号/数组和push范围；不重新解析公开库类型。
/// SPIR-V只用于入口名/执行模型的一致性检查，完整stage IO独立核对时才重新反射。
pub fn stored_pipeline_entry(
    compiled: &shader_pipeline::compiled::CompiledShaderPipeline,
    index: usize,
) -> Result<ReflectedEntry> {
    use shader_pipeline::{
        compiled::ShaderStage,
        reflection::{BindingArray, BindingKind},
    };
    let stage = compiled.stages.get(index).ok_or_else(invalid_shader)?;
    let expected = match stage.stage {
        ShaderStage::Compute => rspirv::spirv::ExecutionModel::GLCompute,
        ShaderStage::Vertex => rspirv::spirv::ExecutionModel::Vertex,
        ShaderStage::Fragment => rspirv::spirv::ExecutionModel::Fragment,
    };
    let module = rspirv::dr::load_words(&stage.spirv).map_err(|_| invalid_shader())?;
    let present = module.entry_points.iter().any(|point| matches!(
        (point.operands.first(), point.operands.get(2)),
        (Some(rspirv::dr::Operand::ExecutionModel(model)), Some(rspirv::dr::Operand::LiteralString(name)))
        if *model == expected && name == &stage.entry_point
    ));
    if !present {
        return Err(invalid_shader());
    }
    let mut descriptors = Vec::new();
    for (set, bindings) in compiled
        .reflection
        .effective_descriptor_sets()
        .into_iter()
        .enumerate()
    {
        for (binding, value) in bindings {
            let descriptor_type = match value.kind {
                BindingKind::StorageBuffer => "StorageBuffer(ReadWrite)",
                BindingKind::UniformBuffer => "UniformBuffer",
                BindingKind::CombinedImageSampler => "CombinedImageSampler",
                BindingKind::StorageImage => "StorageImage(ReadWrite)",
                BindingKind::SampledImage => "SampledImage",
                BindingKind::Sampler => "Sampler",
            };
            let count = match value.array {
                BindingArray::Fixed(count) => count,
                BindingArray::Runtime => 0,
            };
            let reflected_type = compiled
                .reflection
                .uniform_block
                .as_ref()
                .filter(|block| block.binding.set == set as u32 && block.binding.binding == binding)
                .map_or(
                    serde_json::Value::Null,
                    |block| serde_json::json!({"minimum_bytes":block.byte_size}),
                );
            descriptors.push(ReflectedDescriptor {
                set: set as u32,
                binding,
                name: value.name.clone(),
                count,
                descriptor_type: descriptor_type.into(),
                reflected_type,
            });
        }
    }
    let push_constants = if compiled.reflection.push_constant_size == 0 {
        Vec::new()
    } else {
        vec![ReflectedPushConstant {
            reflected_type: serde_json::json!({"minimum_bytes":compiled.reflection.push_constant_size}),
        }]
    };
    let inputs = if stage.stage == ShaderStage::Vertex {
        compiled
            .reflection
            .effective_vertex_inputs()
            .into_values()
            .map(|input| {
                let ty = input.ty.to_spirq().ok_or_else(invalid_shader)?;
                Ok(ReflectedLocation {
                    location: input.location,
                    component: 0,
                    reflected_type: reflected_type_json(&ty),
                })
            })
            .collect::<Result<Vec<_>>>()?
    } else {
        Vec::new()
    };
    Ok(ReflectedEntry {
        entry_point: stage.entry_point.clone(),
        stage: serialized_stage_name(stage.stage).into(),
        local_size: if stage.stage == ShaderStage::Compute {
            compiled.reflection.local_size
        } else {
            None
        },
        descriptors,
        push_constants,
        inputs,
        outputs: Vec::new(),
    })
}
