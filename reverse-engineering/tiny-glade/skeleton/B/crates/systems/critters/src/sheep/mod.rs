// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/critters/src/sheep/mod.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: be7f1f2ce844a96a0a92aab5ee5eee2cef98cfdbccd54731ff652f0612cd8bc3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 332
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":332,"record_offset":264,"rva":6805520,"va":5375514640}
// 编译实例 RVAs: 0x67d810
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$bevy_ecs..schedule..config..ScheduleConfigs$LT$T$GT$$u20$as$u20$bevy_ecs..schedule..config..IntoScheduleConfigs$LT$T$C$$LP$$RP$$GT$$GT$::run_if
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9445315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375514640,"raw_hex":"e8c31f9000","size":5,"soft_fail":false,"target_va":5384960000,"va":5375514680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=3
//     target_aliases=19 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5898616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375514640,"raw_hex":"e888fea5ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5375514723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9448920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375514640,"raw_hex":"e8d82d9000","size":5,"soft_fail":false,"target_va":5384963616,"va":5375514691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::registry::EventRegistry::register_event
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9528360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375514640,"raw_hex":"e828649100","size":5,"soft_fail":false,"target_va":5385043152,"va":5375514787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9537683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375514640,"raw_hex":"e893889100","size":5,"soft_fail":false,"target_va":5385052352,"va":5375514664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] bevy_ecs::event::registry::EventRegistry::register_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t9526571","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375514640,"raw_hex":"e92b5d9100","size":5,"soft_fail":false,"target_va":5385041568,"va":5375514992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":332,"record_offset":420,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":9230,"source_line":358,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":332,"record_offset":392,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":332,"record_offset":872,"parent_record_offset":848,"ancestor_record_offset":264,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":332,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":264,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":332,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":264,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":332,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":264,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":332,"record_offset":824,"parent_record_offset":792,"ancestor_record_offset":264,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":332,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":264,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs
//     evidence={"module":332,"record_offset":444,"parent_record_offset":420,"ancestor_record_offset":264,"inlinee":9217,"source_line":339,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":332,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":264,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":332,"record_offset":1040,"parent_record_offset":1016,"ancestor_record_offset":264,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":332,"record_offset":548,"parent_record_offset":528,"ancestor_record_offset":264,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":332,"record_offset":596,"parent_record_offset":572,"ancestor_record_offset":264,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":332,"record_offset":572,"parent_record_offset":548,"ancestor_record_offset":264,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":332,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":264,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":332,"record_offset":472,"parent_record_offset":444,"ancestor_record_offset":264,"inlinee":9218,"source_line":1825,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":332,"record_offset":528,"parent_record_offset":504,"ancestor_record_offset":264,"inlinee":9220,"source_line":1828,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":332,"record_offset":684,"parent_record_offset":624,"ancestor_record_offset":264,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":332,"record_offset":504,"parent_record_offset":472,"ancestor_record_offset":264,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":332,"record_offset":704,"parent_record_offset":684,"ancestor_record_offset":264,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":332,"record_offset":992,"parent_record_offset":472,"ancestor_record_offset":264,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":332,"record_offset":744,"parent_record_offset":572,"ancestor_record_offset":264,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":332,"record_offset":624,"parent_record_offset":596,"ancestor_record_offset":264,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":332,"record_offset":656,"parent_record_offset":624,"ancestor_record_offset":264,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// === 完整可用 TPI 类型引用图（包含参数表、返回类型、字段等实际记录） ===
// TPI 0x1000:
// ---
// Kind:            LF_ARGLIST
// ArgList:
//   ArgIndices:      [  ]
// ...
// TPI 0x1001:
// ---
// Kind:            LF_PROCEDURE
// Procedure:
//   ReturnType:      3
//   CallConv:        NearC
//   Options:         [ None ]
//   ParameterCount:  0
//   ArgumentList:    4096
// ...

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_critters::sheep::SheepPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$12::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::decorator_plugin::DecoratorPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::backend::icc::IccProfilePlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::CritterPlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::sheep::SheepPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::gpu_lut::GpuLutPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::components::drawable::PrevTransformPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::render::fsr_plugin::plugin_impl::FsrPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::dlss_plugin::plugin_impl::DlssPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::rollercoaster::RollercoasterPlugin>", "constructor": "bevy_app::plugin::sealed::impl$12::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::decorator_plugin::DecoratorPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::backend::icc::IccProfilePlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::CritterPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::sheep::SheepPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::gpu_lut::GpuLutPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::components::drawable::PrevTransformPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::render::fsr_plugin::plugin_impl::FsrPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::dlss_plugin::plugin_impl::DlssPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::rollercoaster::RollercoasterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<system_critters::sheep::SheepPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["system_critters::sheep::SheepPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<system_critters::sheep::SheepPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["system_critters::sheep::SheepPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<system_critters::sheep::SheepPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["system_critters::sheep::SheepPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<system_critters::sheep::SheepPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["system_critters::sheep::SheepPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<system_critters::sheep::SheepPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["system_critters::sheep::SheepPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<system_critters::sheep::SheepPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["system_critters::sheep::SheepPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<system_critters::sheep::SheepPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["system_critters::sheep::SheepPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<system_critters::sheep::SheepPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["system_critters::sheep::SheepPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_critters::sheep::impl$0::build
// 信息 ID: deb8f8f49a8f1be4c41e5d1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
