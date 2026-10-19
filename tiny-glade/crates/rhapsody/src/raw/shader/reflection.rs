//! Shader reflection: stored and SPIR-V reflected entries and interface checks.
use super::*;

/// 原 spirq 分叉生成的已保存反射条目；通用 SPIR-V 解析不在本模块重写。
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReflectedEntry {
    pub entry_point: String,
    pub stage: String,
    /// Concrete default workgroup dimensions for this compute entry. None
    /// means no supported execution-size contract was resolved; hot reload
    /// cannot assume compatibility for such an entry.
    #[serde(default)]
    pub local_size: Option<[u32; 3]>,
    pub descriptors: Vec<ReflectedDescriptor>,
    #[serde(default)]
    pub push_constants: Vec<ReflectedPushConstant>,
    #[serde(default)]
    pub inputs: Vec<ReflectedLocation>,
    #[serde(default)]
    pub outputs: Vec<ReflectedLocation>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReflectedLocation {
    pub location: u32,
    pub component: u32,
    #[serde(rename = "type")]
    pub reflected_type: serde_json::Value,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReflectedDescriptor {
    pub set: u32,
    pub binding: u32,
    pub name: String,
    pub count: u32,
    pub descriptor_type: String,
    #[serde(default, rename = "type")]
    pub reflected_type: serde_json::Value,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReflectedPushConstant {
    #[serde(rename = "type")]
    pub reflected_type: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InterfaceError {
    Stage(String),
    Descriptor(String),
    IncompatibleBinding(u32, u32),
    MissingSampler(String),
    InvalidPushConstant,
    RuntimeBindingNotLast(u32, u32),
    InvalidLayout,
    IncompatibleLocation(u32, u32),
    /// A vertex input has no byte size (`Type::nbyte()` is None, e.g. an
    /// opaque resource type).
    VertexInputByteCount,
    /// Unsupported vertex attribute type; the payload is the type's Debug text.
    UnsupportedVertexAttributeType(String),
    /// A compute shader declares stage inputs.
    ComputeHasVertexInputs,
    /// A compute shader has no workgroup size (`local_size` is None).
    MissingGroupSize,
}
pub mod original_panics {
    pub const COMPUTE_INPUTS_NOT_EMPTY: &str =
        "assertion failed: compiled.reflection.inputs.is_empty()";
    pub const VALID_GROUP_SIZE: &str = "a valid group size";
    pub const FAILED_CREATING_SHADER_PIPELINE_PREFIX: &str = "Failed creating shader pipeline ";
    pub const FAILED_CREATING_SHADER_PIPELINE_SEP: &str = ": ";
    pub const UNABLE_TO_CREATE_GRAPHICS_PIPELINE: &str = "Unable to create graphics pipeline";
    pub const VERTEX_INPUT_ATTRIB_BYTE_COUNT: &str = "vertex input attrib byte count";
    pub const UNSUPPORTED_VERTEX_ATTRIBUTE_TYPE_PREFIX: &str =
        "Unsupported vertex attribute type: ";
    pub const UNWRAP_ON_ERR: &str = "called `Result::unwrap()` on an `Err` value";

    /// compute 管线创建失败（453:39）的完整 panic 文本。
    pub fn failed_creating_shader_pipeline(name: &str, err: &dyn std::fmt::Debug) -> String {
        format!(
            "{FAILED_CREATING_SHADER_PIPELINE_PREFIX}{name}{FAILED_CREATING_SHADER_PIPELINE_SEP}{err:?}"
        )
    }
}
/// Reflection checks before compute pipeline creation: no stage inputs, then
/// a known local size.
pub fn validate_compute_reflection(
    reflection: &crate::shader_pipeline::reflection::ShaderReflection,
) -> Result<[u32; 3]> {
    if !reflection.vertex_inputs.is_empty() {
        return Err(InterfaceError::ComputeHasVertexInputs.into());
    }
    reflection
        .local_size
        .ok_or_else(|| InterfaceError::MissingGroupSize.into())
}
impl std::fmt::Display for InterfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VertexInputByteCount => {
                f.write_str(original_panics::VERTEX_INPUT_ATTRIB_BYTE_COUNT)
            }
            Self::UnsupportedVertexAttributeType(ty) => {
                write!(
                    f,
                    "{}{ty}",
                    original_panics::UNSUPPORTED_VERTEX_ATTRIBUTE_TYPE_PREFIX
                )
            }
            Self::ComputeHasVertexInputs => f.write_str(original_panics::COMPUTE_INPUTS_NOT_EMPTY),
            Self::MissingGroupSize => f.write_str(original_panics::VALID_GROUP_SIZE),
            Self::MissingSampler(name) => write!(f, "Sampler not found: {name}"),
            _ => write!(f, "着色器接口无效：{self:?}"),
        }
    }
}
impl std::error::Error for InterfaceError {}
impl From<InterfaceError> for crate::Error {
    fn from(source: InterfaceError) -> Self {
        Self::invalid_usage_with("着色器接口无效", source)
    }
}

/// Reflect full module resource schemas using the shared SPIR-Q type/layout implementation.
pub fn reflect_spirv_entries(words: &[u32]) -> Result<Vec<ReflectedEntry>> {
    reflect_spirv_entries_with_resources(words, true)
}
pub(crate) fn reflect_spirv_entries_used(words: &[u32]) -> Result<Vec<ReflectedEntry>> {
    use rspirv::{binary::Assemble, dr::Operand, spirv::Op};
    use std::collections::{HashMap, HashSet};
    let module = rspirv::dr::load_words(words).map_err(|_| invalid_shader())?;
    let functions = module
        .functions
        .iter()
        .map(|function| {
            function
                .def
                .as_ref()
                .and_then(|definition| definition.result_id)
                .map(|id| (id, function))
                .ok_or_else(invalid_shader)
        })
        .collect::<Result<HashMap<_, _>>>()?;
    let globals = module
        .types_global_values
        .iter()
        .filter(|instruction| instruction.class.opcode == Op::Variable)
        .filter_map(|instruction| instruction.result_id)
        .collect::<HashSet<_>>();
    let mut result = Vec::new();
    for entry in &module.entry_points {
        let Some(Operand::IdRef(root)) = entry.operands.get(1) else {
            return Err(invalid_shader());
        };
        let mut pending = vec![*root];
        let mut reached = HashSet::new();
        let mut used = HashSet::new();
        while let Some(id) = pending.pop() {
            if !reached.insert(id) {
                continue;
            }
            let function = functions.get(&id).ok_or_else(invalid_shader)?;
            for instruction in function.all_inst_iter() {
                if instruction.class.opcode == Op::FunctionCall {
                    let Some(Operand::IdRef(callee)) = instruction.operands.first() else {
                        return Err(invalid_shader());
                    };
                    pending.push(*callee);
                }
                for operand in &instruction.operands {
                    if let Operand::IdRef(id) = operand
                        && globals.contains(id)
                    {
                        used.insert(*id);
                    }
                }
            }
        }
        // The pinned SPIR-Q fork consumes function instructions in its definition
        // scan, so ref_all_rscs(false) silently misses every static resource use.
        // Select variables from the real call tree, then reuse its type/layout
        // reflection on this private view. The driver's module stays unchanged.
        let mut view = module.clone();
        view.entry_points = vec![entry.clone()];
        let interface = &mut view.entry_points[0].operands;
        let mut position = 0;
        interface.retain(|operand| {
            position += 1;
            position <= 3 || !matches!(operand, Operand::IdRef(id) if globals.contains(id) && !used.contains(id))
        });
        view.functions.retain(|function| {
            function
                .def
                .as_ref()
                .and_then(|definition| definition.result_id)
                .is_some_and(|id| reached.contains(&id))
        });
        view.types_global_values.retain(|instruction| {
            instruction.class.opcode != Op::Variable
                || instruction.result_id.is_some_and(|id| used.contains(&id))
        });
        result.extend(reflect_spirv_entries_with_resources(
            &view.assemble(),
            true,
        )?);
    }
    Ok(result)
}
fn reflect_spirv_entries_with_resources(
    words: &[u32],
    all_resources: bool,
) -> Result<Vec<ReflectedEntry>> {
    if words.len() < 5 || words.first().copied() != Some(0x07230203) {
        return Err(invalid_shader());
    }
    // This pinned reflector seeks a name section before decorations and types.
    // OpName is optional; add one to a reflection-only copy when absent.
    let mut module = rspirv::dr::load_words(words).map_err(|_| invalid_shader())?;
    let normalized;
    let words = if module.debug_names.is_empty() {
        let function = module
            .entry_points
            .first()
            .and_then(|entry| entry.operands.get(1))
            .and_then(|operand| {
                if let rspirv::dr::Operand::IdRef(id) = operand {
                    Some(*id)
                } else {
                    None
                }
            })
            .ok_or_else(invalid_shader)?;
        module.debug_names.push(rspirv::dr::Instruction::new(
            rspirv::spirv::Op::Name,
            None,
            None,
            vec![
                rspirv::dr::Operand::IdRef(function),
                rspirv::dr::Operand::LiteralString("rhapsody_reflection_entry".into()),
            ],
        ));
        use rspirv::binary::Assemble;
        normalized = module.assemble();
        normalized.as_slice()
    } else {
        words
    };
    use spirq::{ReflectConfig, SpirvBinary, Variable};
    let reflected = ReflectConfig::new()
        .spv(SpirvBinary::from(words))
        .ref_all_rscs(all_resources)
        .combine_img_samplers(false)
        .gen_unique_names(false)
        .reflect()
        .map_err(|_| invalid_shader())?;
    reflected
        .into_iter()
        .map(|entry| {
            let mut local_size = None;
            for mode in &entry.exec_modes {
                if matches!(
                    format!("{:?}", mode.exec_mode).as_str(),
                    "LocalSize" | "LocalSizeId"
                ) {
                    if mode.operands.len() != 3 {
                        return Err(invalid_shader());
                    }
                    // Pipeline descriptors currently supply no specialization
                    // overrides, so reflected constant defaults are effective.
                    let size = [
                        mode.operands[0].value.to_u32(),
                        mode.operands[1].value.to_u32(),
                        mode.operands[2].value.to_u32(),
                    ];
                    if size.contains(&0) || local_size.is_some_and(|old| old != size) {
                        return Err(invalid_shader());
                    }
                    local_size = Some(size);
                }
            }
            let mut descriptors = Vec::new();
            let mut inputs = Vec::new();
            let mut outputs = Vec::new();
            let mut push_constants = Vec::new();
            for variable in entry.vars {
                match variable {
                    Variable::Descriptor {
                        name,
                        desc_bind,
                        desc_ty,
                        ty,
                        nbind,
                    } => {
                        descriptors.push(ReflectedDescriptor {
                            set: desc_bind.set(),
                            binding: desc_bind.bind(),
                            name: name.unwrap_or_default(),
                            count: nbind,
                            descriptor_type: format!("{desc_ty:?}"),
                            reflected_type: reflected_type_json(&ty),
                        });
                    }
                    Variable::Input { location, ty, .. } => inputs.push(ReflectedLocation {
                        location: location.loc(),
                        component: location.comp(),
                        reflected_type: reflected_type_json(&ty),
                    }),
                    Variable::Output { location, ty, .. } => outputs.push(ReflectedLocation {
                        location: location.loc(),
                        component: location.comp(),
                        reflected_type: reflected_type_json(&ty),
                    }),
                    Variable::PushConstant { ty, .. } => {
                        push_constants.push(ReflectedPushConstant {
                            reflected_type: reflected_type_json(&ty),
                        })
                    }
                    // 特殊化常量由调用者另行提供，当前运行时接口计划不生成默认值。
                    Variable::SpecConstant { .. } => {}
                }
            }
            descriptors.sort_by_key(|value| (value.set, value.binding));
            inputs.sort_by_key(|value| (value.location, value.component));
            outputs.sort_by_key(|value| (value.location, value.component));
            Ok(ReflectedEntry {
                entry_point: entry.name,
                stage: format!("{:?}", entry.exec_model),
                local_size,
                descriptors,
                inputs,
                outputs,
                push_constants,
            })
        })
        .collect()
}

/// 复用既有公开fork反射字段投影，类型解析仍由spirq执行。
pub(super) fn reflected_type_json(ty: &spirq::ty::Type) -> serde_json::Value {
    use serde_json::json;
    use spirq::ty::Type;
    let detail = match ty {
        Type::Struct(value) => {
            json!({"kind":"Struct","members":value.members.iter().map(|member| json!({"name":member.name,"offset":member.offset,"type":reflected_type_json(&member.ty)})).collect::<Vec<_>>()})
        }
        Type::Array(value) => {
            json!({"kind":"Array","count":value.nrepeat,"stride":value.stride,"element":reflected_type_json(&value.proto_ty)})
        }
        Type::Matrix(value) => {
            json!({"kind":"Matrix","axis_order":value.major.map(|major|format!("{major:?}")),"vector":value.vec_ty.to_string(),"count":value.nvec,"stride":value.stride})
        }
        Type::DevicePointer(value) => {
            json!({"kind":"DevicePointer","target":reflected_type_json(&value.pointee_ty)})
        }
        _ => json!({"kind":ty.to_string()}),
    };
    json!({"minimum_bytes":ty.nbyte(),"detail":detail})
}

/// 从真实容器stage记录取得SPIR-V，不使用外部catalog偏移或magic扫描。
/// 序列化stage/入口名与payload不一致时，在Vulkan对象创建前拒绝。
pub fn compiled_pipeline_entries(
    compiled: &crate::shader_pipeline::compiled::CompiledShaderPipeline,
) -> Result<Vec<ReflectedEntry>> {
    use crate::shader_pipeline::compiled::ShaderStage;
    compiled
        .stages
        .iter()
        .map(|stage| {
            let expected = match stage.stage {
                ShaderStage::Compute => "GLCompute",
                ShaderStage::Vertex => "Vertex",
                ShaderStage::Fragment => "Fragment",
            };
            reflect_spirv_entries(&stage.spirv)?
                .into_iter()
                .find(|entry| entry.entry_point == stage.entry_point && entry.stage == expected)
                .ok_or_else(invalid_shader)
        })
        .collect()
}
