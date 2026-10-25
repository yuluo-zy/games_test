//! CPU checks for the GPU validation gate; no Vulkan loader is invoked.
mod gpu_test_support;

#[test]
fn required_gpu_initialization_failure_is_an_error_instead_of_a_skip() {
    let panic = std::panic::catch_unwind(|| {
        gpu_test_support::initialize_with_requirement::<(), _>(
            true,
            "injected device creation",
            Err("injected initialization error"),
        )
    })
    .expect_err("required GPU validation must not silently skip");
    let message = panic
        .downcast_ref::<String>()
        .expect("formatted diagnostic");
    assert!(message.contains("RHAPSODY_REQUIRE_VULKAN=1"));
    assert!(message.contains("injected device creation"));
    assert!(message.contains("injected initialization error"));
}

#[test]
fn optional_gpu_initialization_can_skip_and_success_is_unchanged_in_both_modes() {
    assert_eq!(
        gpu_test_support::initialize::<_, &str>("successful CPU fixture", Ok(7)),
        Some(7)
    );
    assert_eq!(
        gpu_test_support::initialize_with_requirement::<(), _>(
            false,
            "injected missing loader",
            Err("not installed"),
        ),
        None
    );
    for required in [false, true] {
        assert_eq!(
            gpu_test_support::initialize_with_requirement::<_, &str>(
                required,
                "injected successful initialization",
                Ok(7),
            ),
            Some(7)
        );
    }
}
