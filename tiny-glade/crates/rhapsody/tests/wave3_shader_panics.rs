//! 管线创建路径的原 panic 文案（`.rne` 0x1416a3720 raster / 0x1416a4f1a compute 冷尾段）与两个 compute 检查。
use rhapsody::raw::shader::{original_panics as op, validate_compute_reflection, InterfaceError};
use rhapsody::shader_pipeline::reflection::{ShaderReflection, VertexInput};
use rhapsody::shader_pipeline::reflection_ty::{ScalarType, Type};

fn empty() -> ShaderReflection {
    ShaderReflection {
        descriptor_sets: vec![],
        uniform_block: None,
        binding_names: vec![],
        binding_name_hashes: vec![],
        vertex_inputs: vec![],
        local_size: Some([8, 8, 1]),
        push_constant_size: 0,
    }
}

#[test]
fn compute_checks_follow_assert_then_expect_order() {
    assert_eq!(validate_compute_reflection(&empty()).unwrap(), [8, 8, 1]);
    let mut r = empty();
    r.local_size = None;
    assert_eq!(validate_compute_reflection(&r).unwrap_err().to_string(), "a valid group size");
    // 两个都不满足时先报 assert（机器码顺序 397 → 460）。
    r.vertex_inputs.push(("a".into(), VertexInput { location: 0, ty: Type::Scalar(ScalarType::Float(4)) }));
    assert_eq!(
        validate_compute_reflection(&r).unwrap_err().to_string(),
        "assertion failed: compiled.reflection.inputs.is_empty()"
    );
}

#[test]
fn vertex_attribute_panic_texts_are_verbatim() {
    assert_eq!(InterfaceError::VertexInputByteCount.to_string(), "vertex input attrib byte count");
    assert_eq!(
        InterfaceError::UnsupportedVertexAttributeType("Sampler".into()).to_string(),
        "Unsupported vertex attribute type: Sampler"
    );
    assert_eq!(op::UNABLE_TO_CREATE_GRAPHICS_PIPELINE.len(), 34);
    assert_eq!(op::VERTEX_INPUT_ATTRIB_BYTE_COUNT.len(), 30);
    assert_eq!(op::COMPUTE_INPUTS_NOT_EMPTY.len(), 55);
    assert_eq!(op::VALID_GROUP_SIZE.len(), 18);
    assert_eq!(op::UNWRAP_ON_ERR.len(), 43);
    assert_eq!(
        op::failed_creating_shader_pipeline("dof/fullres.cs.hlsl", &"oops"),
        "Failed creating shader pipeline dof/fullres.cs.hlsl: \"oops\""
    );
}
