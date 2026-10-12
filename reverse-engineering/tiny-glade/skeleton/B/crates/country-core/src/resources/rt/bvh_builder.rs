// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/rt/bvh_builder.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: abe0cdd0b70902690cf11f0d8853a95eaeae2752907976d20ce33601d5434b34
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: bvh_builder.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 493, 581, 1451, 1585, 1610, 1681
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh_builder::RtBvhBuilderGlue::is_idle
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6ACF
// 首条位置: {"module":493,"inlinee":27343}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":493,"record_offset":29744,"parent_record_offset":29716,"ancestor_record_offset":29716,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":493,"record_offset":29768,"parent_record_offset":29744,"ancestor_record_offset":29716,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh_builder::bvh_builder_worker
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":581,"record_offset":73524,"rva":11685200,"va":5380394320}
// 编译实例 RVAs: 0xb24d50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2248469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380394320,"raw_hex":"e8154f2200","size":5,"soft_fail":false,"target_va":5382644528,"va":5380396054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400524e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11349237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380394320,"raw_hex":"e80bd352ff","size":5,"soft_fail":false,"target_va":5369046240,"va":5380395472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d0df0
//     owner=unresolved_namespace; count=1
//     target_aliases=21 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5589080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380394320,"raw_hex":"e8a8b7aaff","size":5,"soft_fail":false,"target_va":5374807536,"va":5380396611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5313517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380394320,"raw_hex":"e813ecaeff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380395016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4879884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380394320,"raw_hex":"e8f489b5ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5380395911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088b0c0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2729262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380394320,"raw_hex":"e8d25ad6ff","size":5,"soft_fail":false,"target_va":5377667264,"va":5380396521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c7a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t226102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380394320,"raw_hex":"e836730300","size":5,"soft_fail":false,"target_va":5380622240,"va":5380396133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14130d1b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8290951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380394320,"raw_hex":"e887827e00","size":5,"soft_fail":false,"target_va":5388685744,"va":5380394788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>,std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> > > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380394320,"raw_hex":"e87eb7ffff","size":5,"soft_fail":false,"target_va":5380377856,"va":5380396413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::rt::bvh::build_gpu_bvh
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-95472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380394320,"raw_hex":"e8108bfeff","size":5,"soft_fail":false,"target_va":5380299344,"va":5380394811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t31143101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380394320,"raw_hex":"e8bd34db01","size":5,"soft_fail":false,"target_va":5411538608,"va":5380395502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t31144753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380394320,"raw_hex":"e8313bdb01","size":5,"soft_fail":false,"target_va":5411540288,"va":5380395530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::write_contended
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31144487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380394320,"raw_hex":"e8273adb01","size":5,"soft_fail":false,"target_va":5411539984,"va":5380395492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2296671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380394320,"raw_hex":"e85f0b2300","size":5,"soft_fail":false,"target_va":5382691200,"va":5380394524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":581,"record_offset":73712,"parent_record_offset":73524,"ancestor_record_offset":73524,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":581,"record_offset":79792,"parent_record_offset":73524,"ancestor_record_offset":73524,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":581,"record_offset":75700,"parent_record_offset":73524,"ancestor_record_offset":73524,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":581,"record_offset":75636,"parent_record_offset":73524,"ancestor_record_offset":73524,"inlinee":15461,"source_line":911,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":581,"record_offset":74312,"parent_record_offset":73524,"ancestor_record_offset":73524,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":581,"record_offset":77520,"parent_record_offset":73524,"ancestor_record_offset":73524,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":581,"record_offset":74196,"parent_record_offset":73524,"ancestor_record_offset":73524,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":581,"record_offset":74772,"parent_record_offset":73524,"ancestor_record_offset":73524,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":581,"record_offset":79824,"parent_record_offset":73524,"ancestor_record_offset":73524,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":581,"record_offset":79708,"parent_record_offset":73524,"ancestor_record_offset":73524,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":581,"record_offset":75196,"parent_record_offset":73524,"ancestor_record_offset":73524,"inlinee":4948,"source_line":440,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::rwlock::RwLock::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":581,"record_offset":73740,"parent_record_offset":73524,"ancestor_record_offset":73524,"inlinee":13083,"source_line":457,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":581,"record_offset":76988,"parent_record_offset":76964,"ancestor_record_offset":73524,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":581,"record_offset":76964,"parent_record_offset":76872,"ancestor_record_offset":73524,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":581,"record_offset":76900,"parent_record_offset":76872,"ancestor_record_offset":73524,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":581,"record_offset":76872,"parent_record_offset":76848,"ancestor_record_offset":73524,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":581,"record_offset":76848,"parent_record_offset":76824,"ancestor_record_offset":73524,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":581,"record_offset":80508,"parent_record_offset":80472,"ancestor_record_offset":73524,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":581,"record_offset":80472,"parent_record_offset":80448,"ancestor_record_offset":73524,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":581,"record_offset":75724,"parent_record_offset":75700,"ancestor_record_offset":73524,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":581,"record_offset":75488,"parent_record_offset":75460,"ancestor_record_offset":73524,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":581,"record_offset":75548,"parent_record_offset":75516,"ancestor_record_offset":73524,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":581,"record_offset":75664,"parent_record_offset":75636,"ancestor_record_offset":73524,"inlinee":15463,"source_line":23,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":581,"record_offset":75748,"parent_record_offset":75724,"ancestor_record_offset":73524,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":581,"record_offset":74220,"parent_record_offset":74196,"ancestor_record_offset":73524,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":581,"record_offset":76932,"parent_record_offset":76900,"ancestor_record_offset":73524,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":581,"record_offset":79880,"parent_record_offset":79848,"ancestor_record_offset":73524,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":581,"record_offset":79908,"parent_record_offset":79848,"ancestor_record_offset":73524,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":581,"record_offset":75432,"parent_record_offset":75400,"ancestor_record_offset":73524,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":581,"record_offset":74344,"parent_record_offset":74312,"ancestor_record_offset":73524,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":581,"record_offset":80540,"parent_record_offset":80508,"ancestor_record_offset":73524,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":581,"record_offset":74248,"parent_record_offset":74220,"ancestor_record_offset":73524,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":581,"record_offset":74276,"parent_record_offset":74220,"ancestor_record_offset":73524,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":581,"record_offset":74116,"parent_record_offset":74088,"ancestor_record_offset":73524,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":581,"record_offset":74604,"parent_record_offset":74416,"ancestor_record_offset":73524,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":581,"record_offset":73808,"parent_record_offset":73776,"ancestor_record_offset":73524,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":581,"record_offset":74700,"parent_record_offset":74664,"ancestor_record_offset":73524,"inlinee":7030,"source_line":2986,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":581,"record_offset":74020,"parent_record_offset":73988,"ancestor_record_offset":73524,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":581,"record_offset":73832,"parent_record_offset":73808,"ancestor_record_offset":73524,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":581,"record_offset":74044,"parent_record_offset":74020,"ancestor_record_offset":73524,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":581,"record_offset":74628,"parent_record_offset":74604,"ancestor_record_offset":73524,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":581,"record_offset":74724,"parent_record_offset":74700,"ancestor_record_offset":73524,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":581,"record_offset":80600,"parent_record_offset":80472,"ancestor_record_offset":73524,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":581,"record_offset":75460,"parent_record_offset":75400,"ancestor_record_offset":73524,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":581,"record_offset":75516,"parent_record_offset":75460,"ancestor_record_offset":73524,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":581,"record_offset":75252,"parent_record_offset":75224,"ancestor_record_offset":73524,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":581,"record_offset":75400,"parent_record_offset":75308,"ancestor_record_offset":73524,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":581,"record_offset":75280,"parent_record_offset":75252,"ancestor_record_offset":73524,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":581,"record_offset":75372,"parent_record_offset":75308,"ancestor_record_offset":73524,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":581,"record_offset":75344,"parent_record_offset":75308,"ancestor_record_offset":73524,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":581,"record_offset":75308,"parent_record_offset":75280,"ancestor_record_offset":73524,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":581,"record_offset":79664,"parent_record_offset":79640,"ancestor_record_offset":73524,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":581,"record_offset":79936,"parent_record_offset":79848,"ancestor_record_offset":73524,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":581,"record_offset":79848,"parent_record_offset":79824,"ancestor_record_offset":73524,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":581,"record_offset":79960,"parent_record_offset":79936,"ancestor_record_offset":73524,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":581,"record_offset":75224,"parent_record_offset":75196,"ancestor_record_offset":73524,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":581,"record_offset":73988,"parent_record_offset":73960,"ancestor_record_offset":73524,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":581,"record_offset":73960,"parent_record_offset":73932,"ancestor_record_offset":73524,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":581,"record_offset":74416,"parent_record_offset":74376,"ancestor_record_offset":73524,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":581,"record_offset":74088,"parent_record_offset":73896,"ancestor_record_offset":73524,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":581,"record_offset":73896,"parent_record_offset":73868,"ancestor_record_offset":73524,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockWriteGuard::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":581,"record_offset":73868,"parent_record_offset":73740,"ancestor_record_offset":73524,"inlinee":13086,"source_line":694,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$32::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":581,"record_offset":74376,"parent_record_offset":74344,"ancestor_record_offset":73524,"inlinee":13088,"source_line":826,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":581,"record_offset":73776,"parent_record_offset":73740,"ancestor_record_offset":73524,"inlinee":13084,"source_line":172,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write_unlock
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":581,"record_offset":74664,"parent_record_offset":74376,"ancestor_record_offset":73524,"inlinee":13089,"source_line":182,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":581,"record_offset":73932,"parent_record_offset":73896,"ancestor_record_offset":73524,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00581.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120A8
// 首条位置: {"module":1610,"inlinee":73896}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1610,"record_offset":108572,"parent_record_offset":108488,"ancestor_record_offset":108488,"inlinee":30845,"source_line":1235,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::rt::DirtyCow::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/mod.rs
//     evidence={"module":1610,"record_offset":108516,"parent_record_offset":108488,"ancestor_record_offset":108488,"inlinee":26291,"source_line":99,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1610,"record_offset":108540,"parent_record_offset":108516,"ancestor_record_offset":108488,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":108648,"parent_record_offset":108624,"ancestor_record_offset":108488,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1610,"record_offset":108624,"parent_record_offset":108596,"ancestor_record_offset":108488,"inlinee":6336,"source_line":2001,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExact::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1610,"record_offset":108596,"parent_record_offset":108572,"ancestor_record_offset":108488,"inlinee":30847,"source_line":1866,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11568
// 首条位置: {"module":1451,"inlinee":71016}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::transform_point3
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":1585,"record_offset":55048,"parent_record_offset":55004,"ancestor_record_offset":55004,"inlinee":25215,"source_line":372,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1585,"record_offset":55188,"parent_record_offset":55092,"ancestor_record_offset":55004,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1585,"record_offset":55160,"parent_record_offset":55092,"ancestor_record_offset":55004,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1585,"record_offset":55128,"parent_record_offset":55092,"ancestor_record_offset":55004,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1585,"record_offset":55092,"parent_record_offset":55048,"ancestor_record_offset":55004,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1585,"record_offset":55404,"parent_record_offset":55048,"ancestor_record_offset":55004,"inlinee":24484,"source_line":1323,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11564
// 首条位置: {"module":1451,"inlinee":71012}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::iter::impl$182::all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1451,"record_offset":548,"parent_record_offset":520,"ancestor_record_offset":520,"inlinee":4615,"source_line":262,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1451,"record_offset":648,"parent_record_offset":620,"ancestor_record_offset":520,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1451,"record_offset":620,"parent_record_offset":596,"ancestor_record_offset":520,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/bvh_builder.rs
//     evidence={"module":1451,"record_offset":572,"parent_record_offset":548,"ancestor_record_offset":520,"inlinee":71014,"source_line":89,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":596,"parent_record_offset":572,"ancestor_record_offset":520,"inlinee":25834,"source_line":420,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11566
// 首条位置: {"module":1451,"inlinee":71014}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":596,"parent_record_offset":572,"ancestor_record_offset":572,"inlinee":25834,"source_line":420,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1451,"record_offset":648,"parent_record_offset":620,"ancestor_record_offset":572,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1451,"record_offset":620,"parent_record_offset":596,"ancestor_record_offset":572,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/01451.jsonl.gz"}
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
// 完整结构及原始类型记录: bvh_builder.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::rt::bvh_builder::InputTriangle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>", "constructor": "core::iter::adapters::filter_map::impl$2::next", "arguments": ["country_core::resources::rt::bvh_builder::InputTriangle", "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>", "country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::rt::bvh_builder::InputTriangle", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> >", "constructor": "core::ops::control_flow::ControlFlow", "arguments": ["country_core::resources::rt::bvh_builder::InputTriangle", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >", "constructor": "enum2$", "arguments": ["core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> >", "constructor": "core::iter::traits::iterator::Iterator::find_map::check::closure_env$0", "arguments": ["tuple$<array$<glam::f32::vec3::Vec3,3>,u32>", "country_core::resources::rt::bvh_builder::InputTriangle", "ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$33::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > >", "constructor": "core::iter::adapters::flatten::impl$33::try_fold::flatten::closure_env$0", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >", "tuple$<>", "enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >", "core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$31::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$33::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >", "constructor": "core::iter::adapters::flatten::impl$31::iter_try_fold::flatten::closure_env$0", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >", "tuple$<>", "enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >", "core::iter::adapters::flatten::impl$33::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$31::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$33::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >", "constructor": "core::iter::adapters::map::impl$2::try_fold", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >", "std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >", "country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0", "tuple$<>", "core::iter::adapters::flatten::impl$31::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$33::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >", "enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::rt::bvh_builder::RtBvhBuilderGlue
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::rwlock::RwLock<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>", "constructor": "std::sync::poison::rwlock::RwLock", "arguments": ["country_core::resources::rt::bvh_builder::RtBvhBuilderGlue"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::poison::rwlock::RwLock<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::poison::rwlock::RwLock<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>", "constructor": "std::sync::poison::rwlock::RwLockWriteGuard", "arguments": ["country_core::resources::rt::bvh_builder::RtBvhBuilderGlue"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::rwlock::RwLockReadGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>", "constructor": "std::sync::poison::rwlock::RwLockReadGuard", "arguments": ["country_core::resources::rt::bvh_builder::RtBvhBuilderGlue"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockReadGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >", "constructor": "std::sync::poison::PoisonError", "arguments": ["std::sync::poison::rwlock::RwLockReadGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockReadGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockReadGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >", "constructor": "std::sync::poison::PoisonError", "arguments": ["std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>,std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> > >", "constructor": "core::result::Result", "arguments": ["std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>", "std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>,std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> > > >", "constructor": "enum2$", "arguments": ["core::result::Result<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>,std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>,std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>,std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<std::sync::poison::rwlock::RwLock<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["std::sync::poison::rwlock::RwLock<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::impl$1::fmt<std::sync::poison::rwlock::RwLockReadGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >", "constructor": "std::sync::poison::impl$1::fmt", "arguments": ["std::sync::poison::rwlock::RwLockReadGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::rt::bvh_builder::RtBvhBuilderOutput
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::resources::rt::bvh_builder::RtBvhBuilderOutput>", "constructor": "core::option::Option", "arguments": ["country_core::resources::rt::bvh_builder::RtBvhBuilderOutput"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::resources::rt::bvh_builder::RtBvhBuilderOutput> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::resources::rt::bvh_builder::RtBvhBuilderOutput>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<country_core::resources::rt::bvh_builder::RtBvhBuilderOutput> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<country_core::resources::rt::bvh_builder::RtBvhBuilderOutput> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: bvh_builder.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: bvh_builder.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

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
// 函数: country_core::resources::rt::bvh_builder::RtBvhBuilderGlue::is_idle
// 信息 ID: aa27f937e5b2d6a168e9ac39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::rt::RtBvhBuilderGlueResource::is_idle; module=493, inline_record=29716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh_builder::bvh_builder_worker
// 信息 ID: 3269fb1377f391179d3f76a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$7,tuple$<> > @ 0xd9914 -> 0xb24d50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0
// 信息 ID: cfff634ae6014e20f010cdc3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::rt::bvh_builder::bvh_builder_worker", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0 <- core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$31::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$33::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$31::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$33::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; module=1610, inline_record=108488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure$0
// 信息 ID: 20f712b6097c6dfc19303f01
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::rt::bvh_builder::bvh_builder_worker", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0 <- core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$31::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$33::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$31::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$33::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$31::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$33::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; module=1585, inline_record=55004; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; module=1585, inline_record=56948; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; module=1585, inline_record=58604; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$31::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$33::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; module=1610, inline_record=109076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$1
// 信息 ID: 93304fdb005d8efc8d5e9f42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::rt::bvh_builder::bvh_builder_worker", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<array$<glam::f32::vec3::Vec3,3>,u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1 <- core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1 <- core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$31::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$33::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<array$<glam::f32::vec3::Vec3,3>,u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; module=1451, inline_record=520; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$1::closure$0
// 信息 ID: f8b18cf47624b8b14112df53
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::rt::bvh_builder::bvh_builder_worker", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<array$<glam::f32::vec3::Vec3,3>,u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; module=1451, inline_record=572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
