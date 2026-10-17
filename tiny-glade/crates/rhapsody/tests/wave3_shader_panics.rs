//! compute 管线反射的两个前置检查（group size 必须存在、inputs 必须为空）。
use rhapsody::raw::shader::validate_compute_reflection;
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
    assert_eq!(
        validate_compute_reflection(&r).unwrap_err().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    assert_eq!(
        std::error::Error::source(&validate_compute_reflection(&r).unwrap_err())
            .unwrap()
            .to_string(),
        "a valid group size"
    );
    // 两个都不满足时先报 assert（机器码顺序 397 → 460）。
    r.vertex_inputs.push((
        "a".into(),
        VertexInput {
            location: 0,
            ty: Type::Scalar(ScalarType::Float(4)),
        },
    ));
    assert_eq!(
        validate_compute_reflection(&r).unwrap_err().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    assert_eq!(
        std::error::Error::source(&validate_compute_reflection(&r).unwrap_err())
            .unwrap()
            .to_string(),
        "assertion failed: compiled.reflection.inputs.is_empty()"
    );
}
