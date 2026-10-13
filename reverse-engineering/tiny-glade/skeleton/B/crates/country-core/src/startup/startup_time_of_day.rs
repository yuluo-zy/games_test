// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/startup/startup_time_of_day.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d2dd9d67a6a426d0b94c3e1145d056a6b01fbd8d6a19796a85c1f237ec17f303
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_time_of_day.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 486, 570
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_time_of_day::startup_time_of_day
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":486,"record_offset":38672,"rva":9033456,"va":5377742576}
// 编译实例 RVAs: 0x89d6f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: startup_time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2842248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377742576,"raw_hex":"e878a1d4ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5377742979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: startup_time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2838536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377742576,"raw_hex":"e8f8afd4ff","size":5,"soft_fail":false,"target_va":5374904480,"va":5377743011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: startup_time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2661225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377742576,"raw_hex":"e89764d7ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377742724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414051e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11958942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377742576,"raw_hex":"e89e7ab600","size":5,"soft_fail":false,"target_va":5389701600,"va":5377742653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33800050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377742576,"raw_hex":"e872bf0302","size":5,"soft_fail":false,"target_va":5411542880,"va":5377742825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4962852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377742576,"raw_hex":"e824ba4b00","size":5,"soft_fail":false,"target_va":5382705472,"va":5377742615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: startup_time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-2661270","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377742576,"raw_hex":"e96a64d7ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377742769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":486,"record_offset":38984,"parent_record_offset":38672,"ancestor_record_offset":38672,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":486,"record_offset":39040,"parent_record_offset":38672,"ancestor_record_offset":38672,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":486,"record_offset":38944,"parent_record_offset":38672,"ancestor_record_offset":38672,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_time_of_day::load_default_tod
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_time_of_day.rs
//     evidence={"module":486,"record_offset":38832,"parent_record_offset":38672,"ancestor_record_offset":38672,"inlinee":26957,"source_line":10,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":39236,"parent_record_offset":39212,"ancestor_record_offset":38672,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":39212,"parent_record_offset":39124,"ancestor_record_offset":38672,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":39152,"parent_record_offset":39124,"ancestor_record_offset":38672,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":39124,"parent_record_offset":39100,"ancestor_record_offset":38672,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":39100,"parent_record_offset":39080,"ancestor_record_offset":38672,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":486,"record_offset":39008,"parent_record_offset":38984,"ancestor_record_offset":38672,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":486,"record_offset":39180,"parent_record_offset":39152,"ancestor_record_offset":38672,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":486,"record_offset":39060,"parent_record_offset":39040,"ancestor_record_offset":38672,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":486,"record_offset":38892,"parent_record_offset":38832,"ancestor_record_offset":38672,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":486,"record_offset":38864,"parent_record_offset":38832,"ancestor_record_offset":38672,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_time_of_day::load_default_tod
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x694D
// 首条位置: {"module":486,"inlinee":26957}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":486,"record_offset":38892,"parent_record_offset":38832,"ancestor_record_offset":38832,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":486,"record_offset":38864,"parent_record_offset":38832,"ancestor_record_offset":38832,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/00486.jsonl.gz"}
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
// 完整结构及原始类型记录: startup_time_of_day.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: startup_time_of_day.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: startup_time_of_day.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_time_of_day::load_default_tod
// 信息 ID: a8763463531599477483f2c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_time_of_day::startup_time_of_day; module=486, inline_record=38832; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_glade_tod_and_weather::load_glade_tod_and_weather; module=570, inline_record=24056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_time_of_day::startup_time_of_day
// 信息 ID: 859f8ecbfbb5ce5330cd6177
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fe679a79fd8758296facd6e6: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>)> > @ 0xa4c040 -> 0x89d6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>)> > > @ 0x14c6dee -> 0x89d6f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
