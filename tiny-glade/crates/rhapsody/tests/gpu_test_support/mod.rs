//! Test-only initialization policy. GPU validation jobs set
//! RHAPSODY_REQUIRE_VULKAN=1 so unavailable/broken initialization cannot look like
//! a passing GPU test. Developer machines may still explicitly report SKIP.
pub fn initialize<T, E: std::fmt::Display>(stage: &str, result: Result<T, E>) -> Option<T> {
    let required = std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|value| value == "1");
    initialize_with_requirement(required, stage, result)
}

pub fn initialize_with_requirement<T, E: std::fmt::Display>(
    required: bool,
    stage: &str,
    result: Result<T, E>,
) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(error) => {
            assert!(
                !required,
                "RHAPSODY_REQUIRE_VULKAN=1: GPU initialization failed at {stage}: {error}"
            );
            eprintln!("SKIP Vulkan initialization at {stage}: {error}");
            None
        }
    }
}
