// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/utils/load_json.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 6e5a848aef3a64b14d0f4ffdc6bdd16818708104c19ffe2a8e518cee5d54e532
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_json.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 426, 1308, 1321, 1394, 2281, 2756, 2761, 3537, 4260, 4370
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh<alloc::string::String>
// 记录形式: compiled_procedure; instances=2
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":426,"record_offset":47820,"rva":5998624,"va":5374707744}
// 编译实例 RVAs: 0x5b8820, 0x1e64a60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$str$u20$as$u20$serde_json..value..index..Index$GT$::index_into
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t10831090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e8f244a500","size":5,"soft_fail":false,"target_va":5385539984,"va":5374708889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2396184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e8e86fdbff","size":5,"soft_fail":false,"target_va":5372314208,"va":5374710387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2389816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e8c888dbff","size":5,"soft_fail":false,"target_va":5372320512,"va":5374710323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2083700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e88c34e0ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5374708719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1959296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e8801ae2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374708987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=20
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1959137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e81f1be2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374708844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140546de0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-468648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e858d9f8ff","size":5,"soft_fail":false,"target_va":5374242272,"va":5374710915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bf340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e8f7640000","size":5,"soft_fail":false,"target_va":5374735168,"va":5374709316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1182163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e8d3091200","size":5,"soft_fail":false,"target_va":5375891872,"va":5374709704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e2f30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1220633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e819a01200","size":5,"soft_fail":false,"target_va":5375930160,"va":5374709522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1225259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e82bb21200","size":5,"soft_fail":false,"target_va":5375936112,"va":5374710848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14084b050
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2696705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e801262900","size":5,"soft_fail":false,"target_va":5377405008,"va":5374708298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140852b50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2727680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e8009f2900","size":5,"soft_fail":false,"target_va":5377436496,"va":5374708811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d02200
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18230995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400578656,"raw_hex":"e82dd1e9fe","size":5,"soft_fail":false,"target_va":5382349312,"va":5400580302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d02460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18231621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400578656,"raw_hex":"e8bbcee9fe","size":5,"soft_fail":false,"target_va":5382349920,"va":5400581536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18215087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400578656,"raw_hex":"e8510feafe","size":5,"soft_fail":false,"target_va":5382365264,"va":5400580346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d43420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7907440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e870a87800","size":5,"soft_fail":false,"target_va":5382616096,"va":5374708651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d75540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8112013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e88dc77b00","size":5,"soft_fail":false,"target_va":5382821184,"va":5374709166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8599795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e8f3388300","size":5,"soft_fail":false,"target_va":5383308144,"va":5374708344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b07d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7030698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400578656,"raw_hex":"e856b894ff","size":5,"soft_fail":false,"target_va":5393549264,"va":5400579957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417da3b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6861368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400578656,"raw_hex":"e8c84d97ff","size":5,"soft_fail":false,"target_va":5393720240,"va":5400581603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141eea7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t546727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400578656,"raw_hex":"e8a7570800","size":5,"soft_fail":false,"target_va":5401126832,"va":5400580100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28268006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e8e655af01","size":5,"soft_fail":false,"target_va":5402978144,"va":5374710133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2653624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e8b87d2800","size":5,"soft_fail":false,"target_va":5377364336,"va":5374710707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28268522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e8ea57af01","size":5,"soft_fail":false,"target_va":5402978688,"va":5374710161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t28268996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e8c459af01","size":5,"soft_fail":false,"target_va":5402978880,"va":5374709879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28268958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e89e59af01","size":5,"soft_fail":false,"target_va":5402978944,"va":5374709981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-22226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e82ea9ffff","size":5,"soft_fail":false,"target_va":5374687520,"va":5374709741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28270275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e8c35eaf01","size":5,"soft_fail":false,"target_va":5402980128,"va":5374709848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-20127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e861b1ffff","size":5,"soft_fail":false,"target_va":5374687712,"va":5374707834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t28272740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e86468af01","size":5,"soft_fail":false,"target_va":5402980656,"va":5374707911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16012273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400578656,"raw_hex":"e80fac0bff","size":5,"soft_fail":false,"target_va":5384567376,"va":5400579644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::set_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16012309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400578656,"raw_hex":"e8ebab0bff","size":5,"soft_fail":false,"target_va":5384567584,"va":5400579888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::set_source_file
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16012187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400578656,"raw_hex":"e865ac0bff","size":5,"soft_fail":false,"target_va":5384567504,"va":5400579686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::load_json::read_attribute_from_json
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10945567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e81f04a700","size":5,"soft_fail":false,"target_va":5385654832,"va":5374709260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2645139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e8935c2800","size":5,"soft_fail":false,"target_va":5377353776,"va":5374708632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1648497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e871271900","size":5,"soft_fail":false,"target_va":5376356912,"va":5374708410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1652233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e809361900","size":5,"soft_fail":false,"target_va":5376360560,"va":5374708322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1648324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e8c4261900","size":5,"soft_fail":false,"target_va":5376356144,"va":5374707815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::from_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t542961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400578656,"raw_hex":"e8f1480800","size":5,"soft_fail":false,"target_va":5401122560,"va":5400579594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2697146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e8ba272900","size":5,"soft_fail":false,"target_va":5377405616,"va":5374708465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2657522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374707744,"raw_hex":"e8f28c2800","size":5,"soft_fail":false,"target_va":5377366224,"va":5374708697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18289469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400578656,"raw_hex":"e8c3ece8fe","size":5,"soft_fail":false,"target_va":5382289808,"va":5400579272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":53972,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":54060,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":426,"record_offset":51004,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":426,"record_offset":51032,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":426,"record_offset":52304,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":49732,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":54248,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":54176,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":50816,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":426,"record_offset":53832,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":51192,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":24287,"source_line":24,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":53416,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":24560,"source_line":48,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_source_file
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":51716,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":24704,"source_line":32,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":426,"record_offset":51060,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":426,"record_offset":50920,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":49768,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":48064,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":48640,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":426,"record_offset":52240,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":24705,"source_line":424,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::index::impl$5::index
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":426,"record_offset":52072,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":22994,"source_line":212,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":426,"record_offset":50788,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":426,"record_offset":49648,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":10965,"source_line":2109,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$56::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":426,"record_offset":51688,"parent_record_offset":47820,"ancestor_record_offset":47820,"inlinee":4727,"source_line":1902,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":52704,"parent_record_offset":52680,"ancestor_record_offset":47820,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":52728,"parent_record_offset":52704,"ancestor_record_offset":47820,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":52004,"parent_record_offset":51980,"ancestor_record_offset":47820,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":52680,"parent_record_offset":52584,"ancestor_record_offset":47820,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":51980,"parent_record_offset":51924,"ancestor_record_offset":47820,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":426,"record_offset":55348,"parent_record_offset":55320,"ancestor_record_offset":47820,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":426,"record_offset":55408,"parent_record_offset":55348,"ancestor_record_offset":47820,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":51952,"parent_record_offset":51924,"ancestor_record_offset":47820,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":54656,"parent_record_offset":54632,"ancestor_record_offset":47820,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":52584,"parent_record_offset":52552,"ancestor_record_offset":47820,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":52552,"parent_record_offset":52520,"ancestor_record_offset":47820,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":52800,"parent_record_offset":52552,"ancestor_record_offset":47820,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":51924,"parent_record_offset":51896,"ancestor_record_offset":47820,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":54632,"parent_record_offset":54604,"ancestor_record_offset":47820,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":52868,"parent_record_offset":52840,"ancestor_record_offset":47820,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":52488,"parent_record_offset":52436,"ancestor_record_offset":47820,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":52520,"parent_record_offset":52488,"ancestor_record_offset":47820,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":52348,"parent_record_offset":52304,"ancestor_record_offset":47820,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":53144,"parent_record_offset":53120,"ancestor_record_offset":47820,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":54604,"parent_record_offset":54572,"ancestor_record_offset":47820,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":54272,"parent_record_offset":54248,"ancestor_record_offset":47820,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":54320,"parent_record_offset":54296,"ancestor_record_offset":47820,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":426,"record_offset":53200,"parent_record_offset":53144,"ancestor_record_offset":47820,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":426,"record_offset":52840,"parent_record_offset":52436,"ancestor_record_offset":47820,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":426,"record_offset":52392,"parent_record_offset":52348,"ancestor_record_offset":47820,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":426,"record_offset":52436,"parent_record_offset":52392,"ancestor_record_offset":47820,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":426,"record_offset":52624,"parent_record_offset":52584,"ancestor_record_offset":47820,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":426,"record_offset":52648,"parent_record_offset":52624,"ancestor_record_offset":47820,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":50140,"parent_record_offset":50112,"ancestor_record_offset":47820,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":51516,"parent_record_offset":51488,"ancestor_record_offset":47820,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":50192,"parent_record_offset":50168,"ancestor_record_offset":47820,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":50168,"parent_record_offset":50140,"ancestor_record_offset":47820,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":50080,"parent_record_offset":50048,"ancestor_record_offset":47820,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":50112,"parent_record_offset":50080,"ancestor_record_offset":47820,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":50284,"parent_record_offset":50112,"ancestor_record_offset":47820,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":50396,"parent_record_offset":50372,"ancestor_record_offset":47820,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":426,"record_offset":50884,"parent_record_offset":50856,"ancestor_record_offset":47820,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":426,"record_offset":52924,"parent_record_offset":52896,"ancestor_record_offset":47820,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":426,"record_offset":52988,"parent_record_offset":52952,"ancestor_record_offset":47820,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":426,"record_offset":52896,"parent_record_offset":52868,"ancestor_record_offset":47820,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":426,"record_offset":53120,"parent_record_offset":52988,"ancestor_record_offset":47820,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":426,"record_offset":55440,"parent_record_offset":55348,"ancestor_record_offset":47820,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":426,"record_offset":54000,"parent_record_offset":53972,"ancestor_record_offset":47820,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":426,"record_offset":50216,"parent_record_offset":50192,"ancestor_record_offset":47820,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":426,"record_offset":53576,"parent_record_offset":53540,"ancestor_record_offset":47820,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":426,"record_offset":48276,"parent_record_offset":48180,"ancestor_record_offset":47820,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":49976,"parent_record_offset":49952,"ancestor_record_offset":47820,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":49232,"parent_record_offset":48772,"ancestor_record_offset":47820,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":50020,"parent_record_offset":49864,"ancestor_record_offset":47820,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":53044,"parent_record_offset":53016,"ancestor_record_offset":47820,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":52124,"parent_record_offset":52072,"ancestor_record_offset":47820,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":426,"record_offset":48588,"parent_record_offset":48560,"ancestor_record_offset":47820,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=63
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":50348,"parent_record_offset":50048,"ancestor_record_offset":47820,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":426,"record_offset":53880,"parent_record_offset":53856,"ancestor_record_offset":47820,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":49008,"parent_record_offset":48984,"ancestor_record_offset":47820,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":54116,"parent_record_offset":54060,"ancestor_record_offset":47820,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":48984,"parent_record_offset":48960,"ancestor_record_offset":47820,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":49192,"parent_record_offset":49160,"ancestor_record_offset":47820,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":54140,"parent_record_offset":54116,"ancestor_record_offset":47820,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":52756,"parent_record_offset":52728,"ancestor_record_offset":47820,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":50240,"parent_record_offset":50216,"ancestor_record_offset":47820,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":50748,"parent_record_offset":49832,"ancestor_record_offset":47820,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":49696,"parent_record_offset":49672,"ancestor_record_offset":47820,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":426,"record_offset":48212,"parent_record_offset":48180,"ancestor_record_offset":47820,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":426,"record_offset":48240,"parent_record_offset":48212,"ancestor_record_offset":47820,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":426,"record_offset":53856,"parent_record_offset":53832,"ancestor_record_offset":47820,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":48960,"parent_record_offset":48936,"ancestor_record_offset":47820,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":49288,"parent_record_offset":49264,"ancestor_record_offset":47820,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":52952,"parent_record_offset":52924,"ancestor_record_offset":47820,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":48936,"parent_record_offset":48828,"ancestor_record_offset":47820,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":426,"record_offset":48180,"parent_record_offset":48152,"ancestor_record_offset":47820,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":426,"record_offset":48668,"parent_record_offset":48640,"ancestor_record_offset":47820,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":426,"record_offset":48800,"parent_record_offset":48772,"ancestor_record_offset":47820,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":426,"record_offset":48772,"parent_record_offset":48740,"ancestor_record_offset":47820,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":48716,"parent_record_offset":48692,"ancestor_record_offset":47820,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":48740,"parent_record_offset":48716,"ancestor_record_offset":47820,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":48692,"parent_record_offset":48668,"ancestor_record_offset":47820,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":49264,"parent_record_offset":48740,"ancestor_record_offset":47820,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":48376,"parent_record_offset":48344,"ancestor_record_offset":47820,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":48536,"parent_record_offset":48376,"ancestor_record_offset":47820,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":48344,"parent_record_offset":48320,"ancestor_record_offset":47820,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":48096,"parent_record_offset":48064,"ancestor_record_offset":47820,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":48560,"parent_record_offset":48536,"ancestor_record_offset":47820,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":48152,"parent_record_offset":48120,"ancestor_record_offset":47820,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":48120,"parent_record_offset":48096,"ancestor_record_offset":47820,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":48828,"parent_record_offset":48800,"ancestor_record_offset":47820,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":49076,"parent_record_offset":48828,"ancestor_record_offset":47820,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":49048,"parent_record_offset":48828,"ancestor_record_offset":47820,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":48880,"parent_record_offset":48828,"ancestor_record_offset":47820,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":426,"record_offset":50944,"parent_record_offset":50920,"ancestor_record_offset":47820,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":426,"record_offset":50968,"parent_record_offset":50944,"ancestor_record_offset":47820,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":51216,"parent_record_offset":51192,"ancestor_record_offset":47820,"inlinee":22684,"source_line":11,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":426,"record_offset":50856,"parent_record_offset":50816,"ancestor_record_offset":47820,"inlinee":24709,"source_line":9,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":426,"record_offset":51128,"parent_record_offset":51088,"ancestor_record_offset":47820,"inlinee":24710,"source_line":15,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":426,"record_offset":53016,"parent_record_offset":52988,"ancestor_record_offset":47820,"inlinee":24707,"source_line":27,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":49800,"parent_record_offset":49768,"ancestor_record_offset":47820,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":426,"record_offset":49900,"parent_record_offset":49864,"ancestor_record_offset":47820,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":50048,"parent_record_offset":49864,"ancestor_record_offset":47820,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":50320,"parent_record_offset":50048,"ancestor_record_offset":47820,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::Value::as_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":426,"record_offset":53084,"parent_record_offset":53016,"ancestor_record_offset":47820,"inlinee":24708,"source_line":627,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":426,"record_offset":52096,"parent_record_offset":52072,"ancestor_record_offset":47820,"inlinee":22996,"source_line":123,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":426,"record_offset":51244,"parent_record_offset":51216,"ancestor_record_offset":47820,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":426,"record_offset":49672,"parent_record_offset":49648,"ancestor_record_offset":47820,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":426,"record_offset":51292,"parent_record_offset":51268,"ancestor_record_offset":47820,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":426,"record_offset":51368,"parent_record_offset":51340,"ancestor_record_offset":47820,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":426,"record_offset":51488,"parent_record_offset":51340,"ancestor_record_offset":47820,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":426,"record_offset":51268,"parent_record_offset":51244,"ancestor_record_offset":47820,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":426,"record_offset":49924,"parent_record_offset":49900,"ancestor_record_offset":47820,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":426,"record_offset":49952,"parent_record_offset":49924,"ancestor_record_offset":47820,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":426,"record_offset":49864,"parent_record_offset":49832,"ancestor_record_offset":47820,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":426,"record_offset":49832,"parent_record_offset":49800,"ancestor_record_offset":47820,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh<ref$<alloc::string::String> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":426,"record_offset":22504,"rva":5988784,"va":5374697904}
// 编译实例 RVAs: 0x5b61b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$str$u20$as$u20$serde_json..value..index..Index$GT$::index_into
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t10840959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e87f6ba500","size":5,"soft_fail":false,"target_va":5385539984,"va":5374699020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2386280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e89896dbff","size":5,"soft_fail":false,"target_va":5372314208,"va":5374700483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2379912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e878afdbff","size":5,"soft_fail":false,"target_va":5372320512,"va":5374700419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2073834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e8165be0ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5374698853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1949427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e80d41e2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374699118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1949268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e8ac41e2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374698975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140546de0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-458744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e80800f9ff","size":5,"soft_fail":false,"target_va":5374242272,"va":5374701011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bf340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e8878b0000","size":5,"soft_fail":false,"target_va":5374735168,"va":5374699444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1192035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e863301200","size":5,"soft_fail":false,"target_va":5375891872,"va":5374699832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e2f30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1230505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e8a9c61200","size":5,"soft_fail":false,"target_va":5375930160,"va":5374699650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1235163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e8dbd81200","size":5,"soft_fail":false,"target_va":5375936112,"va":5374700944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14084b050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2706574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e88e4c2900","size":5,"soft_fail":false,"target_va":5377405008,"va":5374698429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140852b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2737549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e88dc52900","size":5,"soft_fail":false,"target_va":5377436496,"va":5374698942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d43420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7917306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e8face7800","size":5,"soft_fail":false,"target_va":5382616096,"va":5374698785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d75540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8121885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e81dee7b00","size":5,"soft_fail":false,"target_va":5382821184,"va":5374699294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8609664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e8805f8300","size":5,"soft_fail":false,"target_va":5383308144,"va":5374698475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28277904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e8907caf01","size":5,"soft_fail":false,"target_va":5402978144,"va":5374700235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2663528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e868a42800","size":5,"soft_fail":false,"target_va":5377364336,"va":5374700803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28278422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e8967eaf01","size":5,"soft_fail":false,"target_va":5402978688,"va":5374700261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28278894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e86e80af01","size":5,"soft_fail":false,"target_va":5402978880,"va":5374699981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28278856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e84880af01","size":5,"soft_fail":false,"target_va":5402978944,"va":5374700083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e8becfffff","size":5,"soft_fail":false,"target_va":5374687520,"va":5374699869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28280173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e86d85af01","size":5,"soft_fail":false,"target_va":5402980128,"va":5374699950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e8d7d7ffff","size":5,"soft_fail":false,"target_va":5374687712,"va":5374697988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28282586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e8da8eaf01","size":5,"soft_fail":false,"target_va":5402980656,"va":5374698065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::load_json::read_attribute_from_json
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10955439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e8af2aa700","size":5,"soft_fail":false,"target_va":5385654832,"va":5374699388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2655005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e81d832800","size":5,"soft_fail":false,"target_va":5377353776,"va":5374698766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1658366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e8fe4d1900","size":5,"soft_fail":false,"target_va":5376356912,"va":5374698541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1662102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e8965c1900","size":5,"soft_fail":false,"target_va":5376360560,"va":5374698453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1658171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e83b4d1900","size":5,"soft_fail":false,"target_va":5376356144,"va":5374697968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2707012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e8444e2900","size":5,"soft_fail":false,"target_va":5377405616,"va":5374698599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2667388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374697904,"raw_hex":"e87cb32800","size":5,"soft_fail":false,"target_va":5377366224,"va":5374698831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":28696,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":28784,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":426,"record_offset":25728,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":426,"record_offset":22760,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":426,"record_offset":25756,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":426,"record_offset":27028,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":24456,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":28972,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":28900,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":25540,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":426,"record_offset":28556,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":25916,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":24287,"source_line":24,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":28140,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":24560,"source_line":48,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_source_file
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":26440,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":24704,"source_line":32,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":426,"record_offset":25784,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":426,"record_offset":25644,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":24492,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":22788,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":23364,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":426,"record_offset":26964,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":24705,"source_line":424,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::index::impl$5::index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":426,"record_offset":26796,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":22994,"source_line":212,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":426,"record_offset":25512,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":426,"record_offset":24372,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":10965,"source_line":2109,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$56::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":426,"record_offset":26412,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":4727,"source_line":1902,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":27428,"parent_record_offset":27404,"ancestor_record_offset":22504,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":27452,"parent_record_offset":27428,"ancestor_record_offset":22504,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":26728,"parent_record_offset":26704,"ancestor_record_offset":22504,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":27404,"parent_record_offset":27308,"ancestor_record_offset":22504,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":26704,"parent_record_offset":26648,"ancestor_record_offset":22504,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":426,"record_offset":30072,"parent_record_offset":30044,"ancestor_record_offset":22504,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":426,"record_offset":30132,"parent_record_offset":30072,"ancestor_record_offset":22504,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":26676,"parent_record_offset":26648,"ancestor_record_offset":22504,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":29380,"parent_record_offset":29356,"ancestor_record_offset":22504,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":27308,"parent_record_offset":27276,"ancestor_record_offset":22504,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":27276,"parent_record_offset":27244,"ancestor_record_offset":22504,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":27524,"parent_record_offset":27276,"ancestor_record_offset":22504,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":26648,"parent_record_offset":26620,"ancestor_record_offset":22504,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":29356,"parent_record_offset":29328,"ancestor_record_offset":22504,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":27592,"parent_record_offset":27564,"ancestor_record_offset":22504,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":27212,"parent_record_offset":27160,"ancestor_record_offset":22504,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":27244,"parent_record_offset":27212,"ancestor_record_offset":22504,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":27072,"parent_record_offset":27028,"ancestor_record_offset":22504,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":27868,"parent_record_offset":27844,"ancestor_record_offset":22504,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":29328,"parent_record_offset":29296,"ancestor_record_offset":22504,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":28996,"parent_record_offset":28972,"ancestor_record_offset":22504,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":29044,"parent_record_offset":29020,"ancestor_record_offset":22504,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":426,"record_offset":27924,"parent_record_offset":27868,"ancestor_record_offset":22504,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":426,"record_offset":27564,"parent_record_offset":27160,"ancestor_record_offset":22504,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":426,"record_offset":27116,"parent_record_offset":27072,"ancestor_record_offset":22504,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":426,"record_offset":27160,"parent_record_offset":27116,"ancestor_record_offset":22504,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":426,"record_offset":27348,"parent_record_offset":27308,"ancestor_record_offset":22504,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":426,"record_offset":27372,"parent_record_offset":27348,"ancestor_record_offset":22504,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":24864,"parent_record_offset":24836,"ancestor_record_offset":22504,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":26240,"parent_record_offset":26212,"ancestor_record_offset":22504,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":24916,"parent_record_offset":24892,"ancestor_record_offset":22504,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":24892,"parent_record_offset":24864,"ancestor_record_offset":22504,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":24804,"parent_record_offset":24772,"ancestor_record_offset":22504,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":24836,"parent_record_offset":24804,"ancestor_record_offset":22504,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":25008,"parent_record_offset":24836,"ancestor_record_offset":22504,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":25120,"parent_record_offset":25096,"ancestor_record_offset":22504,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":426,"record_offset":25608,"parent_record_offset":25580,"ancestor_record_offset":22504,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":426,"record_offset":27648,"parent_record_offset":27620,"ancestor_record_offset":22504,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":426,"record_offset":27712,"parent_record_offset":27676,"ancestor_record_offset":22504,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":426,"record_offset":27620,"parent_record_offset":27592,"ancestor_record_offset":22504,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":426,"record_offset":27844,"parent_record_offset":27712,"ancestor_record_offset":22504,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":426,"record_offset":30164,"parent_record_offset":30072,"ancestor_record_offset":22504,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":426,"record_offset":28724,"parent_record_offset":28696,"ancestor_record_offset":22504,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":426,"record_offset":24940,"parent_record_offset":24916,"ancestor_record_offset":22504,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":426,"record_offset":28300,"parent_record_offset":28264,"ancestor_record_offset":22504,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":426,"record_offset":23000,"parent_record_offset":22904,"ancestor_record_offset":22504,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":24700,"parent_record_offset":24676,"ancestor_record_offset":22504,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":23956,"parent_record_offset":23496,"ancestor_record_offset":22504,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":24744,"parent_record_offset":24588,"ancestor_record_offset":22504,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":27768,"parent_record_offset":27740,"ancestor_record_offset":22504,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":26848,"parent_record_offset":26796,"ancestor_record_offset":22504,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":426,"record_offset":23312,"parent_record_offset":23284,"ancestor_record_offset":22504,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":25072,"parent_record_offset":24772,"ancestor_record_offset":22504,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":426,"record_offset":28604,"parent_record_offset":28580,"ancestor_record_offset":22504,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":23732,"parent_record_offset":23708,"ancestor_record_offset":22504,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":28840,"parent_record_offset":28784,"ancestor_record_offset":22504,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":23708,"parent_record_offset":23684,"ancestor_record_offset":22504,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":23916,"parent_record_offset":23884,"ancestor_record_offset":22504,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":28864,"parent_record_offset":28840,"ancestor_record_offset":22504,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":27480,"parent_record_offset":27452,"ancestor_record_offset":22504,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":24964,"parent_record_offset":24940,"ancestor_record_offset":22504,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":25472,"parent_record_offset":24556,"ancestor_record_offset":22504,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":24420,"parent_record_offset":24396,"ancestor_record_offset":22504,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":426,"record_offset":22936,"parent_record_offset":22904,"ancestor_record_offset":22504,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":426,"record_offset":22964,"parent_record_offset":22936,"ancestor_record_offset":22504,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":426,"record_offset":28580,"parent_record_offset":28556,"ancestor_record_offset":22504,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":23684,"parent_record_offset":23660,"ancestor_record_offset":22504,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":24012,"parent_record_offset":23988,"ancestor_record_offset":22504,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":27676,"parent_record_offset":27648,"ancestor_record_offset":22504,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":23660,"parent_record_offset":23552,"ancestor_record_offset":22504,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":426,"record_offset":22904,"parent_record_offset":22876,"ancestor_record_offset":22504,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":426,"record_offset":23392,"parent_record_offset":23364,"ancestor_record_offset":22504,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":426,"record_offset":23524,"parent_record_offset":23496,"ancestor_record_offset":22504,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":426,"record_offset":23496,"parent_record_offset":23464,"ancestor_record_offset":22504,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":23440,"parent_record_offset":23416,"ancestor_record_offset":22504,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":23464,"parent_record_offset":23440,"ancestor_record_offset":22504,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":23416,"parent_record_offset":23392,"ancestor_record_offset":22504,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":23988,"parent_record_offset":23464,"ancestor_record_offset":22504,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":23100,"parent_record_offset":23068,"ancestor_record_offset":22504,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":23260,"parent_record_offset":23100,"ancestor_record_offset":22504,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":23068,"parent_record_offset":23044,"ancestor_record_offset":22504,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":22820,"parent_record_offset":22788,"ancestor_record_offset":22504,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":23284,"parent_record_offset":23260,"ancestor_record_offset":22504,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":22876,"parent_record_offset":22844,"ancestor_record_offset":22504,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":22844,"parent_record_offset":22820,"ancestor_record_offset":22504,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":23552,"parent_record_offset":23524,"ancestor_record_offset":22504,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":23800,"parent_record_offset":23552,"ancestor_record_offset":22504,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":23772,"parent_record_offset":23552,"ancestor_record_offset":22504,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":23604,"parent_record_offset":23552,"ancestor_record_offset":22504,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":426,"record_offset":25668,"parent_record_offset":25644,"ancestor_record_offset":22504,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":426,"record_offset":25692,"parent_record_offset":25668,"ancestor_record_offset":22504,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":25940,"parent_record_offset":25916,"ancestor_record_offset":22504,"inlinee":22684,"source_line":11,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":426,"record_offset":25580,"parent_record_offset":25540,"ancestor_record_offset":22504,"inlinee":24709,"source_line":9,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":426,"record_offset":25852,"parent_record_offset":25812,"ancestor_record_offset":22504,"inlinee":24710,"source_line":15,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":426,"record_offset":27740,"parent_record_offset":27712,"ancestor_record_offset":22504,"inlinee":24707,"source_line":27,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":24524,"parent_record_offset":24492,"ancestor_record_offset":22504,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":426,"record_offset":24624,"parent_record_offset":24588,"ancestor_record_offset":22504,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":24772,"parent_record_offset":24588,"ancestor_record_offset":22504,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":25044,"parent_record_offset":24772,"ancestor_record_offset":22504,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::Value::as_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":426,"record_offset":27808,"parent_record_offset":27740,"ancestor_record_offset":22504,"inlinee":24708,"source_line":627,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":426,"record_offset":26820,"parent_record_offset":26796,"ancestor_record_offset":22504,"inlinee":22996,"source_line":123,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":426,"record_offset":25968,"parent_record_offset":25940,"ancestor_record_offset":22504,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":426,"record_offset":24396,"parent_record_offset":24372,"ancestor_record_offset":22504,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":426,"record_offset":26016,"parent_record_offset":25992,"ancestor_record_offset":22504,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":426,"record_offset":26092,"parent_record_offset":26064,"ancestor_record_offset":22504,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":426,"record_offset":26212,"parent_record_offset":26064,"ancestor_record_offset":22504,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":426,"record_offset":25992,"parent_record_offset":25968,"ancestor_record_offset":22504,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":426,"record_offset":24648,"parent_record_offset":24624,"ancestor_record_offset":22504,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":426,"record_offset":24676,"parent_record_offset":24648,"ancestor_record_offset":22504,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":426,"record_offset":24588,"parent_record_offset":24556,"ancestor_record_offset":22504,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":426,"record_offset":24556,"parent_record_offset":24524,"ancestor_record_offset":22504,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":426,"record_offset":39256,"rva":5995344,"va":5374704464}
// 编译实例 RVAs: 0x5b7b50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$str$u20$as$u20$serde_json..value..index..Index$GT$::index_into
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t10834399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e8df51a500","size":5,"soft_fail":false,"target_va":5385539984,"va":5374705580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2392840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e8f87cdbff","size":5,"soft_fail":false,"target_va":5372314208,"va":5374707043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2386472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e8d895dbff","size":5,"soft_fail":false,"target_va":5372320512,"va":5374706979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2080394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e87641e0ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5374705413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1955987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e86d27e2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374705678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1955828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e80c28e2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374705535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140546de0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-465304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e868e6f8ff","size":5,"soft_fail":false,"target_va":5374242272,"va":5374707571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bf340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e8e7710000","size":5,"soft_fail":false,"target_va":5374735168,"va":5374706004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t466542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e86e1e0700","size":5,"soft_fail":false,"target_va":5375171056,"va":5374704509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1185475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e8c3161200","size":5,"soft_fail":false,"target_va":5375891872,"va":5374706392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e2f30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1223945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e809ad1200","size":5,"soft_fail":false,"target_va":5375930160,"va":5374706210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1228603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e83bbf1200","size":5,"soft_fail":false,"target_va":5375936112,"va":5374707504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14084b050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2700014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e8ee322900","size":5,"soft_fail":false,"target_va":5377405008,"va":5374704989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140852b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2730989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e8edab2900","size":5,"soft_fail":false,"target_va":5377436496,"va":5374705502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d43420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7910746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e85ab57800","size":5,"soft_fail":false,"target_va":5382616096,"va":5374705345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d75540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8115325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e87dd47b00","size":5,"soft_fail":false,"target_va":5382821184,"va":5374705854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8603104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e8e0458300","size":5,"soft_fail":false,"target_va":5383308144,"va":5374705035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28271344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e8f062af01","size":5,"soft_fail":false,"target_va":5402978144,"va":5374706795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2656968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e8c88a2800","size":5,"soft_fail":false,"target_va":5377364336,"va":5374707363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28271862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e8f664af01","size":5,"soft_fail":false,"target_va":5402978688,"va":5374706821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28272334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e8ce66af01","size":5,"soft_fail":false,"target_va":5402978880,"va":5374706541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28272296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e8a866af01","size":5,"soft_fail":false,"target_va":5402978944,"va":5374706643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-18914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e81eb6ffff","size":5,"soft_fail":false,"target_va":5374687520,"va":5374706429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28273613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e8cd6baf01","size":5,"soft_fail":false,"target_va":5402980128,"va":5374706510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-16847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e831beffff","size":5,"soft_fail":false,"target_va":5374687712,"va":5374704554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28276020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e83475af01","size":5,"soft_fail":false,"target_va":5402980656,"va":5374704631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::load_json::read_attribute_from_json
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10948879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e80f11a700","size":5,"soft_fail":false,"target_va":5385654832,"va":5374705948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2648445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e87d692800","size":5,"soft_fail":false,"target_va":5377353776,"va":5374705326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1651806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e85e341900","size":5,"soft_fail":false,"target_va":5376356912,"va":5374705101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1655542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e8f6421900","size":5,"soft_fail":false,"target_va":5376360560,"va":5374705013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1651605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e895331900","size":5,"soft_fail":false,"target_va":5376356144,"va":5374704534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2700452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e8a4342900","size":5,"soft_fail":false,"target_va":5377405616,"va":5374705159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2660828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374704464,"raw_hex":"e8dc992800","size":5,"soft_fail":false,"target_va":5377366224,"va":5374705391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":45612,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":45700,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":426,"record_offset":42644,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":426,"record_offset":39508,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":426,"record_offset":42672,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":426,"record_offset":43944,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":41372,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":45888,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":45816,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":42456,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":426,"record_offset":45472,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":42832,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":24287,"source_line":24,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":45056,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":24560,"source_line":48,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_source_file
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":43356,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":24704,"source_line":32,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":426,"record_offset":42700,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":426,"record_offset":42560,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":41408,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":39704,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":40280,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":426,"record_offset":43880,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":24705,"source_line":424,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::index::impl$5::index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":426,"record_offset":43712,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":22994,"source_line":212,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":426,"record_offset":42428,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":426,"record_offset":41288,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":10965,"source_line":2109,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$56::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":426,"record_offset":43328,"parent_record_offset":39256,"ancestor_record_offset":39256,"inlinee":4727,"source_line":1902,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":44344,"parent_record_offset":44320,"ancestor_record_offset":39256,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":44368,"parent_record_offset":44344,"ancestor_record_offset":39256,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":43644,"parent_record_offset":43620,"ancestor_record_offset":39256,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":44320,"parent_record_offset":44224,"ancestor_record_offset":39256,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":43620,"parent_record_offset":43564,"ancestor_record_offset":39256,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":426,"record_offset":46988,"parent_record_offset":46960,"ancestor_record_offset":39256,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":426,"record_offset":47048,"parent_record_offset":46988,"ancestor_record_offset":39256,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":43592,"parent_record_offset":43564,"ancestor_record_offset":39256,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":46296,"parent_record_offset":46272,"ancestor_record_offset":39256,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":44224,"parent_record_offset":44192,"ancestor_record_offset":39256,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":44192,"parent_record_offset":44160,"ancestor_record_offset":39256,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":44440,"parent_record_offset":44192,"ancestor_record_offset":39256,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":43564,"parent_record_offset":43536,"ancestor_record_offset":39256,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":46272,"parent_record_offset":46244,"ancestor_record_offset":39256,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":44508,"parent_record_offset":44480,"ancestor_record_offset":39256,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":44128,"parent_record_offset":44076,"ancestor_record_offset":39256,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":44160,"parent_record_offset":44128,"ancestor_record_offset":39256,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":43988,"parent_record_offset":43944,"ancestor_record_offset":39256,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":44784,"parent_record_offset":44760,"ancestor_record_offset":39256,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":46244,"parent_record_offset":46212,"ancestor_record_offset":39256,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":45912,"parent_record_offset":45888,"ancestor_record_offset":39256,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":45960,"parent_record_offset":45936,"ancestor_record_offset":39256,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":426,"record_offset":44840,"parent_record_offset":44784,"ancestor_record_offset":39256,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":426,"record_offset":44480,"parent_record_offset":44076,"ancestor_record_offset":39256,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":426,"record_offset":44032,"parent_record_offset":43988,"ancestor_record_offset":39256,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":426,"record_offset":44076,"parent_record_offset":44032,"ancestor_record_offset":39256,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":426,"record_offset":44264,"parent_record_offset":44224,"ancestor_record_offset":39256,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":426,"record_offset":44288,"parent_record_offset":44264,"ancestor_record_offset":39256,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":41780,"parent_record_offset":41752,"ancestor_record_offset":39256,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":43156,"parent_record_offset":43128,"ancestor_record_offset":39256,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":41832,"parent_record_offset":41808,"ancestor_record_offset":39256,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":41808,"parent_record_offset":41780,"ancestor_record_offset":39256,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":41720,"parent_record_offset":41688,"ancestor_record_offset":39256,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":41752,"parent_record_offset":41720,"ancestor_record_offset":39256,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":41924,"parent_record_offset":41752,"ancestor_record_offset":39256,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":42036,"parent_record_offset":42012,"ancestor_record_offset":39256,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":426,"record_offset":42524,"parent_record_offset":42496,"ancestor_record_offset":39256,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":426,"record_offset":44564,"parent_record_offset":44536,"ancestor_record_offset":39256,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":426,"record_offset":44628,"parent_record_offset":44592,"ancestor_record_offset":39256,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":426,"record_offset":44536,"parent_record_offset":44508,"ancestor_record_offset":39256,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":426,"record_offset":44760,"parent_record_offset":44628,"ancestor_record_offset":39256,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":426,"record_offset":47080,"parent_record_offset":46988,"ancestor_record_offset":39256,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":426,"record_offset":45640,"parent_record_offset":45612,"ancestor_record_offset":39256,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":426,"record_offset":41856,"parent_record_offset":41832,"ancestor_record_offset":39256,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":426,"record_offset":45216,"parent_record_offset":45180,"ancestor_record_offset":39256,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":426,"record_offset":39916,"parent_record_offset":39820,"ancestor_record_offset":39256,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":41616,"parent_record_offset":41592,"ancestor_record_offset":39256,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":40872,"parent_record_offset":40412,"ancestor_record_offset":39256,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":41660,"parent_record_offset":41504,"ancestor_record_offset":39256,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":44684,"parent_record_offset":44656,"ancestor_record_offset":39256,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":43764,"parent_record_offset":43712,"ancestor_record_offset":39256,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":426,"record_offset":40228,"parent_record_offset":40200,"ancestor_record_offset":39256,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":41988,"parent_record_offset":41688,"ancestor_record_offset":39256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":426,"record_offset":45520,"parent_record_offset":45496,"ancestor_record_offset":39256,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":40648,"parent_record_offset":40624,"ancestor_record_offset":39256,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":45756,"parent_record_offset":45700,"ancestor_record_offset":39256,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":40624,"parent_record_offset":40600,"ancestor_record_offset":39256,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":40832,"parent_record_offset":40800,"ancestor_record_offset":39256,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":45780,"parent_record_offset":45756,"ancestor_record_offset":39256,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":44396,"parent_record_offset":44368,"ancestor_record_offset":39256,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":41880,"parent_record_offset":41856,"ancestor_record_offset":39256,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":42388,"parent_record_offset":41472,"ancestor_record_offset":39256,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":41336,"parent_record_offset":41312,"ancestor_record_offset":39256,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":426,"record_offset":39852,"parent_record_offset":39820,"ancestor_record_offset":39256,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":426,"record_offset":39880,"parent_record_offset":39852,"ancestor_record_offset":39256,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":426,"record_offset":45496,"parent_record_offset":45472,"ancestor_record_offset":39256,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":40600,"parent_record_offset":40576,"ancestor_record_offset":39256,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":40928,"parent_record_offset":40904,"ancestor_record_offset":39256,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":44592,"parent_record_offset":44564,"ancestor_record_offset":39256,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":40576,"parent_record_offset":40468,"ancestor_record_offset":39256,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":426,"record_offset":39820,"parent_record_offset":39792,"ancestor_record_offset":39256,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":426,"record_offset":40308,"parent_record_offset":40280,"ancestor_record_offset":39256,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":426,"record_offset":40440,"parent_record_offset":40412,"ancestor_record_offset":39256,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":426,"record_offset":40412,"parent_record_offset":40380,"ancestor_record_offset":39256,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":40356,"parent_record_offset":40332,"ancestor_record_offset":39256,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":40380,"parent_record_offset":40356,"ancestor_record_offset":39256,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":40332,"parent_record_offset":40308,"ancestor_record_offset":39256,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":40904,"parent_record_offset":40380,"ancestor_record_offset":39256,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":40016,"parent_record_offset":39984,"ancestor_record_offset":39256,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":40176,"parent_record_offset":40016,"ancestor_record_offset":39256,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":39984,"parent_record_offset":39960,"ancestor_record_offset":39256,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":39736,"parent_record_offset":39704,"ancestor_record_offset":39256,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":40200,"parent_record_offset":40176,"ancestor_record_offset":39256,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":39792,"parent_record_offset":39760,"ancestor_record_offset":39256,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":39760,"parent_record_offset":39736,"ancestor_record_offset":39256,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":40468,"parent_record_offset":40440,"ancestor_record_offset":39256,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":40716,"parent_record_offset":40468,"ancestor_record_offset":39256,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":40688,"parent_record_offset":40468,"ancestor_record_offset":39256,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":40520,"parent_record_offset":40468,"ancestor_record_offset":39256,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":426,"record_offset":42584,"parent_record_offset":42560,"ancestor_record_offset":39256,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":426,"record_offset":42608,"parent_record_offset":42584,"ancestor_record_offset":39256,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":42856,"parent_record_offset":42832,"ancestor_record_offset":39256,"inlinee":22684,"source_line":11,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":426,"record_offset":42496,"parent_record_offset":42456,"ancestor_record_offset":39256,"inlinee":24709,"source_line":9,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":426,"record_offset":42768,"parent_record_offset":42728,"ancestor_record_offset":39256,"inlinee":24710,"source_line":15,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":426,"record_offset":44656,"parent_record_offset":44628,"ancestor_record_offset":39256,"inlinee":24707,"source_line":27,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":41440,"parent_record_offset":41408,"ancestor_record_offset":39256,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":426,"record_offset":41540,"parent_record_offset":41504,"ancestor_record_offset":39256,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":41688,"parent_record_offset":41504,"ancestor_record_offset":39256,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":41960,"parent_record_offset":41688,"ancestor_record_offset":39256,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::Value::as_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":426,"record_offset":44724,"parent_record_offset":44656,"ancestor_record_offset":39256,"inlinee":24708,"source_line":627,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":426,"record_offset":43736,"parent_record_offset":43712,"ancestor_record_offset":39256,"inlinee":22996,"source_line":123,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":426,"record_offset":42884,"parent_record_offset":42856,"ancestor_record_offset":39256,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":426,"record_offset":41312,"parent_record_offset":41288,"ancestor_record_offset":39256,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":426,"record_offset":39604,"parent_record_offset":39580,"ancestor_record_offset":39256,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":426,"record_offset":39652,"parent_record_offset":39628,"ancestor_record_offset":39256,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":426,"record_offset":39628,"parent_record_offset":39604,"ancestor_record_offset":39256,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":426,"record_offset":42932,"parent_record_offset":42908,"ancestor_record_offset":39256,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":426,"record_offset":43008,"parent_record_offset":42980,"ancestor_record_offset":39256,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":426,"record_offset":43128,"parent_record_offset":42980,"ancestor_record_offset":39256,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":426,"record_offset":42908,"parent_record_offset":42884,"ancestor_record_offset":39256,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":426,"record_offset":39580,"parent_record_offset":39556,"ancestor_record_offset":39256,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":426,"record_offset":39556,"parent_record_offset":39532,"ancestor_record_offset":39256,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":426,"record_offset":39532,"parent_record_offset":39508,"ancestor_record_offset":39256,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":426,"record_offset":41564,"parent_record_offset":41540,"ancestor_record_offset":39256,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":426,"record_offset":41592,"parent_record_offset":41564,"ancestor_record_offset":39256,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":426,"record_offset":41504,"parent_record_offset":41472,"ancestor_record_offset":39256,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":426,"record_offset":41472,"parent_record_offset":41440,"ancestor_record_offset":39256,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh<ref$<str$> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":426,"record_offset":30904,"rva":5992064,"va":5374701184}
// 编译实例 RVAs: 0x5b6e80, 0x1dd1720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$str$u20$as$u20$serde_json..value..index..Index$GT$::index_into
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t10837679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e8af5ea500","size":5,"soft_fail":false,"target_va":5385539984,"va":5374702300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2389560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e8c889dbff","size":5,"soft_fail":false,"target_va":5372314208,"va":5374703763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2383192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e8a8a2dbff","size":5,"soft_fail":false,"target_va":5372320512,"va":5374703699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2077114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e8464ee0ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5374702133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1952707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e83d34e2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374702398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1952548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e8dc34e2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374702255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140546de0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-462024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e838f3f8ff","size":5,"soft_fail":false,"target_va":5374242272,"va":5374704291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bf340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e8b77e0000","size":5,"soft_fail":false,"target_va":5374735168,"va":5374702724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1188755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e893231200","size":5,"soft_fail":false,"target_va":5375891872,"va":5374703112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e2f30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1227225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e8d9b91200","size":5,"soft_fail":false,"target_va":5375930160,"va":5374702930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1231883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e80bcc1200","size":5,"soft_fail":false,"target_va":5375936112,"va":5374704224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14084b050
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2703294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e8be3f2900","size":5,"soft_fail":false,"target_va":5377405008,"va":5374701709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140852b50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2734269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e8bdb82900","size":5,"soft_fail":false,"target_va":5377436496,"va":5374702222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409b10f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21105795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399975712,"raw_hex":"e87df3bdfe","size":5,"soft_fail":false,"target_va":5378871536,"va":5399977326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409b1350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21106389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399975712,"raw_hex":"e82bf1bdfe","size":5,"soft_fail":false,"target_va":5378872144,"va":5399978528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409b3510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21096591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399975712,"raw_hex":"e87117befe","size":5,"soft_fail":false,"target_va":5378880784,"va":5399977370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d43420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7914026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e82ac27800","size":5,"soft_fail":false,"target_va":5382616096,"va":5374702065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d75540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8118605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e84de17b00","size":5,"soft_fail":false,"target_va":5382821184,"va":5374702574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8606384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e8b0528300","size":5,"soft_fail":false,"target_va":5383308144,"va":5374701755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414b63a0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9551560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399975712,"raw_hex":"e838416eff","size":5,"soft_fail":false,"target_va":5390427040,"va":5399978595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414e6920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9351930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399975712,"raw_hex":"e8064d71ff","size":5,"soft_fail":false,"target_va":5390625056,"va":5399976981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ec6d60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1003703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399975712,"raw_hex":"e8b7500f00","size":5,"soft_fail":false,"target_va":5400980832,"va":5399977124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28274624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e8c06faf01","size":5,"soft_fail":false,"target_va":5402978144,"va":5374703515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2660248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e898972800","size":5,"soft_fail":false,"target_va":5377364336,"va":5374704083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28275142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e8c671af01","size":5,"soft_fail":false,"target_va":5402978688,"va":5374703541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t28275614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e89e73af01","size":5,"soft_fail":false,"target_va":5402978880,"va":5374703261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28275576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e87873af01","size":5,"soft_fail":false,"target_va":5402978944,"va":5374703363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-15634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e8eec2ffff","size":5,"soft_fail":false,"target_va":5374687520,"va":5374703149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28276893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e89d78af01","size":5,"soft_fail":false,"target_va":5402980128,"va":5374703230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-13559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e809cbffff","size":5,"soft_fail":false,"target_va":5374687712,"va":5374701266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t28279308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e80c82af01","size":5,"soft_fail":false,"target_va":5402980656,"va":5374701343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15409287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399975712,"raw_hex":"e879df14ff","size":5,"soft_fail":false,"target_va":5384567376,"va":5399976658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::set_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15409333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399975712,"raw_hex":"e84bdf14ff","size":5,"soft_fail":false,"target_va":5384567584,"va":5399976912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::set_source_file
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15409201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399975712,"raw_hex":"e8cfdf14ff","size":5,"soft_fail":false,"target_va":5384567504,"va":5399976700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::load_json::read_attribute_from_json
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10952159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e8df1da700","size":5,"soft_fail":false,"target_va":5385654832,"va":5374702668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2651725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e84d762800","size":5,"soft_fail":false,"target_va":5377353776,"va":5374702046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1655086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e82e411900","size":5,"soft_fail":false,"target_va":5376356912,"va":5374701821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1658822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e8c64f1900","size":5,"soft_fail":false,"target_va":5376360560,"va":5374701733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1654893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e86d401900","size":5,"soft_fail":false,"target_va":5376356144,"va":5374701246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::from_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t999947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399975712,"raw_hex":"e80b420f00","size":5,"soft_fail":false,"target_va":5400976560,"va":5399976608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2703732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e874412900","size":5,"soft_fail":false,"target_va":5377405616,"va":5374701879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2664108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701184,"raw_hex":"e8aca62800","size":5,"soft_fail":false,"target_va":5377366224,"va":5374702111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h93c6532eedf5b01e (.llvm.12250732642055207085)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21053648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399975712,"raw_hex":"e830bfbefe","size":5,"soft_fail":false,"target_va":5378922640,"va":5399976283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":37048,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":37136,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":426,"record_offset":34080,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":426,"record_offset":34108,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":426,"record_offset":35380,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":32808,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":37324,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":37252,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":33892,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":426,"record_offset":36908,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":34268,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":24287,"source_line":24,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":36492,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":24560,"source_line":48,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_source_file
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":34792,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":24704,"source_line":32,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":426,"record_offset":34136,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":426,"record_offset":33996,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":32844,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":31140,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":31716,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":426,"record_offset":35316,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":24705,"source_line":424,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::index::impl$5::index
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":426,"record_offset":35148,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":22994,"source_line":212,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":426,"record_offset":33864,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":426,"record_offset":32724,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":10965,"source_line":2109,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$56::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":426,"record_offset":34764,"parent_record_offset":30904,"ancestor_record_offset":30904,"inlinee":4727,"source_line":1902,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":35780,"parent_record_offset":35756,"ancestor_record_offset":30904,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":35804,"parent_record_offset":35780,"ancestor_record_offset":30904,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":35080,"parent_record_offset":35056,"ancestor_record_offset":30904,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":35756,"parent_record_offset":35660,"ancestor_record_offset":30904,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":35056,"parent_record_offset":35000,"ancestor_record_offset":30904,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":426,"record_offset":38424,"parent_record_offset":38396,"ancestor_record_offset":30904,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":426,"record_offset":38484,"parent_record_offset":38424,"ancestor_record_offset":30904,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":35028,"parent_record_offset":35000,"ancestor_record_offset":30904,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":37732,"parent_record_offset":37708,"ancestor_record_offset":30904,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":35660,"parent_record_offset":35628,"ancestor_record_offset":30904,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":35628,"parent_record_offset":35596,"ancestor_record_offset":30904,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":35876,"parent_record_offset":35628,"ancestor_record_offset":30904,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":35000,"parent_record_offset":34972,"ancestor_record_offset":30904,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":37708,"parent_record_offset":37680,"ancestor_record_offset":30904,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":35944,"parent_record_offset":35916,"ancestor_record_offset":30904,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":35564,"parent_record_offset":35512,"ancestor_record_offset":30904,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":35596,"parent_record_offset":35564,"ancestor_record_offset":30904,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":35424,"parent_record_offset":35380,"ancestor_record_offset":30904,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":36220,"parent_record_offset":36196,"ancestor_record_offset":30904,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":37680,"parent_record_offset":37648,"ancestor_record_offset":30904,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":37348,"parent_record_offset":37324,"ancestor_record_offset":30904,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":37396,"parent_record_offset":37372,"ancestor_record_offset":30904,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":426,"record_offset":36276,"parent_record_offset":36220,"ancestor_record_offset":30904,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":426,"record_offset":35916,"parent_record_offset":35512,"ancestor_record_offset":30904,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":426,"record_offset":35468,"parent_record_offset":35424,"ancestor_record_offset":30904,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":426,"record_offset":35512,"parent_record_offset":35468,"ancestor_record_offset":30904,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":426,"record_offset":35700,"parent_record_offset":35660,"ancestor_record_offset":30904,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":426,"record_offset":35724,"parent_record_offset":35700,"ancestor_record_offset":30904,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":33216,"parent_record_offset":33188,"ancestor_record_offset":30904,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":34592,"parent_record_offset":34564,"ancestor_record_offset":30904,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":33268,"parent_record_offset":33244,"ancestor_record_offset":30904,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":33244,"parent_record_offset":33216,"ancestor_record_offset":30904,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":33156,"parent_record_offset":33124,"ancestor_record_offset":30904,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":33188,"parent_record_offset":33156,"ancestor_record_offset":30904,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":33360,"parent_record_offset":33188,"ancestor_record_offset":30904,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":33472,"parent_record_offset":33448,"ancestor_record_offset":30904,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":426,"record_offset":33960,"parent_record_offset":33932,"ancestor_record_offset":30904,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":426,"record_offset":36000,"parent_record_offset":35972,"ancestor_record_offset":30904,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":426,"record_offset":36064,"parent_record_offset":36028,"ancestor_record_offset":30904,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":426,"record_offset":35972,"parent_record_offset":35944,"ancestor_record_offset":30904,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":426,"record_offset":36196,"parent_record_offset":36064,"ancestor_record_offset":30904,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":426,"record_offset":38516,"parent_record_offset":38424,"ancestor_record_offset":30904,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":426,"record_offset":37076,"parent_record_offset":37048,"ancestor_record_offset":30904,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":426,"record_offset":33292,"parent_record_offset":33268,"ancestor_record_offset":30904,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":426,"record_offset":36652,"parent_record_offset":36616,"ancestor_record_offset":30904,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":426,"record_offset":31352,"parent_record_offset":31256,"ancestor_record_offset":30904,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":33052,"parent_record_offset":33028,"ancestor_record_offset":30904,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":32308,"parent_record_offset":31848,"ancestor_record_offset":30904,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":33096,"parent_record_offset":32940,"ancestor_record_offset":30904,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":36120,"parent_record_offset":36092,"ancestor_record_offset":30904,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":35200,"parent_record_offset":35148,"ancestor_record_offset":30904,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":426,"record_offset":31664,"parent_record_offset":31636,"ancestor_record_offset":30904,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":33424,"parent_record_offset":33124,"ancestor_record_offset":30904,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":426,"record_offset":36956,"parent_record_offset":36932,"ancestor_record_offset":30904,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":32084,"parent_record_offset":32060,"ancestor_record_offset":30904,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":37192,"parent_record_offset":37136,"ancestor_record_offset":30904,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":32060,"parent_record_offset":32036,"ancestor_record_offset":30904,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":32268,"parent_record_offset":32236,"ancestor_record_offset":30904,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":37216,"parent_record_offset":37192,"ancestor_record_offset":30904,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":35832,"parent_record_offset":35804,"ancestor_record_offset":30904,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":33316,"parent_record_offset":33292,"ancestor_record_offset":30904,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":33824,"parent_record_offset":32908,"ancestor_record_offset":30904,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":32772,"parent_record_offset":32748,"ancestor_record_offset":30904,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":426,"record_offset":31288,"parent_record_offset":31256,"ancestor_record_offset":30904,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":426,"record_offset":31316,"parent_record_offset":31288,"ancestor_record_offset":30904,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":426,"record_offset":36932,"parent_record_offset":36908,"ancestor_record_offset":30904,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":32036,"parent_record_offset":32012,"ancestor_record_offset":30904,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":32364,"parent_record_offset":32340,"ancestor_record_offset":30904,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":36028,"parent_record_offset":36000,"ancestor_record_offset":30904,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":32012,"parent_record_offset":31904,"ancestor_record_offset":30904,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":426,"record_offset":31256,"parent_record_offset":31228,"ancestor_record_offset":30904,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":426,"record_offset":31744,"parent_record_offset":31716,"ancestor_record_offset":30904,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":426,"record_offset":31876,"parent_record_offset":31848,"ancestor_record_offset":30904,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":426,"record_offset":31848,"parent_record_offset":31816,"ancestor_record_offset":30904,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":31792,"parent_record_offset":31768,"ancestor_record_offset":30904,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":31816,"parent_record_offset":31792,"ancestor_record_offset":30904,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":31768,"parent_record_offset":31744,"ancestor_record_offset":30904,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":32340,"parent_record_offset":31816,"ancestor_record_offset":30904,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":31452,"parent_record_offset":31420,"ancestor_record_offset":30904,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":31612,"parent_record_offset":31452,"ancestor_record_offset":30904,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":31420,"parent_record_offset":31396,"ancestor_record_offset":30904,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":31172,"parent_record_offset":31140,"ancestor_record_offset":30904,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":31636,"parent_record_offset":31612,"ancestor_record_offset":30904,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":31228,"parent_record_offset":31196,"ancestor_record_offset":30904,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":31196,"parent_record_offset":31172,"ancestor_record_offset":30904,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":31904,"parent_record_offset":31876,"ancestor_record_offset":30904,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":32152,"parent_record_offset":31904,"ancestor_record_offset":30904,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":32124,"parent_record_offset":31904,"ancestor_record_offset":30904,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":31956,"parent_record_offset":31904,"ancestor_record_offset":30904,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":426,"record_offset":34020,"parent_record_offset":33996,"ancestor_record_offset":30904,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":426,"record_offset":34044,"parent_record_offset":34020,"ancestor_record_offset":30904,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":34292,"parent_record_offset":34268,"ancestor_record_offset":30904,"inlinee":22684,"source_line":11,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":426,"record_offset":33932,"parent_record_offset":33892,"ancestor_record_offset":30904,"inlinee":24709,"source_line":9,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":426,"record_offset":34204,"parent_record_offset":34164,"ancestor_record_offset":30904,"inlinee":24710,"source_line":15,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":426,"record_offset":36092,"parent_record_offset":36064,"ancestor_record_offset":30904,"inlinee":24707,"source_line":27,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":32876,"parent_record_offset":32844,"ancestor_record_offset":30904,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":426,"record_offset":32976,"parent_record_offset":32940,"ancestor_record_offset":30904,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":33124,"parent_record_offset":32940,"ancestor_record_offset":30904,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":33396,"parent_record_offset":33124,"ancestor_record_offset":30904,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::Value::as_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":426,"record_offset":36160,"parent_record_offset":36092,"ancestor_record_offset":30904,"inlinee":24708,"source_line":627,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":426,"record_offset":35172,"parent_record_offset":35148,"ancestor_record_offset":30904,"inlinee":22996,"source_line":123,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":426,"record_offset":34320,"parent_record_offset":34292,"ancestor_record_offset":30904,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":426,"record_offset":32748,"parent_record_offset":32724,"ancestor_record_offset":30904,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":426,"record_offset":34368,"parent_record_offset":34344,"ancestor_record_offset":30904,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":426,"record_offset":34444,"parent_record_offset":34416,"ancestor_record_offset":30904,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":426,"record_offset":34564,"parent_record_offset":34416,"ancestor_record_offset":30904,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":426,"record_offset":34344,"parent_record_offset":34320,"ancestor_record_offset":30904,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":426,"record_offset":33000,"parent_record_offset":32976,"ancestor_record_offset":30904,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":426,"record_offset":33028,"parent_record_offset":33000,"ancestor_record_offset":30904,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":426,"record_offset":32940,"parent_record_offset":32908,"ancestor_record_offset":30904,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":426,"record_offset":32908,"parent_record_offset":32876,"ancestor_record_offset":30904,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>
// 记录形式: compiled_procedure; instances=2
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":426,"record_offset":13372,"rva":5985392,"va":5374694512}
// 编译实例 RVAs: 0x5b5470, 0x1e63de0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$str$u20$as$u20$serde_json..value..index..Index$GT$::index_into
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t10844341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8b578a500","size":5,"soft_fail":false,"target_va":5385539984,"va":5374695638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2382920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8b8a3dbff","size":5,"soft_fail":false,"target_va":5372314208,"va":5374697123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2376552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e898bcdbff","size":5,"soft_fail":false,"target_va":5372320512,"va":5374697059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2070449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e84f68e0ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5374695468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1946045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8434ee2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374695736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=20
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1945886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8e24ee2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374695593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140546de0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-455384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8280df9ff","size":5,"soft_fail":false,"target_va":5374242272,"va":5374697651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bf340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8c7980000","size":5,"soft_fail":false,"target_va":5374735168,"va":5374696052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=2
//     target_aliases=28 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t476491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e84b450700","size":5,"soft_fail":false,"target_va":5375171056,"va":5374694560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1195427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8a33d1200","size":5,"soft_fail":false,"target_va":5375891872,"va":5374696440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e2f30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1233897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8e9d31200","size":5,"soft_fail":false,"target_va":5375930160,"va":5374696258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1238523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8fbe51200","size":5,"soft_fail":false,"target_va":5375936112,"va":5374697584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14084b050
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2709953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8c1592900","size":5,"soft_fail":false,"target_va":5377405008,"va":5374695050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140852b50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2740931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8c3d22900","size":5,"soft_fail":false,"target_va":5377436496,"va":5374695560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d02200
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18227779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400575456,"raw_hex":"e8bddde9fe","size":5,"soft_fail":false,"target_va":5382349312,"va":5400577086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d02460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18228405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400575456,"raw_hex":"e84bdbe9fe","size":5,"soft_fail":false,"target_va":5382349920,"va":5400578320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18211871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400575456,"raw_hex":"e8e11beafe","size":5,"soft_fail":false,"target_va":5382365264,"va":5400577130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d43420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7920691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e833dc7800","size":5,"soft_fail":false,"target_va":5382616096,"va":5374695400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d75540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8125277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e85dfb7b00","size":5,"soft_fail":false,"target_va":5382821184,"va":5374695902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8613043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8b36c8300","size":5,"soft_fail":false,"target_va":5383308144,"va":5374695096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b07d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7027483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400575456,"raw_hex":"e8e5c494ff","size":5,"soft_fail":false,"target_va":5393549264,"va":5400576742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417da3b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6858152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400575456,"raw_hex":"e8585a97ff","size":5,"soft_fail":false,"target_va":5393720240,"va":5400578387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141eea7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t549943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400575456,"raw_hex":"e837640800","size":5,"soft_fail":false,"target_va":5401126832,"va":5400576884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28281270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8b689af01","size":5,"soft_fail":false,"target_va":5402978144,"va":5374696869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2666888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e888b12800","size":5,"soft_fail":false,"target_va":5377364336,"va":5374697443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28281786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8ba8baf01","size":5,"soft_fail":false,"target_va":5402978688,"va":5374696897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t28282260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8948daf01","size":5,"soft_fail":false,"target_va":5402978880,"va":5374696615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28282222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e86e8daf01","size":5,"soft_fail":false,"target_va":5402978944,"va":5374696717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-8962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8fedcffff","size":5,"soft_fail":false,"target_va":5374687520,"va":5374696477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28283539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e89392af01","size":5,"soft_fail":false,"target_va":5402980128,"va":5374696584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-6891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e815e5ffff","size":5,"soft_fail":false,"target_va":5374687712,"va":5374694598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t28285976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8189caf01","size":5,"soft_fail":false,"target_va":5402980656,"va":5374694675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16009054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400575456,"raw_hex":"e8a2b80bff","size":5,"soft_fail":false,"target_va":5384567376,"va":5400576425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::set_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16009094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400575456,"raw_hex":"e87ab80bff","size":5,"soft_fail":false,"target_va":5384567584,"va":5400576673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::set_source_file
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16008968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400575456,"raw_hex":"e8f8b80bff","size":5,"soft_fail":false,"target_va":5384567504,"va":5400576467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::load_json::read_attribute_from_json
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10958831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8ef37a700","size":5,"soft_fail":false,"target_va":5385654832,"va":5374695996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2658390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e856902800","size":5,"soft_fail":false,"target_va":5377353776,"va":5374695381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1661745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8315b1900","size":5,"soft_fail":false,"target_va":5376356912,"va":5374695162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1665481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8c9691900","size":5,"soft_fail":false,"target_va":5376360560,"va":5374695074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1661560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8785a1900","size":5,"soft_fail":false,"target_va":5376356144,"va":5374694579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::from_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t546180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400575456,"raw_hex":"e884550800","size":5,"soft_fail":false,"target_va":5401122560,"va":5400576375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2710397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e87d5b2900","size":5,"soft_fail":false,"target_va":5377405616,"va":5374695214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2670773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374694512,"raw_hex":"e8b5c02800","size":5,"soft_fail":false,"target_va":5377366224,"va":5374695446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18286253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400575456,"raw_hex":"e853f9e8fe","size":5,"soft_fail":false,"target_va":5382289808,"va":5400576056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":19696,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":19784,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":426,"record_offset":16728,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":426,"record_offset":16756,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":426,"record_offset":18028,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":15456,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":19972,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":19900,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":16540,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":426,"record_offset":19556,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":16916,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":24287,"source_line":24,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":19140,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":24560,"source_line":48,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_source_file
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":17440,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":24704,"source_line":32,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":426,"record_offset":16784,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":426,"record_offset":16644,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":15492,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":13788,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":14364,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":426,"record_offset":17964,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":24705,"source_line":424,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::index::impl$5::index
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":426,"record_offset":17796,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":22994,"source_line":212,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":426,"record_offset":16512,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":426,"record_offset":15372,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":10965,"source_line":2109,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$56::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":426,"record_offset":17412,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":4727,"source_line":1902,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":426,"record_offset":13620,"parent_record_offset":13372,"ancestor_record_offset":13372,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":18428,"parent_record_offset":18404,"ancestor_record_offset":13372,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":18452,"parent_record_offset":18428,"ancestor_record_offset":13372,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":17728,"parent_record_offset":17704,"ancestor_record_offset":13372,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":18404,"parent_record_offset":18308,"ancestor_record_offset":13372,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":17704,"parent_record_offset":17648,"ancestor_record_offset":13372,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":426,"record_offset":21072,"parent_record_offset":21044,"ancestor_record_offset":13372,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":426,"record_offset":21132,"parent_record_offset":21072,"ancestor_record_offset":13372,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":17676,"parent_record_offset":17648,"ancestor_record_offset":13372,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":20380,"parent_record_offset":20356,"ancestor_record_offset":13372,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":18308,"parent_record_offset":18276,"ancestor_record_offset":13372,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":18276,"parent_record_offset":18244,"ancestor_record_offset":13372,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":18524,"parent_record_offset":18276,"ancestor_record_offset":13372,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":17648,"parent_record_offset":17620,"ancestor_record_offset":13372,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":20356,"parent_record_offset":20328,"ancestor_record_offset":13372,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":18592,"parent_record_offset":18564,"ancestor_record_offset":13372,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":18212,"parent_record_offset":18160,"ancestor_record_offset":13372,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":18244,"parent_record_offset":18212,"ancestor_record_offset":13372,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":18072,"parent_record_offset":18028,"ancestor_record_offset":13372,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":18868,"parent_record_offset":18844,"ancestor_record_offset":13372,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":20328,"parent_record_offset":20296,"ancestor_record_offset":13372,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":19996,"parent_record_offset":19972,"ancestor_record_offset":13372,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":20044,"parent_record_offset":20020,"ancestor_record_offset":13372,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":426,"record_offset":18924,"parent_record_offset":18868,"ancestor_record_offset":13372,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":426,"record_offset":18564,"parent_record_offset":18160,"ancestor_record_offset":13372,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":426,"record_offset":18116,"parent_record_offset":18072,"ancestor_record_offset":13372,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":426,"record_offset":18160,"parent_record_offset":18116,"ancestor_record_offset":13372,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":426,"record_offset":18348,"parent_record_offset":18308,"ancestor_record_offset":13372,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":426,"record_offset":18372,"parent_record_offset":18348,"ancestor_record_offset":13372,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":15864,"parent_record_offset":15836,"ancestor_record_offset":13372,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":17240,"parent_record_offset":17212,"ancestor_record_offset":13372,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":15916,"parent_record_offset":15892,"ancestor_record_offset":13372,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":15892,"parent_record_offset":15864,"ancestor_record_offset":13372,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":15804,"parent_record_offset":15772,"ancestor_record_offset":13372,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":15836,"parent_record_offset":15804,"ancestor_record_offset":13372,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":16008,"parent_record_offset":15836,"ancestor_record_offset":13372,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":16120,"parent_record_offset":16096,"ancestor_record_offset":13372,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":426,"record_offset":16608,"parent_record_offset":16580,"ancestor_record_offset":13372,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":426,"record_offset":18648,"parent_record_offset":18620,"ancestor_record_offset":13372,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":426,"record_offset":18712,"parent_record_offset":18676,"ancestor_record_offset":13372,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":426,"record_offset":18620,"parent_record_offset":18592,"ancestor_record_offset":13372,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":426,"record_offset":18844,"parent_record_offset":18712,"ancestor_record_offset":13372,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":426,"record_offset":21164,"parent_record_offset":21072,"ancestor_record_offset":13372,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":426,"record_offset":19724,"parent_record_offset":19696,"ancestor_record_offset":13372,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":426,"record_offset":15940,"parent_record_offset":15916,"ancestor_record_offset":13372,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":426,"record_offset":19300,"parent_record_offset":19264,"ancestor_record_offset":13372,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":426,"record_offset":14000,"parent_record_offset":13904,"ancestor_record_offset":13372,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":15700,"parent_record_offset":15676,"ancestor_record_offset":13372,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":14956,"parent_record_offset":14496,"ancestor_record_offset":13372,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":15744,"parent_record_offset":15588,"ancestor_record_offset":13372,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":18768,"parent_record_offset":18740,"ancestor_record_offset":13372,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":17848,"parent_record_offset":17796,"ancestor_record_offset":13372,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":426,"record_offset":14312,"parent_record_offset":14284,"ancestor_record_offset":13372,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=75
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":16072,"parent_record_offset":15772,"ancestor_record_offset":13372,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":426,"record_offset":19604,"parent_record_offset":19580,"ancestor_record_offset":13372,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":14732,"parent_record_offset":14708,"ancestor_record_offset":13372,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":19840,"parent_record_offset":19784,"ancestor_record_offset":13372,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":14708,"parent_record_offset":14684,"ancestor_record_offset":13372,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":14916,"parent_record_offset":14884,"ancestor_record_offset":13372,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":19864,"parent_record_offset":19840,"ancestor_record_offset":13372,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":18480,"parent_record_offset":18452,"ancestor_record_offset":13372,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":15964,"parent_record_offset":15940,"ancestor_record_offset":13372,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":16472,"parent_record_offset":15556,"ancestor_record_offset":13372,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":426,"record_offset":15420,"parent_record_offset":15396,"ancestor_record_offset":13372,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":426,"record_offset":13936,"parent_record_offset":13904,"ancestor_record_offset":13372,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":426,"record_offset":13964,"parent_record_offset":13936,"ancestor_record_offset":13372,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":426,"record_offset":19580,"parent_record_offset":19556,"ancestor_record_offset":13372,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":14684,"parent_record_offset":14660,"ancestor_record_offset":13372,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":15012,"parent_record_offset":14988,"ancestor_record_offset":13372,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":18676,"parent_record_offset":18648,"ancestor_record_offset":13372,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":14660,"parent_record_offset":14552,"ancestor_record_offset":13372,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":426,"record_offset":13904,"parent_record_offset":13876,"ancestor_record_offset":13372,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":426,"record_offset":14392,"parent_record_offset":14364,"ancestor_record_offset":13372,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":426,"record_offset":14524,"parent_record_offset":14496,"ancestor_record_offset":13372,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":426,"record_offset":14496,"parent_record_offset":14464,"ancestor_record_offset":13372,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":14440,"parent_record_offset":14416,"ancestor_record_offset":13372,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":14464,"parent_record_offset":14440,"ancestor_record_offset":13372,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":14416,"parent_record_offset":14392,"ancestor_record_offset":13372,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":14988,"parent_record_offset":14464,"ancestor_record_offset":13372,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":14100,"parent_record_offset":14068,"ancestor_record_offset":13372,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":14260,"parent_record_offset":14100,"ancestor_record_offset":13372,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":14068,"parent_record_offset":14044,"ancestor_record_offset":13372,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":13820,"parent_record_offset":13788,"ancestor_record_offset":13372,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":14284,"parent_record_offset":14260,"ancestor_record_offset":13372,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":13876,"parent_record_offset":13844,"ancestor_record_offset":13372,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":13844,"parent_record_offset":13820,"ancestor_record_offset":13372,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":14552,"parent_record_offset":14524,"ancestor_record_offset":13372,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":14800,"parent_record_offset":14552,"ancestor_record_offset":13372,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":14772,"parent_record_offset":14552,"ancestor_record_offset":13372,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":14604,"parent_record_offset":14552,"ancestor_record_offset":13372,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":426,"record_offset":16668,"parent_record_offset":16644,"ancestor_record_offset":13372,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":426,"record_offset":16692,"parent_record_offset":16668,"ancestor_record_offset":13372,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":426,"record_offset":16940,"parent_record_offset":16916,"ancestor_record_offset":13372,"inlinee":22684,"source_line":11,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":426,"record_offset":16580,"parent_record_offset":16540,"ancestor_record_offset":13372,"inlinee":24709,"source_line":9,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":426,"record_offset":16852,"parent_record_offset":16812,"ancestor_record_offset":13372,"inlinee":24710,"source_line":15,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":426,"record_offset":18740,"parent_record_offset":18712,"ancestor_record_offset":13372,"inlinee":24707,"source_line":27,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":15524,"parent_record_offset":15492,"ancestor_record_offset":13372,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":426,"record_offset":15624,"parent_record_offset":15588,"ancestor_record_offset":13372,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":15772,"parent_record_offset":15588,"ancestor_record_offset":13372,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":16044,"parent_record_offset":15772,"ancestor_record_offset":13372,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::Value::as_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":426,"record_offset":18808,"parent_record_offset":18740,"ancestor_record_offset":13372,"inlinee":24708,"source_line":627,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":426,"record_offset":17820,"parent_record_offset":17796,"ancestor_record_offset":13372,"inlinee":22996,"source_line":123,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":426,"record_offset":16968,"parent_record_offset":16940,"ancestor_record_offset":13372,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":426,"record_offset":15396,"parent_record_offset":15372,"ancestor_record_offset":13372,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":426,"record_offset":13692,"parent_record_offset":13668,"ancestor_record_offset":13372,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":426,"record_offset":13740,"parent_record_offset":13716,"ancestor_record_offset":13372,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":426,"record_offset":13716,"parent_record_offset":13692,"ancestor_record_offset":13372,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":426,"record_offset":17016,"parent_record_offset":16992,"ancestor_record_offset":13372,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":426,"record_offset":17092,"parent_record_offset":17064,"ancestor_record_offset":13372,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":426,"record_offset":17212,"parent_record_offset":17064,"ancestor_record_offset":13372,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":426,"record_offset":16992,"parent_record_offset":16968,"ancestor_record_offset":13372,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":426,"record_offset":13668,"parent_record_offset":13644,"ancestor_record_offset":13372,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":426,"record_offset":13644,"parent_record_offset":13620,"ancestor_record_offset":13372,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":426,"record_offset":15648,"parent_record_offset":15624,"ancestor_record_offset":13372,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":426,"record_offset":15676,"parent_record_offset":15648,"ancestor_record_offset":13372,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":426,"record_offset":15588,"parent_record_offset":15556,"ancestor_record_offset":13372,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":426,"record_offset":15556,"parent_record_offset":15524,"ancestor_record_offset":13372,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6085
// 首条位置: {"module":426,"inlinee":24709}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":426,"record_offset":16608,"parent_record_offset":16580,"ancestor_record_offset":16580,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6086
// 首条位置: {"module":426,"inlinee":24710}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":426,"record_offset":16880,"parent_record_offset":16852,"ancestor_record_offset":16852,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=23
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6083
// 首条位置: {"module":426,"inlinee":24707}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":426,"record_offset":18768,"parent_record_offset":18740,"ancestor_record_offset":18740,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_u64
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":426,"record_offset":18808,"parent_record_offset":18740,"ancestor_record_offset":18740,"inlinee":24708,"source_line":627,"archive":"_pdb/A/modules/00426.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF038
// 首条位置: {"module":1308,"inlinee":61496}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1308,"record_offset":114608,"parent_record_offset":114584,"ancestor_record_offset":114584,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1394,"record_offset":27660,"rva":16945712,"va":5385654832}
// 编译实例 RVAs: 0x1029230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$str$u20$as$u20$serde_json..value..index..Index$GT$::index_into
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t-114920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385654832,"raw_hex":"e8183ffeff","size":5,"soft_fail":false,"target_va":5385539984,"va":5385654899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12905780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385654832,"raw_hex":"e8cc123bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5385655487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d765e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2830458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385654832,"raw_hex":"e886cfd4ff","size":5,"soft_fail":false,"target_va":5382825440,"va":5385655893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d788d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2821222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385654832,"raw_hex":"e89af3d4ff","size":5,"soft_fail":false,"target_va":5382834384,"va":5385655601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d78bd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2820123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385654832,"raw_hex":"e8e5f7d4ff","size":5,"soft_fail":false,"target_va":5382835152,"va":5385655270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d7c3d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2806248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385654832,"raw_hex":"e8182ed5ff","size":5,"soft_fail":false,"target_va":5382849488,"va":5385655731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d81df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2783500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385654832,"raw_hex":"e8f486d5ff","size":5,"soft_fail":false,"target_va":5382872560,"va":5385656055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420ca2d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17436207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385654832,"raw_hex":"e82f0e0a01","size":5,"soft_fail":false,"target_va":5403091664,"va":5385655452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8125149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385654832,"raw_hex":"e8230584ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5385655032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t17322769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385654832,"raw_hex":"e811530801","size":5,"soft_fail":false,"target_va":5402978880,"va":5385656106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t17322995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385654832,"raw_hex":"e8f3530801","size":5,"soft_fail":false,"target_va":5402979152,"va":5385656152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1394,"record_offset":28000,"parent_record_offset":27660,"ancestor_record_offset":27660,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1394,"record_offset":28908,"parent_record_offset":27660,"ancestor_record_offset":27660,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1394,"record_offset":28852,"parent_record_offset":27660,"ancestor_record_offset":27660,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1394,"record_offset":28732,"parent_record_offset":27660,"ancestor_record_offset":27660,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1394,"record_offset":29428,"parent_record_offset":27660,"ancestor_record_offset":27660,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1394,"record_offset":28768,"parent_record_offset":27660,"ancestor_record_offset":27660,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1394,"record_offset":28084,"parent_record_offset":27660,"ancestor_record_offset":27660,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1394,"record_offset":28704,"parent_record_offset":27660,"ancestor_record_offset":27660,"inlinee":24705,"source_line":424,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::index::impl$5::index
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":1394,"record_offset":27832,"parent_record_offset":27660,"ancestor_record_offset":27660,"inlinee":22994,"source_line":212,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1394,"record_offset":29180,"parent_record_offset":29156,"ancestor_record_offset":27660,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1394,"record_offset":29156,"parent_record_offset":29056,"ancestor_record_offset":27660,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1394,"record_offset":28048,"parent_record_offset":28024,"ancestor_record_offset":27660,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1394,"record_offset":29088,"parent_record_offset":29056,"ancestor_record_offset":27660,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1394,"record_offset":29056,"parent_record_offset":29028,"ancestor_record_offset":27660,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1394,"record_offset":28976,"parent_record_offset":28940,"ancestor_record_offset":27660,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1394,"record_offset":28876,"parent_record_offset":28852,"ancestor_record_offset":27660,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1394,"record_offset":28108,"parent_record_offset":28084,"ancestor_record_offset":27660,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1394,"record_offset":31404,"parent_record_offset":31380,"ancestor_record_offset":27660,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1394,"record_offset":29120,"parent_record_offset":29088,"ancestor_record_offset":27660,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1394,"record_offset":28024,"parent_record_offset":28000,"ancestor_record_offset":27660,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1394,"record_offset":27884,"parent_record_offset":27832,"ancestor_record_offset":27660,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1394,"record_offset":29004,"parent_record_offset":28940,"ancestor_record_offset":27660,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1394,"record_offset":28816,"parent_record_offset":28792,"ancestor_record_offset":27660,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1394,"record_offset":28132,"parent_record_offset":28108,"ancestor_record_offset":27660,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1394,"record_offset":28156,"parent_record_offset":28132,"ancestor_record_offset":27660,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1394,"record_offset":28792,"parent_record_offset":28768,"ancestor_record_offset":27660,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1394,"record_offset":30744,"parent_record_offset":30708,"ancestor_record_offset":27660,"inlinee":24878,"source_line":264,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$6::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1394,"record_offset":28940,"parent_record_offset":28908,"ancestor_record_offset":27660,"inlinee":24880,"source_line":264,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$7::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1394,"record_offset":32208,"parent_record_offset":32172,"ancestor_record_offset":27660,"inlinee":24874,"source_line":264,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$8::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1394,"record_offset":31476,"parent_record_offset":31440,"ancestor_record_offset":27660,"inlinee":24872,"source_line":264,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$9::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1394,"record_offset":30040,"parent_record_offset":30004,"ancestor_record_offset":27660,"inlinee":24876,"source_line":264,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":1394,"record_offset":27856,"parent_record_offset":27832,"ancestor_record_offset":27660,"inlinee":22996,"source_line":123,"archive":"_pdb/A/modules/01394.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF14D
// 首条位置: {"module":1321,"inlinee":61773}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":44676,"parent_record_offset":44420,"ancestor_record_offset":44420,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1321,"record_offset":44620,"parent_record_offset":44420,"ancestor_record_offset":44420,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1321,"record_offset":44500,"parent_record_offset":44420,"ancestor_record_offset":44420,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":45060,"parent_record_offset":44420,"ancestor_record_offset":44420,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1321,"record_offset":44536,"parent_record_offset":44420,"ancestor_record_offset":44420,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1321,"record_offset":44472,"parent_record_offset":44420,"ancestor_record_offset":44420,"inlinee":24705,"source_line":424,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":45224,"parent_record_offset":45200,"ancestor_record_offset":44420,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":45200,"parent_record_offset":45108,"ancestor_record_offset":44420,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":45136,"parent_record_offset":45108,"ancestor_record_offset":44420,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":45108,"parent_record_offset":45084,"ancestor_record_offset":44420,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":44644,"parent_record_offset":44620,"ancestor_record_offset":44420,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1321,"record_offset":45164,"parent_record_offset":45136,"ancestor_record_offset":44420,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":45084,"parent_record_offset":45060,"ancestor_record_offset":44420,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1321,"record_offset":44584,"parent_record_offset":44560,"ancestor_record_offset":44420,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1321,"record_offset":44704,"parent_record_offset":44676,"ancestor_record_offset":44420,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1321,"record_offset":44732,"parent_record_offset":44704,"ancestor_record_offset":44420,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1321,"record_offset":44560,"parent_record_offset":44536,"ancestor_record_offset":44420,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF032
// 首条位置: {"module":1308,"inlinee":61490}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1308,"record_offset":106336,"parent_record_offset":106228,"ancestor_record_offset":106228,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1308,"record_offset":106260,"parent_record_offset":106228,"ancestor_record_offset":106228,"inlinee":28287,"source_line":646,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::Number::as_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1308,"record_offset":106292,"parent_record_offset":106260,"ancestor_record_offset":106228,"inlinee":28288,"source_line":240,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF13A
// 首条位置: {"module":1321,"inlinee":61754}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":23336,"parent_record_offset":23024,"ancestor_record_offset":23024,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1321,"record_offset":23280,"parent_record_offset":23024,"ancestor_record_offset":23024,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1321,"record_offset":23104,"parent_record_offset":23024,"ancestor_record_offset":23024,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":23624,"parent_record_offset":23024,"ancestor_record_offset":23024,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1321,"record_offset":23196,"parent_record_offset":23024,"ancestor_record_offset":23024,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1321,"record_offset":23076,"parent_record_offset":23024,"ancestor_record_offset":23024,"inlinee":24705,"source_line":424,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":23788,"parent_record_offset":23764,"ancestor_record_offset":23024,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":23764,"parent_record_offset":23672,"ancestor_record_offset":23024,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":23700,"parent_record_offset":23672,"ancestor_record_offset":23024,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":23672,"parent_record_offset":23648,"ancestor_record_offset":23024,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":23304,"parent_record_offset":23280,"ancestor_record_offset":23024,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1321,"record_offset":23160,"parent_record_offset":23136,"ancestor_record_offset":23024,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1321,"record_offset":23728,"parent_record_offset":23700,"ancestor_record_offset":23024,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":23648,"parent_record_offset":23624,"ancestor_record_offset":23024,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1321,"record_offset":23244,"parent_record_offset":23220,"ancestor_record_offset":23024,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1321,"record_offset":23364,"parent_record_offset":23336,"ancestor_record_offset":23024,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1321,"record_offset":23392,"parent_record_offset":23364,"ancestor_record_offset":23024,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1321,"record_offset":23220,"parent_record_offset":23196,"ancestor_record_offset":23024,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::read_attribute_from_json::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":1321,"record_offset":23136,"parent_record_offset":23104,"ancestor_record_offset":23024,"inlinee":61755,"source_line":81,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF13B
// 首条位置: {"module":1321,"inlinee":61755}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1321,"record_offset":23160,"parent_record_offset":23136,"ancestor_record_offset":23136,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$1::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF070
// 首条位置: {"module":1308,"inlinee":61552}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1308,"record_offset":225664,"parent_record_offset":225556,"ancestor_record_offset":225556,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1308,"record_offset":225588,"parent_record_offset":225556,"ancestor_record_offset":225556,"inlinee":28287,"source_line":646,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::Number::as_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1308,"record_offset":225620,"parent_record_offset":225588,"ancestor_record_offset":225556,"inlinee":28288,"source_line":240,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF13C
// 首条位置: {"module":1321,"inlinee":61756}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":25656,"parent_record_offset":25344,"ancestor_record_offset":25344,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1321,"record_offset":25600,"parent_record_offset":25344,"ancestor_record_offset":25344,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1321,"record_offset":25424,"parent_record_offset":25344,"ancestor_record_offset":25344,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":25752,"parent_record_offset":25344,"ancestor_record_offset":25344,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1321,"record_offset":25516,"parent_record_offset":25344,"ancestor_record_offset":25344,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1321,"record_offset":25396,"parent_record_offset":25344,"ancestor_record_offset":25344,"inlinee":24705,"source_line":424,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":25920,"parent_record_offset":25896,"ancestor_record_offset":25344,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":25896,"parent_record_offset":25804,"ancestor_record_offset":25344,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":25832,"parent_record_offset":25804,"ancestor_record_offset":25344,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":25804,"parent_record_offset":25780,"ancestor_record_offset":25344,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":25624,"parent_record_offset":25600,"ancestor_record_offset":25344,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1321,"record_offset":25480,"parent_record_offset":25456,"ancestor_record_offset":25344,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1321,"record_offset":25860,"parent_record_offset":25832,"ancestor_record_offset":25344,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":25780,"parent_record_offset":25752,"ancestor_record_offset":25344,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1321,"record_offset":25564,"parent_record_offset":25540,"ancestor_record_offset":25344,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1321,"record_offset":25684,"parent_record_offset":25656,"ancestor_record_offset":25344,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1321,"record_offset":25712,"parent_record_offset":25684,"ancestor_record_offset":25344,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1321,"record_offset":25540,"parent_record_offset":25516,"ancestor_record_offset":25344,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::read_attribute_from_json::closure$2::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":1321,"record_offset":25456,"parent_record_offset":25424,"ancestor_record_offset":25344,"inlinee":61757,"source_line":104,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$2::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF13D
// 首条位置: {"module":1321,"inlinee":61757}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1321,"record_offset":25480,"parent_record_offset":25456,"ancestor_record_offset":25456,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$2::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF086
// 首条位置: {"module":1308,"inlinee":61574}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1308,"record_offset":353628,"parent_record_offset":353520,"ancestor_record_offset":353520,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1308,"record_offset":353552,"parent_record_offset":353520,"ancestor_record_offset":353520,"inlinee":28287,"source_line":646,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::Number::as_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1308,"record_offset":353584,"parent_record_offset":353552,"ancestor_record_offset":353520,"inlinee":28288,"source_line":240,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF06C
// 首条位置: {"module":1308,"inlinee":61548}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1308,"record_offset":219864,"parent_record_offset":219760,"ancestor_record_offset":219760,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1308,"record_offset":219788,"parent_record_offset":219760,"ancestor_record_offset":219760,"inlinee":28283,"source_line":608,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::Number::as_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1308,"record_offset":219820,"parent_record_offset":219788,"ancestor_record_offset":219760,"inlinee":28284,"source_line":188,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF056
// 首条位置: {"module":1308,"inlinee":61526}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1308,"record_offset":171328,"parent_record_offset":171220,"ancestor_record_offset":171220,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1308,"record_offset":171252,"parent_record_offset":171220,"ancestor_record_offset":171220,"inlinee":28287,"source_line":646,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::Number::as_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1308,"record_offset":171284,"parent_record_offset":171252,"ancestor_record_offset":171220,"inlinee":28288,"source_line":240,"archive":"_pdb/A/modules/01308.jsonl.gz"}
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
// 完整结构及原始类型记录: load_json.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_json.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_json.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 8; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh::closure$0
// 信息 ID: 40dafda5f8e0923979bcc8d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::load_json::load_json_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>; module=426, inline_record=16580; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<alloc::string::String> >; module=426, inline_record=25580; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<str$> >; module=426, inline_record=33932; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> >; module=426, inline_record=42496; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<alloc::string::String>; module=426, inline_record=50856; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<str$> >; module=4260, inline_record=3268; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>; module=4370, inline_record=3452; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<alloc::string::String>; module=4370, inline_record=11040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh::closure$1
// 信息 ID: 6ea2ce7d81ece321dc4ca2cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::load_json::load_json_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>; module=426, inline_record=16852; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<alloc::string::String> >; module=426, inline_record=25852; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<str$> >; module=426, inline_record=34204; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> >; module=426, inline_record=42768; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<alloc::string::String>; module=426, inline_record=51128; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<str$> >; module=4260, inline_record=3540; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>; module=4370, inline_record=3724; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<alloc::string::String>; module=4370, inline_record=11312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh::closure$2
// 信息 ID: bfe210f83376915fe7459afa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::utils::load_json::load_json_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>; module=426, inline_record=18740; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<alloc::string::String> >; module=426, inline_record=27740; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<str$> >; module=426, inline_record=36092; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> >; module=426, inline_record=44656; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<alloc::string::String>; module=426, inline_record=53016; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<str$> >; module=4260, inline_record=4580; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>; module=4370, inline_record=4764; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<alloc::string::String>; module=4370, inline_record=12352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh::closure$3
// 信息 ID: 4f39f37224998ee0485b7101
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "country_core::utils::load_json::load_json_as_mesh", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::load_json_as_mesh::closure_env$3 <- alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_as_mesh::closure_env$3<std::path::PathBuf> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::load_json_as_mesh::closure_env$3 <- alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_as_mesh::closure_env$3<ref$<str$> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::load_json_as_mesh::closure_env$3 <- alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_as_mesh::closure_env$3<alloc::string::String> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_as_mesh::closure_env$3<std::path::PathBuf> > >; module=1308, inline_record=114584; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_as_mesh::closure_env$3<ref$<str$> > > >; module=2281, inline_record=32728; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_as_mesh::closure_env$3<alloc::string::String> > >; module=2756, inline_record=32660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh<alloc::string::String>
// 信息 ID: b6f87f08671aa640f455de8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::load_json::load_json_as_mesh -> ["alloc::string::String"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<country_core::render::mesh::Mesh,core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$19::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > > > @ 0xde6a6b -> 0x5b8820 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<country_core::render::mesh::Mesh,core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$19::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > > > @ 0xde6b5a -> 0x5b8820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators::closure$1 @ 0x1c482bd -> 0x1e64a60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh<ref$<alloc::string::String> >
// 信息 ID: 5f99b601d3386440d8d29597
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] country_core::utils::load_json::load_json_as_mesh -> ["ref$<alloc::string::String>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::spawn_brush_previews::spawn_brush_previews @ 0x698c42 -> 0x5b61b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> >
// 信息 ID: 7b9b1159555c111524a2bbc6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["std::path::PathBuf"]; 字段布局 = unknown。
// [类型关系] country_core::utils::load_json::load_json_as_mesh -> ["ref$<std::path::PathBuf>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0> @ 0x1008a7f -> 0x5b7b50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh<ref$<str$> >
// 信息 ID: b5f37f76cb36609aad0937ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] country_core::utils::load_json::load_json_as_mesh -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 67 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_glade_tod_and_weather::load_rain_particles @ 0x53856c -> 0x5b6e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_garden_brush::startup_garden_brush @ 0x538a33 -> 0x5b6e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_garden_brush::startup_garden_brush @ 0x538c03 -> 0x5b6e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sheep::startup_sheep @ 0x598725 -> 0x5b6e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sheep::startup_sheep @ 0x5988af -> 0x5b6e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sheep::startup_sheep @ 0x598a47 -> 0x5b6e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sheep::startup_sheep @ 0x5978a2 -> 0x5b6e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sheep::startup_sheep @ 0x597939 -> 0x5b6e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::logo_animation::startup_logo_test @ 0x59aba1 -> 0x5b6e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::logo_animation::startup_logo_test @ 0x59ad73 -> 0x5b6e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_roof::startup_roof @ 0x59bee7 -> 0x5b6e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_roof::startup_roof @ 0x59c0b5 -> 0x5b6e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_roof::startup_roof @ 0x59c24a -> 0x5b6e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_roof::startup_roof @ 0x59c374 -> 0x5b6e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x604bd7 -> 0x5b6e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x604f86 -> 0x5b6e80 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>
// 信息 ID: 767ac1bd8017e7a98a8776a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::load_json::load_json_as_mesh -> ["std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_meadow::init_meadow @ 0x653987 -> 0x5b5470 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::clearing::init_clearing @ 0x6c8e36 -> 0x5b5470 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::load_prefabs @ 0x111110d -> 0x5b5470 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x1c43b28 -> 0x1e63de0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json
// 信息 ID: ce2314d5f3bb6c96a5404e17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> @ 0x5b5a3c -> 0x1029230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<alloc::string::String> > @ 0x5b677c -> 0x1029230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0x5b744c -> 0x1029230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> > @ 0x5b811c -> 0x1029230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<alloc::string::String> @ 0x5b8e0c -> 0x1029230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0x1dd1c6c -> 0x1029230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> @ 0x1e6433c -> 0x1029230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<alloc::string::String> @ 0x1e64fcc -> 0x1029230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$0
// 信息 ID: 1c80f0ab73042be2e124c11b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,4>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$0 <- core::iter::adapters::map::impl$2::fold<array$<f32,4>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,4>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,4>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$0 <- core::iter::adapters::map::impl$2::fold<array$<f32,4>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,4>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,4>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<f32,4>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,4>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,4>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$0> > > >; module=1321, inline_record=44420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$0::closure$0
// 信息 ID: e201dba77d9fed051e7863d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure$0::closure_env$0> >; module=1308, inline_record=106228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$1
// 信息 ID: d4ddc8c346225ce2f2e3e56e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$1 <- core::iter::adapters::map::impl$2::fold<array$<f32,3>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$1 <- core::iter::adapters::map::impl$2::fold<array$<f32,3>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<f32,3>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1> > > >; module=1321, inline_record=23024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$1::closure$0
// 信息 ID: 027183cd6a58a52de4dc33e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<f32,3>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1> > > >; module=1321, inline_record=23136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$1::closure$1
// 信息 ID: 6b244af4dab64c974bec1ccf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure$1::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure$1::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure$1::closure_env$1> >; module=1308, inline_record=225556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$2
// 信息 ID: 3d7c00d6694e3a831d775441
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,2>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$2 <- core::iter::adapters::map::impl$2::fold<array$<f32,2>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,2>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$2 <- core::iter::adapters::map::impl$2::fold<array$<f32,2>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,2>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<f32,2>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,2>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2> > > >; module=1321, inline_record=25344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$2::closure$0
// 信息 ID: 029f8ea2fd48dcbb2b107cae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<f32,2>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,2>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2> > > >; module=1321, inline_record=25456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$2::closure$1
// 信息 ID: 1bf5ee86b4eeb339b10dc1bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure$2::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure$2::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure$2::closure_env$1> >; module=1308, inline_record=353520; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$3
// 信息 ID: 04a4f18be35675c9f99a87ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$3 <- alloc::vec::spec_from_iter::impl$0::from_iter<i32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<i32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$3> >; module=1308, inline_record=219760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$4
// 信息 ID: eaefbac7d973f763551b67eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$4 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$4> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$4> >; module=1308, inline_record=171220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
