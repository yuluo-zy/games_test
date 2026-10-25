//! D2 编译检查脚手架（仅声明 ui_systems + 依赖的共享模块；不入 workspace）。
pub mod apply_decorator_op;
pub mod backup;
pub mod blueprint;
pub mod cull_oob_decorators;
pub mod decorator_resources;
pub mod deferred_decorator_history;
pub mod derive_decorator_subtype;
pub mod merge_proposal_wall_decorators;
pub mod ui_systems;
