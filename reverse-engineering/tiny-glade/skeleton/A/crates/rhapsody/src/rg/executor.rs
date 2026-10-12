// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/rg/executor.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f1d6480d5c648ca7640f1b8b6b03a48fc6edb1b80ee52a4276fb73fb9ad7856e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: executor.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 178, 556, 567
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"PDB_registry_path_only","version":"0.7.4"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::executor::GraphExecutor::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":567,"record_offset":4888,"rva":7561472,"va":5376270592}
// 编译实例 RVAs: 0x736100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14005b630
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7187857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376270592,"raw_hex":"e86f5292ff","size":5,"soft_fail":false,"target_va":5369083440,"va":5376271292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3645902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376270592,"raw_hex":"e8325ec8ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5376270921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376270592,"raw_hex":"e89f85ffff","size":5,"soft_fail":false,"target_va":5376239920,"va":5376271244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376270592,"raw_hex":"e82f340000","size":5,"soft_fail":false,"target_va":5376284224,"va":5376270860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26709040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376270592,"raw_hex":"e8308c9701","size":5,"soft_fail":false,"target_va":5402980128,"va":5376271083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1095320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376270592,"raw_hex":"e898b61000","size":5,"soft_fail":false,"target_va":5377366224,"va":5376270899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*296(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376270592,"raw_hex":"ff9728010000","size":6,"soft_fail":false,"va":5376270669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::create_query_pool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":567,"record_offset":5252,"parent_record_offset":4888,"ancestor_record_offset":4888,"inlinee":30842,"source_line":2573,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_semaphore
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":567,"record_offset":5080,"parent_record_offset":4888,"ancestor_record_offset":4888,"inlinee":30837,"source_line":2129,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::QueryPoolCreateInfo::query_count
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":567,"record_offset":5224,"parent_record_offset":4888,"ancestor_record_offset":4888,"inlinee":30841,"source_line":7695,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$349::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":567,"record_offset":5052,"parent_record_offset":4888,"ancestor_record_offset":4888,"inlinee":30836,"source_line":7615,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$354::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":567,"record_offset":5196,"parent_record_offset":4888,"ancestor_record_offset":4888,"inlinee":30839,"source_line":7667,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":5952,"parent_record_offset":4888,"ancestor_record_offset":4888,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":567,"record_offset":5396,"parent_record_offset":4888,"ancestor_record_offset":4888,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":567,"record_offset":5896,"parent_record_offset":4888,"ancestor_record_offset":4888,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":567,"record_offset":5368,"parent_record_offset":4888,"ancestor_record_offset":4888,"inlinee":30618,"source_line":116,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":567,"record_offset":5440,"parent_record_offset":4888,"ancestor_record_offset":4888,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":567,"record_offset":5108,"parent_record_offset":5080,"ancestor_record_offset":4888,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":567,"record_offset":5132,"parent_record_offset":5108,"ancestor_record_offset":4888,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":567,"record_offset":5156,"parent_record_offset":5132,"ancestor_record_offset":4888,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":567,"record_offset":5732,"parent_record_offset":5704,"ancestor_record_offset":4888,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":567,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":4888,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":567,"record_offset":5760,"parent_record_offset":5704,"ancestor_record_offset":4888,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":567,"record_offset":5808,"parent_record_offset":5784,"ancestor_record_offset":4888,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":567,"record_offset":5660,"parent_record_offset":5636,"ancestor_record_offset":4888,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":5976,"parent_record_offset":5952,"ancestor_record_offset":4888,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":5832,"parent_record_offset":5808,"ancestor_record_offset":4888,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":567,"record_offset":5496,"parent_record_offset":5468,"ancestor_record_offset":4888,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":567,"record_offset":5584,"parent_record_offset":5552,"ancestor_record_offset":4888,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":567,"record_offset":5704,"parent_record_offset":5552,"ancestor_record_offset":4888,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":567,"record_offset":5468,"parent_record_offset":5440,"ancestor_record_offset":4888,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":567,"record_offset":5608,"parent_record_offset":5584,"ancestor_record_offset":4888,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":567,"record_offset":5636,"parent_record_offset":5608,"ancestor_record_offset":4888,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":567,"record_offset":5552,"parent_record_offset":5524,"ancestor_record_offset":4888,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":567,"record_offset":5524,"parent_record_offset":5496,"ancestor_record_offset":4888,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::executor::GraphExecutor::wait_submits_done
// 记录形式: compiled_procedure; instances=1
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":567,"record_offset":6040,"rva":7562192,"va":5376271312}
// 编译实例 RVAs: 0x7363d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140743130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t51759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376271312,"raw_hex":"e82fca0000","size":5,"soft_fail":false,"target_va":5376323888,"va":5376272124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26706419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376271312,"raw_hex":"e8f3819701","size":5,"soft_fail":false,"target_va":5402978688,"va":5376272264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26706625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376271312,"raw_hex":"e8c1829701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376272250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376271312,"raw_hex":"e8b1f3ffff","size":5,"soft_fail":false,"target_va":5376269024,"va":5376272170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26708326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376271312,"raw_hex":"e866899701","size":5,"soft_fail":false,"target_va":5402980128,"va":5376271797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376271312,"raw_hex":"e887f7ffff","size":5,"soft_fail":false,"target_va":5376269216,"va":5376271380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26709201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376271312,"raw_hex":"e8d18c9701","size":5,"soft_fail":false,"target_va":5402980656,"va":5376271450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t84947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376271312,"raw_hex":"e8d34b0100","size":5,"soft_fail":false,"target_va":5376356912,"va":5376271960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t88827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376271312,"raw_hex":"e8fb5a0100","size":5,"soft_fail":false,"target_va":5376360560,"va":5376271728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t84788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376271312,"raw_hex":"e8344b0100","size":5,"soft_fail":false,"target_va":5376356144,"va":5376271351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hf5f0f90a77fe959e (.llvm.16944548595553555407)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10777687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376271312,"raw_hex":"e85774a400","size":5,"soft_fail":false,"target_va":5387049440,"va":5376271748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":567,"record_offset":8820,"parent_record_offset":6040,"ancestor_record_offset":6040,"inlinee":10716,"source_line":1610,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::ArrayVec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":567,"record_offset":9048,"parent_record_offset":6040,"ancestor_record_offset":6040,"inlinee":15307,"source_line":178,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":567,"record_offset":9008,"parent_record_offset":6040,"ancestor_record_offset":6040,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":7788,"parent_record_offset":6040,"ancestor_record_offset":6040,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":6200,"parent_record_offset":6040,"ancestor_record_offset":6040,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":6776,"parent_record_offset":6040,"ancestor_record_offset":6040,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":567,"record_offset":8948,"parent_record_offset":8820,"ancestor_record_offset":6040,"inlinee":10717,"source_line":165,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":567,"record_offset":8852,"parent_record_offset":8820,"ancestor_record_offset":6040,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":567,"record_offset":8876,"parent_record_offset":8852,"ancestor_record_offset":6040,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":567,"record_offset":9168,"parent_record_offset":9144,"ancestor_record_offset":6040,"inlinee":15311,"source_line":113,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":567,"record_offset":9144,"parent_record_offset":9112,"ancestor_record_offset":6040,"inlinee":15310,"source_line":713,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":567,"record_offset":9256,"parent_record_offset":9200,"ancestor_record_offset":6040,"inlinee":10360,"source_line":715,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":567,"record_offset":9080,"parent_record_offset":9048,"ancestor_record_offset":6040,"inlinee":15308,"source_line":39,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":567,"record_offset":9200,"parent_record_offset":9112,"ancestor_record_offset":6040,"inlinee":15312,"source_line":54,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::try_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":567,"record_offset":9112,"parent_record_offset":9080,"ancestor_record_offset":6040,"inlinee":15309,"source_line":43,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":567,"record_offset":8172,"parent_record_offset":8144,"ancestor_record_offset":6040,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":567,"record_offset":8224,"parent_record_offset":8200,"ancestor_record_offset":6040,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":567,"record_offset":8200,"parent_record_offset":8172,"ancestor_record_offset":6040,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":567,"record_offset":8112,"parent_record_offset":8084,"ancestor_record_offset":6040,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":567,"record_offset":8144,"parent_record_offset":8112,"ancestor_record_offset":6040,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":567,"record_offset":8316,"parent_record_offset":8144,"ancestor_record_offset":6040,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":567,"record_offset":8428,"parent_record_offset":8404,"ancestor_record_offset":6040,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":567,"record_offset":8248,"parent_record_offset":8224,"ancestor_record_offset":6040,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":8900,"parent_record_offset":8876,"ancestor_record_offset":6040,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":567,"record_offset":6412,"parent_record_offset":6316,"ancestor_record_offset":6040,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":567,"record_offset":8008,"parent_record_offset":7980,"ancestor_record_offset":6040,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":567,"record_offset":7372,"parent_record_offset":6912,"ancestor_record_offset":6040,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":567,"record_offset":8056,"parent_record_offset":7884,"ancestor_record_offset":6040,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":6724,"parent_record_offset":6696,"ancestor_record_offset":6040,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":8380,"parent_record_offset":8084,"ancestor_record_offset":6040,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":567,"record_offset":8972,"parent_record_offset":8948,"ancestor_record_offset":6040,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":7148,"parent_record_offset":7124,"ancestor_record_offset":6040,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":6040,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":7332,"parent_record_offset":7300,"ancestor_record_offset":6040,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":8272,"parent_record_offset":8248,"ancestor_record_offset":6040,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":567,"record_offset":8780,"parent_record_offset":7852,"ancestor_record_offset":6040,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":567,"record_offset":9292,"parent_record_offset":9080,"ancestor_record_offset":6040,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":567,"record_offset":6348,"parent_record_offset":6316,"ancestor_record_offset":6040,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":6376,"parent_record_offset":6348,"ancestor_record_offset":6040,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":6040,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":7428,"parent_record_offset":7404,"ancestor_record_offset":6040,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":7076,"parent_record_offset":6968,"ancestor_record_offset":6040,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":567,"record_offset":6316,"parent_record_offset":6288,"ancestor_record_offset":6040,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":567,"record_offset":6808,"parent_record_offset":6776,"ancestor_record_offset":6040,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":567,"record_offset":6940,"parent_record_offset":6912,"ancestor_record_offset":6040,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":567,"record_offset":6912,"parent_record_offset":6880,"ancestor_record_offset":6040,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":567,"record_offset":6856,"parent_record_offset":6832,"ancestor_record_offset":6040,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":567,"record_offset":6880,"parent_record_offset":6856,"ancestor_record_offset":6040,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":567,"record_offset":6832,"parent_record_offset":6808,"ancestor_record_offset":6040,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":567,"record_offset":7404,"parent_record_offset":6880,"ancestor_record_offset":6040,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":6512,"parent_record_offset":6480,"ancestor_record_offset":6040,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":6672,"parent_record_offset":6512,"ancestor_record_offset":6040,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":6480,"parent_record_offset":6456,"ancestor_record_offset":6040,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":6232,"parent_record_offset":6200,"ancestor_record_offset":6040,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":6696,"parent_record_offset":6672,"ancestor_record_offset":6040,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":6288,"parent_record_offset":6256,"ancestor_record_offset":6040,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":6256,"parent_record_offset":6232,"ancestor_record_offset":6040,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":567,"record_offset":6968,"parent_record_offset":6940,"ancestor_record_offset":6040,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":567,"record_offset":7216,"parent_record_offset":6968,"ancestor_record_offset":6040,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":567,"record_offset":7188,"parent_record_offset":6968,"ancestor_record_offset":6040,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":567,"record_offset":7020,"parent_record_offset":6968,"ancestor_record_offset":6040,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":7820,"parent_record_offset":7788,"ancestor_record_offset":6040,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":567,"record_offset":7920,"parent_record_offset":7884,"ancestor_record_offset":6040,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":8084,"parent_record_offset":7884,"ancestor_record_offset":6040,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":8352,"parent_record_offset":8084,"ancestor_record_offset":6040,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":567,"record_offset":7948,"parent_record_offset":7920,"ancestor_record_offset":6040,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":567,"record_offset":7980,"parent_record_offset":7948,"ancestor_record_offset":6040,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":567,"record_offset":7884,"parent_record_offset":7852,"ancestor_record_offset":6040,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":567,"record_offset":7852,"parent_record_offset":7820,"ancestor_record_offset":6040,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::executor::GraphExecutor::wait_gpu_queue_trim
// 记录形式: compiled_procedure; instances=1
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":567,"record_offset":9332,"rva":7563280,"va":5376272400}
// 编译实例 RVAs: 0x736810
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1407437b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t52375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376272400,"raw_hex":"e897cc0000","size":5,"soft_fail":false,"target_va":5376325552,"va":5376273172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26705470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376272400,"raw_hex":"e83e7e9701","size":5,"soft_fail":false,"target_va":5402978688,"va":5376273213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376272400,"raw_hex":"e82aefffff","size":5,"soft_fail":false,"target_va":5376269024,"va":5376273329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26707232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376272400,"raw_hex":"e820859701","size":5,"soft_fail":false,"target_va":5402980128,"va":5376272891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376272400,"raw_hex":"e845f3ffff","size":5,"soft_fail":false,"target_va":5376269216,"va":5376272470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26708111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376272400,"raw_hex":"e88f889701","size":5,"soft_fail":false,"target_va":5402980656,"va":5376272540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t83847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376272400,"raw_hex":"e887470100","size":5,"soft_fail":false,"target_va":5376356912,"va":5376273060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t87733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376272400,"raw_hex":"e8b5560100","size":5,"soft_fail":false,"target_va":5376360560,"va":5376272822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t83698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376272400,"raw_hex":"e8f2460100","size":5,"soft_fail":false,"target_va":5376356144,"va":5376272441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hf5f0f90a77fe959e (.llvm.16944548595553555407)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10776593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376272400,"raw_hex":"e81170a400","size":5,"soft_fail":false,"target_va":5387049440,"va":5376272842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-4182","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5376272400,"raw_hex":"e9aaefffff","size":5,"soft_fail":false,"target_va":5376269024,"va":5376273201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":567,"record_offset":12140,"parent_record_offset":9332,"ancestor_record_offset":9332,"inlinee":22347,"source_line":1520,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":567,"record_offset":12112,"parent_record_offset":9332,"ancestor_record_offset":9332,"inlinee":30843,"source_line":439,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":11080,"parent_record_offset":9332,"ancestor_record_offset":9332,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":9492,"parent_record_offset":9332,"ancestor_record_offset":9332,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":10068,"parent_record_offset":9332,"ancestor_record_offset":9332,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":567,"record_offset":12164,"parent_record_offset":12140,"ancestor_record_offset":9332,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":567,"record_offset":12196,"parent_record_offset":12164,"ancestor_record_offset":9332,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":567,"record_offset":12264,"parent_record_offset":12164,"ancestor_record_offset":9332,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":567,"record_offset":12288,"parent_record_offset":12264,"ancestor_record_offset":9332,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":567,"record_offset":12312,"parent_record_offset":12288,"ancestor_record_offset":9332,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":12232,"parent_record_offset":12196,"ancestor_record_offset":9332,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":567,"record_offset":11464,"parent_record_offset":11436,"ancestor_record_offset":9332,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":567,"record_offset":11516,"parent_record_offset":11492,"ancestor_record_offset":9332,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":567,"record_offset":11492,"parent_record_offset":11464,"ancestor_record_offset":9332,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":567,"record_offset":11404,"parent_record_offset":11376,"ancestor_record_offset":9332,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":567,"record_offset":11436,"parent_record_offset":11404,"ancestor_record_offset":9332,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":567,"record_offset":11608,"parent_record_offset":11436,"ancestor_record_offset":9332,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":567,"record_offset":11720,"parent_record_offset":11696,"ancestor_record_offset":9332,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":567,"record_offset":11540,"parent_record_offset":11516,"ancestor_record_offset":9332,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":567,"record_offset":9704,"parent_record_offset":9608,"ancestor_record_offset":9332,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":567,"record_offset":11300,"parent_record_offset":11272,"ancestor_record_offset":9332,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":567,"record_offset":10664,"parent_record_offset":10204,"ancestor_record_offset":9332,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":567,"record_offset":11348,"parent_record_offset":11176,"ancestor_record_offset":9332,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":10016,"parent_record_offset":9988,"ancestor_record_offset":9332,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":11672,"parent_record_offset":11376,"ancestor_record_offset":9332,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":10440,"parent_record_offset":10416,"ancestor_record_offset":9332,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":9332,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":10624,"parent_record_offset":10592,"ancestor_record_offset":9332,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":11564,"parent_record_offset":11540,"ancestor_record_offset":9332,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":567,"record_offset":12072,"parent_record_offset":11144,"ancestor_record_offset":9332,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":567,"record_offset":9640,"parent_record_offset":9608,"ancestor_record_offset":9332,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":9668,"parent_record_offset":9640,"ancestor_record_offset":9332,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":10392,"parent_record_offset":10368,"ancestor_record_offset":9332,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":10720,"parent_record_offset":10696,"ancestor_record_offset":9332,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":10368,"parent_record_offset":10260,"ancestor_record_offset":9332,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":567,"record_offset":9608,"parent_record_offset":9580,"ancestor_record_offset":9332,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":567,"record_offset":10100,"parent_record_offset":10068,"ancestor_record_offset":9332,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":567,"record_offset":10232,"parent_record_offset":10204,"ancestor_record_offset":9332,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":567,"record_offset":10204,"parent_record_offset":10172,"ancestor_record_offset":9332,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":567,"record_offset":10148,"parent_record_offset":10124,"ancestor_record_offset":9332,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":567,"record_offset":10172,"parent_record_offset":10148,"ancestor_record_offset":9332,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":567,"record_offset":10124,"parent_record_offset":10100,"ancestor_record_offset":9332,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":567,"record_offset":10696,"parent_record_offset":10172,"ancestor_record_offset":9332,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":9804,"parent_record_offset":9772,"ancestor_record_offset":9332,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":9964,"parent_record_offset":9804,"ancestor_record_offset":9332,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":9772,"parent_record_offset":9748,"ancestor_record_offset":9332,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":9524,"parent_record_offset":9492,"ancestor_record_offset":9332,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":9988,"parent_record_offset":9964,"ancestor_record_offset":9332,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":9580,"parent_record_offset":9548,"ancestor_record_offset":9332,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":9548,"parent_record_offset":9524,"ancestor_record_offset":9332,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":567,"record_offset":10260,"parent_record_offset":10232,"ancestor_record_offset":9332,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":567,"record_offset":10508,"parent_record_offset":10260,"ancestor_record_offset":9332,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":567,"record_offset":10480,"parent_record_offset":10260,"ancestor_record_offset":9332,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":567,"record_offset":10312,"parent_record_offset":10260,"ancestor_record_offset":9332,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":11112,"parent_record_offset":11080,"ancestor_record_offset":9332,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":567,"record_offset":11212,"parent_record_offset":11176,"ancestor_record_offset":9332,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":11376,"parent_record_offset":11176,"ancestor_record_offset":9332,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":11644,"parent_record_offset":11376,"ancestor_record_offset":9332,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":567,"record_offset":11240,"parent_record_offset":11212,"ancestor_record_offset":9332,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":567,"record_offset":11272,"parent_record_offset":11240,"ancestor_record_offset":9332,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":567,"record_offset":11176,"parent_record_offset":11144,"ancestor_record_offset":9332,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":567,"record_offset":11144,"parent_record_offset":11112,"ancestor_record_offset":9332,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources>
// 记录形式: compiled_procedure; instances=1
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":178,"record_offset":3464,"rva":3009312,"va":5371718432}
// 编译实例 RVAs: 0x2deb20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=3
//     target_aliases=1889 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3000064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e80039d2ff","size":5,"soft_fail":false,"target_va":5368720672,"va":5371720731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14005b630
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2644794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8c6a4d7ff","size":5,"soft_fail":false,"target_va":5369083440,"va":5371728229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400746b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2536999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8d949d9ff","size":5,"soft_fail":false,"target_va":5369185968,"va":5371722962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140075940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2527883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8756dd9ff","size":5,"soft_fail":false,"target_va":5369190720,"va":5371718598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140075e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2532834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e81e5ad9ff","size":5,"soft_fail":false,"target_va":5369191968,"va":5371724797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dce30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2107192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8c8d8dfff","size":5,"soft_fail":false,"target_va":5369613872,"va":5371721059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140269bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-484529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e84f9bf8ff","size":5,"soft_fail":false,"target_va":5371239360,"va":5371723884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14026c110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-472863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8e1c8f8ff","size":5,"soft_fail":false,"target_va":5371248912,"va":5371721770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c9330
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-94273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8bf8ffeff","size":5,"soft_fail":false,"target_va":5371630384,"va":5371724652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360110
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t519782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e866ee0700","size":5,"soft_fail":false,"target_va":5372248336,"va":5371728549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140365550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t540796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e87c400800","size":5,"soft_fail":false,"target_va":5372269904,"va":5371729103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403793e0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t626615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8b78f0900","size":5,"soft_fail":false,"target_va":5372351456,"va":5371724836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140385770
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t675260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8bc4d0a00","size":5,"soft_fail":false,"target_va":5372401520,"va":5371726255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038aa00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t695462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8a69c0a00","size":5,"soft_fail":false,"target_va":5372422656,"va":5371727189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140390c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t721656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8f8020b00","size":5,"soft_fail":false,"target_va":5372447792,"va":5371726131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1030631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8e7b90f00","size":5,"soft_fail":false,"target_va":5372749696,"va":5371719060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1030132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8f4b70f00","size":5,"soft_fail":false,"target_va":5372749712,"va":5371719575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4519172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e804f54400","size":5,"soft_fail":false,"target_va":5376239920,"va":5371720743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407378d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4556327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e827864500","size":5,"soft_fail":false,"target_va":5376276688,"va":5371720356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14073a3e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4568712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e888b64500","size":5,"soft_fail":false,"target_va":5376287712,"va":5371718995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407430f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4604863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8bf434600","size":5,"soft_fail":false,"target_va":5376323824,"va":5371718956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209ccc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31180292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e804c6db01","size":5,"soft_fail":false,"target_va":5402905792,"va":5371725495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a5e00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31221420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8ac66dc01","size":5,"soft_fail":false,"target_va":5402942976,"va":5371721551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31872782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e80e57e601","size":5,"soft_fail":false,"target_va":5403593360,"va":5371720573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31252795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e83be1dc01","size":5,"soft_fail":false,"target_va":5402978144,"va":5371725344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5810423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8f7a85800","size":5,"soft_fail":false,"target_va":5377529888,"va":5371719460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-412077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e853b6f9ff","size":5,"soft_fail":false,"target_va":5371308432,"va":5371720504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31253597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e85de4dc01","size":5,"soft_fail":false,"target_va":5402978688,"va":5371725086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31252946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8d2e1dc01","size":5,"soft_fail":false,"target_va":5402978880,"va":5371725929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31253884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e87ce5dc01","size":5,"soft_fail":false,"target_va":5402978944,"va":5371725055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-6515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e88de6ffff","size":5,"soft_fail":false,"target_va":5371717440,"va":5371723950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t31254937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e899e9dc01","size":5,"soft_fail":false,"target_va":5402980128,"va":5371725186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31254928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e890e9dc01","size":5,"soft_fail":false,"target_va":5402980304,"va":5371725371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31254914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e882e9dc01","size":5,"soft_fail":false,"target_va":5402980320,"va":5371725401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31255332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e824ebdc01","size":5,"soft_fail":false,"target_va":5402980656,"va":5371725319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5634419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e873f95500","size":5,"soft_fail":false,"target_va":5377353776,"va":5371719352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t4636737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e841c04600","size":5,"soft_fail":false,"target_va":5376356912,"va":5371720170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t4640584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e848cf4600","size":5,"soft_fail":false,"target_va":5376360560,"va":5371719971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4636815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e88fc04600","size":5,"soft_fail":false,"target_va":5376357744,"va":5371720924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t4636332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8acbe4600","size":5,"soft_fail":false,"target_va":5376356144,"va":5371719807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::command_buffer::CommandBuffer::begin
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4613211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e85b644600","size":5,"soft_fail":false,"target_va":5376332896,"va":5371719680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::command_buffer::CommandBuffer::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4610417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e871594600","size":5,"soft_fail":false,"target_va":5376333440,"va":5371723018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::get_setup_cb_submit_info
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4596544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e840234600","size":5,"soft_fail":false,"target_va":5376315104,"va":5371718555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4592381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8fd124600","size":5,"soft_fail":false,"target_va":5376316512,"va":5371724126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::submit::SubmitInfo::command_buffer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4494132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e834934400","size":5,"soft_fail":false,"target_va":5376217488,"va":5371723351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::submit::SubmitInfo::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4497746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e852a14400","size":5,"soft_fail":false,"target_va":5376217392,"va":5371719641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::submit::SubmitInfo::signal_semaphore
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4495736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e878994400","size":5,"soft_fail":false,"target_va":5376217920,"va":5371722179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::submit::SubmitInfo::wait_semaphore
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4497108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8d49e4400","size":5,"soft_fail":false,"target_va":5376217728,"va":5371720615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::present
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4554146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8a27d4500","size":5,"soft_fail":false,"target_va":5376278752,"va":5371724601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::color_attachment
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4619125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8757b4600","size":5,"soft_fail":false,"target_va":5376340816,"va":5371721686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4618901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8957a4600","size":5,"soft_fail":false,"target_va":5376340512,"va":5371721606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::storage
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4619101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e85d7b4600","size":5,"soft_fail":false,"target_va":5376340752,"va":5371721646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::executor::GraphExecutor::begin_top_level_timestamp_query
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4551339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8ab724500","size":5,"soft_fail":false,"target_va":5376273600,"va":5371722256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::executor::GraphExecutor::end_top_level_timestamp_query
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4550679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e817704500","size":5,"soft_fail":false,"target_va":5376273680,"va":5371722996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::executor::validate_swapchain_and_acquire_image
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4553623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8977b4500","size":5,"soft_fail":false,"target_va":5376273856,"va":5371720228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_registry::ResourceRegistry::provide_lazy_texture
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4614359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8d7684600","size":5,"soft_fail":false,"target_va":5376334816,"va":5371720452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h3d9350ee38791ca5 (.llvm.8305701337189838128)
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t905155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371718432,"raw_hex":"e8c3cf0d00","size":5,"soft_fail":false,"target_va":5372625168,"va":5371720008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":178,"record_offset":18080,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":10291,"source_line":1685,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":178,"record_offset":5140,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":14840,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":14356,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":3964,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":3772,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":5644,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":178,"record_offset":3860,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":178,"record_offset":5056,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":178,"record_offset":17208,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":178,"record_offset":5084,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":178,"record_offset":4080,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":178,"record_offset":14292,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":178,"record_offset":9820,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":178,"record_offset":14320,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":178,"record_offset":14696,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":178,"record_offset":18044,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":178,"record_offset":5224,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":178,"record_offset":5928,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":178,"record_offset":5112,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":178,"record_offset":4972,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":178,"record_offset":7008,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":178,"record_offset":5992,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":178,"record_offset":14812,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":11108,"source_line":71,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$11::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":178,"record_offset":8084,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":12247,"source_line":23,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::compiled::CompiledRenderGraph::provide_lazy_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/compiled.rs
//     evidence={"module":178,"record_offset":5756,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":7670,"source_line":144,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":178,"record_offset":10904,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":12248,"source_line":2663,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":178,"record_offset":9876,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":178,"record_offset":4448,"parent_record_offset":4424,"ancestor_record_offset":3464,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":178,"record_offset":4472,"parent_record_offset":4448,"ancestor_record_offset":3464,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":178,"record_offset":5400,"parent_record_offset":5376,"ancestor_record_offset":3464,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":178,"record_offset":4424,"parent_record_offset":4328,"ancestor_record_offset":3464,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":178,"record_offset":5376,"parent_record_offset":5276,"ancestor_record_offset":3464,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":178,"record_offset":18296,"parent_record_offset":18080,"ancestor_record_offset":3464,"inlinee":6132,"source_line":171,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":178,"record_offset":18140,"parent_record_offset":18116,"ancestor_record_offset":3464,"inlinee":7711,"source_line":751,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":178,"record_offset":18116,"parent_record_offset":18080,"ancestor_record_offset":3464,"inlinee":7710,"source_line":188,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":178,"record_offset":18320,"parent_record_offset":18296,"ancestor_record_offset":3464,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":178,"record_offset":18200,"parent_record_offset":18080,"ancestor_record_offset":3464,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":178,"record_offset":18224,"parent_record_offset":18200,"ancestor_record_offset":3464,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":178,"record_offset":5188,"parent_record_offset":5164,"ancestor_record_offset":3464,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":178,"record_offset":18164,"parent_record_offset":18140,"ancestor_record_offset":3464,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":178,"record_offset":5308,"parent_record_offset":5276,"ancestor_record_offset":3464,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":178,"record_offset":5860,"parent_record_offset":5836,"ancestor_record_offset":3464,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":178,"record_offset":4328,"parent_record_offset":4296,"ancestor_record_offset":3464,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":178,"record_offset":4296,"parent_record_offset":4264,"ancestor_record_offset":3464,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":178,"record_offset":4544,"parent_record_offset":4296,"ancestor_record_offset":3464,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":178,"record_offset":5276,"parent_record_offset":5248,"ancestor_record_offset":3464,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":5836,"parent_record_offset":5792,"ancestor_record_offset":3464,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":4608,"parent_record_offset":4584,"ancestor_record_offset":3464,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":5792,"parent_record_offset":5756,"ancestor_record_offset":3464,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":4232,"parent_record_offset":4188,"ancestor_record_offset":3464,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":4264,"parent_record_offset":4232,"ancestor_record_offset":3464,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":4116,"parent_record_offset":4080,"ancestor_record_offset":3464,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":4780,"parent_record_offset":4748,"ancestor_record_offset":3464,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":178,"record_offset":17620,"parent_record_offset":17288,"ancestor_record_offset":3464,"inlinee":6040,"source_line":112,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":178,"record_offset":17288,"parent_record_offset":17264,"ancestor_record_offset":3464,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":178,"record_offset":17812,"parent_record_offset":17788,"ancestor_record_offset":3464,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":178,"record_offset":17692,"parent_record_offset":17668,"ancestor_record_offset":3464,"inlinee":6045,"source_line":223,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":178,"record_offset":4868,"parent_record_offset":4780,"ancestor_record_offset":3464,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":178,"record_offset":4584,"parent_record_offset":4188,"ancestor_record_offset":3464,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":178,"record_offset":4152,"parent_record_offset":4116,"ancestor_record_offset":3464,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":178,"record_offset":4188,"parent_record_offset":4152,"ancestor_record_offset":3464,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":178,"record_offset":10556,"parent_record_offset":10532,"ancestor_record_offset":3464,"inlinee":8319,"source_line":649,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":178,"record_offset":4368,"parent_record_offset":4328,"ancestor_record_offset":3464,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":178,"record_offset":4392,"parent_record_offset":4368,"ancestor_record_offset":3464,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":178,"record_offset":7436,"parent_record_offset":7408,"ancestor_record_offset":3464,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":178,"record_offset":7488,"parent_record_offset":7464,"ancestor_record_offset":3464,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":178,"record_offset":7464,"parent_record_offset":7436,"ancestor_record_offset":3464,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":178,"record_offset":7376,"parent_record_offset":7344,"ancestor_record_offset":3464,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":178,"record_offset":7408,"parent_record_offset":7376,"ancestor_record_offset":3464,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":178,"record_offset":7580,"parent_record_offset":7408,"ancestor_record_offset":3464,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":178,"record_offset":7692,"parent_record_offset":7668,"ancestor_record_offset":3464,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":178,"record_offset":11616,"parent_record_offset":11592,"ancestor_record_offset":3464,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":178,"record_offset":10132,"parent_record_offset":10108,"ancestor_record_offset":3464,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":178,"record_offset":10216,"parent_record_offset":10192,"ancestor_record_offset":3464,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":178,"record_offset":10156,"parent_record_offset":10132,"ancestor_record_offset":3464,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":178,"record_offset":4656,"parent_record_offset":4632,"ancestor_record_offset":3464,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":178,"record_offset":4716,"parent_record_offset":4680,"ancestor_record_offset":3464,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":178,"record_offset":17668,"parent_record_offset":17644,"ancestor_record_offset":3464,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":178,"record_offset":17644,"parent_record_offset":17620,"ancestor_record_offset":3464,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":178,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":3464,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":178,"record_offset":4748,"parent_record_offset":4716,"ancestor_record_offset":3464,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":178,"record_offset":3800,"parent_record_offset":3772,"ancestor_record_offset":3464,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":178,"record_offset":7512,"parent_record_offset":7488,"ancestor_record_offset":3464,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":178,"record_offset":5340,"parent_record_offset":5308,"ancestor_record_offset":3464,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":178,"record_offset":18248,"parent_record_offset":18224,"ancestor_record_offset":3464,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":178,"record_offset":11848,"parent_record_offset":11820,"ancestor_record_offset":3464,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":178,"record_offset":6872,"parent_record_offset":6784,"ancestor_record_offset":3464,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":178,"record_offset":10328,"parent_record_offset":10304,"ancestor_record_offset":3464,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":178,"record_offset":7268,"parent_record_offset":7240,"ancestor_record_offset":3464,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":178,"record_offset":9844,"parent_record_offset":9820,"ancestor_record_offset":3464,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":178,"record_offset":6592,"parent_record_offset":6132,"ancestor_record_offset":3464,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":178,"record_offset":5164,"parent_record_offset":5140,"ancestor_record_offset":3464,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":178,"record_offset":7316,"parent_record_offset":7140,"ancestor_record_offset":3464,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":178,"record_offset":6956,"parent_record_offset":6928,"ancestor_record_offset":3464,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":178,"record_offset":17740,"parent_record_offset":17716,"ancestor_record_offset":3464,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=50
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":178,"record_offset":5248,"parent_record_offset":5224,"ancestor_record_offset":3464,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":178,"record_offset":3828,"parent_record_offset":3772,"ancestor_record_offset":3464,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":178,"record_offset":10492,"parent_record_offset":10468,"ancestor_record_offset":3464,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":178,"record_offset":10468,"parent_record_offset":10444,"ancestor_record_offset":3464,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":178,"record_offset":12056,"parent_record_offset":12032,"ancestor_record_offset":3464,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":178,"record_offset":3932,"parent_record_offset":3860,"ancestor_record_offset":3464,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":178,"record_offset":6368,"parent_record_offset":6344,"ancestor_record_offset":3464,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":178,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":3464,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":178,"record_offset":17716,"parent_record_offset":17692,"ancestor_record_offset":3464,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":178,"record_offset":3896,"parent_record_offset":3860,"ancestor_record_offset":3464,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":178,"record_offset":14748,"parent_record_offset":14720,"ancestor_record_offset":3464,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":178,"record_offset":4500,"parent_record_offset":4472,"ancestor_record_offset":3464,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":178,"record_offset":4820,"parent_record_offset":4780,"ancestor_record_offset":3464,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":178,"record_offset":8044,"parent_record_offset":7096,"ancestor_record_offset":3464,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":178,"record_offset":6816,"parent_record_offset":6784,"ancestor_record_offset":3464,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":178,"record_offset":5668,"parent_record_offset":5644,"ancestor_record_offset":3464,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":178,"record_offset":3996,"parent_record_offset":3964,"ancestor_record_offset":3464,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":178,"record_offset":6840,"parent_record_offset":6816,"ancestor_record_offset":3464,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":178,"record_offset":4028,"parent_record_offset":3996,"ancestor_record_offset":3464,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":178,"record_offset":5692,"parent_record_offset":5668,"ancestor_record_offset":3464,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":178,"record_offset":6320,"parent_record_offset":6296,"ancestor_record_offset":3464,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":178,"record_offset":6648,"parent_record_offset":6624,"ancestor_record_offset":3464,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":178,"record_offset":4680,"parent_record_offset":4656,"ancestor_record_offset":3464,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":178,"record_offset":6296,"parent_record_offset":6188,"ancestor_record_offset":3464,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":178,"record_offset":6784,"parent_record_offset":6756,"ancestor_record_offset":3464,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":178,"record_offset":6028,"parent_record_offset":5992,"ancestor_record_offset":3464,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":178,"record_offset":6160,"parent_record_offset":6132,"ancestor_record_offset":3464,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":178,"record_offset":6132,"parent_record_offset":6100,"ancestor_record_offset":3464,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":178,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":3464,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":178,"record_offset":6100,"parent_record_offset":6076,"ancestor_record_offset":3464,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":178,"record_offset":6052,"parent_record_offset":6028,"ancestor_record_offset":3464,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":178,"record_offset":6624,"parent_record_offset":6100,"ancestor_record_offset":3464,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":178,"record_offset":6756,"parent_record_offset":6724,"ancestor_record_offset":3464,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":178,"record_offset":6904,"parent_record_offset":6756,"ancestor_record_offset":3464,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":178,"record_offset":6724,"parent_record_offset":6696,"ancestor_record_offset":3464,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":178,"record_offset":6696,"parent_record_offset":5992,"ancestor_record_offset":3464,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":178,"record_offset":6928,"parent_record_offset":6904,"ancestor_record_offset":3464,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":178,"record_offset":6188,"parent_record_offset":6160,"ancestor_record_offset":3464,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":178,"record_offset":6436,"parent_record_offset":6188,"ancestor_record_offset":3464,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":178,"record_offset":6408,"parent_record_offset":6188,"ancestor_record_offset":3464,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":178,"record_offset":6240,"parent_record_offset":6188,"ancestor_record_offset":3464,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":178,"record_offset":4996,"parent_record_offset":4972,"ancestor_record_offset":3464,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":178,"record_offset":5020,"parent_record_offset":4996,"ancestor_record_offset":3464,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":178,"record_offset":10836,"parent_record_offset":9908,"ancestor_record_offset":3464,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":178,"record_offset":9908,"parent_record_offset":9876,"ancestor_record_offset":3464,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":178,"record_offset":9948,"parent_record_offset":9908,"ancestor_record_offset":3464,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":10444,"parent_record_offset":10420,"ancestor_record_offset":3464,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":10420,"parent_record_offset":10396,"ancestor_record_offset":3464,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":10744,"parent_record_offset":10720,"ancestor_record_offset":3464,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":12032,"parent_record_offset":11088,"ancestor_record_offset":3464,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":10684,"parent_record_offset":10004,"ancestor_record_offset":3464,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":10720,"parent_record_offset":9976,"ancestor_record_offset":3464,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":9976,"parent_record_offset":9908,"ancestor_record_offset":3464,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":11088,"parent_record_offset":11064,"ancestor_record_offset":3464,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":10860,"parent_record_offset":10836,"ancestor_record_offset":3464,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":11892,"parent_record_offset":11088,"ancestor_record_offset":3464,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":10004,"parent_record_offset":9976,"ancestor_record_offset":3464,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":11160,"parent_record_offset":11128,"ancestor_record_offset":3464,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":11340,"parent_record_offset":11160,"ancestor_record_offset":3464,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":11504,"parent_record_offset":11160,"ancestor_record_offset":3464,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":11536,"parent_record_offset":11504,"ancestor_record_offset":3464,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":11128,"parent_record_offset":11088,"ancestor_record_offset":3464,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":10080,"parent_record_offset":10004,"ancestor_record_offset":3464,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":11820,"parent_record_offset":11796,"ancestor_record_offset":3464,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":11796,"parent_record_offset":11128,"ancestor_record_offset":3464,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":178,"record_offset":10276,"parent_record_offset":10248,"ancestor_record_offset":3464,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":178,"record_offset":10304,"parent_record_offset":10276,"ancestor_record_offset":3464,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":178,"record_offset":10364,"parent_record_offset":10248,"ancestor_record_offset":3464,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":178,"record_offset":10248,"parent_record_offset":10004,"ancestor_record_offset":3464,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":10052,"parent_record_offset":10004,"ancestor_record_offset":3464,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":10396,"parent_record_offset":10004,"ancestor_record_offset":3464,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":11560,"parent_record_offset":11536,"ancestor_record_offset":3464,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":178,"record_offset":12004,"parent_record_offset":11088,"ancestor_record_offset":3464,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":178,"record_offset":10108,"parent_record_offset":10004,"ancestor_record_offset":3464,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":178,"record_offset":11592,"parent_record_offset":11504,"ancestor_record_offset":3464,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":178,"record_offset":10192,"parent_record_offset":10004,"ancestor_record_offset":3464,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":178,"record_offset":10592,"parent_record_offset":10004,"ancestor_record_offset":3464,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":178,"record_offset":11380,"parent_record_offset":11340,"ancestor_record_offset":3464,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":178,"record_offset":11064,"parent_record_offset":11040,"ancestor_record_offset":3464,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":178,"record_offset":10532,"parent_record_offset":10396,"ancestor_record_offset":3464,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":178,"record_offset":7052,"parent_record_offset":7008,"ancestor_record_offset":3464,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":178,"record_offset":8588,"parent_record_offset":8552,"ancestor_record_offset":3464,"inlinee":7257,"source_line":601,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":178,"record_offset":9168,"parent_record_offset":8896,"ancestor_record_offset":3464,"inlinee":7265,"source_line":602,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":178,"record_offset":7180,"parent_record_offset":7140,"ancestor_record_offset":3464,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":178,"record_offset":7344,"parent_record_offset":7140,"ancestor_record_offset":3464,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":178,"record_offset":7616,"parent_record_offset":7344,"ancestor_record_offset":3464,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":178,"record_offset":8112,"parent_record_offset":8084,"ancestor_record_offset":3464,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::executor::impl$0::execute::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/executor.rs
//     evidence={"module":178,"record_offset":10948,"parent_record_offset":10904,"ancestor_record_offset":3464,"inlinee":12250,"source_line":221,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":178,"record_offset":11040,"parent_record_offset":10904,"ancestor_record_offset":3464,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":178,"record_offset":7208,"parent_record_offset":7180,"ancestor_record_offset":3464,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":178,"record_offset":7240,"parent_record_offset":7208,"ancestor_record_offset":3464,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":178,"record_offset":7140,"parent_record_offset":7096,"ancestor_record_offset":3464,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":178,"record_offset":7096,"parent_record_offset":7052,"ancestor_record_offset":3464,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::executor::impl$0::execute::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 221
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2FDA
// 首条位置: {"module":178,"inlinee":12250}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":178,"record_offset":10992,"parent_record_offset":10948,"ancestor_record_offset":10948,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::executor::GraphExecutor::last_known_frame_duration_ns
// 记录形式: compiled_procedure; instances=1
// 源码行: 325
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":567,"record_offset":12360,"rva":7564240,"va":5376273360}
// 编译实例 RVAs: 0x736bd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26705568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376273360,"raw_hex":"e8a07e9701","size":5,"soft_fail":false,"target_va":5402979152,"va":5376273579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::Buffer::mapped_slice_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-32865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376273360,"raw_hex":"e89f7fffff","size":5,"soft_fail":false,"target_va":5376240512,"va":5376273372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$9::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":12512,"parent_record_offset":12360,"ancestor_record_offset":12360,"inlinee":30844,"source_line":1680,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::executor::GraphExecutor::begin_top_level_timestamp_query
// 记录形式: compiled_procedure; instances=1
// 源码行: 363
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":567,"record_offset":12544,"rva":7564480,"va":5376273600}
// 编译实例 RVAs: 0x736cc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*1008(%rsi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376273600,"raw_hex":"ff96f0030000","size":6,"soft_fail":false,"va":5376273636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_reset_query_pool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":567,"record_offset":12704,"parent_record_offset":12544,"ancestor_record_offset":12544,"inlinee":30846,"source_line":2105,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::cmd_write_timestamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":567,"record_offset":12732,"parent_record_offset":12544,"ancestor_record_offset":12544,"inlinee":30847,"source_line":2117,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::executor::GraphExecutor::end_top_level_timestamp_query
// 记录形式: compiled_procedure; instances=1
// 源码行: 378
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":567,"record_offset":12764,"rva":7564560,"va":5376273680}
// 编译实例 RVAs: 0x736d10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141177460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10749700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376273680,"raw_hex":"e80407a400","size":5,"soft_fail":false,"target_va":5387023456,"va":5376273751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26705038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376273680,"raw_hex":"e88e7c9701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376273837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*1016(%rsi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376273680,"raw_hex":"ff96f8030000","size":6,"soft_fail":false,"va":5376273728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_copy_query_pool_results
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":567,"record_offset":12988,"parent_record_offset":12764,"ancestor_record_offset":12764,"inlinee":30848,"source_line":1784,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::cmd_write_timestamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":567,"record_offset":12928,"parent_record_offset":12764,"ancestor_record_offset":12764,"inlinee":30847,"source_line":2117,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":567,"record_offset":13016,"parent_record_offset":12764,"ancestor_record_offset":12764,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::get_vk_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":567,"record_offset":12956,"parent_record_offset":12764,"ancestor_record_offset":12764,"inlinee":30771,"source_line":450,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::executor::validate_swapchain_and_acquire_image
// 记录形式: compiled_procedure; instances=1
// 源码行: 403
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":567,"record_offset":13048,"rva":7564736,"va":5376273856}
// 编译实例 RVAs: 0x736dc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3524512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376273856,"raw_hex":"e86038caff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376274219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376273856,"raw_hex":"e8aa78ffff","size":5,"soft_fail":false,"target_va":5376239920,"va":5376274561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140743de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: executor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t52592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376273856,"raw_hex":"e870cd0000","size":5,"soft_fail":false,"target_va":5376327136,"va":5376274539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t26703814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376273856,"raw_hex":"e8c6779701","size":5,"soft_fail":false,"target_va":5402978944,"va":5376275125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1079608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376273856,"raw_hex":"e838791000","size":5,"soft_fail":false,"target_va":5377353776,"va":5376274163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::acquire_next_texture
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376273856,"raw_hex":"e812110000","size":5,"soft_fail":false,"target_va":5376278288,"va":5376273913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":567,"record_offset":13296,"parent_record_offset":13048,"ancestor_record_offset":13048,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":567,"record_offset":13704,"parent_record_offset":13048,"ancestor_record_offset":13048,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":567,"record_offset":13324,"parent_record_offset":13048,"ancestor_record_offset":13048,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":13380,"parent_record_offset":13048,"ancestor_record_offset":13048,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":567,"record_offset":13352,"parent_record_offset":13048,"ancestor_record_offset":13048,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":567,"record_offset":13212,"parent_record_offset":13048,"ancestor_record_offset":13048,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":13544,"parent_record_offset":13520,"ancestor_record_offset":13048,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":13520,"parent_record_offset":13464,"ancestor_record_offset":13048,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":13492,"parent_record_offset":13464,"ancestor_record_offset":13048,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":13464,"parent_record_offset":13436,"ancestor_record_offset":13048,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":13408,"parent_record_offset":13380,"ancestor_record_offset":13048,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":567,"record_offset":13236,"parent_record_offset":13212,"ancestor_record_offset":13048,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":567,"record_offset":13260,"parent_record_offset":13236,"ancestor_record_offset":13048,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::texture::impl$0::drop
// 记录形式: compiled_procedure; instances=1
// 源码行: 469
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":80636,"rva":11552,"va":5368720672}
// 编译实例 RVAs: 0x2d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::executor::impl$1::drop
// 记录形式: compiled_procedure; instances=1
// 源码行: 469
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":80748,"rva":11552,"va":5368720672}
// 编译实例 RVAs: 0x2d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: executor.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::executor::FinishedFrame
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "arrayvec::errors::CapacityError<rhapsody::rg::executor::FinishedFrame>", "constructor": "arrayvec::errors::CapacityError", "arguments": ["rhapsody::rg::executor::FinishedFrame"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<arrayvec::errors::CapacityError<rhapsody::rg::executor::FinishedFrame> >", "constructor": "core::ptr::drop_in_place", "arguments": ["arrayvec::errors::CapacityError<rhapsody::rg::executor::FinishedFrame>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arrayvec::errors::impl$3::fmt<rhapsody::rg::executor::FinishedFrame>", "constructor": "arrayvec::errors::impl$3::fmt", "arguments": ["rhapsody::rg::executor::FinishedFrame"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::executor::FrameInFlight
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque::grow<rhapsody::rg::executor::FrameInFlight,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque::grow", "arguments": ["rhapsody::rg::executor::FrameInFlight", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::executor::GraphExecutor
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 9; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::executor::GraphExecutor::execute", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::executor::GraphExecutor>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::executor::GraphExecutor"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: executor.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: executor.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 115; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >)
// 候选参数类型: ["bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >", "bevy_ecs::system::commands::Commands", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >"]
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>", "tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>,bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>", "bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>", "bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>", "bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "tuple$<bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::impl$0::drop
// 信息 ID: 32deec9f9ac7f952c83b8a4f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 03d77369acb10f9e8a8c49c5: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>)
// [泛型宿主/闭包/直接引用方参数候选] 076821a8e8d882e6d45dbb01: void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowClutterCandidates>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// [泛型宿主/闭包/直接引用方参数候选] 0c4383e19e8e41ddd102d7ce: void (*)(bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// [泛型宿主/闭包/直接引用方参数候选] 0f7c39de2bbbbcade72fdc5b: void (*)(bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>,bevy_ecs::change_detection::Res<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::With<country_core::resources::critters::Critter> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,tuple$<bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 121a48a843a3661860cc0c60: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// [泛型宿主/闭包/直接引用方参数候选] 188ee5116e6a2f635b4a8b5b: void (*)(bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::system::system_param::Local<system_critters::wren::WrenGen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Wren> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::BirdSpots>,tuple$<> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 1ac3b09f94008d55466c958a: void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// [泛型宿主/闭包/直接引用方参数候选] 1bdb6576ac14098c1ed7fdf1: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 其余候选接口见完整附档，共 115 条。
// 引用方: 原程序直接转移 514 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12d50 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12dc0 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5d93c -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64cf3 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64d55 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x6525e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x6535e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x6545e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x6555e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x664ec -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x6655c -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >),void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >)> @ 0x8b872 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)> @ 0x8bb13 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x8bd82 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)> @ 0x8bee1 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x8c110 -> 0x2d20 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::executor::GraphExecutor::begin_top_level_timestamp_query
// 信息 ID: 98a0247d566461d1c816ec96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2dfa10 -> 0x736cc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::executor::GraphExecutor::end_top_level_timestamp_query
// 信息 ID: cf5a8a2087b94606fb0c02ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2dfcf4 -> 0x736d10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources>
// 信息 ID: ead5fdf95affcd32d43191d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::executor::GraphExecutor::execute -> ["country_core::render::pass_builder::pass_resources::PassResources"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x304047 -> 0x2deb20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::executor::GraphExecutor::last_known_frame_duration_ns
// 信息 ID: 3c64617b105ae8e5992e9613
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x304109 -> 0x736bd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::executor::GraphExecutor::new
// 信息 ID: 888302e064f7a871c0964abe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$7,tuple$<> > @ 0x1b4e4 -> 0x736100 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x3cdf11 -> 0x736100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::executor::GraphExecutor::wait_gpu_queue_trim
// 信息 ID: 7532147c416ab6ce154039d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x3044da -> 0x736810 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::executor::GraphExecutor::wait_submits_done
// 信息 ID: 75bf22a9a163a2c4e20cc6a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x303d16 -> 0x7363d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::executor::impl$0::execute::closure$2
// 信息 ID: 42d812da3c62661d3f24da4d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "rhapsody::rg::executor::impl$0::execute", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources>; module=178, inline_record=10948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::executor::impl$1::drop
// 信息 ID: 2c5cf39d7050422247b25c34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 03d77369acb10f9e8a8c49c5: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>)
// [泛型宿主/闭包/直接引用方参数候选] 076821a8e8d882e6d45dbb01: void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowClutterCandidates>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// [泛型宿主/闭包/直接引用方参数候选] 0c4383e19e8e41ddd102d7ce: void (*)(bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// [泛型宿主/闭包/直接引用方参数候选] 0f7c39de2bbbbcade72fdc5b: void (*)(bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>,bevy_ecs::change_detection::Res<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::With<country_core::resources::critters::Critter> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,tuple$<bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 121a48a843a3661860cc0c60: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// [泛型宿主/闭包/直接引用方参数候选] 188ee5116e6a2f635b4a8b5b: void (*)(bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::system::system_param::Local<system_critters::wren::WrenGen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Wren> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::BirdSpots>,tuple$<> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 1ac3b09f94008d55466c958a: void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// [泛型宿主/闭包/直接引用方参数候选] 1bdb6576ac14098c1ed7fdf1: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 其余候选接口见完整附档，共 115 条。
// 引用方: 原程序直接转移 514 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12d50 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12dc0 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5d93c -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64cf3 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64d55 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x6525e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x6535e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x6545e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x6555e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x664ec -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x6655c -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >),void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >)> @ 0x8b872 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)> @ 0x8bb13 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x8bd82 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)> @ 0x8bee1 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x8c110 -> 0x2d20 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::executor::validate_swapchain_and_acquire_image
// 信息 ID: 8657040b2c0bc39b139aabde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2df224 -> 0x736dc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
