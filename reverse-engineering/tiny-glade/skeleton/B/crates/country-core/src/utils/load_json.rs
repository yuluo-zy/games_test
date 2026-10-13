// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/utils/load_json.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 36fda2a7aed9759522b8c91bc16c4a7d671b2b12e4dc56c8a7d24cbe56aacb4b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_json.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 476, 577, 1503, 1534, 1546, 1585, 1610, 2611, 3796, 4480
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
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
// 首条位置: {"module":577,"record_offset":48960,"rva":11603664,"va":5380312784}
// 编译实例 RVAs: 0xb10ed0, 0x1287d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2330739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e873902300","size":5,"soft_fail":false,"target_va":5382644528,"va":5380313784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10713063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e819885cff","size":5,"soft_fail":false,"target_va":5369600800,"va":5380313858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5415414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e80a5eadff","size":5,"soft_fail":false,"target_va":5374900736,"va":5380316145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5412360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e8f869adff","size":5,"soft_fail":false,"target_va":5374904480,"va":5380316835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5232683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e8d527b0ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380314166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=22
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5232520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e87828b0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380314019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4797626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e846cbb6ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5380313653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085da90
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2834952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e8f8bdd4ff","size":5,"soft_fail":false,"target_va":5377481360,"va":5380316307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2658724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e85c6ed7ff","size":5,"soft_fail":false,"target_va":5377657648,"va":5380316367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888d70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2656851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e8ad75d7ff","size":5,"soft_fail":false,"target_va":5377658224,"va":5380315070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2590926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e83277d8ff","size":5,"soft_fail":false,"target_va":5377724224,"va":5380315145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140949170
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9696044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388139824,"raw_hex":"e8d40c6cff","size":5,"soft_fail":false,"target_va":5378445680,"va":5388141719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a35240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-902366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e8223bf2ff","size":5,"soft_fail":false,"target_va":5379412544,"va":5380314905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2391382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e8567d2400","size":5,"soft_fail":false,"target_va":5382704752,"va":5380313365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2426745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e879072500","size":5,"soft_fail":false,"target_va":5382740736,"va":5380313986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141259400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-195112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388139824,"raw_hex":"e8d805fdff","size":5,"soft_fail":false,"target_va":5387949056,"va":5388144163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141259560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-192611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388139824,"raw_hex":"e89d0ffdff","size":5,"soft_fail":false,"target_va":5387949408,"va":5388142014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412598b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-193765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388139824,"raw_hex":"e81b0bfdff","size":5,"soft_fail":false,"target_va":5387950256,"va":5388144016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125a270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-189471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388139824,"raw_hex":"e8e11bfdff","size":5,"soft_fail":false,"target_va":5387952752,"va":5388142218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141288e80
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388139824,"raw_hex":"e868030000","size":5,"soft_fail":false,"target_va":5388144256,"va":5388143379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9389661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e85d468f00","size":5,"soft_fail":false,"target_va":5389703472,"va":5380313806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414744d0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9842935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e8f7309600","size":5,"soft_fail":false,"target_va":5390157008,"va":5380314068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420054c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14144431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388139824,"raw_hex":"e8afd3d700","size":5,"soft_fail":false,"target_va":5402285248,"va":5388140812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142006a50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14148980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388139824,"raw_hex":"e874e5d700","size":5,"soft_fail":false,"target_va":5402290768,"va":5388141783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31356542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e87e76de01","size":5,"soft_fail":false,"target_va":5411669968,"va":5380313421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::map::IntoIter$LT$K$C$V$C$A$GT$::dying_next
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t14005377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388139824,"raw_hex":"e881b4d500","size":5,"soft_fail":false,"target_va":5402147568,"va":5388142186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31225178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e85a75dc01","size":5,"soft_fail":false,"target_va":5411540963,"va":5380315780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2338040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e8f8ac2300","size":5,"soft_fail":false,"target_va":5382655024,"va":5380316979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31226213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e86579dc01","size":5,"soft_fail":false,"target_va":5411541584,"va":5380315366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t31226295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e8b779dc01","size":5,"soft_fail":false,"target_va":5411541776,"va":5380315476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31226710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e8567bdc01","size":5,"soft_fail":false,"target_va":5411542064,"va":5380315349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31226291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e8b379dc01","size":5,"soft_fail":false,"target_va":5411541904,"va":5380315608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::render::mesh::VertexAttributeValues> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-20758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e8eaaeffff","size":5,"soft_fail":false,"target_va":5380296000,"va":5380316753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31227430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e8267edc01","size":5,"soft_fail":false,"target_va":5411542880,"va":5380315445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-16748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e894beffff","size":5,"soft_fail":false,"target_va":5380296144,"va":5380312887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t31230398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e8be89dc01","size":5,"soft_fail":false,"target_va":5411543360,"va":5380312957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8916420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388139824,"raw_hex":"e83cf277ff","size":5,"soft_fail":false,"target_va":5379224464,"va":5388140879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::set_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8916710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388139824,"raw_hex":"e81af177ff","size":5,"soft_fail":false,"target_va":5379224720,"va":5388141425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::set_source_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8916286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388139824,"raw_hex":"e8c2f277ff","size":5,"soft_fail":false,"target_va":5379224640,"va":5388140921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::load_json::read_attribute_from_json
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2962584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e868cbd2ff","size":5,"soft_fail":false,"target_va":5377352208,"va":5380314787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1072041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e8a95b1000","size":5,"soft_fail":false,"target_va":5381385536,"va":5380313490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1077247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e8ff6f1000","size":5,"soft_fail":false,"target_va":5381390640,"va":5380313388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1071213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e86d581000","size":5,"soft_fail":false,"target_va":5381386480,"va":5380315262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1071841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e8e15a1000","size":5,"soft_fail":false,"target_va":5381384704,"va":5380312858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2391915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e86b7f2400","size":5,"soft_fail":false,"target_va":5382705472,"va":5380313552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23504363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388139824,"raw_hex":"e8eba56601","size":5,"soft_fail":false,"target_va":5411644800,"va":5388140432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8509226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e82ad78100","size":5,"soft_fail":false,"target_va":5388825296,"va":5380316065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t31221369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380312784,"raw_hex":"e87966dc01","size":5,"soft_fail":false,"target_va":5411537104,"va":5380315730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":56452,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":577,"record_offset":56508,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":577,"record_offset":51940,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":53284,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":54384,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":19409,"source_line":3107,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":50680,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":56708,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":577,"record_offset":56636,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":577,"record_offset":51752,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":577,"record_offset":52252,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":26960,"source_line":26,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":577,"record_offset":54872,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":30837,"source_line":55,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_source_file
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":577,"record_offset":52684,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":30872,"source_line":39,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":577,"record_offset":51972,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":51856,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":50716,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":49200,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":49648,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":577,"record_offset":53220,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":26180,"source_line":424,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::index::impl$5::index
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":577,"record_offset":53052,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":26177,"source_line":212,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":577,"record_offset":51724,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":577,"record_offset":50624,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":5177,"source_line":2250,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":577,"record_offset":52656,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":13531,"source_line":2045,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":53700,"parent_record_offset":53676,"ancestor_record_offset":48960,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":53724,"parent_record_offset":53700,"ancestor_record_offset":48960,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":52980,"parent_record_offset":52956,"ancestor_record_offset":48960,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":53676,"parent_record_offset":53636,"ancestor_record_offset":48960,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":52956,"parent_record_offset":52900,"ancestor_record_offset":48960,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1534,"record_offset":19072,"parent_record_offset":19040,"ancestor_record_offset":11140,"inlinee":6930,"source_line":188,"archive":"_pdb/B/modules/01534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1534,"record_offset":19228,"parent_record_offset":19072,"ancestor_record_offset":11140,"inlinee":6033,"source_line":1680,"archive":"_pdb/B/modules/01534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$34::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1534,"record_offset":19176,"parent_record_offset":19148,"ancestor_record_offset":11140,"inlinee":8153,"source_line":1702,"archive":"_pdb/B/modules/01534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":57096,"parent_record_offset":57072,"ancestor_record_offset":48960,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":53560,"parent_record_offset":53524,"ancestor_record_offset":48960,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":52928,"parent_record_offset":52900,"ancestor_record_offset":48960,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":52900,"parent_record_offset":52876,"ancestor_record_offset":48960,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":57144,"parent_record_offset":57120,"ancestor_record_offset":48960,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":57120,"parent_record_offset":57096,"ancestor_record_offset":48960,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":53636,"parent_record_offset":53596,"ancestor_record_offset":48960,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":53596,"parent_record_offset":53560,"ancestor_record_offset":48960,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":52876,"parent_record_offset":52852,"ancestor_record_offset":48960,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":55640,"parent_record_offset":55600,"ancestor_record_offset":48960,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":57072,"parent_record_offset":57048,"ancestor_record_offset":48960,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":53808,"parent_record_offset":53780,"ancestor_record_offset":48960,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":53488,"parent_record_offset":53428,"ancestor_record_offset":48960,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":53524,"parent_record_offset":53488,"ancestor_record_offset":48960,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":53332,"parent_record_offset":53284,"ancestor_record_offset":48960,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":54084,"parent_record_offset":54060,"ancestor_record_offset":48960,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":57048,"parent_record_offset":57020,"ancestor_record_offset":48960,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":577,"record_offset":56732,"parent_record_offset":56708,"ancestor_record_offset":48960,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":577,"record_offset":56780,"parent_record_offset":56756,"ancestor_record_offset":48960,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":577,"record_offset":54140,"parent_record_offset":54084,"ancestor_record_offset":48960,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":577,"record_offset":53780,"parent_record_offset":53428,"ancestor_record_offset":48960,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":577,"record_offset":53380,"parent_record_offset":53332,"ancestor_record_offset":48960,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":577,"record_offset":53428,"parent_record_offset":53380,"ancestor_record_offset":48960,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":577,"record_offset":55664,"parent_record_offset":55640,"ancestor_record_offset":48960,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":577,"record_offset":55688,"parent_record_offset":55664,"ancestor_record_offset":48960,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":51104,"parent_record_offset":51076,"ancestor_record_offset":48960,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":52488,"parent_record_offset":52460,"ancestor_record_offset":48960,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":51156,"parent_record_offset":51132,"ancestor_record_offset":48960,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":51132,"parent_record_offset":51104,"ancestor_record_offset":48960,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":51044,"parent_record_offset":51012,"ancestor_record_offset":48960,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":51076,"parent_record_offset":51044,"ancestor_record_offset":48960,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":51248,"parent_record_offset":51076,"ancestor_record_offset":48960,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":51360,"parent_record_offset":51336,"ancestor_record_offset":48960,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":577,"record_offset":54784,"parent_record_offset":54756,"ancestor_record_offset":48960,"inlinee":19414,"source_line":1594,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":577,"record_offset":51820,"parent_record_offset":51792,"ancestor_record_offset":48960,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":577,"record_offset":53864,"parent_record_offset":53836,"ancestor_record_offset":48960,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":577,"record_offset":54472,"parent_record_offset":54440,"ancestor_record_offset":48960,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":577,"record_offset":53928,"parent_record_offset":53892,"ancestor_record_offset":48960,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":53836,"parent_record_offset":53808,"ancestor_record_offset":48960,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":54060,"parent_record_offset":53928,"ancestor_record_offset":48960,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":54412,"parent_record_offset":54384,"ancestor_record_offset":48960,"inlinee":19410,"source_line":3198,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":54756,"parent_record_offset":54728,"ancestor_record_offset":48960,"inlinee":19413,"source_line":3205,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":54440,"parent_record_offset":54412,"ancestor_record_offset":48960,"inlinee":6060,"source_line":2576,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":577,"record_offset":51180,"parent_record_offset":51156,"ancestor_record_offset":48960,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":55712,"parent_record_offset":55688,"ancestor_record_offset":48960,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":55736,"parent_record_offset":55712,"ancestor_record_offset":48960,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":54824,"parent_record_offset":54556,"ancestor_record_offset":48960,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":55028,"parent_record_offset":54996,"ancestor_record_offset":48960,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":577,"record_offset":49348,"parent_record_offset":49316,"ancestor_record_offset":48960,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":577,"record_offset":50880,"parent_record_offset":50840,"ancestor_record_offset":48960,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":54496,"parent_record_offset":54472,"ancestor_record_offset":48960,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":53984,"parent_record_offset":53956,"ancestor_record_offset":48960,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":53104,"parent_record_offset":53052,"ancestor_record_offset":48960,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":577,"record_offset":52028,"parent_record_offset":51996,"ancestor_record_offset":48960,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":577,"record_offset":52056,"parent_record_offset":51996,"ancestor_record_offset":48960,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":577,"record_offset":49596,"parent_record_offset":49568,"ancestor_record_offset":48960,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":577,"record_offset":50980,"parent_record_offset":50840,"ancestor_record_offset":48960,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":577,"record_offset":54692,"parent_record_offset":54668,"ancestor_record_offset":48960,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=79
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":51312,"parent_record_offset":51012,"ancestor_record_offset":48960,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":577,"record_offset":56476,"parent_record_offset":56452,"ancestor_record_offset":48960,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":577,"record_offset":54668,"parent_record_offset":54644,"ancestor_record_offset":48960,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":50060,"parent_record_offset":50036,"ancestor_record_offset":48960,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":56576,"parent_record_offset":56508,"ancestor_record_offset":48960,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":50036,"parent_record_offset":50012,"ancestor_record_offset":48960,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":54644,"parent_record_offset":54556,"ancestor_record_offset":48960,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":50244,"parent_record_offset":50216,"ancestor_record_offset":48960,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":56600,"parent_record_offset":56576,"ancestor_record_offset":48960,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":51204,"parent_record_offset":51180,"ancestor_record_offset":48960,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":49988,"parent_record_offset":49964,"ancestor_record_offset":48960,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":50012,"parent_record_offset":49988,"ancestor_record_offset":48960,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":50316,"parent_record_offset":50288,"ancestor_record_offset":48960,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":53892,"parent_record_offset":53864,"ancestor_record_offset":48960,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":49964,"parent_record_offset":49848,"ancestor_record_offset":48960,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":577,"record_offset":49316,"parent_record_offset":49288,"ancestor_record_offset":48960,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":577,"record_offset":49676,"parent_record_offset":49648,"ancestor_record_offset":48960,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":577,"record_offset":49816,"parent_record_offset":49784,"ancestor_record_offset":48960,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":577,"record_offset":49784,"parent_record_offset":49748,"ancestor_record_offset":48960,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":577,"record_offset":49724,"parent_record_offset":49700,"ancestor_record_offset":48960,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":577,"record_offset":49748,"parent_record_offset":49724,"ancestor_record_offset":48960,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":577,"record_offset":49700,"parent_record_offset":49676,"ancestor_record_offset":48960,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":577,"record_offset":50288,"parent_record_offset":49748,"ancestor_record_offset":48960,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":49288,"parent_record_offset":49256,"ancestor_record_offset":48960,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":49256,"parent_record_offset":49232,"ancestor_record_offset":48960,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":49448,"parent_record_offset":49416,"ancestor_record_offset":48960,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":49544,"parent_record_offset":49448,"ancestor_record_offset":48960,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":49416,"parent_record_offset":49392,"ancestor_record_offset":48960,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":49232,"parent_record_offset":49200,"ancestor_record_offset":48960,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":49568,"parent_record_offset":49544,"ancestor_record_offset":48960,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":577,"record_offset":49848,"parent_record_offset":49816,"ancestor_record_offset":48960,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":577,"record_offset":50132,"parent_record_offset":49848,"ancestor_record_offset":48960,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":577,"record_offset":50104,"parent_record_offset":49848,"ancestor_record_offset":48960,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":577,"record_offset":49908,"parent_record_offset":49848,"ancestor_record_offset":48960,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":577,"record_offset":51880,"parent_record_offset":51856,"ancestor_record_offset":48960,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":577,"record_offset":51904,"parent_record_offset":51880,"ancestor_record_offset":48960,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":577,"record_offset":51792,"parent_record_offset":51752,"ancestor_record_offset":48960,"inlinee":33022,"source_line":9,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":577,"record_offset":52188,"parent_record_offset":52148,"ancestor_record_offset":48960,"inlinee":33023,"source_line":15,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":577,"record_offset":53956,"parent_record_offset":53928,"ancestor_record_offset":48960,"inlinee":33018,"source_line":27,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$4
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":577,"record_offset":56060,"parent_record_offset":56032,"ancestor_record_offset":48960,"inlinee":33020,"source_line":38,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":52084,"parent_record_offset":51996,"ancestor_record_offset":48960,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":577,"record_offset":51996,"parent_record_offset":51972,"ancestor_record_offset":48960,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":52108,"parent_record_offset":52084,"ancestor_record_offset":48960,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":50756,"parent_record_offset":50716,"ancestor_record_offset":48960,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":58132,"parent_record_offset":58096,"ancestor_record_offset":48960,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":58696,"parent_record_offset":58424,"ancestor_record_offset":48960,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":577,"record_offset":50908,"parent_record_offset":50880,"ancestor_record_offset":48960,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":51012,"parent_record_offset":50840,"ancestor_record_offset":48960,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":51284,"parent_record_offset":51012,"ancestor_record_offset":48960,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::Value::as_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":577,"record_offset":56088,"parent_record_offset":56060,"ancestor_record_offset":48960,"inlinee":33021,"source_line":491,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::Value::as_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":577,"record_offset":54024,"parent_record_offset":53956,"ancestor_record_offset":48960,"inlinee":33019,"source_line":627,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":577,"record_offset":53076,"parent_record_offset":53052,"ancestor_record_offset":48960,"inlinee":26179,"source_line":123,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":577,"record_offset":52288,"parent_record_offset":52252,"ancestor_record_offset":48960,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":577,"record_offset":50648,"parent_record_offset":50624,"ancestor_record_offset":48960,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":577,"record_offset":52344,"parent_record_offset":52316,"ancestor_record_offset":48960,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":577,"record_offset":52460,"parent_record_offset":52404,"ancestor_record_offset":48960,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":577,"record_offset":52316,"parent_record_offset":52288,"ancestor_record_offset":48960,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":577,"record_offset":50936,"parent_record_offset":50908,"ancestor_record_offset":48960,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":577,"record_offset":50840,"parent_record_offset":50796,"ancestor_record_offset":48960,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":577,"record_offset":50796,"parent_record_offset":50756,"ancestor_record_offset":48960,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00577.jsonl.gz"}
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
// 首条位置: {"module":577,"record_offset":60232,"rva":11607888,"va":5380317008}
// 编译实例 RVAs: 0xb11f50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2326564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e824802300","size":5,"soft_fail":false,"target_va":5382644528,"va":5380317959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10717238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e8ca775cff","size":5,"soft_fail":false,"target_va":5369600800,"va":5380318033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5419446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e84a4eadff","size":5,"soft_fail":false,"target_va":5374900736,"va":5380320177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5416392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e8385aadff","size":5,"soft_fail":false,"target_va":5374904480,"va":5380320867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5236858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e88617b0ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380318341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5236695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e82918b0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380318194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4801800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e8f8bab6ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5380317827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085da90
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2838984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e838aed4ff","size":5,"soft_fail":false,"target_va":5377481360,"va":5380320339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2662757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e89b5ed7ff","size":5,"soft_fail":false,"target_va":5377657648,"va":5380320400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888d70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2661008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e87065d7ff","size":5,"soft_fail":false,"target_va":5377658224,"va":5380319227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2595083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e8f566d8ff","size":5,"soft_fail":false,"target_va":5377724224,"va":5380319302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a35240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-906523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e8e52af2ff","size":5,"soft_fail":false,"target_va":5379412544,"va":5380319062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2387186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e8f26c2400","size":5,"soft_fail":false,"target_va":5382704752,"va":5380317561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2422570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e82af72400","size":5,"soft_fail":false,"target_va":5382740736,"va":5380318161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2510670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e84e4f2600","size":5,"soft_fail":false,"target_va":5382827728,"va":5380317053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9385486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e80e368f00","size":5,"soft_fail":false,"target_va":5389703472,"va":5380317981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414744d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9838760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e8a8209600","size":5,"soft_fail":false,"target_va":5390157008,"va":5380318243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31350523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e8fb5ede01","size":5,"soft_fail":false,"target_va":5411669968,"va":5380319440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31221040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e83065dc01","size":5,"soft_fail":false,"target_va":5411540963,"va":5380319918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2334008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e8389d2300","size":5,"soft_fail":false,"target_va":5382655024,"va":5380321011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31222052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e82469dc01","size":5,"soft_fail":false,"target_va":5411541584,"va":5380319527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t31222159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e88f69dc01","size":5,"soft_fail":false,"target_va":5411541776,"va":5380319612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31222549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e8156bdc01","size":5,"soft_fail":false,"target_va":5411542064,"va":5380319510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31222154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e88a69dc01","size":5,"soft_fail":false,"target_va":5411541904,"va":5380319745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::render::mesh::VertexAttributeValues> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-24787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e82d9fffff","size":5,"soft_fail":false,"target_va":5380296000,"va":5380320782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31223291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e8fb6ddc01","size":5,"soft_fail":false,"target_va":5411542880,"va":5380319584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-20968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e818aeffff","size":5,"soft_fail":false,"target_va":5380296144,"va":5380317107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31226178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e84279dc01","size":5,"soft_fail":false,"target_va":5411543360,"va":5380317177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::load_json::read_attribute_from_json
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2966744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e828bbd2ff","size":5,"soft_fail":false,"target_va":5377352208,"va":5380318947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1067870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e85e4b1000","size":5,"soft_fail":false,"target_va":5381385536,"va":5380317661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1073051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e89b5f1000","size":5,"soft_fail":false,"target_va":5381390640,"va":5380317584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1067081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e849481000","size":5,"soft_fail":false,"target_va":5381386480,"va":5380319394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1067621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e8654a1000","size":5,"soft_fail":false,"target_va":5381384704,"va":5380317078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2387741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e81d6f2400","size":5,"soft_fail":false,"target_va":5382705472,"va":5380317726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8505194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e86ac78100","size":5,"soft_fail":false,"target_va":5388825296,"va":5380320097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31217613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317008,"raw_hex":"e8cd57dc01","size":5,"soft_fail":false,"target_va":5411537104,"va":5380319486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":67912,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":577,"record_offset":67968,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":577,"record_offset":63400,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":577,"record_offset":60480,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":64744,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":65844,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":19409,"source_line":3107,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":62164,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":68168,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":577,"record_offset":68096,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":577,"record_offset":63212,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":577,"record_offset":63712,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":26960,"source_line":26,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":577,"record_offset":66332,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":30837,"source_line":55,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_source_file
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":577,"record_offset":64144,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":30872,"source_line":39,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":577,"record_offset":63432,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":63316,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":62200,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":60676,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":61132,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":577,"record_offset":64680,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":26180,"source_line":424,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::index::impl$5::index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":577,"record_offset":64512,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":26177,"source_line":212,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":577,"record_offset":63184,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":577,"record_offset":62108,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":5177,"source_line":2250,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":577,"record_offset":64116,"parent_record_offset":60232,"ancestor_record_offset":60232,"inlinee":13531,"source_line":2045,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":65160,"parent_record_offset":65136,"ancestor_record_offset":60232,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":65184,"parent_record_offset":65160,"ancestor_record_offset":60232,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":64440,"parent_record_offset":64416,"ancestor_record_offset":60232,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":65136,"parent_record_offset":65096,"ancestor_record_offset":60232,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":64416,"parent_record_offset":64360,"ancestor_record_offset":60232,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":68556,"parent_record_offset":68532,"ancestor_record_offset":60232,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":65020,"parent_record_offset":64984,"ancestor_record_offset":60232,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":64388,"parent_record_offset":64360,"ancestor_record_offset":60232,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":64360,"parent_record_offset":64336,"ancestor_record_offset":60232,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":68604,"parent_record_offset":68580,"ancestor_record_offset":60232,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":68580,"parent_record_offset":68556,"ancestor_record_offset":60232,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":65096,"parent_record_offset":65056,"ancestor_record_offset":60232,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":65056,"parent_record_offset":65020,"ancestor_record_offset":60232,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":64336,"parent_record_offset":64312,"ancestor_record_offset":60232,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":67100,"parent_record_offset":67060,"ancestor_record_offset":60232,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":68532,"parent_record_offset":68508,"ancestor_record_offset":60232,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":65268,"parent_record_offset":65240,"ancestor_record_offset":60232,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":64948,"parent_record_offset":64888,"ancestor_record_offset":60232,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":64984,"parent_record_offset":64948,"ancestor_record_offset":60232,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":64792,"parent_record_offset":64744,"ancestor_record_offset":60232,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":65544,"parent_record_offset":65520,"ancestor_record_offset":60232,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":68508,"parent_record_offset":68480,"ancestor_record_offset":60232,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":577,"record_offset":68192,"parent_record_offset":68168,"ancestor_record_offset":60232,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":577,"record_offset":68240,"parent_record_offset":68216,"ancestor_record_offset":60232,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":577,"record_offset":65600,"parent_record_offset":65544,"ancestor_record_offset":60232,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":577,"record_offset":65240,"parent_record_offset":64888,"ancestor_record_offset":60232,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":577,"record_offset":64840,"parent_record_offset":64792,"ancestor_record_offset":60232,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":577,"record_offset":64888,"parent_record_offset":64840,"ancestor_record_offset":60232,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":577,"record_offset":67124,"parent_record_offset":67100,"ancestor_record_offset":60232,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":577,"record_offset":67148,"parent_record_offset":67124,"ancestor_record_offset":60232,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":62536,"parent_record_offset":62508,"ancestor_record_offset":60232,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":63948,"parent_record_offset":63920,"ancestor_record_offset":60232,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":62588,"parent_record_offset":62564,"ancestor_record_offset":60232,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":62564,"parent_record_offset":62536,"ancestor_record_offset":60232,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":62484,"parent_record_offset":62456,"ancestor_record_offset":60232,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":62508,"parent_record_offset":62484,"ancestor_record_offset":60232,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":62680,"parent_record_offset":62508,"ancestor_record_offset":60232,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":62792,"parent_record_offset":62768,"ancestor_record_offset":60232,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":577,"record_offset":66244,"parent_record_offset":66216,"ancestor_record_offset":60232,"inlinee":19414,"source_line":1594,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":577,"record_offset":63280,"parent_record_offset":63252,"ancestor_record_offset":60232,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":577,"record_offset":65324,"parent_record_offset":65296,"ancestor_record_offset":60232,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":577,"record_offset":65932,"parent_record_offset":65900,"ancestor_record_offset":60232,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":577,"record_offset":65388,"parent_record_offset":65352,"ancestor_record_offset":60232,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":65296,"parent_record_offset":65268,"ancestor_record_offset":60232,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":65520,"parent_record_offset":65388,"ancestor_record_offset":60232,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":65872,"parent_record_offset":65844,"ancestor_record_offset":60232,"inlinee":19410,"source_line":3198,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":66216,"parent_record_offset":66188,"ancestor_record_offset":60232,"inlinee":19413,"source_line":3205,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":65900,"parent_record_offset":65872,"ancestor_record_offset":60232,"inlinee":6060,"source_line":2576,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":577,"record_offset":62612,"parent_record_offset":62588,"ancestor_record_offset":60232,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":67172,"parent_record_offset":67148,"ancestor_record_offset":60232,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":67196,"parent_record_offset":67172,"ancestor_record_offset":60232,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":66284,"parent_record_offset":66016,"ancestor_record_offset":60232,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":66488,"parent_record_offset":66456,"ancestor_record_offset":60232,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":577,"record_offset":60832,"parent_record_offset":60800,"ancestor_record_offset":60232,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":577,"record_offset":62352,"parent_record_offset":62308,"ancestor_record_offset":60232,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":65956,"parent_record_offset":65932,"ancestor_record_offset":60232,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":65444,"parent_record_offset":65416,"ancestor_record_offset":60232,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":64564,"parent_record_offset":64512,"ancestor_record_offset":60232,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":577,"record_offset":63488,"parent_record_offset":63456,"ancestor_record_offset":60232,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":577,"record_offset":63516,"parent_record_offset":63456,"ancestor_record_offset":60232,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":577,"record_offset":61080,"parent_record_offset":61052,"ancestor_record_offset":60232,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":577,"record_offset":63140,"parent_record_offset":62308,"ancestor_record_offset":60232,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":577,"record_offset":66152,"parent_record_offset":66128,"ancestor_record_offset":60232,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":62744,"parent_record_offset":62456,"ancestor_record_offset":60232,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":577,"record_offset":67936,"parent_record_offset":67912,"ancestor_record_offset":60232,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":577,"record_offset":66128,"parent_record_offset":66104,"ancestor_record_offset":60232,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":61544,"parent_record_offset":61520,"ancestor_record_offset":60232,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":68036,"parent_record_offset":67968,"ancestor_record_offset":60232,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":61520,"parent_record_offset":61496,"ancestor_record_offset":60232,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":66104,"parent_record_offset":66016,"ancestor_record_offset":60232,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":61728,"parent_record_offset":61700,"ancestor_record_offset":60232,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":68060,"parent_record_offset":68036,"ancestor_record_offset":60232,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":62636,"parent_record_offset":62612,"ancestor_record_offset":60232,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":61472,"parent_record_offset":61448,"ancestor_record_offset":60232,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":61496,"parent_record_offset":61472,"ancestor_record_offset":60232,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":61800,"parent_record_offset":61772,"ancestor_record_offset":60232,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":65352,"parent_record_offset":65324,"ancestor_record_offset":60232,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":61448,"parent_record_offset":61332,"ancestor_record_offset":60232,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":577,"record_offset":60800,"parent_record_offset":60772,"ancestor_record_offset":60232,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":577,"record_offset":61160,"parent_record_offset":61132,"ancestor_record_offset":60232,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":577,"record_offset":61300,"parent_record_offset":61268,"ancestor_record_offset":60232,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":577,"record_offset":61268,"parent_record_offset":61232,"ancestor_record_offset":60232,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":577,"record_offset":61208,"parent_record_offset":61184,"ancestor_record_offset":60232,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":577,"record_offset":61232,"parent_record_offset":61208,"ancestor_record_offset":60232,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":577,"record_offset":61184,"parent_record_offset":61160,"ancestor_record_offset":60232,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":577,"record_offset":61772,"parent_record_offset":61232,"ancestor_record_offset":60232,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":60772,"parent_record_offset":60740,"ancestor_record_offset":60232,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":60740,"parent_record_offset":60712,"ancestor_record_offset":60232,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":60932,"parent_record_offset":60900,"ancestor_record_offset":60232,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":61028,"parent_record_offset":60932,"ancestor_record_offset":60232,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":60900,"parent_record_offset":60876,"ancestor_record_offset":60232,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":60712,"parent_record_offset":60676,"ancestor_record_offset":60232,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":61052,"parent_record_offset":61028,"ancestor_record_offset":60232,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":577,"record_offset":61332,"parent_record_offset":61300,"ancestor_record_offset":60232,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":577,"record_offset":61616,"parent_record_offset":61332,"ancestor_record_offset":60232,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":577,"record_offset":61588,"parent_record_offset":61332,"ancestor_record_offset":60232,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":577,"record_offset":61392,"parent_record_offset":61332,"ancestor_record_offset":60232,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":577,"record_offset":63340,"parent_record_offset":63316,"ancestor_record_offset":60232,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":577,"record_offset":63364,"parent_record_offset":63340,"ancestor_record_offset":60232,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":577,"record_offset":63252,"parent_record_offset":63212,"ancestor_record_offset":60232,"inlinee":33022,"source_line":9,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":577,"record_offset":63648,"parent_record_offset":63608,"ancestor_record_offset":60232,"inlinee":33023,"source_line":15,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":577,"record_offset":65416,"parent_record_offset":65388,"ancestor_record_offset":60232,"inlinee":33018,"source_line":27,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":577,"record_offset":67520,"parent_record_offset":67492,"ancestor_record_offset":60232,"inlinee":33020,"source_line":38,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":63544,"parent_record_offset":63456,"ancestor_record_offset":60232,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":577,"record_offset":63456,"parent_record_offset":63432,"ancestor_record_offset":60232,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":63568,"parent_record_offset":63544,"ancestor_record_offset":60232,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":62236,"parent_record_offset":62200,"ancestor_record_offset":60232,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":69584,"parent_record_offset":69552,"ancestor_record_offset":60232,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":70084,"parent_record_offset":69824,"ancestor_record_offset":60232,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":577,"record_offset":62380,"parent_record_offset":62352,"ancestor_record_offset":60232,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":62456,"parent_record_offset":62308,"ancestor_record_offset":60232,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":62716,"parent_record_offset":62456,"ancestor_record_offset":60232,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::Value::as_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":577,"record_offset":67548,"parent_record_offset":67520,"ancestor_record_offset":60232,"inlinee":33021,"source_line":491,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::Value::as_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":577,"record_offset":65484,"parent_record_offset":65416,"ancestor_record_offset":60232,"inlinee":33019,"source_line":627,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":577,"record_offset":64536,"parent_record_offset":64512,"ancestor_record_offset":60232,"inlinee":26179,"source_line":123,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":577,"record_offset":63748,"parent_record_offset":63712,"ancestor_record_offset":60232,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":577,"record_offset":62132,"parent_record_offset":62108,"ancestor_record_offset":60232,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":577,"record_offset":60576,"parent_record_offset":60552,"ancestor_record_offset":60232,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":577,"record_offset":60624,"parent_record_offset":60600,"ancestor_record_offset":60232,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":577,"record_offset":60600,"parent_record_offset":60576,"ancestor_record_offset":60232,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":577,"record_offset":63804,"parent_record_offset":63776,"ancestor_record_offset":60232,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":577,"record_offset":63920,"parent_record_offset":63864,"ancestor_record_offset":60232,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":577,"record_offset":63776,"parent_record_offset":63748,"ancestor_record_offset":60232,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":577,"record_offset":60552,"parent_record_offset":60528,"ancestor_record_offset":60232,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":577,"record_offset":60528,"parent_record_offset":60504,"ancestor_record_offset":60232,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":577,"record_offset":60504,"parent_record_offset":60480,"ancestor_record_offset":60232,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":577,"record_offset":62408,"parent_record_offset":62380,"ancestor_record_offset":60232,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":577,"record_offset":62308,"parent_record_offset":62272,"ancestor_record_offset":60232,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":577,"record_offset":62272,"parent_record_offset":62236,"ancestor_record_offset":60232,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00577.jsonl.gz"}
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
// 首条位置: {"module":577,"record_offset":26600,"rva":11595440,"va":5380304560}
// 编译实例 RVAs: 0xb0eeb0, 0x2607f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2339028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8d4b02300","size":5,"soft_fail":false,"target_va":5382644528,"va":5380305495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10704774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e87aa85cff","size":5,"soft_fail":false,"target_va":5369600800,"va":5380305569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5406982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8fa7eadff","size":5,"soft_fail":false,"target_va":5374900736,"va":5380307713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5403928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8e88aadff","size":5,"soft_fail":false,"target_va":5374904480,"va":5380308403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5224394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e83648b0ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380305877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5224231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8d948b0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380305730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4789336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8a8ebb6ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5380305363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085da90
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2826520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8e8ded4ff","size":5,"soft_fail":false,"target_va":5377481360,"va":5380307875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2650293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e84b8fd7ff","size":5,"soft_fail":false,"target_va":5377657648,"va":5380307936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888d70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2648544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e82096d7ff","size":5,"soft_fail":false,"target_va":5377658224,"va":5380306763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2582619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8a597d8ff","size":5,"soft_fail":false,"target_va":5377724224,"va":5380306838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140949170
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30143708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408587568,"raw_hex":"e8240b34fe","size":5,"soft_fail":false,"target_va":5378445680,"va":5408589383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a35240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-894059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8955bf2ff","size":5,"soft_fail":false,"target_va":5379412544,"va":5380306598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2399650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8a29d2400","size":5,"soft_fail":false,"target_va":5382704752,"va":5380305097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2435034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8da272500","size":5,"soft_fail":false,"target_va":5382740736,"va":5380305697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ed4560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24332792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408587568,"raw_hex":"e808b68cfe","size":5,"soft_fail":false,"target_va":5384258912,"va":5408591699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ed47d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24330147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408587568,"raw_hex":"e85dc08cfe","size":5,"soft_fail":false,"target_va":5384259536,"va":5408589678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ed4b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24331173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408587568,"raw_hex":"e85bbc8cfe","size":5,"soft_fail":false,"target_va":5384260384,"va":5408591552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ed74f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24318799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408587568,"raw_hex":"e8b1ec8cfe","size":5,"soft_fail":false,"target_va":5384271088,"va":5408589882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9397950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8be668f00","size":5,"soft_fail":false,"target_va":5389703472,"va":5380305517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414744d0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9851224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e858519600","size":5,"soft_fail":false,"target_va":5390157008,"va":5380305779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141caf720
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9803048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408587568,"raw_hex":"e8d86a6aff","size":5,"soft_fail":false,"target_va":5398787872,"va":5408590915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14285e550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2450059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408587568,"raw_hex":"e88b622500","size":5,"soft_fail":false,"target_va":5411038544,"va":5408588480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14285fae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2454612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408587568,"raw_hex":"e854742500","size":5,"soft_fail":false,"target_va":5411044064,"va":5408589447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31362987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8ab8fde01","size":5,"soft_fail":false,"target_va":5411669968,"va":5380306976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::map::IntoIter$LT$K$C$V$C$A$GT$::dying_next
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9809135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408587568,"raw_hex":"e811536aff","size":5,"soft_fail":false,"target_va":5398780720,"va":5408589850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31233504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8e095dc01","size":5,"soft_fail":false,"target_va":5411540963,"va":5380307454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2346472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8e8cd2300","size":5,"soft_fail":false,"target_va":5382655024,"va":5380308547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31234516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8d499dc01","size":5,"soft_fail":false,"target_va":5411541584,"va":5380307063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t31234623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e83f9adc01","size":5,"soft_fail":false,"target_va":5411541776,"va":5380307148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31235013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8c59bdc01","size":5,"soft_fail":false,"target_va":5411542064,"va":5380307046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31234618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e83a9adc01","size":5,"soft_fail":false,"target_va":5411541904,"va":5380307281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::render::mesh::VertexAttributeValues> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8ddcfffff","size":5,"soft_fail":false,"target_va":5380296000,"va":5380308318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31235755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8ab9edc01","size":5,"soft_fail":false,"target_va":5411542880,"va":5380307120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-8512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8c0deffff","size":5,"soft_fail":false,"target_va":5380296144,"va":5380304651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t31238634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8eaa9dc01","size":5,"soft_fail":false,"target_va":5411543360,"va":5380304721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29364088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408587568,"raw_hex":"e888f03ffe","size":5,"soft_fail":false,"target_va":5379224464,"va":5408588547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::set_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29364374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408587568,"raw_hex":"e86aef3ffe","size":5,"soft_fail":false,"target_va":5379224720,"va":5408589089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::set_source_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29363948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408587568,"raw_hex":"e814f13ffe","size":5,"soft_fail":false,"target_va":5379224640,"va":5408588583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::load_json::read_attribute_from_json
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2954280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8d8ebd2ff","size":5,"soft_fail":false,"target_va":5377352208,"va":5380306483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1080334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e80e7c1000","size":5,"soft_fail":false,"target_va":5381385536,"va":5380305197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1085515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e84b901000","size":5,"soft_fail":false,"target_va":5381390640,"va":5380305120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1079545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8f9781000","size":5,"soft_fail":false,"target_va":5381386480,"va":5380306930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1080077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e80d7b1000","size":5,"soft_fail":false,"target_va":5381384704,"va":5380304622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2400205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e8cd9f2400","size":5,"soft_fail":false,"target_va":5382705472,"va":5380305262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3219110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408587568,"raw_hex":"e8a61e3100","size":5,"soft_fail":false,"target_va":5411809152,"va":5408590037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8517658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e81af88100","size":5,"soft_fail":false,"target_va":5388825296,"va":5380307633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31230077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380304560,"raw_hex":"e87d88dc01","size":5,"soft_fail":false,"target_va":5411537104,"va":5380307022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":34068,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":577,"record_offset":34124,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":577,"record_offset":29556,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":30900,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":32000,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":19409,"source_line":3107,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":28320,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":34324,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":577,"record_offset":34252,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":577,"record_offset":29368,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":577,"record_offset":29868,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":26960,"source_line":26,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":577,"record_offset":32488,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":30837,"source_line":55,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_source_file
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":577,"record_offset":30300,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":30872,"source_line":39,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":577,"record_offset":29588,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":29472,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":28356,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":26832,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":27288,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":577,"record_offset":30836,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":26180,"source_line":424,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::index::impl$5::index
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":577,"record_offset":30668,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":26177,"source_line":212,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":577,"record_offset":29340,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":577,"record_offset":28264,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":5177,"source_line":2250,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":577,"record_offset":30272,"parent_record_offset":26600,"ancestor_record_offset":26600,"inlinee":13531,"source_line":2045,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":31316,"parent_record_offset":31292,"ancestor_record_offset":26600,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":31340,"parent_record_offset":31316,"ancestor_record_offset":26600,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":30596,"parent_record_offset":30572,"ancestor_record_offset":26600,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":31292,"parent_record_offset":31252,"ancestor_record_offset":26600,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":30572,"parent_record_offset":30516,"ancestor_record_offset":26600,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":4480,"record_offset":8168,"parent_record_offset":8136,"ancestor_record_offset":260,"inlinee":6930,"source_line":188,"archive":"_pdb/B/modules/04480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":4480,"record_offset":8324,"parent_record_offset":8168,"ancestor_record_offset":260,"inlinee":6033,"source_line":1680,"archive":"_pdb/B/modules/04480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$34::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":4480,"record_offset":8272,"parent_record_offset":8244,"ancestor_record_offset":260,"inlinee":8153,"source_line":1702,"archive":"_pdb/B/modules/04480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":34712,"parent_record_offset":34688,"ancestor_record_offset":26600,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":31176,"parent_record_offset":31140,"ancestor_record_offset":26600,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":30544,"parent_record_offset":30516,"ancestor_record_offset":26600,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":30516,"parent_record_offset":30492,"ancestor_record_offset":26600,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":34760,"parent_record_offset":34736,"ancestor_record_offset":26600,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":34736,"parent_record_offset":34712,"ancestor_record_offset":26600,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":31252,"parent_record_offset":31212,"ancestor_record_offset":26600,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":31212,"parent_record_offset":31176,"ancestor_record_offset":26600,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":30492,"parent_record_offset":30468,"ancestor_record_offset":26600,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":33256,"parent_record_offset":33216,"ancestor_record_offset":26600,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":34688,"parent_record_offset":34664,"ancestor_record_offset":26600,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":31424,"parent_record_offset":31396,"ancestor_record_offset":26600,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":31104,"parent_record_offset":31044,"ancestor_record_offset":26600,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":31140,"parent_record_offset":31104,"ancestor_record_offset":26600,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":30948,"parent_record_offset":30900,"ancestor_record_offset":26600,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":31700,"parent_record_offset":31676,"ancestor_record_offset":26600,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":34664,"parent_record_offset":34636,"ancestor_record_offset":26600,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":577,"record_offset":34348,"parent_record_offset":34324,"ancestor_record_offset":26600,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":577,"record_offset":34396,"parent_record_offset":34372,"ancestor_record_offset":26600,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":577,"record_offset":31756,"parent_record_offset":31700,"ancestor_record_offset":26600,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":577,"record_offset":31396,"parent_record_offset":31044,"ancestor_record_offset":26600,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":577,"record_offset":30996,"parent_record_offset":30948,"ancestor_record_offset":26600,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":577,"record_offset":31044,"parent_record_offset":30996,"ancestor_record_offset":26600,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":577,"record_offset":33280,"parent_record_offset":33256,"ancestor_record_offset":26600,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":577,"record_offset":33304,"parent_record_offset":33280,"ancestor_record_offset":26600,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":28692,"parent_record_offset":28664,"ancestor_record_offset":26600,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":30104,"parent_record_offset":30076,"ancestor_record_offset":26600,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":28744,"parent_record_offset":28720,"ancestor_record_offset":26600,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":28720,"parent_record_offset":28692,"ancestor_record_offset":26600,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":28640,"parent_record_offset":28612,"ancestor_record_offset":26600,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":28664,"parent_record_offset":28640,"ancestor_record_offset":26600,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":28836,"parent_record_offset":28664,"ancestor_record_offset":26600,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":28948,"parent_record_offset":28924,"ancestor_record_offset":26600,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":577,"record_offset":32400,"parent_record_offset":32372,"ancestor_record_offset":26600,"inlinee":19414,"source_line":1594,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":577,"record_offset":29436,"parent_record_offset":29408,"ancestor_record_offset":26600,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":577,"record_offset":31480,"parent_record_offset":31452,"ancestor_record_offset":26600,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":577,"record_offset":32088,"parent_record_offset":32056,"ancestor_record_offset":26600,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":577,"record_offset":31544,"parent_record_offset":31508,"ancestor_record_offset":26600,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":31452,"parent_record_offset":31424,"ancestor_record_offset":26600,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":31676,"parent_record_offset":31544,"ancestor_record_offset":26600,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":32028,"parent_record_offset":32000,"ancestor_record_offset":26600,"inlinee":19410,"source_line":3198,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":32372,"parent_record_offset":32344,"ancestor_record_offset":26600,"inlinee":19413,"source_line":3205,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":32056,"parent_record_offset":32028,"ancestor_record_offset":26600,"inlinee":6060,"source_line":2576,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":577,"record_offset":28768,"parent_record_offset":28744,"ancestor_record_offset":26600,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":33328,"parent_record_offset":33304,"ancestor_record_offset":26600,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":33352,"parent_record_offset":33328,"ancestor_record_offset":26600,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":32440,"parent_record_offset":32172,"ancestor_record_offset":26600,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":32644,"parent_record_offset":32612,"ancestor_record_offset":26600,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":577,"record_offset":26988,"parent_record_offset":26956,"ancestor_record_offset":26600,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":577,"record_offset":28508,"parent_record_offset":28464,"ancestor_record_offset":26600,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":32112,"parent_record_offset":32088,"ancestor_record_offset":26600,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":31600,"parent_record_offset":31572,"ancestor_record_offset":26600,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":30720,"parent_record_offset":30668,"ancestor_record_offset":26600,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":577,"record_offset":29644,"parent_record_offset":29612,"ancestor_record_offset":26600,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":577,"record_offset":29672,"parent_record_offset":29612,"ancestor_record_offset":26600,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":577,"record_offset":27236,"parent_record_offset":27208,"ancestor_record_offset":26600,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":577,"record_offset":29296,"parent_record_offset":28464,"ancestor_record_offset":26600,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":577,"record_offset":32308,"parent_record_offset":32284,"ancestor_record_offset":26600,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=71
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":28900,"parent_record_offset":28612,"ancestor_record_offset":26600,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":577,"record_offset":34092,"parent_record_offset":34068,"ancestor_record_offset":26600,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":577,"record_offset":32284,"parent_record_offset":32260,"ancestor_record_offset":26600,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":27700,"parent_record_offset":27676,"ancestor_record_offset":26600,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":34192,"parent_record_offset":34124,"ancestor_record_offset":26600,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":27676,"parent_record_offset":27652,"ancestor_record_offset":26600,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":32260,"parent_record_offset":32172,"ancestor_record_offset":26600,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":27884,"parent_record_offset":27856,"ancestor_record_offset":26600,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":34216,"parent_record_offset":34192,"ancestor_record_offset":26600,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":28792,"parent_record_offset":28768,"ancestor_record_offset":26600,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":27628,"parent_record_offset":27604,"ancestor_record_offset":26600,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":27652,"parent_record_offset":27628,"ancestor_record_offset":26600,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":27956,"parent_record_offset":27928,"ancestor_record_offset":26600,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":31508,"parent_record_offset":31480,"ancestor_record_offset":26600,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":27604,"parent_record_offset":27488,"ancestor_record_offset":26600,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":577,"record_offset":26956,"parent_record_offset":26928,"ancestor_record_offset":26600,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":577,"record_offset":27316,"parent_record_offset":27288,"ancestor_record_offset":26600,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":577,"record_offset":27456,"parent_record_offset":27424,"ancestor_record_offset":26600,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":577,"record_offset":27424,"parent_record_offset":27388,"ancestor_record_offset":26600,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":577,"record_offset":27364,"parent_record_offset":27340,"ancestor_record_offset":26600,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":577,"record_offset":27388,"parent_record_offset":27364,"ancestor_record_offset":26600,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":577,"record_offset":27340,"parent_record_offset":27316,"ancestor_record_offset":26600,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":577,"record_offset":27928,"parent_record_offset":27388,"ancestor_record_offset":26600,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":26928,"parent_record_offset":26896,"ancestor_record_offset":26600,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":26896,"parent_record_offset":26868,"ancestor_record_offset":26600,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":27088,"parent_record_offset":27056,"ancestor_record_offset":26600,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":27184,"parent_record_offset":27088,"ancestor_record_offset":26600,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":27056,"parent_record_offset":27032,"ancestor_record_offset":26600,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":26868,"parent_record_offset":26832,"ancestor_record_offset":26600,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":27208,"parent_record_offset":27184,"ancestor_record_offset":26600,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":577,"record_offset":27488,"parent_record_offset":27456,"ancestor_record_offset":26600,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":577,"record_offset":27772,"parent_record_offset":27488,"ancestor_record_offset":26600,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":577,"record_offset":27744,"parent_record_offset":27488,"ancestor_record_offset":26600,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":577,"record_offset":27548,"parent_record_offset":27488,"ancestor_record_offset":26600,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":577,"record_offset":29496,"parent_record_offset":29472,"ancestor_record_offset":26600,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":577,"record_offset":29520,"parent_record_offset":29496,"ancestor_record_offset":26600,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":577,"record_offset":29408,"parent_record_offset":29368,"ancestor_record_offset":26600,"inlinee":33022,"source_line":9,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":577,"record_offset":29804,"parent_record_offset":29764,"ancestor_record_offset":26600,"inlinee":33023,"source_line":15,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":577,"record_offset":31572,"parent_record_offset":31544,"ancestor_record_offset":26600,"inlinee":33018,"source_line":27,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$4
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":577,"record_offset":33676,"parent_record_offset":33648,"ancestor_record_offset":26600,"inlinee":33020,"source_line":38,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":29700,"parent_record_offset":29612,"ancestor_record_offset":26600,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":577,"record_offset":29612,"parent_record_offset":29588,"ancestor_record_offset":26600,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":29724,"parent_record_offset":29700,"ancestor_record_offset":26600,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":28392,"parent_record_offset":28356,"ancestor_record_offset":26600,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":35740,"parent_record_offset":35708,"ancestor_record_offset":26600,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":36240,"parent_record_offset":35980,"ancestor_record_offset":26600,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":577,"record_offset":28536,"parent_record_offset":28508,"ancestor_record_offset":26600,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":28612,"parent_record_offset":28464,"ancestor_record_offset":26600,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":28872,"parent_record_offset":28612,"ancestor_record_offset":26600,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::Value::as_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":577,"record_offset":33704,"parent_record_offset":33676,"ancestor_record_offset":26600,"inlinee":33021,"source_line":491,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::Value::as_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":577,"record_offset":31640,"parent_record_offset":31572,"ancestor_record_offset":26600,"inlinee":33019,"source_line":627,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":577,"record_offset":30692,"parent_record_offset":30668,"ancestor_record_offset":26600,"inlinee":26179,"source_line":123,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":577,"record_offset":29904,"parent_record_offset":29868,"ancestor_record_offset":26600,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":577,"record_offset":28288,"parent_record_offset":28264,"ancestor_record_offset":26600,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":577,"record_offset":29960,"parent_record_offset":29932,"ancestor_record_offset":26600,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":577,"record_offset":30076,"parent_record_offset":30020,"ancestor_record_offset":26600,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":577,"record_offset":29932,"parent_record_offset":29904,"ancestor_record_offset":26600,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":577,"record_offset":28564,"parent_record_offset":28536,"ancestor_record_offset":26600,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":577,"record_offset":28464,"parent_record_offset":28428,"ancestor_record_offset":26600,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":577,"record_offset":28428,"parent_record_offset":28392,"ancestor_record_offset":26600,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00577.jsonl.gz"}
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
// 首条位置: {"module":577,"record_offset":37348,"rva":11599456,"va":5380308576}
// 编译实例 RVAs: 0xb0fe60, 0x1286bf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2334964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8f4a02300","size":5,"soft_fail":false,"target_va":5382644528,"va":5380309559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10708838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e89a985cff","size":5,"soft_fail":false,"target_va":5369600800,"va":5380309633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5411190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e88a6eadff","size":5,"soft_fail":false,"target_va":5374900736,"va":5380311921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5408136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8787aadff","size":5,"soft_fail":false,"target_va":5374904480,"va":5380312611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5228458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e85638b0ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380309941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=22
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5228295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8f938b0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380309794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4793400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8c8dbb6ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5380309427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085da90
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2830728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e878ced4ff","size":5,"soft_fail":false,"target_va":5377481360,"va":5380312083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2654501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8db7ed7ff","size":5,"soft_fail":false,"target_va":5377657648,"va":5380312144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888d70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2652624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e83086d7ff","size":5,"soft_fail":false,"target_va":5377658224,"va":5380310843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2586699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8b587d8ff","size":5,"soft_fail":false,"target_va":5377724224,"va":5380310918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140949170
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9691612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388135408,"raw_hex":"e8241e6cff","size":5,"soft_fail":false,"target_va":5378445680,"va":5388137287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a35240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-898139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8a54bf2ff","size":5,"soft_fail":false,"target_va":5379412544,"va":5380310678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2395605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8d58d2400","size":5,"soft_fail":false,"target_va":5382704752,"va":5380309142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2430970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8fa172500","size":5,"soft_fail":false,"target_va":5382740736,"va":5380309761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2519099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e83b702600","size":5,"soft_fail":false,"target_va":5382827728,"va":5380308624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141259400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-190680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388135408,"raw_hex":"e82817fdff","size":5,"soft_fail":false,"target_va":5387949056,"va":5388139731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141259560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-188179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388135408,"raw_hex":"e8ed20fdff","size":5,"soft_fail":false,"target_va":5387949408,"va":5388137582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412598b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-189333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388135408,"raw_hex":"e86b1cfdff","size":5,"soft_fail":false,"target_va":5387950256,"va":5388139584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125a270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-185039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388135408,"raw_hex":"e8312dfdff","size":5,"soft_fail":false,"target_va":5387952752,"va":5388137786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141288e80
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388135408,"raw_hex":"e8b8140000","size":5,"soft_fail":false,"target_va":5388144256,"va":5388138947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9393886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8de568f00","size":5,"soft_fail":false,"target_va":5389703472,"va":5380309581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414744d0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9847160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e878419600","size":5,"soft_fail":false,"target_va":5390157008,"va":5380309843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420054c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14148866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388135408,"raw_hex":"e802e5d700","size":5,"soft_fail":false,"target_va":5402285248,"va":5388136377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142006a50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14153412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388135408,"raw_hex":"e8c4f6d700","size":5,"soft_fail":false,"target_va":5402290768,"va":5388137351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31360765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8fd86de01","size":5,"soft_fail":false,"target_va":5411669968,"va":5380309198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::map::IntoIter$LT$K$C$V$C$A$GT$::dying_next
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t14009809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388135408,"raw_hex":"e8d1c5d500","size":5,"soft_fail":false,"target_va":5402147568,"va":5388137754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31229405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8dd85dc01","size":5,"soft_fail":false,"target_va":5411540963,"va":5380311553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2342264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e878bd2300","size":5,"soft_fail":false,"target_va":5382655024,"va":5380312755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31230439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8e789dc01","size":5,"soft_fail":false,"target_va":5411541584,"va":5380311140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t31230524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e83c8adc01","size":5,"soft_fail":false,"target_va":5411541776,"va":5380311247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31230936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8d88bdc01","size":5,"soft_fail":false,"target_va":5411542064,"va":5380311123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31230519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8378adc01","size":5,"soft_fail":false,"target_va":5411541904,"va":5380311380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::render::mesh::VertexAttributeValues> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-16531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e86dbfffff","size":5,"soft_fail":false,"target_va":5380296000,"va":5380312526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31231656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8a88edc01","size":5,"soft_fail":false,"target_va":5411542880,"va":5380311219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-12533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e80bcfffff","size":5,"soft_fail":false,"target_va":5380296144,"va":5380308672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t31234613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8359adc01","size":5,"soft_fail":false,"target_va":5411543360,"va":5380308742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8911985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388135408,"raw_hex":"e88f0378ff","size":5,"soft_fail":false,"target_va":5379224464,"va":5388136444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::set_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8912278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388135408,"raw_hex":"e86a0278ff","size":5,"soft_fail":false,"target_va":5379224720,"va":5388136993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::set_source_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8911851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388135408,"raw_hex":"e8150478ff","size":5,"soft_fail":false,"target_va":5379224640,"va":5388136486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::load_json::read_attribute_from_json
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2958360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8e8dbd2ff","size":5,"soft_fail":false,"target_va":5377352208,"va":5380310563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1076264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8286c1000","size":5,"soft_fail":false,"target_va":5381385536,"va":5380309267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1081470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e87e801000","size":5,"soft_fail":false,"target_va":5381390640,"va":5380309165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1075436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8ec681000","size":5,"soft_fail":false,"target_va":5381386480,"va":5380311039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1076056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8586b1000","size":5,"soft_fail":false,"target_va":5381384704,"va":5380308643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2396141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8ed8f2400","size":5,"soft_fail":false,"target_va":5382705472,"va":5380309326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23508795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388135408,"raw_hex":"e83bb76601","size":5,"soft_fail":false,"target_va":5411644800,"va":5388136000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8513450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8aae78100","size":5,"soft_fail":false,"target_va":5388825296,"va":5380311841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t31225596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380308576,"raw_hex":"e8fc76dc01","size":5,"soft_fail":false,"target_va":5411537104,"va":5380311503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":45012,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":577,"record_offset":45068,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":577,"record_offset":40500,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":41844,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":42944,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":19409,"source_line":3107,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":39240,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":45268,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":577,"record_offset":45196,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":577,"record_offset":40312,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":577,"record_offset":40812,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":26960,"source_line":26,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":577,"record_offset":43432,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":30837,"source_line":55,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_source_file
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":577,"record_offset":41244,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":30872,"source_line":39,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":577,"record_offset":40532,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":40416,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":39276,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":37760,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":38208,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":577,"record_offset":41780,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":26180,"source_line":424,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::index::impl$5::index
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":577,"record_offset":41612,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":26177,"source_line":212,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":577,"record_offset":40284,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":577,"record_offset":39184,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":5177,"source_line":2250,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":577,"record_offset":41216,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":13531,"source_line":2045,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":577,"record_offset":37592,"parent_record_offset":37348,"ancestor_record_offset":37348,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":42260,"parent_record_offset":42236,"ancestor_record_offset":37348,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":42284,"parent_record_offset":42260,"ancestor_record_offset":37348,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":41540,"parent_record_offset":41516,"ancestor_record_offset":37348,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":42236,"parent_record_offset":42196,"ancestor_record_offset":37348,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":41516,"parent_record_offset":41460,"ancestor_record_offset":37348,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1534,"record_offset":8368,"parent_record_offset":8336,"ancestor_record_offset":264,"inlinee":6930,"source_line":188,"archive":"_pdb/B/modules/01534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1534,"record_offset":8524,"parent_record_offset":8368,"ancestor_record_offset":264,"inlinee":6033,"source_line":1680,"archive":"_pdb/B/modules/01534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$34::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1534,"record_offset":8472,"parent_record_offset":8444,"ancestor_record_offset":264,"inlinee":8153,"source_line":1702,"archive":"_pdb/B/modules/01534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":45656,"parent_record_offset":45632,"ancestor_record_offset":37348,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":42120,"parent_record_offset":42084,"ancestor_record_offset":37348,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":41488,"parent_record_offset":41460,"ancestor_record_offset":37348,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":41460,"parent_record_offset":41436,"ancestor_record_offset":37348,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":45704,"parent_record_offset":45680,"ancestor_record_offset":37348,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":45680,"parent_record_offset":45656,"ancestor_record_offset":37348,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":42196,"parent_record_offset":42156,"ancestor_record_offset":37348,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":42156,"parent_record_offset":42120,"ancestor_record_offset":37348,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":41436,"parent_record_offset":41412,"ancestor_record_offset":37348,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":44200,"parent_record_offset":44160,"ancestor_record_offset":37348,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":45632,"parent_record_offset":45608,"ancestor_record_offset":37348,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":42368,"parent_record_offset":42340,"ancestor_record_offset":37348,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":42048,"parent_record_offset":41988,"ancestor_record_offset":37348,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":42084,"parent_record_offset":42048,"ancestor_record_offset":37348,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":41892,"parent_record_offset":41844,"ancestor_record_offset":37348,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":42644,"parent_record_offset":42620,"ancestor_record_offset":37348,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":45608,"parent_record_offset":45580,"ancestor_record_offset":37348,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":577,"record_offset":45292,"parent_record_offset":45268,"ancestor_record_offset":37348,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":577,"record_offset":45340,"parent_record_offset":45316,"ancestor_record_offset":37348,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":577,"record_offset":42700,"parent_record_offset":42644,"ancestor_record_offset":37348,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":577,"record_offset":42340,"parent_record_offset":41988,"ancestor_record_offset":37348,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":577,"record_offset":41940,"parent_record_offset":41892,"ancestor_record_offset":37348,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":577,"record_offset":41988,"parent_record_offset":41940,"ancestor_record_offset":37348,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":577,"record_offset":44224,"parent_record_offset":44200,"ancestor_record_offset":37348,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":577,"record_offset":44248,"parent_record_offset":44224,"ancestor_record_offset":37348,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":39664,"parent_record_offset":39636,"ancestor_record_offset":37348,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":41048,"parent_record_offset":41020,"ancestor_record_offset":37348,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":39716,"parent_record_offset":39692,"ancestor_record_offset":37348,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":39692,"parent_record_offset":39664,"ancestor_record_offset":37348,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":39604,"parent_record_offset":39572,"ancestor_record_offset":37348,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":39636,"parent_record_offset":39604,"ancestor_record_offset":37348,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":39808,"parent_record_offset":39636,"ancestor_record_offset":37348,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":577,"record_offset":39920,"parent_record_offset":39896,"ancestor_record_offset":37348,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":577,"record_offset":43344,"parent_record_offset":43316,"ancestor_record_offset":37348,"inlinee":19414,"source_line":1594,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":577,"record_offset":40380,"parent_record_offset":40352,"ancestor_record_offset":37348,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":577,"record_offset":42424,"parent_record_offset":42396,"ancestor_record_offset":37348,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":577,"record_offset":43032,"parent_record_offset":43000,"ancestor_record_offset":37348,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":577,"record_offset":42488,"parent_record_offset":42452,"ancestor_record_offset":37348,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":42396,"parent_record_offset":42368,"ancestor_record_offset":37348,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":42620,"parent_record_offset":42488,"ancestor_record_offset":37348,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":42972,"parent_record_offset":42944,"ancestor_record_offset":37348,"inlinee":19410,"source_line":3198,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":43316,"parent_record_offset":43288,"ancestor_record_offset":37348,"inlinee":19413,"source_line":3205,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":43000,"parent_record_offset":42972,"ancestor_record_offset":37348,"inlinee":6060,"source_line":2576,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":577,"record_offset":39740,"parent_record_offset":39716,"ancestor_record_offset":37348,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":44272,"parent_record_offset":44248,"ancestor_record_offset":37348,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":44296,"parent_record_offset":44272,"ancestor_record_offset":37348,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":43384,"parent_record_offset":43116,"ancestor_record_offset":37348,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":43588,"parent_record_offset":43556,"ancestor_record_offset":37348,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":577,"record_offset":37908,"parent_record_offset":37876,"ancestor_record_offset":37348,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":577,"record_offset":39440,"parent_record_offset":39400,"ancestor_record_offset":37348,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":43056,"parent_record_offset":43032,"ancestor_record_offset":37348,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":42544,"parent_record_offset":42516,"ancestor_record_offset":37348,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":41664,"parent_record_offset":41612,"ancestor_record_offset":37348,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":577,"record_offset":40588,"parent_record_offset":40556,"ancestor_record_offset":37348,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":577,"record_offset":40616,"parent_record_offset":40556,"ancestor_record_offset":37348,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":577,"record_offset":38156,"parent_record_offset":38128,"ancestor_record_offset":37348,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":577,"record_offset":39540,"parent_record_offset":39400,"ancestor_record_offset":37348,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":577,"record_offset":43252,"parent_record_offset":43228,"ancestor_record_offset":37348,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=91
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":39872,"parent_record_offset":39572,"ancestor_record_offset":37348,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":577,"record_offset":45036,"parent_record_offset":45012,"ancestor_record_offset":37348,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":577,"record_offset":43228,"parent_record_offset":43204,"ancestor_record_offset":37348,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":38620,"parent_record_offset":38596,"ancestor_record_offset":37348,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":45136,"parent_record_offset":45068,"ancestor_record_offset":37348,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":38596,"parent_record_offset":38572,"ancestor_record_offset":37348,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":43204,"parent_record_offset":43116,"ancestor_record_offset":37348,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":38804,"parent_record_offset":38776,"ancestor_record_offset":37348,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":45160,"parent_record_offset":45136,"ancestor_record_offset":37348,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":39764,"parent_record_offset":39740,"ancestor_record_offset":37348,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":38548,"parent_record_offset":38524,"ancestor_record_offset":37348,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":38572,"parent_record_offset":38548,"ancestor_record_offset":37348,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":38876,"parent_record_offset":38848,"ancestor_record_offset":37348,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":42452,"parent_record_offset":42424,"ancestor_record_offset":37348,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":38524,"parent_record_offset":38408,"ancestor_record_offset":37348,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":577,"record_offset":37876,"parent_record_offset":37848,"ancestor_record_offset":37348,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":577,"record_offset":38236,"parent_record_offset":38208,"ancestor_record_offset":37348,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":577,"record_offset":38376,"parent_record_offset":38344,"ancestor_record_offset":37348,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":577,"record_offset":38344,"parent_record_offset":38308,"ancestor_record_offset":37348,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":577,"record_offset":38284,"parent_record_offset":38260,"ancestor_record_offset":37348,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":577,"record_offset":38308,"parent_record_offset":38284,"ancestor_record_offset":37348,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":577,"record_offset":38260,"parent_record_offset":38236,"ancestor_record_offset":37348,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":577,"record_offset":38848,"parent_record_offset":38308,"ancestor_record_offset":37348,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":37848,"parent_record_offset":37816,"ancestor_record_offset":37348,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":37816,"parent_record_offset":37792,"ancestor_record_offset":37348,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":38008,"parent_record_offset":37976,"ancestor_record_offset":37348,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":38104,"parent_record_offset":38008,"ancestor_record_offset":37348,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":37976,"parent_record_offset":37952,"ancestor_record_offset":37348,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":37792,"parent_record_offset":37760,"ancestor_record_offset":37348,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":577,"record_offset":38128,"parent_record_offset":38104,"ancestor_record_offset":37348,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":577,"record_offset":38408,"parent_record_offset":38376,"ancestor_record_offset":37348,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":577,"record_offset":38692,"parent_record_offset":38408,"ancestor_record_offset":37348,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":577,"record_offset":38664,"parent_record_offset":38408,"ancestor_record_offset":37348,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":577,"record_offset":38468,"parent_record_offset":38408,"ancestor_record_offset":37348,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":577,"record_offset":40440,"parent_record_offset":40416,"ancestor_record_offset":37348,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":577,"record_offset":40464,"parent_record_offset":40440,"ancestor_record_offset":37348,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":577,"record_offset":40352,"parent_record_offset":40312,"ancestor_record_offset":37348,"inlinee":33022,"source_line":9,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":577,"record_offset":40748,"parent_record_offset":40708,"ancestor_record_offset":37348,"inlinee":33023,"source_line":15,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":577,"record_offset":42516,"parent_record_offset":42488,"ancestor_record_offset":37348,"inlinee":33018,"source_line":27,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_as_mesh::closure$4
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":577,"record_offset":44620,"parent_record_offset":44592,"ancestor_record_offset":37348,"inlinee":33020,"source_line":38,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":40644,"parent_record_offset":40556,"ancestor_record_offset":37348,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":577,"record_offset":40556,"parent_record_offset":40532,"ancestor_record_offset":37348,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":40668,"parent_record_offset":40644,"ancestor_record_offset":37348,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":39316,"parent_record_offset":39276,"ancestor_record_offset":37348,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":46692,"parent_record_offset":46656,"ancestor_record_offset":37348,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":47256,"parent_record_offset":46984,"ancestor_record_offset":37348,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":577,"record_offset":39468,"parent_record_offset":39440,"ancestor_record_offset":37348,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":39572,"parent_record_offset":39400,"ancestor_record_offset":37348,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":577,"record_offset":39844,"parent_record_offset":39572,"ancestor_record_offset":37348,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::Value::as_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":577,"record_offset":44648,"parent_record_offset":44620,"ancestor_record_offset":37348,"inlinee":33021,"source_line":491,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::Value::as_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":577,"record_offset":42584,"parent_record_offset":42516,"ancestor_record_offset":37348,"inlinee":33019,"source_line":627,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":577,"record_offset":41636,"parent_record_offset":41612,"ancestor_record_offset":37348,"inlinee":26179,"source_line":123,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":577,"record_offset":40848,"parent_record_offset":40812,"ancestor_record_offset":37348,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":577,"record_offset":39208,"parent_record_offset":39184,"ancestor_record_offset":37348,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":577,"record_offset":37664,"parent_record_offset":37640,"ancestor_record_offset":37348,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":577,"record_offset":37712,"parent_record_offset":37688,"ancestor_record_offset":37348,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":577,"record_offset":37688,"parent_record_offset":37664,"ancestor_record_offset":37348,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":577,"record_offset":40904,"parent_record_offset":40876,"ancestor_record_offset":37348,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":577,"record_offset":41020,"parent_record_offset":40964,"ancestor_record_offset":37348,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":577,"record_offset":40876,"parent_record_offset":40848,"ancestor_record_offset":37348,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":577,"record_offset":37640,"parent_record_offset":37616,"ancestor_record_offset":37348,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":577,"record_offset":37616,"parent_record_offset":37592,"ancestor_record_offset":37348,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":577,"record_offset":39496,"parent_record_offset":39468,"ancestor_record_offset":37348,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":577,"record_offset":39400,"parent_record_offset":39356,"ancestor_record_offset":37348,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":577,"record_offset":39356,"parent_record_offset":39316,"ancestor_record_offset":37348,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x80FE
// 首条位置: {"module":577,"inlinee":33022}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":577,"record_offset":29436,"parent_record_offset":29408,"ancestor_record_offset":29408,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x80FF
// 首条位置: {"module":577,"inlinee":33023}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":577,"record_offset":29832,"parent_record_offset":29804,"ancestor_record_offset":29804,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=20
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x80FA
// 首条位置: {"module":577,"inlinee":33018}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":31600,"parent_record_offset":31572,"ancestor_record_offset":31572,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_u64
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":577,"record_offset":31640,"parent_record_offset":31572,"ancestor_record_offset":31572,"inlinee":33019,"source_line":627,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=20
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x80FC
// 首条位置: {"module":577,"inlinee":33020}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":577,"record_offset":33736,"parent_record_offset":33676,"ancestor_record_offset":33676,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_str
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":577,"record_offset":33704,"parent_record_offset":33676,"ancestor_record_offset":33676,"inlinee":33021,"source_line":491,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":476,"record_offset":63340,"rva":8643088,"va":5377352208}
// 编译实例 RVAs: 0x83e210
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2271506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377352208,"raw_hex":"e8ee56ddff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377352989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2271356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377352208,"raw_hex":"e88457ddff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377352855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412fae80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11257918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377352208,"raw_hex":"e83ec8ab00","size":5,"soft_fail":false,"target_va":5388611200,"va":5377353277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413011a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11283927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377352208,"raw_hex":"e8d72dac00","size":5,"soft_fail":false,"target_va":5388636576,"va":5377352644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141302030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11286865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377352208,"raw_hex":"e85139ac00","size":5,"soft_fail":false,"target_va":5388640304,"va":5377353434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141302940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11289498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377352208,"raw_hex":"e89a43ac00","size":5,"soft_fail":false,"target_va":5388642624,"va":5377353121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414744d0
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12804728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377352208,"raw_hex":"e87862c300","size":5,"soft_fail":false,"target_va":5390157008,"va":5377352275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141476470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12811551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377352208,"raw_hex":"e81f7dc300","size":5,"soft_fail":false,"target_va":5390165104,"va":5377353548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a83e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33988455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377352208,"raw_hex":"e8679f0602","size":5,"soft_fail":false,"target_va":5411341280,"va":5377352820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5487804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377352208,"raw_hex":"e8bcbc5300","size":5,"soft_fail":false,"target_va":5382840208,"va":5377352399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34187940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377352208,"raw_hex":"e8a4aa0902","size":5,"soft_fail":false,"target_va":5411540963,"va":5377353018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t34188017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377352208,"raw_hex":"e8f1aa0902","size":5,"soft_fail":false,"target_va":5411541776,"va":5377353754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t34188437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377352208,"raw_hex":"e895ac0902","size":5,"soft_fail":false,"target_va":5411542064,"va":5377353622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":476,"record_offset":63680,"parent_record_offset":63340,"ancestor_record_offset":63340,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":476,"record_offset":64588,"parent_record_offset":63340,"ancestor_record_offset":63340,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":476,"record_offset":64532,"parent_record_offset":63340,"ancestor_record_offset":63340,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":476,"record_offset":64412,"parent_record_offset":63340,"ancestor_record_offset":63340,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":65132,"parent_record_offset":63340,"ancestor_record_offset":63340,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":476,"record_offset":64448,"parent_record_offset":63340,"ancestor_record_offset":63340,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":476,"record_offset":63764,"parent_record_offset":63340,"ancestor_record_offset":63340,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":476,"record_offset":64384,"parent_record_offset":63340,"ancestor_record_offset":63340,"inlinee":26180,"source_line":424,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::index::impl$5::index
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":476,"record_offset":63512,"parent_record_offset":63340,"ancestor_record_offset":63340,"inlinee":26177,"source_line":212,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":65908,"parent_record_offset":65884,"ancestor_record_offset":63340,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":65932,"parent_record_offset":65908,"ancestor_record_offset":63340,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":64880,"parent_record_offset":64856,"ancestor_record_offset":63340,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":65884,"parent_record_offset":65852,"ancestor_record_offset":63340,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":64856,"parent_record_offset":64764,"ancestor_record_offset":63340,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":476,"record_offset":63728,"parent_record_offset":63704,"ancestor_record_offset":63340,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":65800,"parent_record_offset":65772,"ancestor_record_offset":63340,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":64792,"parent_record_offset":64764,"ancestor_record_offset":63340,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":64764,"parent_record_offset":64740,"ancestor_record_offset":63340,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":65852,"parent_record_offset":65824,"ancestor_record_offset":63340,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":65824,"parent_record_offset":65800,"ancestor_record_offset":63340,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":64740,"parent_record_offset":64716,"ancestor_record_offset":63340,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":66012,"parent_record_offset":65988,"ancestor_record_offset":63340,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":64664,"parent_record_offset":64624,"ancestor_record_offset":63340,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":65744,"parent_record_offset":65700,"ancestor_record_offset":63340,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":65772,"parent_record_offset":65744,"ancestor_record_offset":63340,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":64556,"parent_record_offset":64532,"ancestor_record_offset":63340,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":476,"record_offset":65988,"parent_record_offset":65700,"ancestor_record_offset":63340,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":476,"record_offset":65664,"parent_record_offset":65628,"ancestor_record_offset":63340,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":476,"record_offset":65700,"parent_record_offset":65664,"ancestor_record_offset":63340,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":476,"record_offset":63788,"parent_record_offset":63764,"ancestor_record_offset":63340,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":476,"record_offset":67436,"parent_record_offset":67412,"ancestor_record_offset":63340,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":476,"record_offset":66036,"parent_record_offset":66012,"ancestor_record_offset":63340,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":476,"record_offset":66060,"parent_record_offset":66036,"ancestor_record_offset":63340,"inlinee":6052,"source_line":796,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":476,"record_offset":64824,"parent_record_offset":64792,"ancestor_record_offset":63340,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":476,"record_offset":63704,"parent_record_offset":63680,"ancestor_record_offset":63340,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":476,"record_offset":63564,"parent_record_offset":63512,"ancestor_record_offset":63340,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":64692,"parent_record_offset":64624,"ancestor_record_offset":63340,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":476,"record_offset":64496,"parent_record_offset":64472,"ancestor_record_offset":63340,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":476,"record_offset":63812,"parent_record_offset":63788,"ancestor_record_offset":63340,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":476,"record_offset":63836,"parent_record_offset":63812,"ancestor_record_offset":63340,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":476,"record_offset":64472,"parent_record_offset":64448,"ancestor_record_offset":63340,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$4::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":476,"record_offset":66760,"parent_record_offset":66724,"ancestor_record_offset":63340,"inlinee":26184,"source_line":273,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":476,"record_offset":64624,"parent_record_offset":64588,"ancestor_record_offset":63340,"inlinee":26182,"source_line":273,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$6::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":476,"record_offset":68256,"parent_record_offset":68220,"ancestor_record_offset":63340,"inlinee":26188,"source_line":273,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$7::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":476,"record_offset":67508,"parent_record_offset":67472,"ancestor_record_offset":63340,"inlinee":26186,"source_line":273,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$8::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":476,"record_offset":68600,"parent_record_offset":68568,"ancestor_record_offset":63340,"inlinee":26190,"source_line":273,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":476,"record_offset":63536,"parent_record_offset":63512,"ancestor_record_offset":63340,"inlinee":26179,"source_line":123,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1207A
// 首条位置: {"module":1610,"inlinee":73850}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":49516,"parent_record_offset":49260,"ancestor_record_offset":49260,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1610,"record_offset":49460,"parent_record_offset":49260,"ancestor_record_offset":49260,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1610,"record_offset":49340,"parent_record_offset":49260,"ancestor_record_offset":49260,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":49900,"parent_record_offset":49260,"ancestor_record_offset":49260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1610,"record_offset":49376,"parent_record_offset":49260,"ancestor_record_offset":49260,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1610,"record_offset":49312,"parent_record_offset":49260,"ancestor_record_offset":49260,"inlinee":26180,"source_line":424,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":50084,"parent_record_offset":50060,"ancestor_record_offset":49260,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":50060,"parent_record_offset":49972,"ancestor_record_offset":49260,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":50000,"parent_record_offset":49972,"ancestor_record_offset":49260,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":49972,"parent_record_offset":49948,"ancestor_record_offset":49260,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":49948,"parent_record_offset":49924,"ancestor_record_offset":49260,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":49484,"parent_record_offset":49460,"ancestor_record_offset":49260,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1610,"record_offset":50028,"parent_record_offset":50000,"ancestor_record_offset":49260,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":49924,"parent_record_offset":49900,"ancestor_record_offset":49260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1610,"record_offset":49424,"parent_record_offset":49400,"ancestor_record_offset":49260,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1610,"record_offset":49544,"parent_record_offset":49516,"ancestor_record_offset":49260,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1610,"record_offset":49572,"parent_record_offset":49544,"ancestor_record_offset":49260,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1610,"record_offset":49400,"parent_record_offset":49376,"ancestor_record_offset":49260,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AD1
// 首条位置: {"module":1585,"inlinee":72401}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1585,"record_offset":384428,"parent_record_offset":384320,"ancestor_record_offset":384320,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1585,"record_offset":384352,"parent_record_offset":384320,"ancestor_record_offset":384320,"inlinee":34206,"source_line":646,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::Number::as_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1585,"record_offset":384384,"parent_record_offset":384352,"ancestor_record_offset":384320,"inlinee":34207,"source_line":240,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1208F
// 首条位置: {"module":1610,"inlinee":73871}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":68680,"parent_record_offset":68368,"ancestor_record_offset":68368,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1610,"record_offset":68624,"parent_record_offset":68368,"ancestor_record_offset":68368,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1610,"record_offset":68448,"parent_record_offset":68368,"ancestor_record_offset":68368,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":68968,"parent_record_offset":68368,"ancestor_record_offset":68368,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1610,"record_offset":68540,"parent_record_offset":68368,"ancestor_record_offset":68368,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1610,"record_offset":68420,"parent_record_offset":68368,"ancestor_record_offset":68368,"inlinee":26180,"source_line":424,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":69152,"parent_record_offset":69128,"ancestor_record_offset":68368,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":69128,"parent_record_offset":69040,"ancestor_record_offset":68368,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":69068,"parent_record_offset":69040,"ancestor_record_offset":68368,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":69040,"parent_record_offset":69016,"ancestor_record_offset":68368,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":69016,"parent_record_offset":68992,"ancestor_record_offset":68368,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":68648,"parent_record_offset":68624,"ancestor_record_offset":68368,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1610,"record_offset":68504,"parent_record_offset":68480,"ancestor_record_offset":68368,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1610,"record_offset":69096,"parent_record_offset":69068,"ancestor_record_offset":68368,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":68992,"parent_record_offset":68968,"ancestor_record_offset":68368,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1610,"record_offset":68588,"parent_record_offset":68564,"ancestor_record_offset":68368,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1610,"record_offset":68708,"parent_record_offset":68680,"ancestor_record_offset":68368,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1610,"record_offset":68736,"parent_record_offset":68708,"ancestor_record_offset":68368,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1610,"record_offset":68564,"parent_record_offset":68540,"ancestor_record_offset":68368,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::read_attribute_from_json::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":1610,"record_offset":68480,"parent_record_offset":68448,"ancestor_record_offset":68368,"inlinee":73872,"source_line":84,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12090
// 首条位置: {"module":1610,"inlinee":73872}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1610,"record_offset":68504,"parent_record_offset":68480,"ancestor_record_offset":68480,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$1::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AC0
// 首条位置: {"module":1585,"inlinee":72384}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1585,"record_offset":303184,"parent_record_offset":303076,"ancestor_record_offset":303076,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1585,"record_offset":303108,"parent_record_offset":303076,"ancestor_record_offset":303076,"inlinee":34206,"source_line":646,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::Number::as_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1585,"record_offset":303140,"parent_record_offset":303108,"ancestor_record_offset":303076,"inlinee":34207,"source_line":240,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12076
// 首条位置: {"module":1610,"inlinee":73846}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":36504,"parent_record_offset":36192,"ancestor_record_offset":36192,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1610,"record_offset":36448,"parent_record_offset":36192,"ancestor_record_offset":36192,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1610,"record_offset":36272,"parent_record_offset":36192,"ancestor_record_offset":36192,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":36600,"parent_record_offset":36192,"ancestor_record_offset":36192,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1610,"record_offset":36364,"parent_record_offset":36192,"ancestor_record_offset":36192,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1610,"record_offset":36244,"parent_record_offset":36192,"ancestor_record_offset":36192,"inlinee":26180,"source_line":424,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":36800,"parent_record_offset":36776,"ancestor_record_offset":36192,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":36776,"parent_record_offset":36684,"ancestor_record_offset":36192,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":36716,"parent_record_offset":36684,"ancestor_record_offset":36192,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":36684,"parent_record_offset":36656,"ancestor_record_offset":36192,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":36656,"parent_record_offset":36628,"ancestor_record_offset":36192,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":36472,"parent_record_offset":36448,"ancestor_record_offset":36192,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1610,"record_offset":36328,"parent_record_offset":36304,"ancestor_record_offset":36192,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1610,"record_offset":36744,"parent_record_offset":36716,"ancestor_record_offset":36192,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":36628,"parent_record_offset":36600,"ancestor_record_offset":36192,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1610,"record_offset":36412,"parent_record_offset":36388,"ancestor_record_offset":36192,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1610,"record_offset":36532,"parent_record_offset":36504,"ancestor_record_offset":36192,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1610,"record_offset":36560,"parent_record_offset":36532,"ancestor_record_offset":36192,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1610,"record_offset":36388,"parent_record_offset":36364,"ancestor_record_offset":36192,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::read_attribute_from_json::closure$2::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":1610,"record_offset":36304,"parent_record_offset":36272,"ancestor_record_offset":36192,"inlinee":73847,"source_line":107,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$2::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12077
// 首条位置: {"module":1610,"inlinee":73847}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1610,"record_offset":36328,"parent_record_offset":36304,"ancestor_record_offset":36304,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$2::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11ADA
// 首条位置: {"module":1585,"inlinee":72410}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1585,"record_offset":417996,"parent_record_offset":417888,"ancestor_record_offset":417888,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1585,"record_offset":417920,"parent_record_offset":417888,"ancestor_record_offset":417888,"inlinee":34206,"source_line":646,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::Number::as_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1585,"record_offset":417952,"parent_record_offset":417920,"ancestor_record_offset":417888,"inlinee":34207,"source_line":240,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11ABC
// 首条位置: {"module":1585,"inlinee":72380}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1585,"record_offset":294504,"parent_record_offset":294400,"ancestor_record_offset":294400,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1585,"record_offset":294428,"parent_record_offset":294400,"ancestor_record_offset":294400,"inlinee":34204,"source_line":608,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::Number::as_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1585,"record_offset":294460,"parent_record_offset":294428,"ancestor_record_offset":294400,"inlinee":34205,"source_line":188,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AB7
// 首条位置: {"module":1585,"inlinee":72375}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1585,"record_offset":270788,"parent_record_offset":270680,"ancestor_record_offset":270680,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1585,"record_offset":270712,"parent_record_offset":270680,"ancestor_record_offset":270680,"inlinee":34206,"source_line":646,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::Number::as_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1585,"record_offset":270744,"parent_record_offset":270712,"ancestor_record_offset":270680,"inlinee":34207,"source_line":240,"archive":"_pdb/B/modules/01585.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 17; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh::closure$0
// 信息 ID: b26e0bc8aba529fdcfaa3910
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::load_json::load_json_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<str$> >; module=577, inline_record=29408; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>; module=577, inline_record=40352; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<alloc::string::String>; module=577, inline_record=51792; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> >; module=577, inline_record=63252; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>; module=1534, inline_record=3256; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<alloc::string::String>; module=1534, inline_record=13960; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<str$> >; module=4480, inline_record=3056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh::closure$1
// 信息 ID: 722098169e3d4c7f829e4d5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::load_json::load_json_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<str$> >; module=577, inline_record=29804; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>; module=577, inline_record=40748; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<alloc::string::String>; module=577, inline_record=52188; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> >; module=577, inline_record=63648; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>; module=1534, inline_record=3652; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<alloc::string::String>; module=1534, inline_record=14356; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<str$> >; module=4480, inline_record=3452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh::closure$2
// 信息 ID: c9455ab62a1679d4cf57c03a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::utils::load_json::load_json_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<str$> >; module=577, inline_record=31572; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>; module=577, inline_record=42516; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<alloc::string::String>; module=577, inline_record=53956; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> >; module=577, inline_record=65416; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>; module=1534, inline_record=4648; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<alloc::string::String>; module=1534, inline_record=15352; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<str$> >; module=4480, inline_record=4448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh::closure$4
// 信息 ID: a425e250b6bfd8248e9874aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "country_core::utils::load_json::load_json_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<str$> >; module=577, inline_record=33676; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>; module=577, inline_record=44620; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<alloc::string::String>; module=577, inline_record=56060; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> >; module=577, inline_record=67520; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>; module=1534, inline_record=6488; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<alloc::string::String>; module=1534, inline_record=17192; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_as_mesh<ref$<str$> >; module=4480, inline_record=6288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh<alloc::string::String>
// 信息 ID: 55fdf18db9a4a1d5cac4e062
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::load_json::load_json_as_mesh -> ["alloc::string::String"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::spawn_brush_previews::spawn_brush_previews @ 0x92544f -> 0xb10ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<country_core::render::mesh::Mesh,core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > > > @ 0x1475eab -> 0xb10ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<country_core::render::mesh::Mesh,core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > > > @ 0x1475fb1 -> 0xb10ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators::closure$1 @ 0x7baf7d -> 0x1287d30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> >
// 信息 ID: 06ce9897c91a0cfed6e340fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["std::path::PathBuf"]; 字段布局 = unknown。
// [类型关系] country_core::utils::load_json::load_json_as_mesh -> ["ref$<std::path::PathBuf>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1> @ 0x15f6086 -> 0xb11f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0> @ 0x15f753c -> 0xb11f50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh<ref$<str$> >
// 信息 ID: b004fe0d0c504300fd11df3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] country_core::utils::load_json::load_json_as_mesh -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 68 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_sky_and_sun::startup_sky_and_sun @ 0x864478 -> 0xb0eeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8efdf0 -> 0xb0eeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8eff61 -> 0xb0eeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f003e -> 0xb0eeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f00f1 -> 0xb0eeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f01a4 -> 0xb0eeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f0257 -> 0xb0eeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f030a -> 0xb0eeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f03bd -> 0xb0eeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f0470 -> 0xb0eeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f0523 -> 0xb0eeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f05d6 -> 0xb0eeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f06ab -> 0xb0eeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f0807 -> 0xb0eeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f0999 -> 0xb0eeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x8f1a7a -> 0xb0eeb0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>
// 信息 ID: b1dcfa7ce058dbf441b5d670
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::load_json::load_json_as_mesh -> ["std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_meadow::init_meadow @ 0x863dd6 -> 0xb0fe60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::clearing::init_clearing @ 0xa79327 -> 0xb0fe60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::load_prefabs_impl @ 0x12a2a0f -> 0xb0fe60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x7b68b8 -> 0x1286bf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json
// 信息 ID: 802e4c2fc0509dc98ab8ac8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0xb0f633 -> 0x83e210 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> @ 0xb10623 -> 0x83e210 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<alloc::string::String> @ 0xb116a3 -> 0x83e210 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> > @ 0xb126e3 -> 0x83e210 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> @ 0x1287307 -> 0x83e210 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<alloc::string::String> @ 0x1288457 -> 0x83e210 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0x2608607 -> 0x83e210 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$0
// 信息 ID: 5b7ac7af6efe9716867c9c84
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$0 <- core::iter::adapters::map::impl$2::fold<array$<f32,4>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,4>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,4>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$0 <- core::iter::adapters::map::impl$2::fold<array$<f32,4>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,4>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,4>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<f32,4>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,4>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,4>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$0> > > >; module=1610, inline_record=49260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$0::closure$0
// 信息 ID: f262220c218c1eb2ce714208
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure$0::closure_env$0> >; module=1585, inline_record=384320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$1
// 信息 ID: 83d33486e434ccf31108b4e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$1 <- core::iter::adapters::map::impl$2::fold<array$<f32,3>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$1 <- core::iter::adapters::map::impl$2::fold<array$<f32,3>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<f32,3>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1> > > >; module=1610, inline_record=68368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$1::closure$0
// 信息 ID: fbe59e9b7d3812b6d66a520b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<f32,3>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1> > > >; module=1610, inline_record=68480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$1::closure$1
// 信息 ID: b43056d045d6abb06796e8d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure$1::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure$1::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure$1::closure_env$1> >; module=1585, inline_record=303076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$2
// 信息 ID: 2357f1aa3918e27636c44322
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,2>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$2 <- core::iter::adapters::map::impl$2::fold<array$<f32,2>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,2>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$2 <- core::iter::adapters::map::impl$2::fold<array$<f32,2>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,2>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<f32,2>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,2>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2> > > >; module=1610, inline_record=36192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$2::closure$0
// 信息 ID: c0c022094be35ade364660f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<f32,2>,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,2>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$2> > > >; module=1610, inline_record=36304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$2::closure$1
// 信息 ID: df860341a130c05b8b61a48a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure$2::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure$2::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure$2::closure_env$1> >; module=1585, inline_record=417888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$3
// 信息 ID: 33f9376b4501db31ed69ed7c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$3 <- alloc::vec::spec_from_iter::impl$0::from_iter<i32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<i32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$3> >; module=1585, inline_record=294400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$4
// 信息 ID: 03188a3cca332147f95f5232
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$4 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$4> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$4> >; module=1585, inline_record=270680; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
