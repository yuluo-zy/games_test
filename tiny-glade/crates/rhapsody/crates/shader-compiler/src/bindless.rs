//! Lower a shader package to the renderer's shared descriptor heap ABI.
//!
//! Resource declarations retain their element types and byte layouts. Descriptor
//! arrays with the same Vulkan descriptor type may alias a binding, as specified
//! by Vulkan's descriptor-set interface. One push word selects a storage-buffer
//! parameter block containing the original push bytes followed by heap indices.
use crate::shader_pipeline::{
    compiled::{CompiledShaderPipeline, CompiledShaderStage},
    reflection::{BindingArray, BindingKind, BindingLocation},
};
use rspirv::{
    binary::Assemble,
    dr::{Instruction, Module, Operand},
    spirv::{Capability, Decoration, Op, StorageClass},
};
use std::collections::BTreeMap;

pub const ABI_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BindingSlot {
    pub location: BindingLocation,
    pub kind: BindingKind,
    pub array: BindingArray,
    /// Byte offset in the per-dispatch/draw parameter buffer.
    pub parameter_offset: u32,
    pub parameter_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BindlessAbi {
    pub original_push_bytes: u32,
    pub parameter_bytes: u32,
    pub bindings: Vec<BindingSlot>,
}
impl BindlessAbi {
    pub fn from_pipeline(pipeline: &CompiledShaderPipeline) -> Result<Self, LoweringError> {
        Self::from_pipeline_with_limit(pipeline, 64)
    }
    pub fn from_pipeline_with_limit(
        pipeline: &CompiledShaderPipeline,
        runtime_array_limit: u32,
    ) -> Result<Self, LoweringError> {
        if runtime_array_limit == 0 || runtime_array_limit > 65536 {
            return Err(error("runtime descriptor count must be within 1..=65536"));
        }
        let base = pipeline
            .reflection
            .push_constant_size
            .checked_add(3)
            .map(|n| n & !3)
            .ok_or_else(|| error("push byte size overflows"))?;
        let mut bindings = Vec::new();
        let mut index_count = 0u32;
        for (set, entries) in pipeline
            .reflection
            .effective_descriptor_sets()
            .into_iter()
            .enumerate()
        {
            for (binding, descriptor) in entries {
                let offset = base
                    .checked_add(
                        index_count
                            .checked_mul(4)
                            .ok_or_else(|| error("binding table size overflows"))?,
                    )
                    .ok_or_else(|| error("parameter size overflows"))?;
                let parameter_count = match descriptor.array {
                    BindingArray::Fixed(count) => count,
                    BindingArray::Runtime => runtime_array_limit,
                };
                if parameter_count == 0 {
                    return Err(error("descriptor array is empty"));
                }
                index_count = index_count
                    .checked_add(parameter_count)
                    .ok_or_else(|| error("binding table size overflows"))?;
                bindings.push(BindingSlot {
                    location: BindingLocation {
                        set: set as u32,
                        binding,
                    },
                    kind: descriptor.kind,
                    array: descriptor.array,
                    parameter_offset: offset,
                    parameter_count,
                });
            }
        }
        if bindings.len() > 65536 {
            return Err(error("binding table exceeds the bounded ABI limit"));
        }
        let parameter_bytes = base
            .checked_add(
                index_count
                    .max(1)
                    .checked_mul(4)
                    .ok_or_else(|| error("parameter size overflows"))?,
            )
            .ok_or_else(|| error("parameter size overflows"))?
            .max(4);
        let abi = Self {
            original_push_bytes: pipeline.reflection.push_constant_size,
            parameter_bytes,
            bindings,
        };
        abi.validate()?;
        Ok(abi)
    }
    pub fn validate(&self) -> Result<(), LoweringError> {
        let mut offset = self
            .original_push_bytes
            .checked_add(3)
            .map(|n| n & !3)
            .ok_or_else(|| error("push byte size overflows"))?;
        let base = offset;
        let mut previous = None;
        for slot in &self.bindings {
            let key = (slot.location.set, slot.location.binding);
            if previous.is_some_and(|old| old >= key)
                || slot.parameter_count == 0
                || slot.parameter_offset != offset
                || matches!(slot.array, BindingArray::Fixed(count) if count != slot.parameter_count)
            {
                return Err(error(
                    "shader parameter ABI is not a canonical bounded binding table",
                ));
            }
            previous = Some(key);
            offset = offset
                .checked_add(
                    slot.parameter_count
                        .checked_mul(4)
                        .ok_or_else(|| error("parameter size overflows"))?,
                )
                .ok_or_else(|| error("parameter size overflows"))?;
        }
        if self.parameter_bytes
            != offset.max(
                base.checked_add(4)
                    .ok_or_else(|| error("parameter size overflows"))?,
            )
            || self.parameter_bytes > 0x40000
        {
            return Err(error(
                "shader parameters exceed the bounded parameter-page ABI",
            ));
        }
        Ok(())
    }
    pub fn slot(&self, location: BindingLocation) -> Option<&BindingSlot> {
        self.bindings.iter().find(|slot| slot.location == location)
    }
}

#[derive(Debug)]
pub struct LoweringError(String);
impl std::fmt::Display for LoweringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for LoweringError {}
fn error(message: &str) -> LoweringError {
    LoweringError(message.into())
}
fn id(operand: &Operand) -> Option<u32> {
    if let Operand::IdRef(id) = operand {
        Some(*id)
    } else {
        None
    }
}
fn literal(operand: &Operand) -> Option<u32> {
    if let Operand::LiteralInt32(n) = operand {
        Some(*n)
    } else {
        None
    }
}
fn instruction(
    op: Op,
    ty: Option<u32>,
    result: Option<u32>,
    operands: Vec<Operand>,
) -> Instruction {
    Instruction::new(op, ty, result, operands)
}
fn heap_binding(kind: BindingKind) -> Result<u32, LoweringError> {
    Ok(match kind {
        BindingKind::SampledImage => 0,
        BindingKind::StorageImage => 1,
        BindingKind::StorageBuffer | BindingKind::UniformBuffer => 2,
        BindingKind::Sampler => 3,
        BindingKind::CombinedImageSampler => {
            return Err(error(
                "combined samplers must be split by shader compilation before bindless lowering",
            ));
        }
    })
}
#[derive(Clone, Copy)]
struct Resource {
    element: u32,
    class: StorageClass,
    slot: Option<u32>,
    array: bool,
}
struct Lowerer {
    module: Module,
    next: u32,
    types: Vec<Instruction>,
    constants: BTreeMap<u32, u32>,
    pointers: BTreeMap<(u32, u32), u32>,
    uint: u32,
    push: u32,
    parameters: u32,
    indices_member: u32,
    resources: BTreeMap<u32, Resource>,
    non_uniform: Vec<u32>,
    value_types: BTreeMap<u32, u32>,
    integer_types: BTreeMap<u32, (u32, u32)>,
}
impl Lowerer {
    fn fresh(&mut self) -> u32 {
        let value = self.next;
        self.next += 1;
        value
    }
    fn ty(&mut self, op: Op, operands: Vec<Operand>) -> u32 {
        let result = self.fresh();
        self.types
            .push(instruction(op, None, Some(result), operands));
        result
    }
    fn pointer(&mut self, class: StorageClass, element: u32) -> u32 {
        if let Some(value) = self.pointers.get(&(class as u32, element)) {
            return *value;
        }
        let result = self.ty(
            Op::TypePointer,
            vec![Operand::StorageClass(class), Operand::IdRef(element)],
        );
        self.pointers.insert((class as u32, element), result);
        result
    }
    fn constant(&mut self, value: u32) -> u32 {
        if let Some(result) = self.constants.get(&value) {
            return *result;
        }
        let result = self.fresh();
        self.types.push(instruction(
            Op::Constant,
            Some(self.uint),
            Some(result),
            vec![Operand::LiteralInt32(value)],
        ));
        self.constants.insert(value, result);
        result
    }
    fn decorate(&mut self, target: u32, decoration: Decoration, operands: Vec<Operand>) {
        let mut all = vec![Operand::IdRef(target), Operand::Decoration(decoration)];
        all.extend(operands);
        self.module
            .annotations
            .push(instruction(Op::Decorate, None, None, all));
    }
    fn member_offset(&mut self, target: u32, member: u32, offset: u32) {
        self.module.annotations.push(instruction(
            Op::MemberDecorate,
            None,
            None,
            vec![
                Operand::IdRef(target),
                Operand::LiteralInt32(member),
                Operand::Decoration(Decoration::Offset),
                Operand::LiteralInt32(offset),
            ],
        ));
    }
    fn parameter_index(&mut self, output: &mut Vec<Instruction>) -> u32 {
        let ptr_ty = self.pointer(StorageClass::PushConstant, self.uint);
        let zero = self.constant(0);
        let ptr = self.fresh();
        output.push(instruction(
            Op::AccessChain,
            Some(ptr_ty),
            Some(ptr),
            vec![Operand::IdRef(self.push), Operand::IdRef(zero)],
        ));
        let value = self.fresh();
        output.push(instruction(
            Op::Load,
            Some(self.uint),
            Some(value),
            vec![Operand::IdRef(ptr)],
        ));
        value
    }
    fn resource_index(
        &mut self,
        resource: Resource,
        relative: Option<u32>,
        output: &mut Vec<Instruction>,
    ) -> Result<u32, LoweringError> {
        let parameter = self.parameter_index(output);
        let Some(start) = resource.slot else {
            return Ok(parameter);
        };
        let ptr_ty = self.pointer(StorageClass::StorageBuffer, self.uint);
        let member = self.constant(self.indices_member);
        let mut element = self.constant(start);
        if let Some(relative) = relative {
            let relative_type = self
                .value_types
                .get(&relative)
                .copied()
                .ok_or_else(|| error("descriptor index has no type"))?;
            let mut normalized = relative;
            if relative_type != self.uint {
                let (width, _) = self
                    .integer_types
                    .get(&relative_type)
                    .copied()
                    .ok_or_else(|| error("descriptor array index is not an integer"))?;
                normalized = self.fresh();
                output.push(instruction(
                    if width == 32 {
                        Op::Bitcast
                    } else {
                        Op::UConvert
                    },
                    Some(self.uint),
                    Some(normalized),
                    vec![Operand::IdRef(relative)],
                ));
            }
            let sum = self.fresh();
            output.push(instruction(
                Op::IAdd,
                Some(self.uint),
                Some(sum),
                vec![Operand::IdRef(element), Operand::IdRef(normalized)],
            ));
            if self.non_uniform.contains(&relative) {
                self.decorate(sum, Decoration::NonUniform, vec![]);
            }
            element = sum;
        }
        let ptr = self.fresh();
        output.push(instruction(
            Op::AccessChain,
            Some(ptr_ty),
            Some(ptr),
            vec![
                Operand::IdRef(self.parameters),
                Operand::IdRef(parameter),
                Operand::IdRef(member),
                Operand::IdRef(element),
            ],
        ));
        let value = self.fresh();
        output.push(instruction(
            Op::Load,
            Some(self.uint),
            Some(value),
            vec![Operand::IdRef(ptr)],
        ));
        if relative.is_some_and(|relative| self.non_uniform.contains(&relative)) {
            self.decorate(value, Decoration::NonUniform, vec![]);
        }
        Ok(value)
    }
    fn rewrite(&mut self) -> Result<(), LoweringError> {
        let mut functions = std::mem::take(&mut self.module.functions);
        for function in &mut functions {
            for block in &mut function.blocks {
                let mut output = Vec::with_capacity(block.instructions.len());
                for mut inst in std::mem::take(&mut block.instructions) {
                    for operand_index in 0..inst.operands.len() {
                        let Some(original) = id(&inst.operands[operand_index]) else {
                            continue;
                        };
                        let Some(resource) = self.resources.get(&original).copied() else {
                            continue;
                        };
                        if matches!(inst.class.opcode, Op::Phi | Op::Variable) {
                            return Err(error(
                                "resource pointers in Phi/local initializers require explicit shader adaptation",
                            ));
                        }
                        let array_access = operand_index == 0
                            && matches!(
                                inst.class.opcode,
                                Op::AccessChain | Op::InBoundsAccessChain
                            );
                        let relative = if resource.array && array_access {
                            Some(
                                inst.operands
                                    .get(1)
                                    .and_then(id)
                                    .ok_or_else(|| error("descriptor array access has no index"))?,
                            )
                        } else {
                            None
                        };
                        let index = self.resource_index(resource, relative, &mut output)?;
                        if operand_index == 0
                            && matches!(
                                inst.class.opcode,
                                Op::AccessChain | Op::InBoundsAccessChain
                            )
                        {
                            if resource.array {
                                inst.operands[1] = Operand::IdRef(index);
                            } else {
                                inst.operands.insert(1, Operand::IdRef(index));
                            }
                            break;
                        }
                        if resource.array {
                            return Err(error(
                                "whole descriptor arrays cannot be loaded or passed by pointer",
                            ));
                        }
                        let ptr_ty = self.pointer(resource.class, resource.element);
                        let ptr = self.fresh();
                        output.push(instruction(
                            Op::AccessChain,
                            Some(ptr_ty),
                            Some(ptr),
                            vec![Operand::IdRef(original), Operand::IdRef(index)],
                        ));
                        inst.operands[operand_index] = Operand::IdRef(ptr);
                    }
                    output.push(inst);
                }
                block.instructions = output;
            }
        }
        self.module.functions = functions;
        Ok(())
    }
}

/// Produces the only production shader ABI; the serialized input package remains
/// readable for asset migration. No source text or debug names are needed.
pub fn lower_stage(
    stage: &CompiledShaderStage,
    abi: &BindlessAbi,
) -> Result<CompiledShaderStage, LoweringError> {
    abi.validate()?;
    let mut module = rspirv::dr::load_words(&stage.spirv).map_err(|e| error(&e.to_string()))?;
    let maximum_added_ids = stage.spirv.len().checked_mul(32)
        .and_then(|count| count.checked_add(abi.bindings.len().saturating_mul(4)))
        .and_then(|count| count.checked_add(128)).and_then(|count| u32::try_from(count).ok())
        .ok_or_else(|| error("SPIR-V module exceeds the bounded lowering size"))?;
    let bound = module.header.as_ref().ok_or_else(|| error("missing SPIR-V header"))?.bound;
    if bound.checked_add(maximum_added_ids).is_none() {
        return Err(error("SPIR-V id bound leaves no space for shader lowering"));
    }
    if module.entry_points.iter().filter(|entry| matches!(entry.operands.get(2),
        Some(Operand::LiteralString(name)) if name == &stage.entry_point)).count() != 1 {
        return Err(error("package stage must identify one existing shader entry"));
    }
    if module.types_global_values.iter().filter(|inst| inst.class.opcode == Op::Variable
        && matches!(inst.operands.first(), Some(Operand::StorageClass(StorageClass::PushConstant)))).count() > 1 {
        return Err(error("multiple push variables require selecting a single shader entry before lowering"));
    }
    let mut definitions = BTreeMap::new();
    let mut decorations: BTreeMap<u32, (Option<u32>, Option<u32>)> = BTreeMap::new();
    let mut non_uniform = Vec::new();
    for inst in &module.annotations {
        if inst.class.opcode == Op::Decorate
            && let Some(target) = inst.operands.first().and_then(id)
        {
            match inst.operands.get(1) {
                Some(Operand::Decoration(Decoration::DescriptorSet)) => {
                    decorations.entry(target).or_default().0 =
                        inst.operands.get(2).and_then(literal)
                }
                Some(Operand::Decoration(Decoration::Binding)) => {
                    decorations.entry(target).or_default().1 =
                        inst.operands.get(2).and_then(literal)
                }
                Some(Operand::Decoration(Decoration::NonUniform)) => non_uniform.push(target),
                _ => {}
            }
        }
    }
    for inst in &mut module.types_global_values {
        if matches!(inst.class.opcode, Op::TypePointer | Op::Variable)
            && matches!(
                inst.operands.first(),
                Some(Operand::StorageClass(
                    StorageClass::Uniform | StorageClass::PushConstant
                ))
            )
        {
            // Push-pointer types are converted below after its variable is found.
            if matches!(
                inst.operands.first(),
                Some(Operand::StorageClass(StorageClass::Uniform))
            ) {
                inst.operands[0] = Operand::StorageClass(StorageClass::StorageBuffer);
            }
        }
        if let Some(result) = inst.result_id {
            definitions.insert(result, inst.clone());
        }
    }
    let original_push = module
        .types_global_values
        .iter()
        .find(|inst| {
            inst.class.opcode == Op::Variable
                && matches!(
                    inst.operands.first(),
                    Some(Operand::StorageClass(StorageClass::PushConstant))
                )
        })
        .cloned();
    let next = module
        .header
        .as_ref()
        .ok_or_else(|| error("missing SPIR-V header"))?
        .bound;
    let value_types = module
        .all_inst_iter()
        .filter_map(|inst| Some((inst.result_id?, inst.result_type?)))
        .collect();
    let integer_types = definitions
        .values()
        .filter(|inst| inst.class.opcode == Op::TypeInt)
        .filter_map(|inst| {
            Some((
                inst.result_id?,
                (
                    literal(inst.operands.first()?)?,
                    literal(inst.operands.get(1)?)?,
                ),
            ))
        })
        .collect();
    let mut lower = Lowerer {
        module,
        next,
        types: Vec::new(),
        constants: BTreeMap::new(),
        pointers: BTreeMap::new(),
        uint: 0,
        push: 0,
        parameters: 0,
        indices_member: 0,
        resources: BTreeMap::new(),
        non_uniform,
        value_types,
        integer_types,
    };
    lower.uint = definitions
        .values()
        .find(|inst| {
            inst.class.opcode == Op::TypeInt
                && inst.operands == [Operand::LiteralInt32(32), Operand::LiteralInt32(0)]
        })
        .and_then(|inst| inst.result_id)
        .unwrap_or(0);
    if lower.uint == 0 {
        lower.uint = lower.ty(
            Op::TypeInt,
            vec![Operand::LiteralInt32(32), Operand::LiteralInt32(0)],
        );
    }
    let base = abi.original_push_bytes.div_ceil(4) * 4;
    let count = lower.constant(((abi.parameter_bytes - base) / 4).max(1));
    let index_array = lower.ty(
        Op::TypeArray,
        vec![Operand::IdRef(lower.uint), Operand::IdRef(count)],
    );
    lower.decorate(
        index_array,
        Decoration::ArrayStride,
        vec![Operand::LiteralInt32(4)],
    );
    let (parameter_struct, first_member) = if let Some(push) = &original_push {
        let pointer = definitions
            .get(
                &push
                    .result_type
                    .ok_or_else(|| error("push pointer has no type"))?,
            )
            .ok_or_else(|| error("missing push pointer type"))?;
        let structure = pointer
            .operands
            .get(1)
            .and_then(id)
            .ok_or_else(|| error("invalid push pointer"))?;
        let definition = definitions
            .get(&structure)
            .filter(|i| i.class.opcode == Op::TypeStruct)
            .ok_or_else(|| error("push block is not a struct"))?;
        let mut fields = definition.operands.clone();
        fields.push(Operand::IdRef(index_array));
        let cloned = lower.ty(Op::TypeStruct, fields);
        lower.decorate(cloned, Decoration::Block, vec![]);
        let members = lower
            .module
            .annotations
            .iter()
            .filter(|inst| {
                inst.class.opcode == Op::MemberDecorate
                    && inst.operands.first().and_then(id) == Some(structure)
            })
            .cloned()
            .collect::<Vec<_>>();
        for mut annotation in members {
            annotation.operands[0] = Operand::IdRef(cloned);
            lower.module.annotations.push(annotation);
        }
        (cloned, definition.operands.len() as u32)
    } else {
        let structure = lower.ty(Op::TypeStruct, vec![Operand::IdRef(index_array)]);
        lower.decorate(structure, Decoration::Block, vec![]);
        (structure, 0)
    };
    lower.indices_member = first_member;
    lower.member_offset(parameter_struct, first_member, base);
    let parameter_array = lower.ty(Op::TypeRuntimeArray, vec![Operand::IdRef(parameter_struct)]);
    let parameter_pointer = lower.pointer(StorageClass::StorageBuffer, parameter_array);
    lower.parameters = original_push
        .as_ref()
        .and_then(|p| p.result_id)
        .unwrap_or_else(|| lower.fresh());
    let push_struct = lower.ty(Op::TypeStruct, vec![Operand::IdRef(lower.uint)]);
    lower.decorate(push_struct, Decoration::Block, vec![]);
    lower.member_offset(push_struct, 0, 0);
    let push_pointer = lower.pointer(StorageClass::PushConstant, push_struct);
    lower.push = lower.fresh();
    let mut globals = Vec::new();
    let mut body_types = Vec::new();
    for mut inst in std::mem::take(&mut lower.module.types_global_values) {
        if inst.class.opcode != Op::Variable {
            if inst.class.opcode == Op::TypePointer
                && matches!(
                    inst.operands.first(),
                    Some(Operand::StorageClass(StorageClass::PushConstant))
                )
            {
                inst.operands[0] = Operand::StorageClass(StorageClass::StorageBuffer);
            }
            body_types.push(inst);
            continue;
        }
        let variable = inst
            .result_id
            .ok_or_else(|| error("variable has no result id"))?;
        if original_push.as_ref().and_then(|p| p.result_id) == Some(variable) {
            inst.result_type = Some(parameter_pointer);
            inst.operands[0] = Operand::StorageClass(StorageClass::StorageBuffer);
            lower.resources.insert(
                variable,
                Resource {
                    element: parameter_struct,
                    class: StorageClass::StorageBuffer,
                    slot: None,
                    array: false,
                },
            );
        } else if let Some((Some(set), Some(binding))) = decorations.get(&variable).copied() {
            let slot_index = abi
                .bindings
                .iter()
                .position(|slot| slot.location == BindingLocation { set, binding })
                .ok_or_else(|| error("SPIR-V descriptor is missing from the package contract"))?;
            let slot = &abi.bindings[slot_index];
            let binding = heap_binding(slot.kind)?;
            let pointer = definitions
                .get(
                    &inst
                        .result_type
                        .ok_or_else(|| error("resource has no pointer"))?,
                )
                .ok_or_else(|| error("missing resource pointer"))?;
            let pointee = pointer
                .operands
                .get(1)
                .and_then(id)
                .ok_or_else(|| error("invalid resource pointer"))?;
            let definition = definitions
                .get(&pointee)
                .ok_or_else(|| error("missing resource type"))?;
            let array = matches!(
                definition.class.opcode,
                Op::TypeArray | Op::TypeRuntimeArray
            );
            let element = if array {
                definition
                    .operands
                    .first()
                    .and_then(id)
                    .ok_or_else(|| error("invalid descriptor array"))?
            } else {
                pointee
            };
            if matches!(
                definitions.get(&element).map(|i| i.class.opcode),
                Some(Op::TypeArray | Op::TypeRuntimeArray)
            ) {
                return Err(error(
                    "multidimensional descriptor arrays require flattening by shader compilation",
                ));
            }
            let class = if binding == 2 {
                StorageClass::StorageBuffer
            } else {
                StorageClass::UniformConstant
            };
            let runtime = lower.ty(Op::TypeRuntimeArray, vec![Operand::IdRef(element)]);
            inst.result_type = Some(lower.pointer(class, runtime));
            inst.operands[0] = Operand::StorageClass(class);
            lower.resources.insert(
                variable,
                Resource {
                    element,
                    class,
                    slot: Some((slot.parameter_offset - base) / 4),
                    array,
                },
            );
            lower.decorate(
                variable,
                Decoration::DescriptorSet,
                vec![Operand::LiteralInt32(0)],
            );
            lower.decorate(
                variable,
                Decoration::Binding,
                vec![Operand::LiteralInt32(binding)],
            );
            lower.decorate(variable, Decoration::Aliased, vec![]);
        }
        globals.push(inst);
    }
    if original_push.is_none() {
        globals.push(instruction(
            Op::Variable,
            Some(parameter_pointer),
            Some(lower.parameters),
            vec![Operand::StorageClass(StorageClass::StorageBuffer)],
        ));
    }
    globals.push(instruction(
        Op::Variable,
        Some(push_pointer),
        Some(lower.push),
        vec![Operand::StorageClass(StorageClass::PushConstant)],
    ));
    // Replace original binding decorations, retaining the newly emitted ones.
    let mut seen_binding = BTreeMap::new();
    for (position, inst) in lower.module.annotations.iter().enumerate().rev() {
        if inst.class.opcode == Op::Decorate
            && let (Some(target), Some(Operand::Decoration(decoration))) =
                (inst.operands.first().and_then(id), inst.operands.get(1))
            && matches!(decoration, Decoration::DescriptorSet | Decoration::Binding)
            && lower.resources.contains_key(&target)
        {
            seen_binding
                .entry((target, *decoration as u32))
                .or_insert(position);
        }
    }
    lower.module.annotations = lower
        .module
        .annotations
        .drain(..)
        .enumerate()
        .filter_map(|(position, mut inst)| {
            if inst.class.opcode == Op::Decorate
                && let (Some(target), Some(Operand::Decoration(decoration))) =
                    (inst.operands.first().and_then(id), inst.operands.get(1))
            {
                if *decoration == Decoration::BufferBlock {
                    inst.operands[1] = Operand::Decoration(Decoration::Block);
                } else if lower.resources.contains_key(&target)
                    && (*decoration == Decoration::Restrict
                        || matches!(decoration, Decoration::DescriptorSet | Decoration::Binding)
                            && seen_binding.get(&(target, *decoration as u32)).copied()
                                != Some(position))
                {
                    return None;
                }
            }
            Some(inst)
        })
        .collect();
    lower.decorate(
        lower.parameters,
        Decoration::DescriptorSet,
        vec![Operand::LiteralInt32(0)],
    );
    lower.decorate(
        lower.parameters,
        Decoration::Binding,
        vec![Operand::LiteralInt32(2)],
    );
    lower.decorate(lower.parameters, Decoration::Aliased, vec![]);
    lower.rewrite()?;
    for capability in [
        Capability::RuntimeDescriptorArray,
        Capability::StorageBufferArrayDynamicIndexing,
        Capability::SampledImageArrayDynamicIndexing,
        Capability::StorageImageArrayDynamicIndexing,
    ] {
        if !lower
            .module
            .capabilities
            .iter()
            .any(|i| i.operands == [Operand::Capability(capability)])
        {
            lower.module.capabilities.push(instruction(
                Op::Capability,
                None,
                None,
                vec![Operand::Capability(capability)],
            ));
        }
    }
    if !lower
        .module
        .extensions
        .iter()
        .any(|i| i.operands == [Operand::LiteralString("SPV_EXT_descriptor_indexing".into())])
    {
        lower.module.extensions.push(instruction(
            Op::Extension,
            None,
            None,
            vec![Operand::LiteralString("SPV_EXT_descriptor_indexing".into())],
        ));
    }
    let header = lower.module.header.as_mut().unwrap();
    header.version = header.version.max(0x0001_0300);
    header.bound = lower.next;
    if header.version >= 0x0001_0400 {
        for entry in &mut lower.module.entry_points {
            for variable in [lower.parameters, lower.push] {
                if !entry.operands.contains(&Operand::IdRef(variable)) {
                    entry.operands.push(Operand::IdRef(variable));
                }
            }
        }
    }
    body_types.append(&mut lower.types);
    body_types.append(&mut globals);
    lower.module.types_global_values = body_types;
    Ok(CompiledShaderStage {
        spirv: lower.module.assemble(),
        ..stage.clone()
    })
}

pub fn lower_pipeline(
    pipeline: &CompiledShaderPipeline,
) -> Result<(BindlessAbi, Vec<CompiledShaderStage>), LoweringError> {
    lower_pipeline_with_limit(pipeline, 64)
}
pub fn lower_pipeline_with_limit(
    pipeline: &CompiledShaderPipeline,
    runtime_array_limit: u32,
) -> Result<(BindlessAbi, Vec<CompiledShaderStage>), LoweringError> {
    let abi = BindlessAbi::from_pipeline_with_limit(pipeline, runtime_array_limit)?;
    let stages = pipeline
        .stages
        .iter()
        .map(|stage| lower_stage(stage, &abi))
        .collect::<Result<_, _>>()?;
    Ok((abi, stages))
}
