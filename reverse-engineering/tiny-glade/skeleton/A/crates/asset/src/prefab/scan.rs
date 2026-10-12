// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/asset/src/prefab/scan.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 28f64d6e98033bf21a200ea003d9f186187143aeaf54bd6c3cf9cc90a68dba4f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: scan.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2902, 3977
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use asset; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use ron; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::impl$2::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x172B9
// 首条位置: {"module":2902,"inlinee":94905}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::scan::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":2902,"record_offset":3724,"parent_record_offset":3236,"ancestor_record_offset":3236,"inlinee":94907,"source_line":20,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":4180,"parent_record_offset":3236,"ancestor_record_offset":3236,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2902,"record_offset":3272,"parent_record_offset":3236,"ancestor_record_offset":3236,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2902,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":3236,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2902,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":3236,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2902,"record_offset":3560,"parent_record_offset":3508,"ancestor_record_offset":3236,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2902,"record_offset":3608,"parent_record_offset":3584,"ancestor_record_offset":3236,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2902,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":3236,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":4204,"parent_record_offset":4180,"ancestor_record_offset":3236,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":3632,"parent_record_offset":3608,"ancestor_record_offset":3236,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2902,"record_offset":4252,"parent_record_offset":4228,"ancestor_record_offset":3236,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2902,"record_offset":3692,"parent_record_offset":3272,"ancestor_record_offset":3236,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2902,"record_offset":3748,"parent_record_offset":3724,"ancestor_record_offset":3236,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2902,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":3236,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2902,"record_offset":3396,"parent_record_offset":3368,"ancestor_record_offset":3236,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2902,"record_offset":3508,"parent_record_offset":3368,"ancestor_record_offset":3236,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2902,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":3236,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2902,"record_offset":3420,"parent_record_offset":3396,"ancestor_record_offset":3236,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":2902,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":3236,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2902,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":3236,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2902,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":3236,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::impl$3::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x172BB
// 首条位置: {"module":2902,"inlinee":94907}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2902,"record_offset":3748,"parent_record_offset":3724,"ancestor_record_offset":3724,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2902,"record_offset":4012,"parent_record_offset":3984,"ancestor_record_offset":3724,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2902,"record_offset":4064,"parent_record_offset":4040,"ancestor_record_offset":3724,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2902,"record_offset":4040,"parent_record_offset":3984,"ancestor_record_offset":3724,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2902,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":3724,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2902,"record_offset":3944,"parent_record_offset":3920,"ancestor_record_offset":3724,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":3724,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2902,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":3724,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2902,"record_offset":3872,"parent_record_offset":3844,"ancestor_record_offset":3724,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2902,"record_offset":3984,"parent_record_offset":3844,"ancestor_record_offset":3724,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2902,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":3724,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2902,"record_offset":3896,"parent_record_offset":3872,"ancestor_record_offset":3724,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":2902,"record_offset":3920,"parent_record_offset":3896,"ancestor_record_offset":3724,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2902,"record_offset":3844,"parent_record_offset":3820,"ancestor_record_offset":3724,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2902,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":3724,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::PrefabScanResult::add_file_to_prefab_dependency
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x172C3
// 首条位置: {"module":2902,"inlinee":94915}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::PrefabDependencies::add_file_to_prefab_dependency
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2902,"record_offset":716,"rva":26110352,"va":5394819472}
// 编译实例 RVAs: 0x18e6990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$Q$u20$as$u20$hashbrown..Equivalent$LT$K$GT$$GT$::equivalent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3908087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394819472,"raw_hex":"e8f7a13b00","size":5,"soft_fail":false,"target_va":5398727808,"va":5394819716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ceb70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23167174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394819472,"raw_hex":"e83a7f9efe","size":5,"soft_fail":false,"target_va":5371652976,"va":5394820145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22070217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394819472,"raw_hex":"e8373caffe","size":5,"soft_fail":false,"target_va":5372749696,"va":5394819908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22070562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394819472,"raw_hex":"e8de3aaffe","size":5,"soft_fail":false,"target_va":5372749712,"va":5394820269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14041b5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21804485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394819472,"raw_hex":"e83b4ab3fe","size":5,"soft_fail":false,"target_va":5373015536,"va":5394820016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140852b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17383395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394819472,"raw_hex":"e81dc0f6fe","size":5,"soft_fail":false,"target_va":5377436496,"va":5394819886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ca06f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3906798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394819472,"raw_hex":"e8ee9c3b00","size":5,"soft_fail":false,"target_va":5398726384,"va":5394819581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ca1740
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3910568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394819472,"raw_hex":"e8a8ab3b00","size":5,"soft_fail":false,"target_va":5398730560,"va":5394819987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8158071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394819472,"raw_hex":"e8777b7c00","size":5,"soft_fail":false,"target_va":5402978144,"va":5394820068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-23514958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394819472,"raw_hex":"e8b23099fe","size":5,"soft_fail":false,"target_va":5371304880,"va":5394819833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2902,"record_offset":2284,"parent_record_offset":716,"ancestor_record_offset":716,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2902,"record_offset":2440,"parent_record_offset":716,"ancestor_record_offset":716,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2902,"record_offset":1940,"parent_record_offset":716,"ancestor_record_offset":716,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":2552,"parent_record_offset":716,"ancestor_record_offset":716,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2902,"record_offset":896,"parent_record_offset":716,"ancestor_record_offset":716,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2902,"record_offset":2524,"parent_record_offset":716,"ancestor_record_offset":716,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$56::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2902,"record_offset":2256,"parent_record_offset":716,"ancestor_record_offset":716,"inlinee":4727,"source_line":1902,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2902,"record_offset":2344,"parent_record_offset":2320,"ancestor_record_offset":716,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2902,"record_offset":2368,"parent_record_offset":2344,"ancestor_record_offset":716,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2902,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":716,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2902,"record_offset":2320,"parent_record_offset":2284,"ancestor_record_offset":716,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2902,"record_offset":2796,"parent_record_offset":2740,"ancestor_record_offset":716,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2902,"record_offset":2768,"parent_record_offset":2740,"ancestor_record_offset":716,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2902,"record_offset":2012,"parent_record_offset":1980,"ancestor_record_offset":716,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2902,"record_offset":2740,"parent_record_offset":2712,"ancestor_record_offset":716,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2902,"record_offset":2464,"parent_record_offset":2440,"ancestor_record_offset":716,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2902,"record_offset":2964,"parent_record_offset":2928,"ancestor_record_offset":716,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2902,"record_offset":2120,"parent_record_offset":2096,"ancestor_record_offset":716,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2902,"record_offset":1980,"parent_record_offset":1940,"ancestor_record_offset":716,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2902,"record_offset":2488,"parent_record_offset":2464,"ancestor_record_offset":716,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2902,"record_offset":1160,"parent_record_offset":1136,"ancestor_record_offset":716,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2902,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":716,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2902,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":716,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2902,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":716,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":2072,"parent_record_offset":1940,"ancestor_record_offset":716,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2902,"record_offset":1608,"parent_record_offset":1584,"ancestor_record_offset":716,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2902,"record_offset":1500,"parent_record_offset":1476,"ancestor_record_offset":716,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2902,"record_offset":2996,"parent_record_offset":2964,"ancestor_record_offset":716,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":2396,"parent_record_offset":2368,"ancestor_record_offset":716,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":2044,"parent_record_offset":1940,"ancestor_record_offset":716,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2902,"record_offset":2156,"parent_record_offset":2120,"ancestor_record_offset":716,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2902,"record_offset":2180,"parent_record_offset":2156,"ancestor_record_offset":716,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2902,"record_offset":2212,"parent_record_offset":2120,"ancestor_record_offset":716,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2902,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":716,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2902,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":716,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2902,"record_offset":1648,"parent_record_offset":1428,"ancestor_record_offset":716,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2902,"record_offset":976,"parent_record_offset":944,"ancestor_record_offset":716,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2902,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":716,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2902,"record_offset":1536,"parent_record_offset":1428,"ancestor_record_offset":716,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2902,"record_offset":1476,"parent_record_offset":1452,"ancestor_record_offset":716,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2902,"record_offset":1772,"parent_record_offset":1056,"ancestor_record_offset":716,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2902,"record_offset":1452,"parent_record_offset":1428,"ancestor_record_offset":716,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2902,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":716,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2902,"record_offset":1004,"parent_record_offset":944,"ancestor_record_offset":716,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2902,"record_offset":1056,"parent_record_offset":1028,"ancestor_record_offset":716,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2902,"record_offset":1308,"parent_record_offset":1276,"ancestor_record_offset":716,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2902,"record_offset":1336,"parent_record_offset":1308,"ancestor_record_offset":716,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2902,"record_offset":1396,"parent_record_offset":1276,"ancestor_record_offset":716,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2902,"record_offset":1276,"parent_record_offset":1056,"ancestor_record_offset":716,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2902,"record_offset":1108,"parent_record_offset":1056,"ancestor_record_offset":716,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2902,"record_offset":1428,"parent_record_offset":1056,"ancestor_record_offset":716,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2902,"record_offset":1136,"parent_record_offset":1056,"ancestor_record_offset":716,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2902,"record_offset":1220,"parent_record_offset":1056,"ancestor_record_offset":716,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2902,"record_offset":1680,"parent_record_offset":1056,"ancestor_record_offset":716,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::scan_asset_tree
// 记录形式: compiled_procedure; instances=1
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2902,"record_offset":3100,"rva":26111344,"va":5394820464}
// 编译实例 RVAs: 0x18e6d70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14036efd0
//     owner=unresolved_namespace; count=1
//     target_aliases=27 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22511801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394820464,"raw_hex":"e8477fa8fe","size":5,"soft_fail":false,"target_va":5372309456,"va":5394821252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=3
//     target_aliases=16 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22195516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394820464,"raw_hex":"e8c452adfe","size":5,"soft_fail":false,"target_va":5372625024,"va":5394820535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22071152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394820464,"raw_hex":"e89038affe","size":5,"soft_fail":false,"target_va":5372749712,"va":5394820859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df1810
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11491135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394820464,"raw_hex":"e8c1a850ff","size":5,"soft_fail":false,"target_va":5383329808,"va":5394820938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::prefab::scan::scan_asset_tree_walker
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394820464,"raw_hex":"e8b9020000","size":5,"soft_fail":false,"target_va":5394821456,"va":5394820754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$asset..prefab..scan..PrefabScanResult$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3914558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394820464,"raw_hex":"e83ebb3b00","size":5,"soft_fail":false,"target_va":5398735824,"va":5394821261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t3918236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394820464,"raw_hex":"e89cc93b00","size":5,"soft_fail":false,"target_va":5398739040,"va":5394820799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-17454294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394820464,"raw_hex":"e82aabf5fe","size":5,"soft_fail":false,"target_va":5377366224,"va":5394820513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2902,"record_offset":4296,"parent_record_offset":3100,"ancestor_record_offset":3100,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$13::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":2902,"record_offset":4324,"parent_record_offset":3100,"ancestor_record_offset":3100,"inlinee":94909,"source_line":6,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::scan::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":2902,"record_offset":3236,"parent_record_offset":3100,"ancestor_record_offset":3100,"inlinee":94905,"source_line":14,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":4820,"parent_record_offset":3100,"ancestor_record_offset":3100,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2902,"record_offset":4792,"parent_record_offset":3100,"ancestor_record_offset":3100,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2902,"record_offset":5304,"parent_record_offset":5280,"ancestor_record_offset":3100,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2902,"record_offset":5280,"parent_record_offset":5216,"ancestor_record_offset":3100,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2902,"record_offset":5248,"parent_record_offset":5216,"ancestor_record_offset":3100,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2902,"record_offset":5216,"parent_record_offset":5184,"ancestor_record_offset":3100,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2902,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":3100,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::scan::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":2902,"record_offset":3724,"parent_record_offset":3236,"ancestor_record_offset":3100,"inlinee":94907,"source_line":20,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2902,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":3100,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2902,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":3100,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2902,"record_offset":3560,"parent_record_offset":3508,"ancestor_record_offset":3100,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2902,"record_offset":3608,"parent_record_offset":3584,"ancestor_record_offset":3100,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2902,"record_offset":5452,"parent_record_offset":5424,"ancestor_record_offset":3100,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2902,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":3100,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":4180,"parent_record_offset":3236,"ancestor_record_offset":3100,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":3632,"parent_record_offset":3608,"ancestor_record_offset":3100,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2902,"record_offset":4252,"parent_record_offset":4228,"ancestor_record_offset":3100,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2902,"record_offset":3692,"parent_record_offset":3272,"ancestor_record_offset":3100,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2902,"record_offset":3272,"parent_record_offset":3236,"ancestor_record_offset":3100,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2902,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":3100,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2902,"record_offset":3396,"parent_record_offset":3368,"ancestor_record_offset":3100,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2902,"record_offset":3508,"parent_record_offset":3368,"ancestor_record_offset":3100,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2902,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":3100,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2902,"record_offset":3420,"parent_record_offset":3396,"ancestor_record_offset":3100,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":2902,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":3100,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2902,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":3100,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2902,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":3100,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::scan_asset_tree_walker
// 记录形式: compiled_procedure; instances=1
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2902,"record_offset":6864,"rva":26112336,"va":5394821456}
// 编译实例 RVAs: 0x18e7150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17414256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e89047f6fe","size":5,"soft_fail":false,"target_va":5377408080,"va":5394822331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-17358830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e81220f7fe","size":5,"soft_fail":false,"target_va":5377465440,"va":5394824265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ceb70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23171235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e85d6f9efe","size":5,"soft_fail":false,"target_va":5371652976,"va":5394824206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfa00
//     owner=unresolved_namespace; count=8
//     target_aliases=3 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23167000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e8e87f9efe","size":5,"soft_fail":false,"target_va":5371656704,"va":5394823699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036ef60
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22517192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e8386aa8fe","size":5,"soft_fail":false,"target_va":5372309344,"va":5394826531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22507514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e80690a8fe","size":5,"soft_fail":false,"target_va":5372314208,"va":5394821717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22073811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e82d2eaffe","size":5,"soft_fail":false,"target_va":5372749696,"va":5394823502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=21
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22072048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e81035affe","size":5,"soft_fail":false,"target_va":5372749712,"va":5394821755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=11
//     target_aliases=28 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19650526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e82228d4fe","size":5,"soft_fail":false,"target_va":5375171056,"va":5394821577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14084b050
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17417551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e8b13af6fe","size":5,"soft_fail":false,"target_va":5377405008,"va":5394822554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ca1d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3908052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e8d4a13b00","size":5,"soft_fail":false,"target_va":5398732064,"va":5394824007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ca3df0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3918461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e87dca3b00","size":5,"soft_fail":false,"target_va":5398740464,"va":5394821998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ca4a50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3921447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e827d63b00","size":5,"soft_fail":false,"target_va":5398743632,"va":5394822180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8771204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e884d68500","size":5,"soft_fail":false,"target_va":5403593360,"va":5394822151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8153244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e89c687c00","size":5,"soft_fail":false,"target_va":5402978144,"va":5394824895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-23516704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e8e02999fe","size":5,"soft_fail":false,"target_va":5371305232,"va":5394821931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::arcinner_layout_for_value_layout
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17286546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e86e3af8fe","size":5,"soft_fail":false,"target_va":5377536928,"va":5394823469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8695876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e844b08400","size":5,"soft_fail":false,"target_va":5403520512,"va":5394824631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8730902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e816398500","size":5,"soft_fail":false,"target_va":5403555264,"va":5394824357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::prefab::scan::PrefabDependencies::add_file_to_prefab_dependency
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e8a6edffff","size":5,"soft_fail":false,"target_va":5394819472,"va":5394824165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::prefab::scan::load_prefab_config
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e890120000","size":5,"soft_fail":false,"target_va":5394826576,"va":5394821819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::prefab::scan::scan_asset_tree_walker
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e819fbffff","size":5,"soft_fail":false,"target_va":5394821456,"va":5394822706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t8154062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e8ce6b7c00","size":5,"soft_fail":false,"target_va":5402978880,"va":5394824813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8155183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e82f707c00","size":5,"soft_fail":false,"target_va":5402980128,"va":5394824940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t3914980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e8e4bc3b00","size":5,"soft_fail":false,"target_va":5398739040,"va":5394824055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] relative_path::RelativePath::join
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3921347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e8c3d53b00","size":5,"soft_fail":false,"target_va":5398744000,"va":5394822648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_name
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-17413813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e84b49f6fe","size":5,"soft_fail":false,"target_va":5377408720,"va":5394822528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17413864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e81849f6fe","size":5,"soft_fail":false,"target_va":5377408592,"va":5394822451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-17414378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e81647f6fe","size":5,"soft_fail":false,"target_va":5377408224,"va":5394822597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17381787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e865c6f6fe","size":5,"soft_fail":false,"target_va":5377439776,"va":5394821558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17383365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e83bc0f6fe","size":5,"soft_fail":false,"target_va":5377439664,"va":5394823024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_stem
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17383812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e87cbef6fe","size":5,"soft_fail":false,"target_va":5377439520,"va":5394823327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17353881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e86733f7fe","size":5,"soft_fail":false,"target_va":5377468224,"va":5394822100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17348343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394821456,"raw_hex":"e80949f7fe","size":5,"soft_fail":false,"target_va":5377473264,"va":5394821602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2902,"record_offset":7852,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2902,"record_offset":8404,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":8655,"source_line":1192,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::PrefabName::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":2902,"record_offset":11640,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":64883,"source_line":25,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$1::inherit_from
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":2902,"record_offset":8276,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":94911,"source_line":21,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::impl$8::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":2902,"record_offset":13916,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":23247,"source_line":10,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::scan::PrefabScanResult::add_file_to_prefab_dependency
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":2902,"record_offset":14628,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":94915,"source_line":26,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2902,"record_offset":10320,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2902,"record_offset":16812,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2902,"record_offset":10896,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2902,"record_offset":9188,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":7380,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2902,"record_offset":7820,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2902,"record_offset":15728,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2902,"record_offset":14224,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2902,"record_offset":14308,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2902,"record_offset":11056,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2902,"record_offset":14028,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2902,"record_offset":9128,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsString::as_os_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2902,"record_offset":9044,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":37781,"source_line":204,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2902,"record_offset":10168,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] std::fs::Metadata::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2902,"record_offset":8932,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":13851,"source_line":1193,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2902,"record_offset":8304,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":8478,"source_line":2465,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2902,"record_offset":7256,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4766,"source_line":2862,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2902,"record_offset":7088,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2902,"record_offset":7116,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2902,"record_offset":12112,"parent_record_offset":12088,"ancestor_record_offset":6864,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2902,"record_offset":12140,"parent_record_offset":12112,"ancestor_record_offset":6864,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2902,"record_offset":7620,"parent_record_offset":7596,"ancestor_record_offset":6864,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2902,"record_offset":12088,"parent_record_offset":12064,"ancestor_record_offset":6864,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2902,"record_offset":7596,"parent_record_offset":7540,"ancestor_record_offset":6864,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2902,"record_offset":7568,"parent_record_offset":7540,"ancestor_record_offset":6864,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2902,"record_offset":7928,"parent_record_offset":7896,"ancestor_record_offset":6864,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2902,"record_offset":7540,"parent_record_offset":7512,"ancestor_record_offset":6864,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_layout
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2902,"record_offset":12020,"parent_record_offset":11884,"ancestor_record_offset":6864,"inlinee":10173,"source_line":1845,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2902,"record_offset":11884,"parent_record_offset":11840,"ancestor_record_offset":6864,"inlinee":14835,"source_line":1935,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2902,"record_offset":11840,"parent_record_offset":11800,"ancestor_record_offset":6864,"inlinee":14834,"source_line":1949,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::initialize_arcinner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2902,"record_offset":12280,"parent_record_offset":12020,"ancestor_record_offset":6864,"inlinee":10176,"source_line":1877,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2902,"record_offset":13124,"parent_record_offset":13088,"ancestor_record_offset":6864,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$22::allocate_for_layout::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2902,"record_offset":12248,"parent_record_offset":12212,"ancestor_record_offset":6864,"inlinee":10182,"source_line":1852,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$24::allocate_for_slice::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2902,"record_offset":12064,"parent_record_offset":12020,"ancestor_record_offset":6864,"inlinee":14837,"source_line":1939,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$27::from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2902,"record_offset":11800,"parent_record_offset":11760,"ancestor_record_offset":6864,"inlinee":14833,"source_line":2041,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2902,"record_offset":13940,"parent_record_offset":13916,"ancestor_record_offset":6864,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2902,"record_offset":13088,"parent_record_offset":13064,"ancestor_record_offset":6864,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$56::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2902,"record_offset":11760,"parent_record_offset":11720,"ancestor_record_offset":6864,"inlinee":14831,"source_line":3354,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$57::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2902,"record_offset":11720,"parent_record_offset":11680,"ancestor_record_offset":6864,"inlinee":14829,"source_line":3372,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2902,"record_offset":7896,"parent_record_offset":7852,"ancestor_record_offset":6864,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::scan::scan_asset_tree_walker::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":2902,"record_offset":10928,"parent_record_offset":10896,"ancestor_record_offset":6864,"inlinee":94913,"source_line":113,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":2902,"record_offset":11928,"parent_record_offset":11884,"ancestor_record_offset":6864,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":2902,"record_offset":11952,"parent_record_offset":11928,"ancestor_record_offset":6864,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2902,"record_offset":10376,"parent_record_offset":10348,"ancestor_record_offset":6864,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2902,"record_offset":11680,"parent_record_offset":11640,"ancestor_record_offset":6864,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2902,"record_offset":12432,"parent_record_offset":11840,"ancestor_record_offset":6864,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2902,"record_offset":11012,"parent_record_offset":10928,"ancestor_record_offset":6864,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=141
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":7348,"parent_record_offset":7256,"ancestor_record_offset":6864,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2902,"record_offset":14272,"parent_record_offset":14248,"ancestor_record_offset":6864,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2902,"record_offset":12308,"parent_record_offset":12280,"ancestor_record_offset":6864,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2902,"record_offset":13156,"parent_record_offset":13124,"ancestor_record_offset":6864,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2902,"record_offset":14340,"parent_record_offset":14308,"ancestor_record_offset":6864,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":12168,"parent_record_offset":12140,"ancestor_record_offset":6864,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":7960,"parent_record_offset":7852,"ancestor_record_offset":6864,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2902,"record_offset":7308,"parent_record_offset":7284,"ancestor_record_offset":6864,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2902,"record_offset":9156,"parent_record_offset":9128,"ancestor_record_offset":6864,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2902,"record_offset":11984,"parent_record_offset":11884,"ancestor_record_offset":6864,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2902,"record_offset":12212,"parent_record_offset":12020,"ancestor_record_offset":6864,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2902,"record_offset":10404,"parent_record_offset":10376,"ancestor_record_offset":6864,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2902,"record_offset":10432,"parent_record_offset":10404,"ancestor_record_offset":6864,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2902,"record_offset":14248,"parent_record_offset":14224,"ancestor_record_offset":6864,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2902,"record_offset":10348,"parent_record_offset":10320,"ancestor_record_offset":6864,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2902,"record_offset":13964,"parent_record_offset":13940,"ancestor_record_offset":6864,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2902,"record_offset":13988,"parent_record_offset":13964,"ancestor_record_offset":6864,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2902,"record_offset":13184,"parent_record_offset":13124,"ancestor_record_offset":6864,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2902,"record_offset":13216,"parent_record_offset":13088,"ancestor_record_offset":6864,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2902,"record_offset":14176,"parent_record_offset":14152,"ancestor_record_offset":6864,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2902,"record_offset":10956,"parent_record_offset":10928,"ancestor_record_offset":6864,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2902,"record_offset":7164,"parent_record_offset":7140,"ancestor_record_offset":6864,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2902,"record_offset":7212,"parent_record_offset":7188,"ancestor_record_offset":6864,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2902,"record_offset":7188,"parent_record_offset":7164,"ancestor_record_offset":6864,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2902,"record_offset":8956,"parent_record_offset":8932,"ancestor_record_offset":6864,"inlinee":8480,"source_line":1597,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2902,"record_offset":7284,"parent_record_offset":7256,"ancestor_record_offset":6864,"inlinee":4767,"source_line":1926,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2902,"record_offset":7140,"parent_record_offset":7116,"ancestor_record_offset":6864,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":2902,"record_offset":8980,"parent_record_offset":8956,"ancestor_record_offset":6864,"inlinee":8482,"source_line":1026,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":2902,"record_offset":9004,"parent_record_offset":8980,"ancestor_record_offset":6864,"inlinee":8483,"source_line":1032,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::scan_asset_tree_walker::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x172C1
// 首条位置: {"module":2902,"inlinee":94913}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2902,"record_offset":11012,"parent_record_offset":10928,"ancestor_record_offset":10928,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2902,"record_offset":10956,"parent_record_offset":10928,"ancestor_record_offset":10928,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2902,"record_offset":10980,"parent_record_offset":10956,"ancestor_record_offset":10928,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::load_prefab_config
// 记录形式: compiled_procedure; instances=1
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2902,"record_offset":20724,"rva":26117456,"va":5394826576}
// 编译实例 RVAs: 0x18e8550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22077066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394826576,"raw_hex":"e87621affe","size":5,"soft_fail":false,"target_va":5372749712,"va":5394826773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ca4d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3917660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394826576,"raw_hex":"e85cc73b00","size":5,"soft_fail":false,"target_va":5398744368,"va":5394826703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17297124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394826576,"raw_hex":"e81c11f8fe","size":5,"soft_fail":false,"target_va":5377529888,"va":5394827007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h1d09fb2f238b2066 (.llvm.2838924830685125660)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8726924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394826576,"raw_hex":"e88c298500","size":5,"soft_fail":false,"target_va":5403553984,"va":5394827055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8728515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394826576,"raw_hex":"e8c32f8500","size":5,"soft_fail":false,"target_va":5403555408,"va":5394826888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17462966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394826576,"raw_hex":"e84a89f5fe","size":5,"soft_fail":false,"target_va":5377364336,"va":5394827297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17421010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394826576,"raw_hex":"e82e2df6fe","size":5,"soft_fail":false,"target_va":5377405616,"va":5394826621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":2902,"record_offset":21920,"parent_record_offset":20724,"ancestor_record_offset":20724,"inlinee":10987,"source_line":58,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::scan::load_prefab_config::do_load
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":2902,"record_offset":20852,"parent_record_offset":20724,"ancestor_record_offset":20724,"inlinee":94917,"source_line":148,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2902,"record_offset":21212,"parent_record_offset":21188,"ancestor_record_offset":20724,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2902,"record_offset":21188,"parent_record_offset":21132,"ancestor_record_offset":20724,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2902,"record_offset":21988,"parent_record_offset":21964,"ancestor_record_offset":20724,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2902,"record_offset":22036,"parent_record_offset":22012,"ancestor_record_offset":20724,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2902,"record_offset":21160,"parent_record_offset":21132,"ancestor_record_offset":20724,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2902,"record_offset":21132,"parent_record_offset":21108,"ancestor_record_offset":20724,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":2902,"record_offset":22100,"parent_record_offset":22076,"ancestor_record_offset":20724,"inlinee":10981,"source_line":316,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":2902,"record_offset":22076,"parent_record_offset":21920,"ancestor_record_offset":20724,"inlinee":10980,"source_line":33,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::scan::load_prefab_config::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":2902,"record_offset":21964,"parent_record_offset":21920,"ancestor_record_offset":20724,"inlinee":94918,"source_line":153,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2902,"record_offset":22012,"parent_record_offset":21988,"ancestor_record_offset":20724,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":21060,"parent_record_offset":20852,"ancestor_record_offset":20724,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2902,"record_offset":20932,"parent_record_offset":20852,"ancestor_record_offset":20724,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2902,"record_offset":21260,"parent_record_offset":20852,"ancestor_record_offset":20724,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":2902,"record_offset":20964,"parent_record_offset":20852,"ancestor_record_offset":20724,"inlinee":10331,"source_line":108,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/options.rs
//     evidence={"module":2902,"record_offset":20988,"parent_record_offset":20964,"ancestor_record_offset":20724,"inlinee":4925,"source_line":51,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2902,"record_offset":20904,"parent_record_offset":20852,"ancestor_record_offset":20724,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::load_prefab_config::do_load
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 148
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x172C5
// 首条位置: {"module":2902,"inlinee":94917}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":21060,"parent_record_offset":20852,"ancestor_record_offset":20852,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2902,"record_offset":20932,"parent_record_offset":20852,"ancestor_record_offset":20852,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2902,"record_offset":21260,"parent_record_offset":20852,"ancestor_record_offset":20852,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":2902,"record_offset":20964,"parent_record_offset":20852,"ancestor_record_offset":20852,"inlinee":10331,"source_line":108,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2902,"record_offset":20904,"parent_record_offset":20852,"ancestor_record_offset":20852,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2902,"record_offset":21212,"parent_record_offset":21188,"ancestor_record_offset":20852,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2902,"record_offset":21188,"parent_record_offset":21132,"ancestor_record_offset":20852,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2902,"record_offset":21160,"parent_record_offset":21132,"ancestor_record_offset":20852,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2902,"record_offset":21132,"parent_record_offset":21108,"ancestor_record_offset":20852,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2902,"record_offset":21084,"parent_record_offset":21060,"ancestor_record_offset":20852,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/options.rs
//     evidence={"module":2902,"record_offset":20988,"parent_record_offset":20964,"ancestor_record_offset":20852,"inlinee":4925,"source_line":51,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::load_prefab_config::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x172C6
// 首条位置: {"module":2902,"inlinee":94918}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2902,"record_offset":21988,"parent_record_offset":21964,"ancestor_record_offset":21964,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2902,"record_offset":22036,"parent_record_offset":22012,"ancestor_record_offset":21964,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02902.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2902,"record_offset":22012,"parent_record_offset":21988,"ancestor_record_offset":21964,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02902.jsonl.gz"}
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
// 完整结构及原始类型记录: scan.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::prefab::scan::PrefabDependencies
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::prefab::scan::PrefabScanResult
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<asset::prefab::scan::PrefabScanResult>", "constructor": "core::ptr::drop_in_place", "arguments": ["asset::prefab::scan::PrefabScanResult"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: scan.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: scan.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: asset::prefab::scan::PrefabDependencies::add_file_to_prefab_dependency
// 信息 ID: 011e582b03236d8149eaeeed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_prefabs::load_prefabs @ 0x11110db -> 0x18e6990 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x18e7be5 -> 0x18e6990 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::PrefabScanResult::add_file_to_prefab_dependency
// 信息 ID: 7f82aff7f26108d5c836744a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree_walker; module=2902, inline_record=14628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::impl$2::default
// 信息 ID: 66a8b2a31bf406c7970a8594
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree; module=2902, inline_record=3236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::impl$3::default
// 信息 ID: 66df0b3cad4d69db48435a03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree; module=2902, inline_record=3724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::load_prefab_config
// 信息 ID: fbf31e5f293f6011dff6c75a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x18e780d -> 0x18e8550 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x18e72bb -> 0x18e8550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::load_prefab_config::closure$0
// 信息 ID: e284c5c3c922d42012a92813
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset::prefab::scan::load_prefab_config", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::load_prefab_config; module=2902, inline_record=21964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::load_prefab_config::do_load
// 信息 ID: 68d1d282e8550bafe229ec4d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::load_prefab_config; module=2902, inline_record=20852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::scan_asset_tree
// 信息 ID: 7b1df5ca321efdabf0158667
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_prefabs::load_prefabs @ 0x11107ba -> 0x18e6d70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::scan_asset_tree_walker
// 信息 ID: 32131b53fea41da37ac8751f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::prefab::scan::scan_asset_tree @ 0x18e6e92 -> 0x18e7150 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x18e7632 -> 0x18e7150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::scan_asset_tree_walker::closure$1
// 信息 ID: 6576f0e4925878a38e7b21d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset::prefab::scan::scan_asset_tree_walker", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree_walker; module=2902, inline_record=10928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
