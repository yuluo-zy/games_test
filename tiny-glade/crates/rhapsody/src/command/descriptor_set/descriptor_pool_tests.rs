use super::*;

#[test]
fn layout_rejects_invalid_input_as_usage_error_with_details() {
    let binding = Binding {
        tag: 0,
        name: "buffer".into(),
        runtime: false,
        count: 0,
        sampler: 0,
    };
    let error = descriptor_set_layout(
        std::slice::from_ref(&binding),
        0,
        vk::ShaderStageFlags::COMPUTE,
        0,
    )
    .unwrap_err();
    assert_eq!(error.kind(), crate::ErrorKind::InvalidUsage);
    assert!(matches!(
        std::error::Error::source(&error).and_then(|source| source.downcast_ref::<Error>()),
        Some(Error::ZeroDescriptorCount)
    ));
    let valid = Binding {
        count: 1,
        ..binding
    };
    let error = descriptor_set_layout(
        &[valid.clone(), valid],
        u64::from(u32::MAX),
        vk::ShaderStageFlags::COMPUTE,
        0,
    )
    .unwrap_err();
    assert_eq!(error.kind(), crate::ErrorKind::InvalidUsage);
}
