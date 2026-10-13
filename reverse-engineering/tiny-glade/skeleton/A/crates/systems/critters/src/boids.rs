// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/critters/src/boids.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 6e4e1da242a8497af0667f9a5cfc313e8ed73ade42d659d59212b1965de5fe01
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: boids.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1012, 2410, 2413, 2414, 3623
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lazy_static; // namespace candidate
//   package identities: [{"name":"lazy_static","source":"PDB_registry_path_only","version":"1.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_critters; // namespace candidate
//   package identities: [{"name":"lazy_static","source":"PDB_registry_path_only","version":"1.4.0"}]
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::boids::impl$5::clone
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6AC
// 首条位置: {"module":1012,"inlinee":50860}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::push
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6A9
// 首条位置: {"module":1012,"inlinee":50857}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1012,"record_offset":4756,"parent_record_offset":4728,"ancestor_record_offset":4728,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1012,"record_offset":4824,"parent_record_offset":4792,"ancestor_record_offset":4728,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1012,"record_offset":4792,"parent_record_offset":4756,"ancestor_record_offset":4728,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1012,"record_offset":4856,"parent_record_offset":4756,"ancestor_record_offset":4728,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::drain
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14455
// 首条位置: {"module":3623,"inlinee":83029}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3623,"record_offset":7676,"parent_record_offset":7648,"ancestor_record_offset":7648,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/03623.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3623,"record_offset":7704,"parent_record_offset":7648,"ancestor_record_offset":7648,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/03623.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3623,"record_offset":7764,"parent_record_offset":7740,"ancestor_record_offset":7648,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03623.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3623,"record_offset":7740,"parent_record_offset":7704,"ancestor_record_offset":7648,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/03623.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3623,"record_offset":7796,"parent_record_offset":7704,"ancestor_record_offset":7648,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03623.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3623,"record_offset":7824,"parent_record_offset":7704,"ancestor_record_offset":7648,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/03623.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3623,"record_offset":7900,"parent_record_offset":7876,"ancestor_record_offset":7648,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03623.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3623,"record_offset":7852,"parent_record_offset":7704,"ancestor_record_offset":7648,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03623.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3623,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":7648,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03623.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::len
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6AE
// 首条位置: {"module":1012,"inlinee":50862}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::clone_state
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6AA
// 首条位置: {"module":1012,"inlinee":50858}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1012,"record_offset":5068,"parent_record_offset":5028,"ancestor_record_offset":5028,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1012,"record_offset":5476,"parent_record_offset":5452,"ancestor_record_offset":5028,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1012,"record_offset":5500,"parent_record_offset":5476,"ancestor_record_offset":5028,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1012,"record_offset":5452,"parent_record_offset":5344,"ancestor_record_offset":5028,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1012,"record_offset":5344,"parent_record_offset":5300,"ancestor_record_offset":5028,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1012,"record_offset":5300,"parent_record_offset":5256,"ancestor_record_offset":5028,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1012,"record_offset":5572,"parent_record_offset":5300,"ancestor_record_offset":5028,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1012,"record_offset":5196,"parent_record_offset":5152,"ancestor_record_offset":5028,"inlinee":7706,"source_line":124,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1012,"record_offset":5152,"parent_record_offset":5108,"ancestor_record_offset":5028,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1012,"record_offset":5108,"parent_record_offset":5068,"ancestor_record_offset":5028,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1012,"record_offset":5256,"parent_record_offset":5196,"ancestor_record_offset":5028,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1012,"record_offset":5396,"parent_record_offset":5344,"ancestor_record_offset":5028,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1012,"record_offset":5420,"parent_record_offset":5396,"ancestor_record_offset":5028,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1012,"record_offset":5648,"parent_record_offset":5612,"ancestor_record_offset":5028,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\take.rs
//     evidence={"module":1012,"record_offset":5612,"parent_record_offset":5196,"ancestor_record_offset":5028,"inlinee":7708,"source_line":38,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1012,"record_offset":5840,"parent_record_offset":5196,"ancestor_record_offset":5028,"inlinee":6259,"source_line":493,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1012,"record_offset":5764,"parent_record_offset":5740,"ancestor_record_offset":5028,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1012,"record_offset":5712,"parent_record_offset":5676,"ancestor_record_offset":5028,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1012,"record_offset":5528,"parent_record_offset":5500,"ancestor_record_offset":5028,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1012,"record_offset":5740,"parent_record_offset":5676,"ancestor_record_offset":5028,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1012,"record_offset":5676,"parent_record_offset":5648,"ancestor_record_offset":5028,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$5::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":1012,"record_offset":5808,"parent_record_offset":5196,"ancestor_record_offset":5028,"inlinee":50860,"source_line":61,"archive":"_pdb/A/modules/01012.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::iter_mut
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6AD
// 首条位置: {"module":1012,"inlinee":50861}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::remove<system_critters::fish::fish::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2414,"record_offset":252,"rva":22712048,"va":5391421168}
// 编译实例 RVAs: 0x15a8ef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: boids.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18671786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391421168,"raw_hex":"e85617e3fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5391421493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::swap_remove::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11556648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391421168,"raw_hex":"e82857b000","size":5,"soft_fail":false,"target_va":5402978176,"va":5391421523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14176120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391421168,"raw_hex":"e888b027ff","size":5,"soft_fail":false,"target_va":5377245296,"va":5391421411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14115913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391421168,"raw_hex":"e8b79b28ff","size":5,"soft_fail":false,"target_va":5377305552,"va":5391421460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::swap_remove
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2414,"record_offset":640,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":9718,"source_line":1487,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":2414,"record_offset":524,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":7276,"source_line":33,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2414,"record_offset":1072,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2414,"record_offset":440,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] system_critters::fish::fish::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":2414,"record_offset":984,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":83037,"source_line":281,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2414,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":252,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2414,"record_offset":1308,"parent_record_offset":1216,"ancestor_record_offset":252,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2414,"record_offset":1244,"parent_record_offset":1216,"ancestor_record_offset":252,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2414,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":252,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2414,"record_offset":684,"parent_record_offset":640,"ancestor_record_offset":252,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2414,"record_offset":776,"parent_record_offset":640,"ancestor_record_offset":252,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2414,"record_offset":1120,"parent_record_offset":1096,"ancestor_record_offset":252,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2414,"record_offset":1168,"parent_record_offset":1144,"ancestor_record_offset":252,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2414,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":252,"inlinee":18499,"source_line":311,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2414,"record_offset":1012,"parent_record_offset":984,"ancestor_record_offset":252,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":2414,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":252,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2414,"record_offset":1272,"parent_record_offset":1244,"ancestor_record_offset":252,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2414,"record_offset":712,"parent_record_offset":640,"ancestor_record_offset":252,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2414,"record_offset":1096,"parent_record_offset":1072,"ancestor_record_offset":252,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2414,"record_offset":488,"parent_record_offset":464,"ancestor_record_offset":252,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2414,"record_offset":576,"parent_record_offset":548,"ancestor_record_offset":252,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2414,"record_offset":604,"parent_record_offset":548,"ancestor_record_offset":252,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2414,"record_offset":464,"parent_record_offset":440,"ancestor_record_offset":252,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::new
// 记录形式: inlinee_declaration; instances=1
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14451
// 首条位置: {"module":2413,"inlinee":83025}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::unobstructed_dir
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1445F
// 首条位置: {"module":2414,"inlinee":83039}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::function::impls::impl$0::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2414,"record_offset":12932,"parent_record_offset":12600,"ancestor_record_offset":12600,"inlinee":33676,"source_line":261,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2414,"record_offset":5776,"parent_record_offset":4948,"ancestor_record_offset":4948,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":2414,"record_offset":5272,"parent_record_offset":4948,"ancestor_record_offset":4948,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":5008,"parent_record_offset":4948,"ancestor_record_offset":4948,"inlinee":22211,"source_line":523,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] system_critters::boids::impl$3::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":2414,"record_offset":5036,"parent_record_offset":4948,"ancestor_record_offset":4948,"inlinee":83028,"source_line":135,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2414,"record_offset":13520,"parent_record_offset":13120,"ancestor_record_offset":12600,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":5512,"parent_record_offset":5272,"ancestor_record_offset":4948,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":5420,"parent_record_offset":5364,"ancestor_record_offset":4948,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":5312,"parent_record_offset":5272,"ancestor_record_offset":4948,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":5600,"parent_record_offset":5540,"ancestor_record_offset":4948,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":5656,"parent_record_offset":5540,"ancestor_record_offset":4948,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":13364,"parent_record_offset":13336,"ancestor_record_offset":12600,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":13392,"parent_record_offset":13336,"ancestor_record_offset":12600,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":13088,"parent_record_offset":13064,"ancestor_record_offset":12600,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":13028,"parent_record_offset":13000,"ancestor_record_offset":12600,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":13224,"parent_record_offset":13200,"ancestor_record_offset":12600,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":13936,"parent_record_offset":13908,"ancestor_record_offset":12600,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2414,"record_offset":5836,"parent_record_offset":5812,"ancestor_record_offset":4948,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2414,"record_offset":5868,"parent_record_offset":5776,"ancestor_record_offset":4948,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2414,"record_offset":5812,"parent_record_offset":5776,"ancestor_record_offset":4948,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2414,"record_offset":5196,"parent_record_offset":5172,"ancestor_record_offset":4948,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2414,"record_offset":5220,"parent_record_offset":5196,"ancestor_record_offset":4948,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::impl$3::load_index_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":2414,"record_offset":13640,"parent_record_offset":13604,"ancestor_record_offset":12600,"inlinee":21432,"source_line":524,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2414,"record_offset":5540,"parent_record_offset":5272,"ancestor_record_offset":4948,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":13484,"parent_record_offset":13120,"ancestor_record_offset":12600,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":13312,"parent_record_offset":13120,"ancestor_record_offset":12600,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":13336,"parent_record_offset":13312,"ancestor_record_offset":12600,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":13256,"parent_record_offset":13120,"ancestor_record_offset":12600,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":13908,"parent_record_offset":13120,"ancestor_record_offset":12600,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":13200,"parent_record_offset":13120,"ancestor_record_offset":12600,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":13428,"parent_record_offset":13120,"ancestor_record_offset":12600,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":13000,"parent_record_offset":12964,"ancestor_record_offset":12600,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":13064,"parent_record_offset":12964,"ancestor_record_offset":12600,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":2414,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":4948,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":2414,"record_offset":5340,"parent_record_offset":5272,"ancestor_record_offset":4948,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":2414,"record_offset":13576,"parent_record_offset":13120,"ancestor_record_offset":12600,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$65::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":2414,"record_offset":13544,"parent_record_offset":13520,"ancestor_record_offset":12600,"inlinee":22963,"source_line":1123,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] lazy_static::lazy::Lazy::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lazy_static-1.4.0/src/inline_lazy.rs
//     evidence={"module":2414,"record_offset":5100,"parent_record_offset":5068,"ancestor_record_offset":4948,"inlinee":17117,"source_line":26,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\once.rs
//     evidence={"module":2414,"record_offset":5136,"parent_record_offset":5100,"ancestor_record_offset":4948,"inlinee":17118,"source_line":139,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::queue::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\once\queue.rs
//     evidence={"module":2414,"record_offset":5172,"parent_record_offset":5136,"ancestor_record_offset":4948,"inlinee":6263,"source_line":117,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref::__stability
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":2414,"record_offset":5068,"parent_record_offset":5036,"ancestor_record_offset":4948,"inlinee":83026,"source_line":140,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::fish::fish::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":2414,"record_offset":12964,"parent_record_offset":12932,"ancestor_record_offset":12600,"inlinee":82976,"source_line":194,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":2414,"record_offset":13604,"parent_record_offset":13120,"ancestor_record_offset":12600,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":2414,"record_offset":13120,"parent_record_offset":12964,"ancestor_record_offset":12600,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::apply_rules
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1445E
// 首条位置: {"module":2414,"inlinee":83038}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":6896,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":4644,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":4616,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2414,"record_offset":2328,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":6620,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":59386,"source_line":808,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":4388,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":2672,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3936,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":2608,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":22227,"source_line":446,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3468,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":4728,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3720,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":22755,"source_line":558,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":6980,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":2212,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":2500,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$18::sub_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3104,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":25913,"source_line":1146,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":2856,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$5::div_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3196,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":13200,"source_line":1005,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3016,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2414,"record_offset":4672,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2414,"record_offset":2796,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] system_critters::boids::Boid::unobstructed_dir
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":2414,"record_offset":4948,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":83039,"source_line":152,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] system_critters::fish::fish::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":2414,"record_offset":11580,"parent_record_offset":8600,"ancestor_record_offset":8600,"inlinee":82976,"source_line":194,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2414,"record_offset":2952,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":21527,"source_line":230,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2414,"record_offset":12116,"parent_record_offset":11728,"ancestor_record_offset":8600,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":5512,"parent_record_offset":5272,"ancestor_record_offset":1944,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":5420,"parent_record_offset":5364,"ancestor_record_offset":1944,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":5312,"parent_record_offset":5272,"ancestor_record_offset":1944,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":5600,"parent_record_offset":5540,"ancestor_record_offset":1944,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":5656,"parent_record_offset":5540,"ancestor_record_offset":1944,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":2980,"parent_record_offset":2952,"ancestor_record_offset":1944,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":11968,"parent_record_offset":11940,"ancestor_record_offset":8600,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":11996,"parent_record_offset":11940,"ancestor_record_offset":8600,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2414,"record_offset":6948,"parent_record_offset":6896,"ancestor_record_offset":1944,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":3044,"parent_record_offset":3016,"ancestor_record_offset":1944,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":3600,"parent_record_offset":3496,"ancestor_record_offset":1944,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":2888,"parent_record_offset":2856,"ancestor_record_offset":1944,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":2252,"parent_record_offset":2212,"ancestor_record_offset":1944,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$336::sub_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":3136,"parent_record_offset":3104,"ancestor_record_offset":1944,"inlinee":25009,"source_line":828,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$392::div_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":3224,"parent_record_offset":3196,"ancestor_record_offset":1944,"inlinee":13202,"source_line":943,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":2540,"parent_record_offset":2500,"ancestor_record_offset":1944,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$0::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2414,"record_offset":12932,"parent_record_offset":12600,"ancestor_record_offset":8600,"inlinee":33676,"source_line":261,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2414,"record_offset":5836,"parent_record_offset":5812,"ancestor_record_offset":1944,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2414,"record_offset":2448,"parent_record_offset":2380,"ancestor_record_offset":1944,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2414,"record_offset":5812,"parent_record_offset":5776,"ancestor_record_offset":1944,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2414,"record_offset":2380,"parent_record_offset":2328,"ancestor_record_offset":1944,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2414,"record_offset":5196,"parent_record_offset":5172,"ancestor_record_offset":1944,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2414,"record_offset":5220,"parent_record_offset":5196,"ancestor_record_offset":1944,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::impl$3::load_index_unchecked
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":2414,"record_offset":12236,"parent_record_offset":12200,"ancestor_record_offset":8600,"inlinee":21432,"source_line":524,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":2414,"record_offset":6708,"parent_record_offset":6620,"ancestor_record_offset":1944,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":2414,"record_offset":5272,"parent_record_offset":4948,"ancestor_record_offset":1944,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2414,"record_offset":5540,"parent_record_offset":5272,"ancestor_record_offset":1944,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":12088,"parent_record_offset":11728,"ancestor_record_offset":8600,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":11916,"parent_record_offset":11728,"ancestor_record_offset":8600,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":11940,"parent_record_offset":11916,"ancestor_record_offset":8600,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":11860,"parent_record_offset":11728,"ancestor_record_offset":8600,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":12504,"parent_record_offset":11728,"ancestor_record_offset":8600,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":11804,"parent_record_offset":11728,"ancestor_record_offset":8600,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":12032,"parent_record_offset":11728,"ancestor_record_offset":8600,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":2636,"parent_record_offset":2608,"ancestor_record_offset":1944,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3816,"parent_record_offset":3788,"ancestor_record_offset":1944,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3788,"parent_record_offset":3752,"ancestor_record_offset":1944,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":6652,"parent_record_offset":6620,"ancestor_record_offset":1944,"inlinee":22227,"source_line":446,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3752,"parent_record_offset":3720,"ancestor_record_offset":1944,"inlinee":22756,"source_line":541,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":5008,"parent_record_offset":4948,"ancestor_record_offset":1944,"inlinee":22211,"source_line":523,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":11672,"parent_record_offset":11580,"ancestor_record_offset":8600,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3496,"parent_record_offset":3468,"ancestor_record_offset":1944,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":11616,"parent_record_offset":11580,"ancestor_record_offset":8600,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":2414,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":1944,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":2414,"record_offset":5340,"parent_record_offset":5272,"ancestor_record_offset":1944,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":2414,"record_offset":12172,"parent_record_offset":11728,"ancestor_record_offset":8600,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$65::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":2414,"record_offset":12140,"parent_record_offset":12116,"ancestor_record_offset":8600,"inlinee":22963,"source_line":1123,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] lazy_static::lazy::Lazy::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lazy_static-1.4.0/src/inline_lazy.rs
//     evidence={"module":2414,"record_offset":5100,"parent_record_offset":5068,"ancestor_record_offset":1944,"inlinee":17117,"source_line":26,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2414,"record_offset":3888,"parent_record_offset":3852,"ancestor_record_offset":1944,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\once.rs
//     evidence={"module":2414,"record_offset":5136,"parent_record_offset":5100,"ancestor_record_offset":1944,"inlinee":17118,"source_line":139,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::queue::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\once\queue.rs
//     evidence={"module":2414,"record_offset":5172,"parent_record_offset":5136,"ancestor_record_offset":1944,"inlinee":6263,"source_line":117,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":2414,"record_offset":5036,"parent_record_offset":4948,"ancestor_record_offset":1944,"inlinee":83028,"source_line":135,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref::__stability
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":2414,"record_offset":5068,"parent_record_offset":5036,"ancestor_record_offset":1944,"inlinee":83026,"source_line":140,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::fish::fish::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":2414,"record_offset":12964,"parent_record_offset":12932,"ancestor_record_offset":8600,"inlinee":82976,"source_line":194,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":2414,"record_offset":12200,"parent_record_offset":11728,"ancestor_record_offset":8600,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":2414,"record_offset":11728,"parent_record_offset":11580,"ancestor_record_offset":8600,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 325
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2414,"record_offset":8384,"rva":22716192,"va":5391425312}
// 编译实例 RVAs: 0x15a9f20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11548547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391425312,"raw_hex":"e88337b000","size":5,"soft_fail":false,"target_va":5402978944,"va":5391430392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::quat::Quat::from_rotation_arc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14079314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391425312,"raw_hex":"e8ae2a29ff","size":5,"soft_fail":false,"target_va":5377348352,"va":5391427661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::once::queue::Once::call
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11545441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391425312,"raw_hex":"e8612bb000","size":5,"soft_fail":false,"target_va":5402975712,"va":5391430266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_critters::boids::Boid::random_vel_change
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391425312,"raw_hex":"e854edffff","size":5,"soft_fail":false,"target_va":5391420704,"va":5391425479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-14487542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391425312,"raw_hex":"e80af022ff","size":5,"soft_fail":false,"target_va":5376939952,"va":5391427489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14485557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391425312,"raw_hex":"e8cbf722ff","size":5,"soft_fail":false,"target_va":5376939968,"va":5391425520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_vec3
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14486755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391425312,"raw_hex":"e81df322ff","size":5,"soft_fail":false,"target_va":5376940208,"va":5391426958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2414,"record_offset":15808,"parent_record_offset":8384,"ancestor_record_offset":8384,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::clamp_length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":16092,"parent_record_offset":8384,"ancestor_record_offset":8384,"inlinee":83040,"source_line":793,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":15836,"parent_record_offset":8384,"ancestor_record_offset":8384,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":16372,"parent_record_offset":8384,"ancestor_record_offset":8384,"inlinee":21853,"source_line":412,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":16576,"parent_record_offset":8384,"ancestor_record_offset":8384,"inlinee":22755,"source_line":558,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":18132,"parent_record_offset":8384,"ancestor_record_offset":8384,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$18::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":17804,"parent_record_offset":8384,"ancestor_record_offset":8384,"inlinee":25913,"source_line":1146,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$33::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":17948,"parent_record_offset":8384,"ancestor_record_offset":8384,"inlinee":14666,"source_line":1299,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":16488,"parent_record_offset":8384,"ancestor_record_offset":8384,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] system_critters::boids::Boid::apply_rules
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":2414,"record_offset":8600,"parent_record_offset":8384,"ancestor_record_offset":8384,"inlinee":83038,"source_line":182,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] system_critters::fish::fish::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":2414,"record_offset":16772,"parent_record_offset":8384,"ancestor_record_offset":8384,"inlinee":82976,"source_line":194,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2414,"record_offset":16032,"parent_record_offset":8384,"ancestor_record_offset":8384,"inlinee":21527,"source_line":230,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2414,"record_offset":12116,"parent_record_offset":11728,"ancestor_record_offset":8384,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":14248,"parent_record_offset":14008,"ancestor_record_offset":8384,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":14156,"parent_record_offset":14100,"ancestor_record_offset":8384,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":14048,"parent_record_offset":14008,"ancestor_record_offset":8384,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":14336,"parent_record_offset":14276,"ancestor_record_offset":8384,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":14392,"parent_record_offset":14276,"ancestor_record_offset":8384,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":16424,"parent_record_offset":16396,"ancestor_record_offset":8384,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":9644,"parent_record_offset":9616,"ancestor_record_offset":8384,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":16396,"parent_record_offset":16372,"ancestor_record_offset":8384,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":11308,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":11280,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2414,"record_offset":15684,"parent_record_offset":15632,"ancestor_record_offset":8384,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2414,"record_offset":8992,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":9708,"parent_record_offset":9680,"ancestor_record_offset":8384,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":10264,"parent_record_offset":10160,"ancestor_record_offset":8384,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":9552,"parent_record_offset":9520,"ancestor_record_offset":8384,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$286::neg
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":17980,"parent_record_offset":17948,"ancestor_record_offset":8384,"inlinee":14668,"source_line":694,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":8916,"parent_record_offset":8876,"ancestor_record_offset":8384,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$336::sub_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":9800,"parent_record_offset":9768,"ancestor_record_offset":8384,"inlinee":25009,"source_line":828,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$392::div_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":9888,"parent_record_offset":9860,"ancestor_record_offset":8384,"inlinee":13202,"source_line":943,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":9204,"parent_record_offset":9164,"ancestor_record_offset":8384,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$0::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2414,"record_offset":12932,"parent_record_offset":12600,"ancestor_record_offset":8384,"inlinee":33676,"source_line":261,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2414,"record_offset":14572,"parent_record_offset":14548,"ancestor_record_offset":8384,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2414,"record_offset":9112,"parent_record_offset":9044,"ancestor_record_offset":8384,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2414,"record_offset":14548,"parent_record_offset":14512,"ancestor_record_offset":8384,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2414,"record_offset":9044,"parent_record_offset":8992,"ancestor_record_offset":8384,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2414,"record_offset":12856,"parent_record_offset":12832,"ancestor_record_offset":8384,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2414,"record_offset":12880,"parent_record_offset":12856,"ancestor_record_offset":8384,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::impl$3::load_index_unchecked
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":2414,"record_offset":12236,"parent_record_offset":12200,"ancestor_record_offset":8384,"inlinee":21432,"source_line":524,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":2414,"record_offset":15444,"parent_record_offset":15356,"ancestor_record_offset":8384,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":2414,"record_offset":14008,"parent_record_offset":12600,"ancestor_record_offset":8384,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2414,"record_offset":14276,"parent_record_offset":14008,"ancestor_record_offset":8384,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":12088,"parent_record_offset":11728,"ancestor_record_offset":8384,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":11916,"parent_record_offset":11728,"ancestor_record_offset":8384,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":11940,"parent_record_offset":11916,"ancestor_record_offset":8384,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":11860,"parent_record_offset":11728,"ancestor_record_offset":8384,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":12504,"parent_record_offset":11728,"ancestor_record_offset":8384,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":11804,"parent_record_offset":11728,"ancestor_record_offset":8384,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2414,"record_offset":12032,"parent_record_offset":11728,"ancestor_record_offset":8384,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":15356,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":59386,"source_line":808,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":11052,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":9300,"parent_record_offset":9272,"ancestor_record_offset":8384,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":10480,"parent_record_offset":10452,"ancestor_record_offset":8384,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":10452,"parent_record_offset":10416,"ancestor_record_offset":8384,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":9272,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":22227,"source_line":446,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":10132,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":11392,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":10416,"parent_record_offset":10384,"ancestor_record_offset":8384,"inlinee":22756,"source_line":541,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":10384,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":22755,"source_line":558,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":12672,"parent_record_offset":12600,"ancestor_record_offset":8384,"inlinee":22211,"source_line":523,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":11672,"parent_record_offset":11580,"ancestor_record_offset":8384,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":8876,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":9164,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$18::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":9768,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":25913,"source_line":1146,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":9520,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$5::div_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":9860,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":13200,"source_line":1005,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":9680,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":2414,"record_offset":14100,"parent_record_offset":14076,"ancestor_record_offset":8384,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":2414,"record_offset":14076,"parent_record_offset":14008,"ancestor_record_offset":8384,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":2414,"record_offset":12172,"parent_record_offset":11728,"ancestor_record_offset":8384,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$65::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":2414,"record_offset":12140,"parent_record_offset":12116,"ancestor_record_offset":8384,"inlinee":22963,"source_line":1123,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] lazy_static::lazy::Lazy::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lazy_static-1.4.0/src/inline_lazy.rs
//     evidence={"module":2414,"record_offset":12760,"parent_record_offset":12732,"ancestor_record_offset":8384,"inlinee":17117,"source_line":26,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2414,"record_offset":11336,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2414,"record_offset":9460,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\once.rs
//     evidence={"module":2414,"record_offset":12796,"parent_record_offset":12760,"ancestor_record_offset":8384,"inlinee":17118,"source_line":139,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::queue::Once::is_completed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\once\queue.rs
//     evidence={"module":2414,"record_offset":12832,"parent_record_offset":12796,"ancestor_record_offset":8384,"inlinee":6263,"source_line":117,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::Boid::unobstructed_dir
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":2414,"record_offset":12600,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":83039,"source_line":152,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":2414,"record_offset":12704,"parent_record_offset":12600,"ancestor_record_offset":8384,"inlinee":83028,"source_line":135,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref::__stability
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":2414,"record_offset":12732,"parent_record_offset":12704,"ancestor_record_offset":8384,"inlinee":83026,"source_line":140,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::fish::fish::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":2414,"record_offset":11580,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":82976,"source_line":194,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":2414,"record_offset":12200,"parent_record_offset":11728,"ancestor_record_offset":8384,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":2414,"record_offset":11728,"parent_record_offset":11580,"ancestor_record_offset":8384,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2414,"record_offset":9616,"parent_record_offset":8600,"ancestor_record_offset":8384,"inlinee":21527,"source_line":230,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 325
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2414,"record_offset":1716,"rva":22712512,"va":5391421632}
// 编译实例 RVAs: 0x15a90c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14159f8f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: boids.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-41130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391421632,"raw_hex":"e8565fffff","size":5,"soft_fail":false,"target_va":5391382768,"va":5391423893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11553643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391421632,"raw_hex":"e86b4bb000","size":5,"soft_fail":false,"target_va":5402978944,"va":5391425296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::quat::Quat::from_rotation_arc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14075232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391421632,"raw_hex":"e8a03a29ff","size":5,"soft_fail":false,"target_va":5377348352,"va":5391423579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::once::queue::Once::call
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11550534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391421632,"raw_hex":"e8463fb000","size":5,"soft_fail":false,"target_va":5402975712,"va":5391425173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_critters::boids::Boid::random_vel_change
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391421632,"raw_hex":"e8a3fbffff","size":5,"soft_fail":false,"target_va":5391420704,"va":5391421816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_critters::wren_boids::sim_wren_boids::closure$3
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391421632,"raw_hex":"e8091b0000","size":5,"soft_fail":false,"target_va":5391430400,"va":5391423474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14484563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391421632,"raw_hex":"e8adfb22ff","size":5,"soft_fail":false,"target_va":5376939952,"va":5391424510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14481888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391421632,"raw_hex":"e8200623ff","size":5,"soft_fail":false,"target_va":5376939968,"va":5391421851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_vec3
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14483110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391421632,"raw_hex":"e85a0123ff","size":5,"soft_fail":false,"target_va":5376940208,"va":5391423313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2414,"record_offset":7072,"parent_record_offset":1716,"ancestor_record_offset":1716,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::clamp_length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":7356,"parent_record_offset":1716,"ancestor_record_offset":1716,"inlinee":83040,"source_line":793,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":7100,"parent_record_offset":1716,"ancestor_record_offset":1716,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":7636,"parent_record_offset":1716,"ancestor_record_offset":1716,"inlinee":21853,"source_line":412,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":7840,"parent_record_offset":1716,"ancestor_record_offset":1716,"inlinee":22755,"source_line":558,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":8292,"parent_record_offset":1716,"ancestor_record_offset":1716,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$18::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":8028,"parent_record_offset":1716,"ancestor_record_offset":1716,"inlinee":25913,"source_line":1146,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$33::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":8116,"parent_record_offset":1716,"ancestor_record_offset":1716,"inlinee":14666,"source_line":1299,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":7752,"parent_record_offset":1716,"ancestor_record_offset":1716,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] system_critters::boids::Boid::apply_rules
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":2414,"record_offset":1944,"parent_record_offset":1716,"ancestor_record_offset":1716,"inlinee":83038,"source_line":182,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2414,"record_offset":7296,"parent_record_offset":1716,"ancestor_record_offset":1716,"inlinee":21527,"source_line":230,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":5512,"parent_record_offset":5272,"ancestor_record_offset":1716,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":5420,"parent_record_offset":5364,"ancestor_record_offset":1716,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":5312,"parent_record_offset":5272,"ancestor_record_offset":1716,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":5600,"parent_record_offset":5540,"ancestor_record_offset":1716,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2414,"record_offset":5656,"parent_record_offset":5540,"ancestor_record_offset":1716,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":7688,"parent_record_offset":7660,"ancestor_record_offset":1716,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":2980,"parent_record_offset":2952,"ancestor_record_offset":1716,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":7660,"parent_record_offset":7636,"ancestor_record_offset":1716,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":4644,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2414,"record_offset":4616,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2414,"record_offset":6948,"parent_record_offset":6896,"ancestor_record_offset":1716,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2414,"record_offset":2328,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":3044,"parent_record_offset":3016,"ancestor_record_offset":1716,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":3600,"parent_record_offset":3496,"ancestor_record_offset":1716,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":2888,"parent_record_offset":2856,"ancestor_record_offset":1716,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$286::neg
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":8144,"parent_record_offset":8116,"ancestor_record_offset":1716,"inlinee":14668,"source_line":694,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":2252,"parent_record_offset":2212,"ancestor_record_offset":1716,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$336::sub_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":3136,"parent_record_offset":3104,"ancestor_record_offset":1716,"inlinee":25009,"source_line":828,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$392::div_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":3224,"parent_record_offset":3196,"ancestor_record_offset":1716,"inlinee":13202,"source_line":943,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2414,"record_offset":2540,"parent_record_offset":2500,"ancestor_record_offset":1716,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2414,"record_offset":5836,"parent_record_offset":5812,"ancestor_record_offset":1716,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2414,"record_offset":2448,"parent_record_offset":2380,"ancestor_record_offset":1716,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2414,"record_offset":5812,"parent_record_offset":5776,"ancestor_record_offset":1716,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2414,"record_offset":2380,"parent_record_offset":2328,"ancestor_record_offset":1716,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2414,"record_offset":5196,"parent_record_offset":5172,"ancestor_record_offset":1716,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2414,"record_offset":5220,"parent_record_offset":5196,"ancestor_record_offset":1716,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":2414,"record_offset":6708,"parent_record_offset":6620,"ancestor_record_offset":1716,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":2414,"record_offset":5272,"parent_record_offset":4948,"ancestor_record_offset":1716,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2414,"record_offset":5540,"parent_record_offset":5272,"ancestor_record_offset":1716,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":6620,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":59386,"source_line":808,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":4388,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":2636,"parent_record_offset":2608,"ancestor_record_offset":1716,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3816,"parent_record_offset":3788,"ancestor_record_offset":1716,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3788,"parent_record_offset":3752,"ancestor_record_offset":1716,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":2608,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":22227,"source_line":446,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3468,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":4728,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3752,"parent_record_offset":3720,"ancestor_record_offset":1716,"inlinee":22756,"source_line":541,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3720,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":22755,"source_line":558,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":5008,"parent_record_offset":4948,"ancestor_record_offset":1716,"inlinee":22211,"source_line":523,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":6980,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":2212,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":2500,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$18::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3104,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":25913,"source_line":1146,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":2856,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$5::div_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3196,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":13200,"source_line":1005,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2414,"record_offset":3016,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":2414,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":1716,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":2414,"record_offset":5340,"parent_record_offset":5272,"ancestor_record_offset":1716,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] lazy_static::lazy::Lazy::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lazy_static-1.4.0/src/inline_lazy.rs
//     evidence={"module":2414,"record_offset":5100,"parent_record_offset":5068,"ancestor_record_offset":1716,"inlinee":17117,"source_line":26,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2414,"record_offset":4672,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2414,"record_offset":2796,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\once.rs
//     evidence={"module":2414,"record_offset":5136,"parent_record_offset":5100,"ancestor_record_offset":1716,"inlinee":17118,"source_line":139,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::queue::Once::is_completed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\once\queue.rs
//     evidence={"module":2414,"record_offset":5172,"parent_record_offset":5136,"ancestor_record_offset":1716,"inlinee":6263,"source_line":117,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::Boid::unobstructed_dir
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":2414,"record_offset":4948,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":83039,"source_line":152,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":2414,"record_offset":5036,"parent_record_offset":4948,"ancestor_record_offset":1716,"inlinee":83028,"source_line":135,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref::__stability
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":2414,"record_offset":5068,"parent_record_offset":5036,"ancestor_record_offset":1716,"inlinee":83026,"source_line":140,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   [pdb_inline_transitive] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2414,"record_offset":2952,"parent_record_offset":1944,"ancestor_record_offset":1716,"inlinee":21527,"source_line":230,"archive":"_pdb/A/modules/02414.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::random_vel_change
// 记录形式: compiled_procedure; instances=1
// 源码行: 384
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2413,"record_offset":2412,"rva":22711584,"va":5391420704}
// 编译实例 RVAs: 0x15a8d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-5570734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391420704,"raw_hex":"e852ffaaff","size":5,"soft_fail":false,"target_va":5385850032,"va":5391420761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::quat::Quat::from_rotation_arc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14072643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391420704,"raw_hex":"e8bd4429ff","size":5,"soft_fail":false,"target_va":5377348352,"va":5391420990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14480906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391420704,"raw_hex":"e8f60923ff","size":5,"soft_fail":false,"target_va":5376939952,"va":5391420853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_vec3
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14480718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391420704,"raw_hex":"e8b20a23ff","size":5,"soft_fail":false,"target_va":5376940208,"va":5391420921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::uniform_sample_cone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14479962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391420704,"raw_hex":"e8a60d23ff","size":5,"soft_fail":false,"target_va":5376940848,"va":5391420805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":2413,"record_offset":2544,"parent_record_offset":2412,"ancestor_record_offset":2412,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/02413.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2413,"record_offset":2568,"parent_record_offset":2544,"ancestor_record_offset":2412,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02413.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2413,"record_offset":3068,"parent_record_offset":2652,"ancestor_record_offset":2412,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/02413.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2413,"record_offset":2976,"parent_record_offset":2920,"ancestor_record_offset":2412,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/02413.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2413,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":2412,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/02413.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2413,"record_offset":2780,"parent_record_offset":2696,"ancestor_record_offset":2412,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/02413.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2413,"record_offset":2840,"parent_record_offset":2652,"ancestor_record_offset":2412,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/02413.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":2413,"record_offset":2652,"parent_record_offset":2544,"ancestor_record_offset":2412,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/02413.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2413,"record_offset":3096,"parent_record_offset":2652,"ancestor_record_offset":2412,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/02413.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2413,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":2412,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/02413.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2413,"record_offset":2592,"parent_record_offset":2568,"ancestor_record_offset":2412,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/02413.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":2413,"record_offset":2724,"parent_record_offset":2696,"ancestor_record_offset":2412,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/02413.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":2413,"record_offset":2696,"parent_record_offset":2652,"ancestor_record_offset":2412,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/02413.jsonl.gz"}
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
// 完整结构及原始类型记录: boids.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_critters::boids::Boid
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<system_critters::boids::Boid,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["system_critters::boids::Boid", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3>", "constructor": "system_critters::boids::Boid::frame_update", "arguments": ["system_critters::wren_boids::sim_wren_boids::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0>", "constructor": "system_critters::boids::Boid::frame_update", "arguments": ["system_critters::fish::fish::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_critters::boids::BoidAux
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::Drain<system_critters::boids::BoidAux,alloc::alloc::Global>", "constructor": "alloc::vec::drain::Drain", "arguments": ["system_critters::boids::BoidAux", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::drain::Drain<system_critters::boids::BoidAux,alloc::alloc::Global>,system_critters::boids::impl$0::drain::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::drain::Drain<system_critters::boids::BoidAux,alloc::alloc::Global>", "system_critters::boids::impl$0::drain::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::drain::Drain<system_critters::boids::BoidAux,alloc::alloc::Global>,system_critters::boids::impl$0::drain::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::drain::Drain<system_critters::boids::BoidAux,alloc::alloc::Global>,system_critters::boids::impl$0::drain::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<system_critters::boids::BoidAux,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["system_critters::boids::BoidAux", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_critters::boids::BoidFlock
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "system_critters::boids::BoidFlock::remove<system_critters::fish::fish::closure_env$1>", "constructor": "system_critters::boids::BoidFlock::remove", "arguments": ["system_critters::fish::fish::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: boids.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: boids.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] array$<glam::f32::sse2::vec3a::Vec3A,32> (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::apply_rules
// 信息 ID: a33462a4b08ae5f7e48b865c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3>; module=2414, inline_record=1944; 不是独立调用边。
// [PDB 内联宿主] system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0>; module=2414, inline_record=8600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0>
// 信息 ID: e65841c72e495067fd980d55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_critters::boids::Boid::frame_update -> ["system_critters::fish::fish::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::fish::fish @ 0xa51657 -> 0x15a9f20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3>
// 信息 ID: 5357847567b66d1f8bf33d6d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_critters::boids::Boid::frame_update -> ["system_critters::wren_boids::sim_wren_boids::closure_env$3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids @ 0x1b41811 -> 0x15a90c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::new
// 信息 ID: 25d0fee0b701fb276c79bab6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::random_vel_change
// 信息 ID: 12e01a87b13a404f65153fad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3> @ 0x15a9178 -> 0x15a8d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0> @ 0x15a9fc7 -> 0x15a8d20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::unobstructed_dir
// 信息 ID: aec91350b946570d802fa799
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3>; module=2414, inline_record=4948; 不是独立调用边。
// [PDB 内联宿主] system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0>; module=2414, inline_record=12600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::clone_state
// 信息 ID: 1d3bff3b9c0049ef09c95234
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::fish::fish; module=1012, inline_record=5028; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3623, inline_record=14724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::drain
// 信息 ID: a621b3f1135bc185e7c5e784
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3623, inline_record=7648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::iter_mut
// 信息 ID: bbd974277bbbdfa3e869d6ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::fish::fish; module=1012, inline_record=5916; 不是独立调用边。
// [PDB 内联宿主] system_critters::fish::fish; module=1012, inline_record=12896; 不是独立调用边。
// [PDB 内联宿主] system_critters::fish::fish; module=1012, inline_record=13184; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3623, inline_record=15564; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3623, inline_record=22424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::len
// 信息 ID: db7d47ae8f381760da33bbc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::fish::fish; module=1012, inline_record=11988; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3623, inline_record=14696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::push
// 信息 ID: 79640baa805a81335713e34b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::fish::fish; module=1012, inline_record=4728; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3623, inline_record=7352; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3623, inline_record=13268; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3623, inline_record=21944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::remove<system_critters::fish::fish::closure_env$1>
// 信息 ID: cce0cc1a876e884c86c749ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_critters::boids::BoidFlock::remove -> ["system_critters::fish::fish::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::fish::fish @ 0xa51f49 -> 0x15a8ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids @ 0x1b4270f -> 0x15a8ef0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::impl$5::clone
// 信息 ID: 87aaeb6df5f43cefe574f2b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::fish::fish; module=1012, inline_record=5808; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3623, inline_record=15484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
