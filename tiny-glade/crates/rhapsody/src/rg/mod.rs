pub mod access_ledger;
pub mod attachments;
pub mod binding;
pub mod cache;
pub mod compiled;
pub mod executor;
pub mod frame;
pub mod frame_imports;
pub mod frame_storage;
pub mod frame_throttle;
pub mod graph;
pub mod import_export;
pub mod parallel;
pub mod plan;
pub mod profiling;
pub mod queue_schedule;
pub mod sync_plan;
pub mod plan_check;
pub mod pass_api;
pub mod pass_bindings;
pub mod pass_builder;
pub mod pass_entries;
pub mod recorder;
pub mod receipt_storage;
pub mod owner_storage;
pub mod resource;
pub mod resource_contracts;
pub mod resource_info;
pub mod resource_pool;
pub mod resource_registry;
pub mod result_cache;
pub mod resource_usage;
pub mod subgraph;
pub mod temporal;
pub mod transient_memory;
pub mod transient_memory_check;
mod transient_resources;
pub mod util;

pub mod resource_rules;

pub mod vulkan;

pub mod transaction;
pub mod frame_loop;

pub mod indirect_count;

static PLAN_VALIDATION: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Enable the independent plan verifiers ([`plan_check`],
/// [`transient_memory_check`]) on every plan-cache miss in builds where they are
/// not already on. Debug builds and the `validation` feature always run them.
pub fn set_plan_validation(enabled: bool) {
    PLAN_VALIDATION.store(enabled, std::sync::atomic::Ordering::Relaxed);
}

/// Whether plan compilation re-verifies its output with the independent checkers.
pub fn validation_enabled() -> bool {
    cfg!(any(debug_assertions, feature = "validation"))
        || PLAN_VALIDATION.load(std::sync::atomic::Ordering::Relaxed)
}
