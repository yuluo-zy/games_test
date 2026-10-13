// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/submit.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 1390295d97f84c04292a6c06e14615c0bcb5b7f594bc95e8f91d2b60a8782470, 926c68efc576bfc37ddf7663414f52f7e4c6086ef8867257a7152ac23d2e69ed
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: submit.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 21, 73, 139, 178, 193, 551, 556, 570, 571, 1587, 1622, 3013
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use bumpalo; // namespace candidate
//   package identities: [{"name":"bumpalo","source":"PDB_registry_path_only","version":"3.12.0"}]
// use core; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"PDB_registry_path_only","version":"1.19.0"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use spin_sleep; // namespace candidate
//   package identities: [{"name":"spin_sleep","source":"PDB_registry_path_only","version":"1.1.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmissionEngine::update_queue_pending_submissions
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":3008,"rva":7509264,"va":5376218384}
// 编译实例 RVAs: 0x729510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26761303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376218384,"raw_hex":"e857589801","size":5,"soft_fail":false,"target_va":5402980128,"va":5376218820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rbx","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5376218384,"raw_hex":"ffd3","size":2,"soft_fail":false,"va":5376218430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":3780,"parent_record_offset":3008,"ancestor_record_offset":3008,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":3808,"parent_record_offset":3008,"ancestor_record_offset":3008,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::Queue::update_pending_submissions
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":551,"record_offset":3176,"parent_record_offset":3008,"ancestor_record_offset":3008,"inlinee":30491,"source_line":114,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":3336,"parent_record_offset":3176,"ancestor_record_offset":3008,"inlinee":22347,"source_line":1520,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":3368,"parent_record_offset":3336,"ancestor_record_offset":3008,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":3504,"parent_record_offset":3176,"ancestor_record_offset":3008,"inlinee":10716,"source_line":1610,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":3400,"parent_record_offset":3368,"ancestor_record_offset":3008,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":3428,"parent_record_offset":3400,"ancestor_record_offset":3008,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":3456,"parent_record_offset":3428,"ancestor_record_offset":3008,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::get_semaphore_counter_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":551,"record_offset":3220,"parent_record_offset":3176,"ancestor_record_offset":3008,"inlinee":30492,"source_line":679,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":551,"record_offset":3248,"parent_record_offset":3220,"ancestor_record_offset":3008,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":551,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":3008,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":551,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":3008,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":3648,"parent_record_offset":3176,"ancestor_record_offset":3008,"inlinee":8949,"source_line":1221,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$62::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":3704,"parent_record_offset":3676,"ancestor_record_offset":3008,"inlinee":4212,"source_line":1549,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$63::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":3732,"parent_record_offset":3704,"ancestor_record_offset":3008,"inlinee":4214,"source_line":1565,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":551,"record_offset":3848,"parent_record_offset":3808,"ancestor_record_offset":3008,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":551,"record_offset":4524,"parent_record_offset":4500,"ancestor_record_offset":3008,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$17::partial_cmp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":551,"record_offset":3676,"parent_record_offset":3648,"ancestor_record_offset":3008,"inlinee":30494,"source_line":77,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$3::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7717
// 首条位置: {"module":551,"inlinee":30487}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmitInfo::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7720
// 首条位置: {"module":551,"record_offset":248,"rva":7508272,"va":5376217392}
// 编译实例 RVAs: 0x729130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::submit::impl$3::default
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":551,"record_offset":372,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":30487,"source_line":10,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmitInfo::command_buffer
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7721
// 首条位置: {"module":551,"record_offset":400,"rva":7508368,"va":5376217488}
// 编译实例 RVAs: 0x729190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072b3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376217488,"raw_hex":"e868210000","size":5,"soft_fail":false,"target_va":5376226240,"va":5376217683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072c6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376217488,"raw_hex":"e878340000","size":5,"soft_fail":false,"target_va":5376231152,"va":5376217715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27375710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376217488,"raw_hex":"e85eb8a101","size":5,"soft_fail":false,"target_va":5403593360,"va":5376217645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4909495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376217488,"raw_hex":"e84916b5ff","size":5,"soft_fail":false,"target_va":5371308048,"va":5376217538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":536,"parent_record_offset":400,"ancestor_record_offset":400,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":612,"parent_record_offset":580,"ancestor_record_offset":400,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":580,"parent_record_offset":536,"ancestor_record_offset":400,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":49012,"parent_record_offset":48972,"ancestor_record_offset":48932,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":644,"parent_record_offset":536,"ancestor_record_offset":400,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmitInfo::wait_semaphore
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":680,"rva":7508608,"va":5376217728}
// 编译实例 RVAs: 0x729280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072c6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376217728,"raw_hex":"e8c6330000","size":5,"soft_fail":false,"target_va":5376231152,"va":5376217893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27375503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376217728,"raw_hex":"e88fb7a101","size":5,"soft_fail":false,"target_va":5403593360,"va":5376217852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4912726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376217728,"raw_hex":"e8aa09b5ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5376217777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":816,"parent_record_offset":680,"ancestor_record_offset":680,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":872,"parent_record_offset":848,"ancestor_record_offset":680,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":848,"parent_record_offset":816,"ancestor_record_offset":680,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":904,"parent_record_offset":816,"ancestor_record_offset":680,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmitInfo::signal_semaphore
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":1064,"rva":7508800,"va":5376217920}
// 编译实例 RVAs: 0x729340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072c6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376217920,"raw_hex":"e807330000","size":5,"soft_fail":false,"target_va":5376231152,"va":5376218084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27375308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376217920,"raw_hex":"e8ccb6a101","size":5,"soft_fail":false,"target_va":5403593360,"va":5376218047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4912916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376217920,"raw_hex":"e8ec08b5ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5376217967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":1200,"parent_record_offset":1064,"ancestor_record_offset":1064,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":1260,"parent_record_offset":1236,"ancestor_record_offset":1064,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":1236,"parent_record_offset":1200,"ancestor_record_offset":1064,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":1292,"parent_record_offset":1200,"ancestor_record_offset":1064,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::carve_off_front
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1814
// 首条位置: {"module":21,"inlinee":6164}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":21,"record_offset":24956,"parent_record_offset":24920,"ancestor_record_offset":24920,"inlinee":6165,"source_line":1886,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":21,"record_offset":25092,"parent_record_offset":24956,"ancestor_record_offset":24920,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":21,"record_offset":25056,"parent_record_offset":25032,"ancestor_record_offset":24920,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":21,"record_offset":25004,"parent_record_offset":24956,"ancestor_record_offset":24920,"inlinee":6166,"source_line":2086,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":21,"record_offset":25032,"parent_record_offset":25004,"ancestor_record_offset":24920,"inlinee":6167,"source_line":1973,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$7::eq
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B64
// 首条位置: {"module":139,"inlinee":11108}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$9::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B87
// 首条位置: {"module":139,"record_offset":3408,"rva":2528512,"va":5371237632}
// 编译实例 RVAs: 0x269500, 0x7459e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6348197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371237632,"raw_hex":"e8a5dd6000","size":5,"soft_fail":false,"target_va":5377585888,"va":5371237686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t6346198","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5371237632,"raw_hex":"e9d6d56000","size":5,"soft_fail":false,"target_va":5377583920,"va":5371237717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$17::partial_cmp
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x771E
// 首条位置: {"module":551,"inlinee":30494}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$62::partial_cmp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":3704,"parent_record_offset":3676,"ancestor_record_offset":3676,"inlinee":4212,"source_line":1549,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$63::cmp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":3732,"parent_record_offset":3704,"ancestor_record_offset":3676,"inlinee":4214,"source_line":1565,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$18::cmp
// 记录形式: inlinee_declaration; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x110DC
// 首条位置: {"module":1587,"inlinee":69852}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmissionEngine::update_pending_submission_timestamp
// 记录形式: compiled_procedure; instances=1
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":1452,"rva":7508992,"va":5376218112}
// 编译实例 RVAs: 0x729400
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":551,"record_offset":1696,"parent_record_offset":1452,"ancestor_record_offset":1452,"inlinee":30488,"source_line":3191,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":551,"record_offset":1616,"parent_record_offset":1452,"ancestor_record_offset":1452,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":1960,"parent_record_offset":1940,"ancestor_record_offset":1452,"inlinee":22347,"source_line":1520,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":1452,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":2796,"parent_record_offset":2764,"ancestor_record_offset":1452,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":2008,"parent_record_offset":1984,"ancestor_record_offset":1452,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":1452,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":2056,"parent_record_offset":2032,"ancestor_record_offset":1452,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":1452,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":551,"record_offset":2532,"parent_record_offset":2508,"ancestor_record_offset":1452,"inlinee":6075,"source_line":85,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":551,"record_offset":1820,"parent_record_offset":1796,"ancestor_record_offset":1452,"inlinee":27434,"source_line":68,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":551,"record_offset":2692,"parent_record_offset":2560,"ancestor_record_offset":1452,"inlinee":25841,"source_line":39,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":551,"record_offset":2508,"parent_record_offset":1744,"ancestor_record_offset":1452,"inlinee":25235,"source_line":144,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":551,"record_offset":1772,"parent_record_offset":1744,"ancestor_record_offset":1452,"inlinee":8593,"source_line":63,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":551,"record_offset":1796,"parent_record_offset":1772,"ancestor_record_offset":1452,"inlinee":8594,"source_line":2898,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":551,"record_offset":1896,"parent_record_offset":1872,"ancestor_record_offset":1452,"inlinee":8630,"source_line":2905,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":551,"record_offset":1720,"parent_record_offset":1696,"ancestor_record_offset":1452,"inlinee":8709,"source_line":3310,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":551,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":1452,"inlinee":8710,"source_line":2618,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":551,"record_offset":1848,"parent_record_offset":1820,"ancestor_record_offset":1452,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":551,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":1452,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":551,"record_offset":2944,"parent_record_offset":2716,"ancestor_record_offset":1452,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":551,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":1452,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":551,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":1452,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":551,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":1452,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":551,"record_offset":2636,"parent_record_offset":2560,"ancestor_record_offset":1452,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":551,"record_offset":2184,"parent_record_offset":2148,"ancestor_record_offset":1452,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":551,"record_offset":1636,"parent_record_offset":1616,"ancestor_record_offset":1452,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":2212,"parent_record_offset":2148,"ancestor_record_offset":1452,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":2560,"parent_record_offset":2532,"ancestor_record_offset":1452,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":2148,"parent_record_offset":2120,"ancestor_record_offset":1452,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$1::update_pending_submission_timestamp::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":551,"record_offset":1940,"parent_record_offset":1916,"ancestor_record_offset":1452,"inlinee":30490,"source_line":105,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$1::update_pending_submission_timestamp::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x771A
// 首条位置: {"module":551,"inlinee":30490}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::front
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":1960,"parent_record_offset":1940,"ancestor_record_offset":1940,"inlinee":22347,"source_line":1520,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":551,"record_offset":2944,"parent_record_offset":2716,"ancestor_record_offset":2716,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":1940,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":2796,"parent_record_offset":2764,"ancestor_record_offset":2716,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":2008,"parent_record_offset":1984,"ancestor_record_offset":1940,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":1940,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":2056,"parent_record_offset":2032,"ancestor_record_offset":1940,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":2716,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmissionEngine::reclaim_command_buffer_resources_all_queues
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7729
// 首条位置: {"module":556,"inlinee":30505}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":79152,"parent_record_offset":79120,"ancestor_record_offset":79120,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":79180,"parent_record_offset":79120,"ancestor_record_offset":79120,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":556,"record_offset":79280,"parent_record_offset":79256,"ancestor_record_offset":79120,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":556,"record_offset":79224,"parent_record_offset":79180,"ancestor_record_offset":79120,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":79256,"parent_record_offset":79180,"ancestor_record_offset":79120,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmissionEngine::queue
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x772A
// 首条位置: {"module":570,"inlinee":30506}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":71448,"parent_record_offset":71408,"ancestor_record_offset":71408,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":71476,"parent_record_offset":71448,"ancestor_record_offset":71408,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":71504,"parent_record_offset":71476,"ancestor_record_offset":71408,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmissionEngine::queue_mut
// 记录形式: compiled_procedure; instances=1
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":4560,"rva":7509712,"va":5376218832}
// 编译实例 RVAs: 0x7296d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26760437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376218832,"raw_hex":"e8f5549801","size":5,"soft_fail":false,"target_va":5402979344,"va":5376218902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":4696,"parent_record_offset":4560,"ancestor_record_offset":4560,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":551,"record_offset":4724,"parent_record_offset":4696,"ancestor_record_offset":4560,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":551,"record_offset":4752,"parent_record_offset":4724,"ancestor_record_offset":4560,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::queue_family_index
// 记录形式: compiled_procedure; instances=1
// 源码行: 148
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":68156,"rva":7614704,"va":5376323824}
// 编译实例 RVAs: 0x7430f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26655004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323824,"raw_hex":"e81cb99601","size":5,"soft_fail":false,"target_va":5402978880,"va":5376323871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":68304,"parent_record_offset":68156,"ancestor_record_offset":68156,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":68328,"parent_record_offset":68156,"ancestor_record_offset":68156,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":139,"record_offset":5412,"rva":2530240,"va":5371239360}
// 编译实例 RVAs: 0x269bc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..collections..vec_deque..VecDeque$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$T$GT$$GT$::extend
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t388889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e819ef0500","size":5,"soft_fail":false,"target_va":5371628928,"va":5371240034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$bumpalo..Bump$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5660745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e849605600","size":5,"soft_fail":false,"target_va":5376902512,"va":5371241762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14005b630
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2160152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8e809dfff","size":5,"soft_fail":false,"target_va":5369083440,"va":5371243587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e5db0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1590468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e83cbbe7ff","size":5,"soft_fail":false,"target_va":5369650608,"va":5371241071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e65d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1587225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8e7c7e7ff","size":5,"soft_fail":false,"target_va":5369652688,"va":5371239908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e68c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1587488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8e0c6e7ff","size":5,"soft_fail":false,"target_va":5369653440,"va":5371240923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e6d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1586465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8dfcae7ff","size":5,"soft_fail":false,"target_va":5369654544,"va":5371241004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1580965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e85be0e7ff","size":5,"soft_fail":false,"target_va":5369660320,"va":5371241280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c9630
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t390368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8e0f40500","size":5,"soft_fail":false,"target_va":5371631152,"va":5371240779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035df30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t998845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8bd3d0f00","size":5,"soft_fail":false,"target_va":5372239664,"va":5371240814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360110
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1004402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e872530f00","size":5,"soft_fail":false,"target_va":5372248336,"va":5371243929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403793e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1109511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e807ee1000","size":5,"soft_fail":false,"target_va":5372351456,"va":5371241940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038aa00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1180017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e871011200","size":5,"soft_fail":false,"target_va":5372422656,"va":5371242634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1510057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8a90a1700","size":5,"soft_fail":false,"target_va":5372749696,"va":5371239634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1507964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e87c021700","size":5,"soft_fail":false,"target_va":5372749712,"va":5371241743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1510170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e81a0b1700","size":5,"soft_fail":false,"target_va":5372749728,"va":5371239553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140733910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5018805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8b5944c00","size":5,"soft_fail":false,"target_va":5376260368,"va":5371241558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14073b120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5049522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8b20c4d00","size":5,"soft_fail":false,"target_va":5376291104,"va":5371241577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d0550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5661360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8b0625600","size":5,"soft_fail":false,"target_va":5376902480,"va":5371241115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209ccc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31663636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e81426e301","size":5,"soft_fail":false,"target_va":5402905792,"va":5371242151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209cf40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31664078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8ce27e301","size":5,"soft_fail":false,"target_va":5402906432,"va":5371242349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31738565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8c54ae401","size":5,"soft_fail":false,"target_va":5402978144,"va":5371239574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t64446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8befb0000","size":5,"soft_fail":false,"target_va":5371305056,"va":5371240605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31736509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8bd42e401","size":5,"soft_fail":false,"target_va":5402978880,"va":5371242366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31737059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8e344e401","size":5,"soft_fail":false,"target_va":5402979152,"va":5371242088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31736671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e85f43e401","size":5,"soft_fail":false,"target_va":5402978944,"va":5371242268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<alloc::boxed::Box<slice2$<rhapsody::raw::submit::SubmitInfo>,alloc::alloc::Global> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e881eaffff","size":5,"soft_fail":false,"target_va":5371237216,"va":5371242714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31737635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e82347e401","size":5,"soft_fail":false,"target_va":5402980128,"va":5371242488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t31729532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e87c27e401","size":5,"soft_fail":false,"target_va":5402968992,"va":5371239455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t31729312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8a026e401","size":5,"soft_fail":false,"target_va":5402970800,"va":5371241483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31725383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e84717e401","size":5,"soft_fail":false,"target_va":5402966736,"va":5371241348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t31726809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8d91ce401","size":5,"soft_fail":false,"target_va":5402968352,"va":5371241538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::Queue::reclaim_command_buffer_resources
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5050067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8d30e4d00","size":5,"soft_fail":false,"target_va":5376291744,"va":5371241672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4974206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e87ee64b00","size":5,"soft_fail":false,"target_va":5376215872,"va":5371241661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::submit::SubmissionEngine::queue_mut
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4978814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e87ef84b00","size":5,"soft_fail":false,"target_va":5376218832,"va":5371240013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::submit::SubmissionEngine::update_pending_submission_timestamp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4978058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e88af54b00","size":5,"soft_fail":false,"target_va":5376218112,"va":5371240049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::submit::SubmissionEngine::update_queue_pending_submissions
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4978891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371239360,"raw_hex":"e8cbf84b00","size":5,"soft_fail":false,"target_va":5376218384,"va":5371239488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5371239360,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5371241438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":139,"record_offset":7820,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":10291,"source_line":1685,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":139,"record_offset":8276,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":139,"record_offset":7756,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":139,"record_offset":8152,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":139,"record_offset":5928,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":139,"record_offset":12244,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":139,"record_offset":6520,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":139,"record_offset":8768,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":139,"record_offset":7628,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$9::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":139,"record_offset":10488,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":11118,"source_line":604,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":139,"record_offset":10552,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":139,"record_offset":10588,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":139,"record_offset":10448,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":139,"record_offset":10360,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":139,"record_offset":6488,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":139,"record_offset":7488,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":139,"record_offset":5700,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":139,"record_offset":5872,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::Queue::with_raw
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":139,"record_offset":8992,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":11110,"source_line":68,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":139,"record_offset":8248,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":11108,"source_line":71,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":139,"record_offset":6864,"parent_record_offset":6840,"ancestor_record_offset":5412,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":139,"record_offset":6888,"parent_record_offset":6864,"ancestor_record_offset":5412,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":139,"record_offset":6404,"parent_record_offset":6380,"ancestor_record_offset":5412,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":139,"record_offset":6840,"parent_record_offset":6752,"ancestor_record_offset":5412,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":139,"record_offset":6380,"parent_record_offset":6160,"ancestor_record_offset":5412,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":139,"record_offset":6296,"parent_record_offset":6160,"ancestor_record_offset":5412,"inlinee":8396,"source_line":283,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":139,"record_offset":6320,"parent_record_offset":6296,"ancestor_record_offset":5412,"inlinee":8397,"source_line":137,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":139,"record_offset":11776,"parent_record_offset":11716,"ancestor_record_offset":5412,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":139,"record_offset":7964,"parent_record_offset":7820,"ancestor_record_offset":5412,"inlinee":6132,"source_line":171,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":139,"record_offset":8088,"parent_record_offset":8064,"ancestor_record_offset":5412,"inlinee":7711,"source_line":751,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":139,"record_offset":8064,"parent_record_offset":7820,"ancestor_record_offset":5412,"inlinee":7710,"source_line":188,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":139,"record_offset":7988,"parent_record_offset":7964,"ancestor_record_offset":5412,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":139,"record_offset":7868,"parent_record_offset":7820,"ancestor_record_offset":5412,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":139,"record_offset":7892,"parent_record_offset":7868,"ancestor_record_offset":5412,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":139,"record_offset":6100,"parent_record_offset":6076,"ancestor_record_offset":5412,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":139,"record_offset":6204,"parent_record_offset":6160,"ancestor_record_offset":5412,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":139,"record_offset":8028,"parent_record_offset":7988,"ancestor_record_offset":5412,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":139,"record_offset":6160,"parent_record_offset":6132,"ancestor_record_offset":5412,"inlinee":8395,"source_line":486,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":139,"record_offset":6132,"parent_record_offset":6044,"ancestor_record_offset":5412,"inlinee":8394,"source_line":406,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":139,"record_offset":6752,"parent_record_offset":6724,"ancestor_record_offset":5412,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":139,"record_offset":6724,"parent_record_offset":6696,"ancestor_record_offset":5412,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":139,"record_offset":6440,"parent_record_offset":6132,"ancestor_record_offset":5412,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":139,"record_offset":10640,"parent_record_offset":10612,"ancestor_record_offset":5412,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":139,"record_offset":8308,"parent_record_offset":8276,"ancestor_record_offset":5412,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":139,"record_offset":6076,"parent_record_offset":6044,"ancestor_record_offset":5412,"inlinee":6979,"source_line":921,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":139,"record_offset":7028,"parent_record_offset":7000,"ancestor_record_offset":5412,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":139,"record_offset":6000,"parent_record_offset":5964,"ancestor_record_offset":5412,"inlinee":8392,"source_line":1139,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":139,"record_offset":6044,"parent_record_offset":6000,"ancestor_record_offset":5412,"inlinee":8393,"source_line":1077,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":139,"record_offset":6668,"parent_record_offset":6628,"ancestor_record_offset":5412,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":139,"record_offset":6696,"parent_record_offset":6668,"ancestor_record_offset":5412,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":139,"record_offset":6556,"parent_record_offset":6520,"ancestor_record_offset":5412,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":139,"record_offset":8792,"parent_record_offset":8768,"ancestor_record_offset":5412,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":139,"record_offset":7236,"parent_record_offset":7196,"ancestor_record_offset":5412,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$29::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":139,"record_offset":9528,"parent_record_offset":9496,"ancestor_record_offset":5412,"inlinee":5330,"source_line":3244,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":139,"record_offset":5964,"parent_record_offset":5928,"ancestor_record_offset":5412,"inlinee":8407,"source_line":3414,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":139,"record_offset":7328,"parent_record_offset":7236,"ancestor_record_offset":5412,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":139,"record_offset":7000,"parent_record_offset":6628,"ancestor_record_offset":5412,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":139,"record_offset":6592,"parent_record_offset":6556,"ancestor_record_offset":5412,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":139,"record_offset":6628,"parent_record_offset":6592,"ancestor_record_offset":5412,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::queue_submit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":139,"record_offset":9556,"parent_record_offset":9496,"ancestor_record_offset":5412,"inlinee":11113,"source_line":2464,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":139,"record_offset":9588,"parent_record_offset":9556,"ancestor_record_offset":5412,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":139,"record_offset":9612,"parent_record_offset":9588,"ancestor_record_offset":5412,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":139,"record_offset":6784,"parent_record_offset":6752,"ancestor_record_offset":5412,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":139,"record_offset":11804,"parent_record_offset":11776,"ancestor_record_offset":5412,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":139,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":5412,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":139,"record_offset":5896,"parent_record_offset":5872,"ancestor_record_offset":5412,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":139,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":5412,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":139,"record_offset":7084,"parent_record_offset":7056,"ancestor_record_offset":5412,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":139,"record_offset":7156,"parent_record_offset":7112,"ancestor_record_offset":5412,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":139,"record_offset":8848,"parent_record_offset":8768,"ancestor_record_offset":5412,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":139,"record_offset":8872,"parent_record_offset":8848,"ancestor_record_offset":5412,"inlinee":4243,"source_line":154,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":139,"record_offset":7452,"parent_record_offset":7428,"ancestor_record_offset":5412,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":139,"record_offset":7056,"parent_record_offset":7028,"ancestor_record_offset":5412,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":139,"record_offset":7196,"parent_record_offset":7156,"ancestor_record_offset":5412,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":139,"record_offset":9652,"parent_record_offset":8992,"ancestor_record_offset":5412,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":139,"record_offset":7656,"parent_record_offset":7628,"ancestor_record_offset":5412,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":139,"record_offset":11828,"parent_record_offset":11804,"ancestor_record_offset":5412,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":139,"record_offset":9188,"parent_record_offset":9096,"ancestor_record_offset":5412,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":139,"record_offset":9220,"parent_record_offset":9188,"ancestor_record_offset":5412,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":139,"record_offset":6228,"parent_record_offset":6204,"ancestor_record_offset":5412,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":139,"record_offset":7916,"parent_record_offset":7892,"ancestor_record_offset":5412,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":139,"record_offset":10516,"parent_record_offset":10488,"ancestor_record_offset":5412,"inlinee":11119,"source_line":2074,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":139,"record_offset":9676,"parent_record_offset":9652,"ancestor_record_offset":5412,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":139,"record_offset":7784,"parent_record_offset":7756,"ancestor_record_offset":5412,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":139,"record_offset":7592,"parent_record_offset":7568,"ancestor_record_offset":5412,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":139,"record_offset":7536,"parent_record_offset":7488,"ancestor_record_offset":5412,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":139,"record_offset":7688,"parent_record_offset":7656,"ancestor_record_offset":5412,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":139,"record_offset":6916,"parent_record_offset":6888,"ancestor_record_offset":5412,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":139,"record_offset":7284,"parent_record_offset":7236,"ancestor_record_offset":5412,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":139,"record_offset":5808,"parent_record_offset":5724,"ancestor_record_offset":5412,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":139,"record_offset":5832,"parent_record_offset":5808,"ancestor_record_offset":5412,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":139,"record_offset":6352,"parent_record_offset":6160,"ancestor_record_offset":5412,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":139,"record_offset":7568,"parent_record_offset":7488,"ancestor_record_offset":5412,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":139,"record_offset":7112,"parent_record_offset":7084,"ancestor_record_offset":5412,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":9756,"parent_record_offset":9724,"ancestor_record_offset":5412,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":5752,"parent_record_offset":5724,"ancestor_record_offset":5412,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":9256,"parent_record_offset":9096,"ancestor_record_offset":5412,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":9960,"parent_record_offset":9928,"ancestor_record_offset":5412,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":9132,"parent_record_offset":9096,"ancestor_record_offset":5412,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":9780,"parent_record_offset":9756,"ancestor_record_offset":5412,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":5776,"parent_record_offset":5752,"ancestor_record_offset":5412,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":9156,"parent_record_offset":9132,"ancestor_record_offset":5412,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":9984,"parent_record_offset":9960,"ancestor_record_offset":5412,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":139,"record_offset":9324,"parent_record_offset":8992,"ancestor_record_offset":5412,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":139,"record_offset":9700,"parent_record_offset":9676,"ancestor_record_offset":5412,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":139,"record_offset":9044,"parent_record_offset":8992,"ancestor_record_offset":5412,"inlinee":5719,"source_line":447,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":139,"record_offset":9904,"parent_record_offset":9880,"ancestor_record_offset":5412,"inlinee":9020,"source_line":1332,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":139,"record_offset":5724,"parent_record_offset":5700,"ancestor_record_offset":5412,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":139,"record_offset":9724,"parent_record_offset":9700,"ancestor_record_offset":5412,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":139,"record_offset":9096,"parent_record_offset":9068,"ancestor_record_offset":5412,"inlinee":5723,"source_line":508,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":139,"record_offset":9068,"parent_record_offset":9044,"ancestor_record_offset":5412,"inlinee":5721,"source_line":107,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":139,"record_offset":9928,"parent_record_offset":9904,"ancestor_record_offset":5412,"inlinee":5735,"source_line":129,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$2::queue_submit::closure$6
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":139,"record_offset":9496,"parent_record_offset":8992,"ancestor_record_offset":5412,"inlinee":11111,"source_line":303,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$2::queue_submit::closure$7
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":139,"record_offset":10388,"parent_record_offset":10360,"ancestor_record_offset":5412,"inlinee":11117,"source_line":307,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$2::queue_submit::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 178
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x21A9
// 首条位置: {"module":73,"inlinee":8617}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":73,"record_offset":94132,"parent_record_offset":94112,"ancestor_record_offset":94112,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":73,"record_offset":94180,"parent_record_offset":94156,"ancestor_record_offset":94112,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":73,"record_offset":94156,"parent_record_offset":94132,"ancestor_record_offset":94112,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$2::queue_submit::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 246
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x21AA
// 首条位置: {"module":73,"inlinee":8618}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":73,"record_offset":98708,"parent_record_offset":98688,"ancestor_record_offset":98688,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":73,"record_offset":98756,"parent_record_offset":98732,"ancestor_record_offset":98688,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":73,"record_offset":98732,"parent_record_offset":98708,"ancestor_record_offset":98688,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$2::queue_submit::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 252
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x21AB
// 首条位置: {"module":73,"inlinee":8619}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":73,"record_offset":104984,"parent_record_offset":104964,"ancestor_record_offset":104964,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":73,"record_offset":105032,"parent_record_offset":105008,"ancestor_record_offset":104964,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":73,"record_offset":105008,"parent_record_offset":104984,"ancestor_record_offset":104964,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$2::queue_submit::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x21A5
// 首条位置: {"module":73,"inlinee":8613}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":73,"record_offset":84468,"parent_record_offset":84448,"ancestor_record_offset":84448,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":73,"record_offset":84516,"parent_record_offset":84492,"ancestor_record_offset":84448,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":73,"record_offset":84492,"parent_record_offset":84468,"ancestor_record_offset":84448,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$2::queue_submit::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1807
// 首条位置: {"module":21,"inlinee":6151}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::definitions::SubmitInfo::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":21,"record_offset":25752,"parent_record_offset":24240,"ancestor_record_offset":24240,"inlinee":6169,"source_line":8028,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":21,"record_offset":24328,"parent_record_offset":24240,"ancestor_record_offset":24240,"inlinee":6153,"source_line":805,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::carve_off_front
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":21,"record_offset":24920,"parent_record_offset":24240,"ancestor_record_offset":24240,"inlinee":6164,"source_line":65,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":21,"record_offset":25900,"parent_record_offset":25876,"ancestor_record_offset":24240,"inlinee":6177,"source_line":39,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":21,"record_offset":24392,"parent_record_offset":24356,"ancestor_record_offset":24240,"inlinee":6155,"source_line":1396,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_with
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":21,"record_offset":24356,"parent_record_offset":24328,"ancestor_record_offset":24240,"inlinee":6154,"source_line":850,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":21,"record_offset":24420,"parent_record_offset":24392,"ancestor_record_offset":24240,"inlinee":6156,"source_line":1411,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":21,"record_offset":24456,"parent_record_offset":24420,"ancestor_record_offset":24240,"inlinee":6157,"source_line":1420,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$8::alloc_layout::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":21,"record_offset":24820,"parent_record_offset":24788,"ancestor_record_offset":24240,"inlinee":6179,"source_line":1397,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$8::alloc_with::inner_writer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":21,"record_offset":24856,"parent_record_offset":24356,"ancestor_record_offset":24240,"inlinee":6162,"source_line":855,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":21,"record_offset":24500,"parent_record_offset":24456,"ancestor_record_offset":24240,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":21,"record_offset":24692,"parent_record_offset":24668,"ancestor_record_offset":24240,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":21,"record_offset":24668,"parent_record_offset":24456,"ancestor_record_offset":24240,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":21,"record_offset":25092,"parent_record_offset":24956,"ancestor_record_offset":24240,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\mod.rs
//     evidence={"module":21,"record_offset":25804,"parent_record_offset":25780,"ancestor_record_offset":24240,"inlinee":6172,"source_line":372,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":21,"record_offset":25828,"parent_record_offset":25804,"ancestor_record_offset":24240,"inlinee":6173,"source_line":56,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":21,"record_offset":25876,"parent_record_offset":25852,"ancestor_record_offset":24240,"inlinee":6175,"source_line":67,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":21,"record_offset":25780,"parent_record_offset":25752,"ancestor_record_offset":24240,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":21,"record_offset":25852,"parent_record_offset":25828,"ancestor_record_offset":24240,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":21,"record_offset":24716,"parent_record_offset":24692,"ancestor_record_offset":24240,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":21,"record_offset":25056,"parent_record_offset":25032,"ancestor_record_offset":24240,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":21,"record_offset":24580,"parent_record_offset":24556,"ancestor_record_offset":24240,"inlinee":6159,"source_line":563,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":21,"record_offset":24556,"parent_record_offset":24456,"ancestor_record_offset":24240,"inlinee":6158,"source_line":1299,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":24740,"parent_record_offset":24716,"ancestor_record_offset":24240,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":21,"record_offset":24788,"parent_record_offset":24392,"ancestor_record_offset":24240,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":21,"record_offset":24956,"parent_record_offset":24920,"ancestor_record_offset":24240,"inlinee":6165,"source_line":1886,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":21,"record_offset":25004,"parent_record_offset":24956,"ancestor_record_offset":24240,"inlinee":6166,"source_line":2086,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":21,"record_offset":25032,"parent_record_offset":25004,"ancestor_record_offset":24240,"inlinee":6167,"source_line":1973,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$2::queue_submit::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 303
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B67
// 首条位置: {"module":139,"inlinee":11111}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$29::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":139,"record_offset":9528,"parent_record_offset":9496,"ancestor_record_offset":9496,"inlinee":5330,"source_line":3244,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::queue_submit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":139,"record_offset":9556,"parent_record_offset":9496,"ancestor_record_offset":9496,"inlinee":11113,"source_line":2464,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":139,"record_offset":9588,"parent_record_offset":9556,"ancestor_record_offset":9496,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":139,"record_offset":9612,"parent_record_offset":9588,"ancestor_record_offset":9496,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$2::queue_submit::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 307
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B6D
// 首条位置: {"module":139,"inlinee":11117}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":139,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":10388,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::wait_submit_done
// 记录形式: compiled_procedure; instances=1
// 源码行: 324
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":68364,"rva":7614768,"va":5376323888}
// 编译实例 RVAs: 0x743130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14005b630
//     owner=unresolved_namespace; count=2
//     target_aliases=11 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7241731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323888,"raw_hex":"e8fd7f91ff","size":5,"soft_fail":false,"target_va":5369083440,"va":5376325166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14073aa70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323888,"raw_hex":"e8cc74ffff","size":5,"soft_fail":false,"target_va":5376289392,"va":5376325023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] backtrace::capture::Backtrace::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t43489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323888,"raw_hex":"e8e1a90000","size":5,"soft_fail":false,"target_va":5376368480,"va":5376324986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26653555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323888,"raw_hex":"e873b39601","size":5,"soft_fail":false,"target_va":5402978688,"va":5376325128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26654367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323888,"raw_hex":"e89fb69601","size":5,"soft_fail":false,"target_va":5402979344,"va":5376324972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-45714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323888,"raw_hex":"e86e4dffff","size":5,"soft_fail":false,"target_va":5376279232,"va":5376324941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26655749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323888,"raw_hex":"e805bc9601","size":5,"soft_fail":false,"target_va":5402980128,"va":5376324374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-44540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323888,"raw_hex":"e80452ffff","size":5,"soft_fail":false,"target_va":5376279424,"va":5376323959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26656622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323888,"raw_hex":"e86ebf9601","size":5,"soft_fail":false,"target_va":5402980656,"va":5376324029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t26644382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323888,"raw_hex":"e89e8f9601","size":5,"soft_fail":false,"target_va":5402968992,"va":5376324605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t26646017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323888,"raw_hex":"e801969601","size":5,"soft_fail":false,"target_va":5402970800,"va":5376324778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t32366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323888,"raw_hex":"e86e7e0000","size":5,"soft_fail":false,"target_va":5376356912,"va":5376324541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t36250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323888,"raw_hex":"e89a8d0000","size":5,"soft_fail":false,"target_va":5376360560,"va":5376324305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t32209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323888,"raw_hex":"e8d17d0000","size":5,"soft_fail":false,"target_va":5376356144,"va":5376323930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::Queue::reclaim_command_buffer_resources
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-33093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323888,"raw_hex":"e8bb7effff","size":5,"soft_fail":false,"target_va":5376291744,"va":5376324832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-108926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323888,"raw_hex":"e88256feff","size":5,"soft_fail":false,"target_va":5376215872,"va":5376324793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hf5f0f90a77fe959e (.llvm.16944548595553555407)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10725110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376323888,"raw_hex":"e8f6a6a300","size":5,"soft_fail":false,"target_va":5387049440,"va":5376324325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*1264(%rsi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376323888,"raw_hex":"ff96f0040000","size":6,"soft_fail":false,"va":5376324740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::wait_semaphores
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":71604,"parent_record_offset":68364,"ancestor_record_offset":68364,"inlinee":31042,"source_line":691,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::SemaphoreWaitInfo::semaphores
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":71548,"parent_record_offset":68364,"ancestor_record_offset":68364,"inlinee":31040,"source_line":21994,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::SemaphoreWaitInfo::values
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":71576,"parent_record_offset":68364,"ancestor_record_offset":68364,"inlinee":31041,"source_line":22000,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$1827::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":71380,"parent_record_offset":68364,"ancestor_record_offset":68364,"inlinee":31038,"source_line":21972,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":71860,"parent_record_offset":68364,"ancestor_record_offset":68364,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":71688,"parent_record_offset":68364,"ancestor_record_offset":68364,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":72588,"parent_record_offset":68364,"ancestor_record_offset":68364,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":71208,"parent_record_offset":68364,"ancestor_record_offset":68364,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":70140,"parent_record_offset":68364,"ancestor_record_offset":68364,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":68552,"parent_record_offset":68364,"ancestor_record_offset":68364,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":69128,"parent_record_offset":68364,"ancestor_record_offset":68364,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmissionEngine::queue
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":570,"record_offset":71408,"parent_record_offset":68364,"ancestor_record_offset":68364,"inlinee":30506,"source_line":132,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmissionEngine::reclaim_command_buffer_resources_all_queues
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":570,"record_offset":72208,"parent_record_offset":68364,"ancestor_record_offset":68364,"inlinee":30505,"source_line":123,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":71448,"parent_record_offset":71408,"ancestor_record_offset":68364,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":72240,"parent_record_offset":72208,"ancestor_record_offset":68364,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":71628,"parent_record_offset":71604,"ancestor_record_offset":68364,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":71652,"parent_record_offset":71628,"ancestor_record_offset":68364,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":70560,"parent_record_offset":70532,"ancestor_record_offset":68364,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":70612,"parent_record_offset":70588,"ancestor_record_offset":68364,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":70588,"parent_record_offset":70560,"ancestor_record_offset":68364,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":70500,"parent_record_offset":70468,"ancestor_record_offset":68364,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":70532,"parent_record_offset":70500,"ancestor_record_offset":68364,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":70704,"parent_record_offset":70532,"ancestor_record_offset":68364,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":70816,"parent_record_offset":70792,"ancestor_record_offset":68364,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":570,"record_offset":71752,"parent_record_offset":71724,"ancestor_record_offset":68364,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":570,"record_offset":70636,"parent_record_offset":70612,"ancestor_record_offset":68364,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":570,"record_offset":68764,"parent_record_offset":68668,"ancestor_record_offset":68364,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":70392,"parent_record_offset":70364,"ancestor_record_offset":68364,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":69724,"parent_record_offset":69264,"ancestor_record_offset":68364,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":70440,"parent_record_offset":70260,"ancestor_record_offset":68364,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":570,"record_offset":69076,"parent_record_offset":69048,"ancestor_record_offset":68364,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":70768,"parent_record_offset":70468,"ancestor_record_offset":68364,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":72372,"parent_record_offset":72348,"ancestor_record_offset":68364,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":69500,"parent_record_offset":69476,"ancestor_record_offset":68364,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":69476,"parent_record_offset":69452,"ancestor_record_offset":68364,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":69684,"parent_record_offset":69652,"ancestor_record_offset":68364,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":70660,"parent_record_offset":70636,"ancestor_record_offset":68364,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":71168,"parent_record_offset":70220,"ancestor_record_offset":68364,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":71316,"parent_record_offset":71232,"ancestor_record_offset":68364,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":71340,"parent_record_offset":71316,"ancestor_record_offset":68364,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":570,"record_offset":68700,"parent_record_offset":68668,"ancestor_record_offset":68364,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":71476,"parent_record_offset":71448,"ancestor_record_offset":68364,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":68728,"parent_record_offset":68700,"ancestor_record_offset":68364,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":71504,"parent_record_offset":71476,"ancestor_record_offset":68364,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":69452,"parent_record_offset":69428,"ancestor_record_offset":68364,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":72348,"parent_record_offset":72268,"ancestor_record_offset":68364,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":69780,"parent_record_offset":69756,"ancestor_record_offset":68364,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":69428,"parent_record_offset":69320,"ancestor_record_offset":68364,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":72268,"parent_record_offset":72208,"ancestor_record_offset":68364,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":570,"record_offset":68668,"parent_record_offset":68640,"ancestor_record_offset":68364,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":570,"record_offset":69160,"parent_record_offset":69128,"ancestor_record_offset":68364,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":570,"record_offset":69292,"parent_record_offset":69264,"ancestor_record_offset":68364,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":570,"record_offset":69264,"parent_record_offset":69232,"ancestor_record_offset":68364,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":570,"record_offset":69208,"parent_record_offset":69184,"ancestor_record_offset":68364,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":570,"record_offset":69232,"parent_record_offset":69208,"ancestor_record_offset":68364,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":570,"record_offset":69184,"parent_record_offset":69160,"ancestor_record_offset":68364,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":570,"record_offset":69756,"parent_record_offset":69232,"ancestor_record_offset":68364,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":68864,"parent_record_offset":68832,"ancestor_record_offset":68364,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":69024,"parent_record_offset":68864,"ancestor_record_offset":68364,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":68832,"parent_record_offset":68808,"ancestor_record_offset":68364,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":68584,"parent_record_offset":68552,"ancestor_record_offset":68364,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":69048,"parent_record_offset":69024,"ancestor_record_offset":68364,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":68640,"parent_record_offset":68608,"ancestor_record_offset":68364,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":68608,"parent_record_offset":68584,"ancestor_record_offset":68364,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":570,"record_offset":69320,"parent_record_offset":69292,"ancestor_record_offset":68364,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":570,"record_offset":69568,"parent_record_offset":69320,"ancestor_record_offset":68364,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":570,"record_offset":69540,"parent_record_offset":69320,"ancestor_record_offset":68364,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":570,"record_offset":69372,"parent_record_offset":69320,"ancestor_record_offset":68364,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":71940,"parent_record_offset":71908,"ancestor_record_offset":68364,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":71260,"parent_record_offset":71232,"ancestor_record_offset":68364,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":71964,"parent_record_offset":71940,"ancestor_record_offset":68364,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":71284,"parent_record_offset":71260,"ancestor_record_offset":68364,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":71884,"parent_record_offset":71860,"ancestor_record_offset":68364,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":71232,"parent_record_offset":71208,"ancestor_record_offset":68364,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":71908,"parent_record_offset":71884,"ancestor_record_offset":68364,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":70180,"parent_record_offset":70140,"ancestor_record_offset":68364,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":570,"record_offset":70304,"parent_record_offset":70260,"ancestor_record_offset":68364,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":70468,"parent_record_offset":70260,"ancestor_record_offset":68364,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":70740,"parent_record_offset":70468,"ancestor_record_offset":68364,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::GraphicsDevice::log_error_info
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/crash.rs
//     evidence={"module":570,"record_offset":71812,"parent_record_offset":71724,"ancestor_record_offset":68364,"inlinee":31045,"source_line":87,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::error::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/error.rs
//     evidence={"module":570,"record_offset":71776,"parent_record_offset":71752,"ancestor_record_offset":68364,"inlinee":30763,"source_line":21,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$2::wait_submit_done::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":570,"record_offset":71724,"parent_record_offset":71688,"ancestor_record_offset":68364,"inlinee":31044,"source_line":341,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":570,"record_offset":70332,"parent_record_offset":70304,"ancestor_record_offset":68364,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":570,"record_offset":70364,"parent_record_offset":70332,"ancestor_record_offset":68364,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":570,"record_offset":70260,"parent_record_offset":70220,"ancestor_record_offset":68364,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":570,"record_offset":70220,"parent_record_offset":70180,"ancestor_record_offset":68364,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$2::wait_submit_done::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 341
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7944
// 首条位置: {"module":570,"inlinee":31044}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":570,"record_offset":71752,"parent_record_offset":71724,"ancestor_record_offset":71724,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::log_error_info
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/crash.rs
//     evidence={"module":570,"record_offset":71812,"parent_record_offset":71724,"ancestor_record_offset":71724,"inlinee":31045,"source_line":87,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::error::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/error.rs
//     evidence={"module":570,"record_offset":71776,"parent_record_offset":71752,"ancestor_record_offset":71724,"inlinee":30763,"source_line":21,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::wait_duration_or_until_submit_done
// 记录形式: compiled_procedure; instances=1
// 源码行: 354
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":72976,"rva":7616432,"va":5376325552}
// 编译实例 RVAs: 0x7437b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..time..Instant$u20$as$u20$core..ops..arith..Sub$GT$::sub
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1117985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e8210f1100","size":5,"soft_fail":false,"target_va":5377444464,"va":5376326474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144680
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27265686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e8960aa001","size":5,"soft_fail":false,"target_va":5403592320,"va":5376326629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144690
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27265662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e87e0aa001","size":5,"soft_fail":false,"target_va":5403592336,"va":5376326669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26651770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e87aac9601","size":5,"soft_fail":false,"target_va":5402978688,"va":5376326913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26652108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e8ccad9601","size":5,"soft_fail":false,"target_va":5402978912,"va":5376326799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26652566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e896af9601","size":5,"soft_fail":false,"target_va":5402979344,"va":5376326773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26652040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e888ad9601","size":5,"soft_fail":false,"target_va":5402978944,"va":5376326899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-47509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e86b46ffff","size":5,"soft_fail":false,"target_va":5376279232,"va":5376326736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26654070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e876b59601","size":5,"soft_fail":false,"target_va":5402980128,"va":5376326053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-46211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e87d4bffff","size":5,"soft_fail":false,"target_va":5376279424,"va":5376325630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26654951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e8e7b89601","size":5,"soft_fail":false,"target_va":5402980656,"va":5376325700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] once_cell::imp::OnceCell$LT$T$GT$::initialize
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t26900063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e85f769a01","size":5,"soft_fail":false,"target_va":5403226688,"va":5376326620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26642616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e8b8889601","size":5,"soft_fail":false,"target_va":5402968992,"va":5376326371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t26644098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e8828e9601","size":5,"soft_fail":false,"target_va":5402970800,"va":5376326697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t30691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e8e3770000","size":5,"soft_fail":false,"target_va":5376356912,"va":5376326216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t34571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e80b870000","size":5,"soft_fail":false,"target_va":5376360560,"va":5376325984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t30539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e84b770000","size":5,"soft_fail":false,"target_va":5376356144,"va":5376325600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hf5f0f90a77fe959e (.llvm.16944548595553555407)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10723431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e867a0a300","size":5,"soft_fail":false,"target_va":5387049440,"va":5376326004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1066556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e83c461000","size":5,"soft_fail":false,"target_va":5377393200,"va":5376326639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1116680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376325552,"raw_hex":"e8080a1100","size":5,"soft_fail":false,"target_va":5377442944,"va":5376326259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*1256(%rbx)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376325552,"raw_hex":"ff93e8040000","size":6,"soft_fail":false,"va":5376326430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::get_semaphore_counter_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":76240,"parent_record_offset":72976,"ancestor_record_offset":72976,"inlinee":30492,"source_line":679,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":76356,"parent_record_offset":72976,"ancestor_record_offset":72976,"inlinee":8949,"source_line":1221,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":73764,"parent_record_offset":72976,"ancestor_record_offset":72976,"inlinee":4202,"source_line":1171,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":77412,"parent_record_offset":72976,"ancestor_record_offset":72976,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":77764,"parent_record_offset":72976,"ancestor_record_offset":72976,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::time::impl$4::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":570,"record_offset":76468,"parent_record_offset":72976,"ancestor_record_offset":72976,"inlinee":31047,"source_line":1125,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":75936,"parent_record_offset":72976,"ancestor_record_offset":72976,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":74904,"parent_record_offset":72976,"ancestor_record_offset":72976,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":73180,"parent_record_offset":72976,"ancestor_record_offset":72976,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":73892,"parent_record_offset":72976,"ancestor_record_offset":72976,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmissionEngine::queue
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":570,"record_offset":76108,"parent_record_offset":72976,"ancestor_record_offset":72976,"inlinee":30506,"source_line":132,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] spin_sleep::native_sleep
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"spin_sleep","version":"1.1.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spin_sleep-1.1.1/src/lib.rs
//     evidence={"module":570,"record_offset":76884,"parent_record_offset":72976,"ancestor_record_offset":72976,"inlinee":12639,"source_line":115,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":76140,"parent_record_offset":76108,"ancestor_record_offset":72976,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":76268,"parent_record_offset":76240,"ancestor_record_offset":72976,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":76292,"parent_record_offset":76268,"ancestor_record_offset":72976,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":76316,"parent_record_offset":76292,"ancestor_record_offset":72976,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":75288,"parent_record_offset":75260,"ancestor_record_offset":72976,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":75340,"parent_record_offset":75316,"ancestor_record_offset":72976,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":75316,"parent_record_offset":75288,"ancestor_record_offset":72976,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":75228,"parent_record_offset":75200,"ancestor_record_offset":72976,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":75260,"parent_record_offset":75228,"ancestor_record_offset":72976,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":75432,"parent_record_offset":75260,"ancestor_record_offset":72976,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":75544,"parent_record_offset":75520,"ancestor_record_offset":72976,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$62::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":73820,"parent_record_offset":73792,"ancestor_record_offset":72976,"inlinee":4212,"source_line":1549,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$63::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":73848,"parent_record_offset":73820,"ancestor_record_offset":72976,"inlinee":4214,"source_line":1565,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":570,"record_offset":76816,"parent_record_offset":76692,"ancestor_record_offset":72976,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":570,"record_offset":75364,"parent_record_offset":75340,"ancestor_record_offset":72976,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":570,"record_offset":73400,"parent_record_offset":73304,"ancestor_record_offset":72976,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":76748,"parent_record_offset":76692,"ancestor_record_offset":72976,"inlinee":9241,"source_line":460,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":76544,"parent_record_offset":76504,"ancestor_record_offset":72976,"inlinee":11118,"source_line":604,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":76780,"parent_record_offset":76748,"ancestor_record_offset":72976,"inlinee":9242,"source_line":1975,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":76576,"parent_record_offset":76544,"ancestor_record_offset":72976,"inlinee":11119,"source_line":2074,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":75124,"parent_record_offset":75096,"ancestor_record_offset":72976,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":76852,"parent_record_offset":76468,"ancestor_record_offset":72976,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":74488,"parent_record_offset":74028,"ancestor_record_offset":72976,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":75172,"parent_record_offset":75000,"ancestor_record_offset":72976,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":570,"record_offset":73712,"parent_record_offset":73684,"ancestor_record_offset":72976,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":75496,"parent_record_offset":75200,"ancestor_record_offset":72976,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":74264,"parent_record_offset":74240,"ancestor_record_offset":72976,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":74240,"parent_record_offset":74216,"ancestor_record_offset":72976,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":74448,"parent_record_offset":74416,"ancestor_record_offset":72976,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":75388,"parent_record_offset":75364,"ancestor_record_offset":72976,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":75896,"parent_record_offset":74968,"ancestor_record_offset":72976,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":76044,"parent_record_offset":75960,"ancestor_record_offset":72976,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":76068,"parent_record_offset":76044,"ancestor_record_offset":72976,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":570,"record_offset":73336,"parent_record_offset":73304,"ancestor_record_offset":72976,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":76168,"parent_record_offset":76140,"ancestor_record_offset":72976,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":73364,"parent_record_offset":73336,"ancestor_record_offset":72976,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":76196,"parent_record_offset":76168,"ancestor_record_offset":72976,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":74216,"parent_record_offset":74192,"ancestor_record_offset":72976,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":74544,"parent_record_offset":74520,"ancestor_record_offset":72976,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":74192,"parent_record_offset":74084,"ancestor_record_offset":72976,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":570,"record_offset":73304,"parent_record_offset":73276,"ancestor_record_offset":72976,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":570,"record_offset":73924,"parent_record_offset":73892,"ancestor_record_offset":72976,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":570,"record_offset":74056,"parent_record_offset":74028,"ancestor_record_offset":72976,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":570,"record_offset":74028,"parent_record_offset":73996,"ancestor_record_offset":72976,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":570,"record_offset":73972,"parent_record_offset":73948,"ancestor_record_offset":72976,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":570,"record_offset":73996,"parent_record_offset":73972,"ancestor_record_offset":72976,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":570,"record_offset":73948,"parent_record_offset":73924,"ancestor_record_offset":72976,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":570,"record_offset":74520,"parent_record_offset":73996,"ancestor_record_offset":72976,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":73500,"parent_record_offset":73468,"ancestor_record_offset":72976,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":73660,"parent_record_offset":73500,"ancestor_record_offset":72976,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":73468,"parent_record_offset":73444,"ancestor_record_offset":72976,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":73216,"parent_record_offset":73180,"ancestor_record_offset":72976,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":73684,"parent_record_offset":73660,"ancestor_record_offset":72976,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":73276,"parent_record_offset":73244,"ancestor_record_offset":72976,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":73244,"parent_record_offset":73216,"ancestor_record_offset":72976,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":570,"record_offset":74084,"parent_record_offset":74056,"ancestor_record_offset":72976,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":570,"record_offset":74332,"parent_record_offset":74084,"ancestor_record_offset":72976,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":570,"record_offset":74304,"parent_record_offset":74084,"ancestor_record_offset":72976,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":570,"record_offset":74136,"parent_record_offset":74084,"ancestor_record_offset":72976,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":77092,"parent_record_offset":77064,"ancestor_record_offset":72976,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":77492,"parent_record_offset":77460,"ancestor_record_offset":72976,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":75988,"parent_record_offset":75960,"ancestor_record_offset":72976,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":77516,"parent_record_offset":77492,"ancestor_record_offset":72976,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":76012,"parent_record_offset":75988,"ancestor_record_offset":72976,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":77120,"parent_record_offset":77092,"ancestor_record_offset":72976,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":570,"record_offset":76504,"parent_record_offset":76468,"ancestor_record_offset":72976,"inlinee":16292,"source_line":693,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":570,"record_offset":76692,"parent_record_offset":76504,"ancestor_record_offset":72976,"inlinee":13444,"source_line":207,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$30::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":570,"record_offset":73792,"parent_record_offset":73764,"ancestor_record_offset":72976,"inlinee":4210,"source_line":92,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":77436,"parent_record_offset":77412,"ancestor_record_offset":72976,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/imp_std.rs
//     evidence={"module":570,"record_offset":77064,"parent_record_offset":77036,"ancestor_record_offset":72976,"inlinee":6941,"source_line":49,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":570,"record_offset":76948,"parent_record_offset":76920,"ancestor_record_offset":72976,"inlinee":6935,"source_line":1308,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":570,"record_offset":77036,"parent_record_offset":77004,"ancestor_record_offset":72976,"inlinee":6939,"source_line":963,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":570,"record_offset":76976,"parent_record_offset":76948,"ancestor_record_offset":72976,"inlinee":6937,"source_line":1115,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":570,"record_offset":77004,"parent_record_offset":76976,"ancestor_record_offset":72976,"inlinee":6938,"source_line":1152,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":570,"record_offset":76920,"parent_record_offset":76884,"ancestor_record_offset":72976,"inlinee":6933,"source_line":1375,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":75960,"parent_record_offset":75936,"ancestor_record_offset":72976,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":77460,"parent_record_offset":77436,"ancestor_record_offset":72976,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":74936,"parent_record_offset":74904,"ancestor_record_offset":72976,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":570,"record_offset":75036,"parent_record_offset":75000,"ancestor_record_offset":72976,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":75200,"parent_record_offset":75000,"ancestor_record_offset":72976,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":75468,"parent_record_offset":75200,"ancestor_record_offset":72976,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":570,"record_offset":75064,"parent_record_offset":75036,"ancestor_record_offset":72976,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":570,"record_offset":75096,"parent_record_offset":75064,"ancestor_record_offset":72976,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":570,"record_offset":75000,"parent_record_offset":74968,"ancestor_record_offset":72976,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":570,"record_offset":74968,"parent_record_offset":74936,"ancestor_record_offset":72976,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00570.jsonl.gz"}
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
// 完整结构及原始类型记录: submit.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::submit::DeviceSubmissionTimestamp
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::ops::range::Range<u64>,rhapsody::raw::submit::DeviceSubmissionTimestamp (*)(u64)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::ops::range::Range<u64>", "rhapsody::raw::submit::DeviceSubmissionTimestamp (*)(u64)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::impl$18::extend<rhapsody::raw::submit::DeviceSubmissionTimestamp,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<u64>,rhapsody::raw::submit::DeviceSubmissionTimestamp (*)(u64)> >", "constructor": "alloc::collections::vec_deque::impl$18::extend", "arguments": ["rhapsody::raw::submit::DeviceSubmissionTimestamp", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::ops::range::Range<u64>,rhapsody::raw::submit::DeviceSubmissionTimestamp (*)(u64)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::submit::QueueIndex
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::submit::QueueIndex>", "constructor": "enum2$", "arguments": ["rhapsody::raw::submit::QueueIndex"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panicking::assert_failed<enum2$<rhapsody::raw::submit::QueueIndex>,enum2$<rhapsody::raw::submit::QueueIndex> >", "constructor": "core::panicking::assert_failed", "arguments": ["enum2$<rhapsody::raw::submit::QueueIndex>", "enum2$<rhapsody::raw::submit::QueueIndex>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<enum2$<rhapsody::raw::submit::QueueIndex> >", "constructor": "core::fmt::impl$51::fmt", "arguments": ["enum2$<rhapsody::raw::submit::QueueIndex>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::submit::SubmissionEngine
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::submit::SubmissionEngine>", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "rhapsody::raw::submit::SubmissionEngine"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::submit::SubmissionEngine> >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::submit::SubmissionEngine>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::submit::SubmitInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 15
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>", "constructor": "core::slice::iter::IterMut", "arguments": ["rhapsody::raw::submit::SubmitInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>", "alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["rhapsody::raw::submit::SubmitInfo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >", "constructor": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$5", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >", "rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["ash::vk::definitions::SubmitInfo", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["ash::vk::definitions::SubmitInfo", "alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<ash::vk::definitions::SubmitInfo,core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["ash::vk::definitions::SubmitInfo", "core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >", "rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>", "constructor": "core::slice::iter::Iter", "arguments": ["rhapsody::raw::submit::SubmitInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$4<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >", "constructor": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$4", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$4<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>", "core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >", "rhapsody::raw::submit::impl$2::queue_submit::closure_env$4<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$4<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ash::vk::definitions::Semaphore", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$4<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >", "constructor": "rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>", "rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >", "constructor": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$1", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>", "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >", "rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::CommandBuffer,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ash::vk::definitions::CommandBuffer", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$2<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >", "constructor": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$2", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$2<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>", "core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >", "rhapsody::raw::submit::impl$2::queue_submit::closure_env$2<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$2<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ash::vk::definitions::Semaphore", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$2<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$3<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >", "constructor": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$3", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::bitflags::PipelineStageFlags> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$3<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>", "core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::bitflags::PipelineStageFlags> >", "rhapsody::raw::submit::impl$2::queue_submit::closure_env$3<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::bitflags::PipelineStageFlags,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::bitflags::PipelineStageFlags> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$3<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ash::vk::bitflags::PipelineStageFlags", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::bitflags::PipelineStageFlags> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$3<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::SubmitInfo,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ash::vk::definitions::SubmitInfo", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "slice2$<rhapsody::raw::submit::SubmitInfo>", "constructor": "slice2$", "arguments": ["rhapsody::raw::submit::SubmitInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<slice2$<rhapsody::raw::submit::SubmitInfo>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["slice2$<rhapsody::raw::submit::SubmitInfo>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<slice2$<rhapsody::raw::submit::SubmitInfo>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<slice2$<rhapsody::raw::submit::SubmitInfo>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >", "constructor": "rhapsody::raw::device::GraphicsDevice::queue_submit", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::submit::SubmitInfo>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::submit::SubmitInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: submit.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: submit.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 10; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] std::ffi::os_str::OsString (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]
// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentDeserializer<serde_json::error::Error> (*)(enum2$<serde::__private::de::content::Content>)
// 候选参数类型: ["enum2$<serde::__private::de::content::Content>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::queue_family_index
// 信息 ID: 9072f77b821ef542c84ab553
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2def51 -> 0x7430f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2ded2c -> 0x7430f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >
// 信息 ID: d6234958d5f7deaa121b0dbe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["rhapsody::raw::submit::SubmitInfo", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::queue_submit -> ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2e006c -> 0x269bc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::wait_duration_or_until_submit_done
// 信息 ID: a37e73e7e66db356aab1acd7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::wait_gpu_queue_trim @ 0x736b14 -> 0x7437b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::wait_submit_done
// 信息 ID: 4ec7e4e7ca957f1e4737145c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::wait_submits_done @ 0x7366fc -> 0x743130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmissionEngine::queue
// 信息 ID: 5168c48d847eef5c7ec15800
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::wait_submit_done; module=570, inline_record=71408; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::wait_duration_or_until_submit_done; module=570, inline_record=76108; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::command_buffer::CommandBuffer::begin; module=571, inline_record=2516; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::present; module=1622, inline_record=20304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmissionEngine::queue_mut
// 信息 ID: 076fda90cae6ee4150acdb2c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > @ 0x269e4d -> 0x7296d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > @ 0x269e86 -> 0x7296d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmissionEngine::reclaim_command_buffer_resources_all_queues
// 信息 ID: 15782f0f7194241cf9ec28e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::impl$5::drop; module=556, inline_record=79120; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources; module=570, inline_record=53780; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::wait_submit_done; module=570, inline_record=72208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmissionEngine::update_pending_submission_timestamp
// 信息 ID: 54c45ccb3715622933accc89
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > @ 0x269e71 -> 0x729400 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmissionEngine::update_queue_pending_submissions
// 信息 ID: 863bbeb876dc90930b8aabfe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > @ 0x269c40 -> 0x729510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources @ 0x741918 -> 0x729510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmitInfo::command_buffer
// 信息 ID: 7772f6d2fd71d8a81f74a80c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2dfe57 -> 0x729190 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::get_setup_cb_submit_info; module=570, inline_record=48932; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmitInfo::new
// 信息 ID: c288f6b378aad503ea018b64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2defd9 -> 0x729130 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::get_setup_cb_submit_info; module=570, inline_record=48876; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmitInfo::signal_semaphore
// 信息 ID: 3b3a0b339dcc00fbe28c442a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2df9c3 -> 0x729340 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmitInfo::wait_semaphore
// 信息 ID: 15fd68b624809aa01f1fb435
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2df3a7 -> 0x729280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::carve_off_front
// 信息 ID: 70bad85060d48c63627c74fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::SubmitInfo,core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > > > >; module=21, inline_record=24920; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::SubmitInfo,core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > > > >; module=21, inline_record=25128; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::SubmitInfo,core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > > > >; module=21, inline_record=25336; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::SubmitInfo,core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > > > >; module=21, inline_record=25544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$17::partial_cmp
// 信息 ID: 448d016b5aa78339948d0791
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::submit::SubmissionEngine::update_queue_pending_submissions; module=551, inline_record=3676; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::submit::SubmissionEngine::update_queue_pending_submissions; module=551, inline_record=4396; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::Queue::reclaim_command_buffer_resources; module=570, inline_record=25632; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$18::cmp
// 信息 ID: f5fdb5f44d22b9687973558d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$1::update_pending_submission_timestamp::closure$0
// 信息 ID: 9943ec8c9a81632db0037caf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::submit::impl$1::update_pending_submission_timestamp", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::submit::SubmissionEngine::update_pending_submission_timestamp; module=551, inline_record=1940; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::submit::SubmissionEngine::update_pending_submission_timestamp; module=551, inline_record=2316; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::submit::SubmissionEngine::update_pending_submission_timestamp; module=551, inline_record=2716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$2::queue_submit::closure$1
// 信息 ID: 9bf507ef033d9ea0295c2d20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::submit::impl$2::queue_submit", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::submit::impl$2::queue_submit::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::CommandBuffer,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::CommandBuffer,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >; module=73, inline_record=94112; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::CommandBuffer,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >; module=73, inline_record=96248; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$2::queue_submit::closure$2
// 信息 ID: 83523ef90eb3800a52d8b4a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "rhapsody::raw::submit::impl$2::queue_submit", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::submit::impl$2::queue_submit::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$2<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$2<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >; module=73, inline_record=98688; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$2<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >; module=73, inline_record=100596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$2::queue_submit::closure$3
// 信息 ID: dc1cd6bb2ef809204e11d1b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "rhapsody::raw::submit::impl$2::queue_submit", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::submit::impl$2::queue_submit::closure_env$3 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::bitflags::PipelineStageFlags,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::bitflags::PipelineStageFlags> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$3<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::bitflags::PipelineStageFlags,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::bitflags::PipelineStageFlags> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$3<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >; module=73, inline_record=104964; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::bitflags::PipelineStageFlags,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::bitflags::PipelineStageFlags> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$3<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >; module=73, inline_record=106872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$2::queue_submit::closure$4
// 信息 ID: 018053418c7e6b1e7972ca19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "rhapsody::raw::submit::impl$2::queue_submit", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::submit::impl$2::queue_submit::closure_env$4 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$4<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$4<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >; module=73, inline_record=84448; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$4<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >; module=73, inline_record=86356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$2::queue_submit::closure$5
// 信息 ID: 10e9361a8f6a4bd7861340af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "rhapsody::raw::submit::impl$2::queue_submit", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::submit::impl$2::queue_submit::closure_env$5 <- core::iter::adapters::map::impl$2::fold<ash::vk::definitions::SubmitInfo,core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::submit::impl$2::queue_submit::closure_env$5 <- core::iter::adapters::map::impl$2::fold<ash::vk::definitions::SubmitInfo,core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::submit::impl$2::queue_submit::closure_env$5 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::SubmitInfo,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::SubmitInfo,core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > > > >; module=21, inline_record=24240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$2::queue_submit::closure$6
// 信息 ID: a6161f9271c1a7ecd7a42b1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "rhapsody::raw::submit::impl$2::queue_submit", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >; module=139, inline_record=9496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$2::queue_submit::closure$7
// 信息 ID: 6166014b2a72e35c13188581
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "rhapsody::raw::submit::impl$2::queue_submit", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >; module=139, inline_record=10388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$2::wait_submit_done::closure$0
// 信息 ID: 30ed492431391eb4b105dea8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::submit::impl$2::wait_submit_done", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::wait_submit_done; module=570, inline_record=71724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$3::default
// 信息 ID: 50985f6e89928674adc67754
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::submit::SubmitInfo::new; module=551, inline_record=372; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::get_setup_cb_submit_info; module=570, inline_record=48900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$7::eq
// 信息 ID: 49460a57f2a33d390f73f470
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >; module=139, inline_record=8248; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources>; module=178, inline_record=14812; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::Queue::reclaim_command_buffer_resources; module=570, inline_record=28856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$9::fmt
// 信息 ID: c9206c8a1e8b4f3c4856018c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<rhapsody::raw::submit::QueueIndex> >; module=193, inline_record=17384; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<rhapsody::raw::submit::QueueIndex> >; module=3013, inline_record=400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
