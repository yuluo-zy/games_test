//! compute 管线反射的两个前置检查（group size 必须存在、inputs 必须为空）。
use rhapsody::raw::shader::{InterfaceError, validate_compute_reflection};
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
fn compute_reflection_rejects_missing_group_size_and_vertex_inputs() {
    assert_eq!(validate_compute_reflection(&empty()).unwrap(), [8, 8, 1]);
    let mut r = empty();
    r.local_size = None;
    assert_eq!(validate_compute_reflection(&r).unwrap_err(), InterfaceError::MissingGroupSize);
    r.vertex_inputs.push(("a".into(), VertexInput { location: 0, ty: Type::Scalar(ScalarType::Float(4)) }));
    let err = validate_compute_reflection(&r).unwrap_err();
    assert_eq!(err, InterfaceError::ComputeHasVertexInputs);
    assert_eq!(rhapsody::Error::from(err).kind(), rhapsody::ErrorKind::InvalidUsage);
}
