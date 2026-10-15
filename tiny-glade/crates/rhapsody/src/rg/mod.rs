pub mod access_ledger;
pub mod attachments;
pub mod binding;
pub mod compiled;
// 既有已恢复 CPU 叶 API 保留兼容导出。
pub mod compiled_graph;
pub mod executor;
pub mod frame;
pub mod frame_throttle;
pub mod graph;
pub mod import_export;
pub mod parallel;
pub mod pass_api;
pub mod pass_bindings;
pub mod pass_builder;
pub mod pass_entries;
pub mod recorder;
pub mod resource;
pub mod resource_info;
pub mod resource_pool;
pub mod resource_registry;
pub mod resource_usage;
pub mod state;
pub mod temporal;
pub mod util;

pub mod resource_rules;

pub mod vulkan;
