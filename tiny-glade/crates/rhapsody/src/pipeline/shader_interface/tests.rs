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
    let reflected = crate::pipeline::shader::reflect_spirv_entries(&words).unwrap();
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
    let all = crate::pipeline::shader::reflect_spirv_entries(&words).unwrap();
    assert_eq!(all[0].descriptors.len(), 2);
    let selected = contracts(&words).unwrap();
    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].descriptors.len(), 1);
    assert_eq!(selected[0].descriptors[0].binding, 0);
    assert_eq!(selected[0].pushes, vec![(0, 4)]);
    assert!(!selected[0].schema.is_empty());
}
