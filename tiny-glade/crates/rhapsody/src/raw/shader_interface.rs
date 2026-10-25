//! Actual entry-point contracts shared by every pipeline creation path.
use super::shader::{PipelineLayout, ReflectedEntry, canonical_type, reflected_stage};
use crate::{Error, Result};
use ash::vk;
use std::{ffi::CStr, sync::Arc};

#[derive(Debug, PartialEq, Eq)]
struct Descriptor {
    set: u32,
    binding: u32,
    kind: vk::DescriptorType,
    count: u32,
}
#[derive(Debug)]
pub(crate) struct EntryContract {
    name: String,
    stage: vk::ShaderStageFlags,
    local_size: Option<[u32; 3]>,
    descriptors: Vec<Descriptor>,
    pushes: Vec<(u64, u64)>,
    /// Full canonical schemas and IO types, rather than a collision-prone hash.
    schema: Vec<u8>,
}
impl EntryContract {
    pub(crate) fn same_interface(&self, other: &Self) -> bool {
        self.stage == other.stage
            && self.local_size == other.local_size
            && (self.stage != vk::ShaderStageFlags::COMPUTE || self.local_size.is_some())
            && self.descriptors == other.descriptors
            && self.pushes == other.pushes
            && self.schema == other.schema
    }
    pub(crate) fn bytes(&self) -> usize {
        self.name.capacity()
            + self.descriptors.capacity() * std::mem::size_of::<Descriptor>()
            + self.pushes.capacity() * std::mem::size_of::<(u64, u64)>()
            + self.schema.capacity()
    }
}
fn descriptor_kind(name: &str) -> Result<vk::DescriptorType> {
    use vk::DescriptorType as T;
    Ok(match name.split('(').next().unwrap_or_default() {
        "Sampler" => T::SAMPLER,
        "CombinedImageSampler" => T::COMBINED_IMAGE_SAMPLER,
        "SampledImage" => T::SAMPLED_IMAGE,
        "StorageImage" => T::STORAGE_IMAGE,
        "UniformTexelBuffer" => T::UNIFORM_TEXEL_BUFFER,
        "StorageTexelBuffer" => T::STORAGE_TEXEL_BUFFER,
        "UniformBuffer" => T::UNIFORM_BUFFER,
        "StorageBuffer" => T::STORAGE_BUFFER,
        "InputAttachment" => T::INPUT_ATTACHMENT,
        "AccelStruct" => T::ACCELERATION_STRUCTURE_KHR,
        _ => {
            return Err(Error::unsupported(format!(
                "unsupported shader descriptor {name}"
            )));
        }
    })
}
fn push_span(ty: &serde_json::Value) -> Result<(u64, u64)> {
    let (start, end) = if ty["detail"]["kind"] == "Struct" {
        let members = ty["detail"]["members"]
            .as_array()
            .ok_or_else(|| Error::invalid_usage("invalid reflected push block"))?;
        let mut start = u64::MAX;
        let mut end = 0;
        for member in members {
            let offset = member["offset"]
                .as_u64()
                .ok_or_else(|| Error::invalid_usage("invalid reflected push offset"))?;
            let size = member["type"]["minimum_bytes"]
                .as_u64()
                .filter(|size| *size > 0)
                .ok_or_else(|| Error::invalid_usage("invalid reflected push size"))?;
            start = start.min(offset);
            end = end.max(
                offset
                    .checked_add(size)
                    .ok_or_else(|| Error::invalid_usage("reflected push overflow"))?,
            );
        }
        (start, end)
    } else {
        (0, ty["minimum_bytes"].as_u64().unwrap_or(0))
    };
    if end <= start {
        return Err(Error::invalid_usage("empty reflected push range"));
    }
    Ok((start, end))
}
fn compact(entry: ReflectedEntry) -> Result<EntryContract> {
    let descriptors = entry
        .descriptors
        .iter()
        .map(|descriptor| {
            Ok(Descriptor {
                set: descriptor.set,
                binding: descriptor.binding,
                kind: descriptor_kind(&descriptor.descriptor_type)?,
                count: descriptor.count,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let pushes = entry
        .push_constants
        .iter()
        .map(|push| push_span(&push.reflected_type))
        .collect::<Result<Vec<_>>>()?;
    let schemas = serde_json::json!({
        "descriptors":entry.descriptors.iter().map(|value| (value.set,value.binding,canonical_type(&value.reflected_type))).collect::<Vec<_>>(),
        "pushes":entry.push_constants.iter().map(|value| canonical_type(&value.reflected_type)).collect::<Vec<_>>(),
        "inputs":entry.inputs.iter().map(|value| (value.location,value.component,canonical_type(&value.reflected_type))).collect::<Vec<_>>(),
        "outputs":entry.outputs.iter().map(|value| (value.location,value.component,canonical_type(&value.reflected_type))).collect::<Vec<_>>(),
    });
    Ok(EntryContract {
        stage: reflected_stage(&entry.stage)?,
        local_size: entry.local_size,
        name: entry.entry_point,
        descriptors,
        pushes,
        schema: serde_json::to_vec(&schemas)
            .map_err(|error| Error::invalid_usage_with("shader interface schema", error))?,
    })
}
pub(crate) fn contracts(words: &[u32]) -> Result<Arc<[EntryContract]>> {
    super::shader::reflect_spirv_entries_used(words)?
        .into_iter()
        .map(compact)
        .collect::<Result<Vec<_>>>()
        .map(Arc::from)
}
pub(crate) fn validate_spirv_layout(
    words: &[u32],
    name: &CStr,
    stage: vk::ShaderStageFlags,
    layout: &PipelineLayout,
) -> Result<()> {
    validate(&contracts(words)?, name, stage, layout).map(|_| ())
}
pub(crate) fn validate(
    contracts: &[EntryContract],
    name: &CStr,
    stage: vk::ShaderStageFlags,
    layout: &PipelineLayout,
) -> Result<usize> {
    use vk::DescriptorType as T;
    let (index, entry) = contracts
        .iter()
        .enumerate()
        .find(|(_, entry)| entry.name.as_bytes() == name.to_bytes() && entry.stage == stage)
        .ok_or_else(|| Error::invalid_usage("shader entry is absent or has the wrong stage"))?;
    for resource in &entry.descriptors {
        let binding = layout
            .sets
            .get(resource.set as usize)
            .and_then(|set| {
                set.plan
                    .bindings
                    .iter()
                    .find(|binding| binding.binding == resource.binding)
            })
            .ok_or_else(|| {
                Error::invalid_usage(format!(
                    "shader requires missing descriptor set {} binding {}",
                    resource.set, resource.binding
                ))
            })?;
        let actual = T::from_raw(binding.descriptor_type);
        let compatible = actual == resource.kind
            || matches!(
                (resource.kind, actual),
                (T::UNIFORM_BUFFER, T::UNIFORM_BUFFER_DYNAMIC)
                    | (T::STORAGE_BUFFER, T::STORAGE_BUFFER_DYNAMIC)
            );
        if !compatible
            || binding.stage_flags & stage.as_raw() == 0
            || binding.descriptor_count == 0
            || resource.count > binding.descriptor_count
        {
            return Err(Error::invalid_usage(format!(
                "shader descriptor set {} binding {} does not fit the pipeline layout",
                resource.set, resource.binding
            )));
        }
    }
    for &(start, end) in &entry.pushes {
        if !layout.push_constants.iter().any(|range| {
            range.stage_flags.contains(stage)
                && u64::from(range.offset) <= start
                && u64::from(range.offset) + u64::from(range.size) >= end
        }) {
            return Err(Error::invalid_usage(
                "shader push block does not fit the pipeline layout",
            ));
        }
    }
    Ok(index)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn workgroup_contracts_follow_each_entry_and_unknown_sizes_are_not_compatible() {
        use rspirv::{binary::Assemble, dr::Builder, spirv::*};
        let mut builder = Builder::new();
        builder.set_version(1, 3);
        builder.capability(Capability::Shader);
        builder.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
        let void = builder.type_void();
        let signature = builder.type_function(void, []);
        for (name, size) in [("wide", [8, 4, 1]), ("narrow", [4, 4, 1])] {
            let function = builder
                .begin_function(void, None, FunctionControl::NONE, signature)
                .unwrap();
            builder.begin_block(None).unwrap();
            builder.ret().unwrap();
            builder.end_function().unwrap();
            builder.entry_point(ExecutionModel::GLCompute, function, name, []);
            builder.execution_mode(function, ExecutionMode::LocalSize, size);
        }
        let words = builder.module().assemble();
        let reflected = super::super::shader::reflect_spirv_entries(&words).unwrap();
        assert_eq!(
            reflected
                .iter()
                .find(|entry| entry.entry_point == "wide")
                .unwrap()
                .local_size,
            Some([8, 4, 1])
        );
        assert_eq!(
            reflected
                .iter()
                .find(|entry| entry.entry_point == "narrow")
                .unwrap()
                .local_size,
            Some([4, 4, 1])
        );
        let first = contracts(&words).unwrap();
        let second = contracts(&words).unwrap();
        assert!(first[0].same_interface(&second[0]));
        assert!(!first[0].same_interface(&first[1]));
        let mut unknown = compact(reflected.into_iter().next().unwrap()).unwrap();
        unknown.local_size = None;
        assert!(!unknown.same_interface(&unknown));
    }

    #[test]
    fn statically_used_resources_follow_calls_and_pointer_arguments_without_dead_functions() {
        use rspirv::{
            binary::Assemble,
            dr::{Builder, Operand},
            spirv::*,
        };
        let mut builder = Builder::new();
        builder.set_version(1, 3);
        builder.capability(Capability::Shader);
        builder.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
        let void = builder.type_void();
        let uint = builder.type_int(32, 0);
        let zero = builder.constant_u32(uint, 0);
        let block = builder.type_struct([uint]);
        builder.decorate(block, Decoration::Block, []);
        builder.member_decorate(block, 0, Decoration::Offset, [Operand::LiteralInt32(0)]);
        let storage_ptr = builder.type_pointer(None, StorageClass::StorageBuffer, block);
        let push_ptr = builder.type_pointer(None, StorageClass::PushConstant, block);
        let storage_scalar = builder.type_pointer(None, StorageClass::StorageBuffer, uint);
        let push_scalar = builder.type_pointer(None, StorageClass::PushConstant, uint);
        let buffers: Vec<_> = (0..2)
            .map(|binding| {
                let id = builder.variable(storage_ptr, None, StorageClass::StorageBuffer, None);
                builder.decorate(id, Decoration::DescriptorSet, [Operand::LiteralInt32(0)]);
                builder.decorate(id, Decoration::Binding, [Operand::LiteralInt32(binding)]);
                id
            })
            .collect();
        let push = builder.variable(push_ptr, None, StorageClass::PushConstant, None);
        let helper_type = builder.type_function(void, [storage_ptr, push_ptr]);
        let main_type = builder.type_function(void, []);
        let helper = builder
            .begin_function(void, None, FunctionControl::NONE, helper_type)
            .unwrap();
        let buffer_argument = builder.function_parameter(storage_ptr).unwrap();
        let push_argument = builder.function_parameter(push_ptr).unwrap();
        builder.begin_block(None).unwrap();
        let source = builder
            .access_chain(push_scalar, None, push_argument, [zero])
            .unwrap();
        let value = builder.load(uint, None, source, None, []).unwrap();
        let target = builder
            .access_chain(storage_scalar, None, buffer_argument, [zero])
            .unwrap();
        builder.store(target, value, None, []).unwrap();
        builder.ret().unwrap();
        builder.end_function().unwrap();
        builder
            .begin_function(void, None, FunctionControl::NONE, main_type)
            .unwrap();
        builder.begin_block(None).unwrap();
        let dead_target = builder
            .access_chain(storage_scalar, None, buffers[1], [zero])
            .unwrap();
        builder.store(dead_target, zero, None, []).unwrap();
        builder.ret().unwrap();
        builder.end_function().unwrap();
        let main = builder
            .begin_function(void, None, FunctionControl::NONE, main_type)
            .unwrap();
        builder.begin_block(None).unwrap();
        builder
            .function_call(void, None, helper, [buffers[0], push])
            .unwrap();
        builder.ret().unwrap();
        builder.end_function().unwrap();
        builder.entry_point(ExecutionModel::GLCompute, main, "main", []);
        builder.execution_mode(main, ExecutionMode::LocalSize, [1, 1, 1]);
        let words = builder.module().assemble();
        let all = super::super::shader::reflect_spirv_entries(&words).unwrap();
        assert_eq!(all[0].descriptors.len(), 2);
        let selected = contracts(&words).unwrap();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].descriptors.len(), 1);
        assert_eq!(selected[0].descriptors[0].binding, 0);
        assert_eq!(selected[0].pushes, vec![(0, 4)]);
        assert!(!selected[0].schema.is_empty());
    }
}
