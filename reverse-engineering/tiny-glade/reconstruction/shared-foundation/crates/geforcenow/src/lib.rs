//! B crates/geforcenow/src/lib.rs: probe the vendor runtime, then release it.
//! SDK implementation is supplied by the original dynamic library.

use thiserror::Error;

pub const RUNTIME_LIBRARY: &str = "GfnRuntimeSdk.dll";
pub const INITIALIZE_SYMBOL: &[u8] = b"gfnInitializeRuntimeSdk";
pub const SHUTDOWN_SYMBOL: &[u8] = b"gfnShutdownRuntimeSdk";
pub const CLOUD_SYMBOL: &[u8] = b"gfnIsRunningInCloud";

/// The original probe preserves the SDK's signed initialization result.
/// SDK enum names are not reconstructed from debug formatting strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GfnInternalError(pub i32);

impl std::fmt::Display for GfnInternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GeForce NOW SDK initialization failed with code {}",
            self.0
        )
    }
}

#[derive(Debug, Error)]
pub enum GfnError {
    #[error(transparent)]
    Library(#[from] libloading::Error),
    #[error("{0}")]
    Internal(GfnInternalError),
}

/// Injectable boundary for the original three SDK calls.
pub trait CloudRuntime {
    fn initialize(&self, language: i32) -> i32;
    fn is_running_in_cloud(&self) -> bool;
    fn shutdown(&self);
}

/// Original B RVA 0x61e660: initialize(0), reject every nonzero result,
/// query cloud state only after success, then shutdown once. Shutdown's SDK
/// result is discarded. Initialization failure never calls shutdown.
pub fn probe_runtime(runtime: &impl CloudRuntime) -> Result<bool, GfnInternalError> {
    let result = runtime.initialize(0);
    if result != 0 {
        return Err(GfnInternalError(result));
    }
    let in_cloud = runtime.is_running_in_cloud();
    runtime.shutdown();
    Ok(in_cloud)
}

/// Load the same DLL and all three exports before initializing it.
/// Missing DLLs/exports propagate libloading errors, as in the original.
pub fn is_running_in_cloud() -> Result<bool, GfnError> {
    // SAFETY: fixed vendor DLL and the original x64 SDK API contract.
    unsafe { probe_runtime_library(std::ffi::OsStr::new(RUNTIME_LIBRARY)) }
}

/// Probe an explicitly selected runtime, for vendor/fixture integration.
///
/// # Safety
/// The loaded library must supply the original three SDK signatures; loading
/// and calling its exports must be safe in the current process.
pub unsafe fn probe_runtime_library(path: &std::ffi::OsStr) -> Result<bool, GfnError> {
    // SAFETY: the vendor SDK exports below have the signatures established at
    // B 0x14061e729..0x14061e782. Library owns them through the entire probe.
    unsafe {
        let library = libloading::Library::new(path)?;
        let initialize: libloading::Symbol<'_, unsafe extern "C" fn(i32) -> i32> =
            library.get(INITIALIZE_SYMBOL)?;
        let shutdown: libloading::Symbol<'_, unsafe extern "C" fn() -> i32> =
            library.get(SHUTDOWN_SYMBOL)?;
        let cloud: libloading::Symbol<'_, unsafe extern "C" fn() -> bool> =
            library.get(CLOUD_SYMBOL)?;
        struct Loaded<'a> {
            initialize: libloading::Symbol<'a, unsafe extern "C" fn(i32) -> i32>,
            shutdown: libloading::Symbol<'a, unsafe extern "C" fn() -> i32>,
            cloud: libloading::Symbol<'a, unsafe extern "C" fn() -> bool>,
        }
        impl CloudRuntime for Loaded<'_> {
            fn initialize(&self, language: i32) -> i32 {
                // SAFETY: signature and library lifetime are established above.
                unsafe { (self.initialize)(language) }
            }
            fn is_running_in_cloud(&self) -> bool {
                // SAFETY: signature and library lifetime are established above.
                unsafe { (self.cloud)() }
            }
            fn shutdown(&self) {
                // SAFETY: initialized SDK and library lifetime established above.
                let _ = unsafe { (self.shutdown)() };
            }
        }
        probe_runtime(&Loaded {
            initialize,
            shutdown,
            cloud,
        })
        .map_err(GfnError::Internal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct Runtime {
        code: i32,
        cloud: bool,
        calls: RefCell<Vec<&'static str>>,
    }
    impl CloudRuntime for Runtime {
        fn initialize(&self, language: i32) -> i32 {
            assert_eq!(language, 0);
            self.calls.borrow_mut().push("initialize");
            self.code
        }
        fn is_running_in_cloud(&self) -> bool {
            self.calls.borrow_mut().push("query");
            self.cloud
        }
        fn shutdown(&self) {
            self.calls.borrow_mut().push("shutdown");
        }
    }

    #[test]
    fn successful_probe_releases_both_cloud_states() {
        for cloud in [false, true] {
            let runtime = Runtime {
                code: 0,
                cloud,
                calls: RefCell::new(Vec::new()),
            };
            assert_eq!(probe_runtime(&runtime), Ok(cloud));
            assert_eq!(*runtime.calls.borrow(), ["initialize", "query", "shutdown"]);
        }
    }

    #[test]
    fn nonzero_initialization_preserves_code_and_skips_query_shutdown() {
        for code in [1, -1, -2, -17, i32::MIN, i32::MAX] {
            let runtime = Runtime {
                code,
                cloud: true,
                calls: RefCell::new(Vec::new()),
            };
            assert_eq!(probe_runtime(&runtime), Err(GfnInternalError(code)));
            assert_eq!(*runtime.calls.borrow(), ["initialize"]);
        }
    }
    #[cfg(windows)]
    #[test]
    #[ignore = "build evidence/feedback-cloud/fixtures/build.ps1 first"]
    fn native_dll_fixture_runs_real_libloading_and_preserves_call_order() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/fixtures/GfnFixtureRuntime.dll");
        // SAFETY: the owned fixture is compiled from retained gfn_runtime.c.
        unsafe {
            let fixture = libloading::Library::new(&path).unwrap();
            let set: libloading::Symbol<'_, unsafe extern "C" fn(i32, u8)> =
                fixture.get(b"gfnFixtureSet").unwrap();
            let sequence: libloading::Symbol<'_, unsafe extern "C" fn() -> i32> =
                fixture.get(b"gfnFixtureSequence").unwrap();
            for code in [0, 1, -1, -2, -17, i32::MIN, i32::MAX] {
                for cloud in [false, true] {
                    set(code, u8::from(cloud));
                    let result = probe_runtime_library(path.as_os_str());
                    if code == 0 {
                        assert_eq!(result.unwrap(), cloud);
                        assert_eq!(sequence(), 123);
                    } else {
                        assert!(
                            matches!(result, Err(GfnError::Internal(GfnInternalError(value))) if value == code)
                        );
                        assert_eq!(sequence(), 1);
                    }
                }
            }
        }
    }
}
