//! 原spirq公开fork生产元数据；游戏自有策略是名字归一、跨stage合并、UBO字段映射。
//! 不读取原bin元数据作为producer；原资产仅用于独立差分测试。
use crate::shader_pipeline::{
    compiled::{CompiledShaderStage, ShaderStage},
    reflection::{
        Binding, BindingArray, BindingKind, BindingLocation, BindingNameHash, ShaderReflection,
        UniformBlock, VertexInput,
    },
    reflection_ty as w,
};
use spirq::{DescriptorType, ReflectConfig, SpirvBinary, Variable, ty as s};
use std::collections::BTreeMap;
fn scalar(x: &s::ScalarType) -> w::ScalarType {
    match *x {
        s::ScalarType::Void => w::ScalarType::Void,
        s::ScalarType::Boolean => w::ScalarType::Boolean,
        s::ScalarType::Signed(n) => w::ScalarType::Signed(n),
        s::ScalarType::Unsigned(n) => w::ScalarType::Unsigned(n),
        s::ScalarType::Float(n) => w::ScalarType::Float(n),
    }
}
fn vector(x: &s::VectorType) -> w::VectorType {
    w::VectorType {
        scalar_ty: scalar(&x.scalar_ty),
        nscalar: x.nscalar,
    }
}
pub fn stored_type(x: &s::Type) -> Result<w::Type, String> {
    Ok(match x {
        s::Type::Scalar(x) => w::Type::Scalar(scalar(x)),
        s::Type::Vector(x) => w::Type::Vector(vector(x)),
        s::Type::Matrix(x) => w::Type::Matrix(w::MatrixType {
            vec_ty: vector(&x.vec_ty),
            nvec: x.nvec,
            major: x.major.map(|m| match m {
                s::MatrixAxisOrder::ColumnMajor => w::MatrixAxisOrder::ColumnMajor,
                s::MatrixAxisOrder::RowMajor => w::MatrixAxisOrder::RowMajor,
            }),
            stride: x.stride.map(|n| n as u64),
        }),
        s::Type::Image(_) => w::Type::Image,
        s::Type::CombinedImageSampler(_) => w::Type::CombinedImageSampler,
        s::Type::SampledImage(_) => w::Type::SampledImage,
        s::Type::StorageImage(_) => w::Type::StorageImage,
        s::Type::Sampler() => w::Type::Sampler,
        s::Type::SubpassData(_) => w::Type::SubpassData,
        s::Type::Array(x) => w::Type::Array(w::ArrayType {
            proto_ty: Box::new(stored_type(&x.proto_ty)?),
            nrepeat: x.nrepeat,
            stride: x.stride.map(|n| n as u64),
        }),
        s::Type::Struct(x) => w::Type::Struct(w::StructType {
            name: x.name.clone(),
            members: x
                .members
                .iter()
                .map(|m| {
                    Ok(w::StructMember {
                        name: m.name.clone(),
                        offset: m.offset as u64,
                        ty: stored_type(&m.ty)?,
                    })
                })
                .collect::<Result<_, String>>()?,
        }),
        s::Type::AccelStruct() => w::Type::AccelStruct,
        s::Type::DeviceAddress() => w::Type::DeviceAddress,
        s::Type::DevicePointer(x) => w::Type::DevicePointer(Box::new(stored_type(&x.pointee_ty)?)),
        _ => return Err("SPIR-Q type has no observed stored representation".into()),
    })
}
fn kind(x: DescriptorType) -> Result<BindingKind, String> {
    Ok(match x {
        DescriptorType::StorageBuffer(_) => BindingKind::StorageBuffer,
        DescriptorType::UniformBuffer() => BindingKind::UniformBuffer,
        DescriptorType::CombinedImageSampler() => BindingKind::CombinedImageSampler,
        DescriptorType::StorageImage(_) => BindingKind::StorageImage,
        DescriptorType::SampledImage() => BindingKind::SampledImage,
        DescriptorType::Sampler() => BindingKind::Sampler,
        _ => return Err("descriptor kind outside original stored schema".into()),
    })
}
fn canonical_name(name: String) -> String {
    name.strip_prefix("__alias_").unwrap_or(&name).to_owned()
}
fn size(x: &s::Type) -> Result<u32, String> {
    x.nbyte()
        .and_then(|n| n.try_into().ok())
        .ok_or_else(|| "missing/oversized reflected byte size".into())
}
fn stage_label(stage: ShaderStage) -> &'static str {
    match stage {
        ShaderStage::Compute => "compute",
        ShaderStage::Vertex => "vertex",
        ShaderStage::Fragment => "fragment",
    }
}
/// descriptor/name/type集合按原发行schema生成；确定排序仅用于可复现新打包，原HashMap顺序不作为语义。
pub fn generate_reflection(stages: &[CompiledShaderStage]) -> Result<ShaderReflection, String> {
    let mut descriptors: BTreeMap<(u32, u32), Binding> = BTreeMap::new();
    let mut inputs: BTreeMap<String, VertexInput> = BTreeMap::new();
    let mut uniform: Option<UniformBlock> = None;
    let mut local_size = None;
    let mut push_constant_size = 0;
    for stage in stages {
        let entries = ReflectConfig::new()
            .spv(SpirvBinary::from(stage.spirv.as_slice()))
            .ref_all_rscs(true)
            .combine_img_samplers(false)
            .gen_unique_names(false)
            .reflect()
            .map_err(|e| e.to_string())?;
        let entry = entries
            .into_iter()
            .find(|e| e.name == stage.entry_point)
            .ok_or("missing reflection entry")?;
        let expected = match stage.stage {
            ShaderStage::Compute => spirq::ExecutionModel::GLCompute,
            ShaderStage::Vertex => spirq::ExecutionModel::Vertex,
            ShaderStage::Fragment => spirq::ExecutionModel::Fragment,
        };
        if entry.exec_model != expected {
            return Err("reflection stage mismatch".into());
        }
        for mode in entry.exec_modes {
            if format!("{:?}", mode.exec_mode) == "LocalSize" {
                let values: Vec<_> = mode.operands.iter().map(|o| o.value.to_u32()).collect();
                if values.len() != 3 || mode.operands.iter().any(|o| o.spec_id.is_some()) {
                    return Err("unobserved local size form".into());
                }
                let value = [values[0], values[1], values[2]];
                if local_size.is_some_and(|old| old != value) {
                    return Err("conflicting local sizes".into());
                }
                local_size = Some(value);
            }
        }
        for var in entry.vars {
            match var {
                Variable::Descriptor {
                    name,
                    desc_bind,
                    desc_ty,
                    ty,
                    nbind,
                } => {
                    let name = canonical_name(name.ok_or("unnamed original descriptor")?);
                    let location = BindingLocation {
                        set: desc_bind.set(),
                        binding: desc_bind.bind(),
                    };
                    let binding = Binding {
                        kind: kind(desc_ty)?,
                        array: if nbind == 0 {
                            BindingArray::Runtime
                        } else {
                            BindingArray::Fixed(nbind)
                        },
                        name: name.clone(),
                    };
                    if let Some(old) =
                        descriptors.insert((location.set, location.binding), binding.clone())
                        && old != binding
                    {
                        // 同槽跨stage必须完全一致（含名字）；名字是绑定键来源，不能择一。
                        let differs: Vec<_> = [
                            ("kind", old.kind != binding.kind),
                            ("array", old.array != binding.array),
                            ("name", old.name != binding.name),
                        ]
                        .into_iter()
                        .filter_map(|(field, differ)| differ.then_some(field))
                        .collect();
                        return Err(format!(
                            "conflicting cross-stage descriptor at set={} binding={} ({} stage `{}` differs in {}): earlier={old:?}; this={binding:?}",
                            location.set,
                            location.binding,
                            stage_label(stage.stage),
                            stage.entry_point,
                            differs.join("/")
                        ));
                    }
                    if name == "$Globals" {
                        let s::Type::Struct(structure) = &ty else {
                            return Err("$Globals is not a reflected struct".into());
                        };
                        let mut name_offsets = BTreeMap::new();
                        for member in &structure.members {
                            name_offsets.insert(
                                member.name.clone().ok_or("unnamed uniform member")?,
                                u32::try_from(member.offset)
                                    .map_err(|_| "oversized uniform offset")?,
                            );
                        }
                        let block = UniformBlock {
                            byte_size: size(&ty)?,
                            name_hash_offsets: name_offsets
                                .iter()
                                .map(|(n, o)| (BindingNameHash::new(n).0, *o))
                                .collect(),
                            name_offsets: name_offsets.into_iter().collect(),
                            binding: location,
                        };
                        if let Some(old) = &uniform
                            && (old.binding != block.binding
                                || old.byte_size != block.byte_size
                                || old.name_offsets != block.name_offsets)
                        {
                            return Err("conflicting cross-stage uniform block".into());
                        }
                        uniform = Some(block);
                    }
                }
                Variable::Input { name, location, ty } if stage.stage == ShaderStage::Vertex => {
                    if location.comp() != 0 {
                        return Err("vertex component outside observed schema".into());
                    }
                    let name = name.ok_or("unnamed vertex input")?;
                    let name = name.strip_prefix("in.var.").unwrap_or(&name).to_owned();
                    let input = VertexInput {
                        location: location.loc(),
                        ty: stored_type(&ty)?,
                    };
                    if let Some(old) = inputs.insert(name, input.clone())
                        && old != input
                    {
                        return Err("conflicting vertex input".into());
                    }
                }
                // 原消费者只建一个 VkPushConstantRange{offset 0, size, 全部可见stage}，
                // 故跨stage取最大字节尺寸即可覆盖各stage块（SPIR-Q 尺寸已含成员偏移）。
                Variable::PushConstant { ty, .. } => {
                    push_constant_size = push_constant_size.max(size(&ty)?)
                }
                _ => {}
            }
        }
    }
    let nsets = descriptors
        .keys()
        .map(|(set, _)| *set as usize + 1)
        .max()
        .unwrap_or(0);
    let mut descriptor_sets = vec![Vec::new(); nsets];
    let mut binding_names = Vec::new();
    let mut binding_name_hashes = Vec::new();
    for ((set, index), binding) in descriptors {
        let location = BindingLocation {
            set,
            binding: index,
        };
        binding_names.push((binding.name.clone(), location));
        binding_name_hashes.push((BindingNameHash::new(&binding.name).0, location));
        descriptor_sets[set as usize].push((index, binding));
    }
    Ok(ShaderReflection {
        descriptor_sets,
        uniform_block: uniform,
        binding_names,
        binding_name_hashes,
        vertex_inputs: inputs.into_iter().collect(),
        local_size,
        push_constant_size,
    })
}
