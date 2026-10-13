// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/rt/bvh_builder.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: abe0cdd0b70902690cf11f0d8853a95eaeae2752907976d20ce33601d5434b34
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: bvh_builder.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 512, 561, 599, 1643, 1682, 1685
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
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
// IPI IDs: 0x73D8
// 首条位置: {"module":512,"inlinee":29656}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":512,"record_offset":65076,"parent_record_offset":65048,"ancestor_record_offset":65048,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":512,"record_offset":65100,"parent_record_offset":65076,"ancestor_record_offset":65048,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00512.jsonl.gz"}
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
// 首条位置: {"module":599,"record_offset":45188,"rva":12863840,"va":5381572960}
// 编译实例 RVAs: 0xc44960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::rwlock::futex::RwLock>::wake_writer_or_readers
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t35042807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381572960,"raw_hex":"e8f7b51602","size":5,"soft_fail":false,"target_va":5416617072,"va":5381574260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::rwlock::futex::RwLock>::write_contended
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t35042543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381572960,"raw_hex":"e8efb41602","size":5,"soft_fail":false,"target_va":5416616768,"va":5381574220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3034730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381572960,"raw_hex":"e86a4e2e00","size":5,"soft_fail":false,"target_va":5384609408,"va":5381574673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-5894934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381572960,"raw_hex":"e8ea0ca6ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381573633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f8c0
//     owner=unresolved_namespace; count=1
//     target_aliases=33 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12343576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381572960,"raw_hex":"e8e8a643ff","size":5,"soft_fail":false,"target_va":5369231552,"va":5381575123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401f1d90
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10826888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381572960,"raw_hex":"e878cb5aff","size":5,"soft_fail":false,"target_va":5370748304,"va":5381575187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140995470
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2817294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381572960,"raw_hex":"e8f202d5ff","size":5,"soft_fail":false,"target_va":5378757744,"va":5381575033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c90b70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t310026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381572960,"raw_hex":"e80abb0400","size":5,"soft_fail":false,"target_va":5381884784,"va":5381574753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c921f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t316402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381572960,"raw_hex":"e8f2d30400","size":5,"soft_fail":false,"target_va":5381890544,"va":5381574137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14160cb90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10256470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381572960,"raw_hex":"e856809c00","size":5,"soft_fail":false,"target_va":5391829904,"va":5381573429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::rt::bvh::build_gpu_bvh
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9157550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381572960,"raw_hex":"e8aebb8b00","size":5,"soft_fail":false,"target_va":5390731008,"va":5381573453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t35046524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381572960,"raw_hex":"e87cc41602","size":5,"soft_fail":false,"target_va":5416619696,"va":5381573167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3155096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381572960,"raw_hex":"e898243000","size":5,"soft_fail":false,"target_va":5384728160,"va":5381573059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":599,"record_offset":45816,"parent_record_offset":45188,"ancestor_record_offset":45188,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":599,"record_offset":52552,"parent_record_offset":45188,"ancestor_record_offset":45188,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":599,"record_offset":47280,"parent_record_offset":45188,"ancestor_record_offset":45188,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::flat_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":599,"record_offset":47216,"parent_record_offset":45188,"ancestor_record_offset":45188,"inlinee":16895,"source_line":1501,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":599,"record_offset":45376,"parent_record_offset":45188,"ancestor_record_offset":45188,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":599,"record_offset":49308,"parent_record_offset":45188,"ancestor_record_offset":45188,"inlinee":11351,"source_line":1943,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":599,"record_offset":46300,"parent_record_offset":45188,"ancestor_record_offset":45188,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":599,"record_offset":46360,"parent_record_offset":45188,"ancestor_record_offset":45188,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":599,"record_offset":52608,"parent_record_offset":45188,"ancestor_record_offset":45188,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":599,"record_offset":52468,"parent_record_offset":45188,"ancestor_record_offset":45188,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":599,"record_offset":46784,"parent_record_offset":45188,"ancestor_record_offset":45188,"inlinee":8400,"source_line":543,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::rwlock::RwLock::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\rwlock.rs
//     evidence={"module":599,"record_offset":45844,"parent_record_offset":45188,"ancestor_record_offset":45188,"inlinee":10572,"source_line":501,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":599,"record_offset":48608,"parent_record_offset":48584,"ancestor_record_offset":45188,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":599,"record_offset":48632,"parent_record_offset":48608,"ancestor_record_offset":45188,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":599,"record_offset":48656,"parent_record_offset":48632,"ancestor_record_offset":45188,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":599,"record_offset":48584,"parent_record_offset":48492,"ancestor_record_offset":45188,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":599,"record_offset":48520,"parent_record_offset":48492,"ancestor_record_offset":45188,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":599,"record_offset":48492,"parent_record_offset":48468,"ancestor_record_offset":45188,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":599,"record_offset":48468,"parent_record_offset":48444,"ancestor_record_offset":45188,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":599,"record_offset":51976,"parent_record_offset":51940,"ancestor_record_offset":45188,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":599,"record_offset":51940,"parent_record_offset":51912,"ancestor_record_offset":45188,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":599,"record_offset":47304,"parent_record_offset":47280,"ancestor_record_offset":45188,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":599,"record_offset":47076,"parent_record_offset":47048,"ancestor_record_offset":45188,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":599,"record_offset":47128,"parent_record_offset":47104,"ancestor_record_offset":45188,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::FlatMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":599,"record_offset":47244,"parent_record_offset":47216,"ancestor_record_offset":45188,"inlinee":16897,"source_line":22,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":599,"record_offset":47328,"parent_record_offset":47304,"ancestor_record_offset":45188,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":599,"record_offset":46324,"parent_record_offset":46300,"ancestor_record_offset":45188,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":599,"record_offset":52216,"parent_record_offset":52188,"ancestor_record_offset":45188,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":599,"record_offset":52244,"parent_record_offset":52216,"ancestor_record_offset":45188,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":599,"record_offset":48552,"parent_record_offset":48520,"ancestor_record_offset":45188,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":599,"record_offset":51568,"parent_record_offset":51544,"ancestor_record_offset":45188,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":599,"record_offset":52576,"parent_record_offset":52552,"ancestor_record_offset":45188,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":599,"record_offset":47020,"parent_record_offset":46988,"ancestor_record_offset":45188,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":599,"record_offset":45408,"parent_record_offset":45376,"ancestor_record_offset":45188,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":599,"record_offset":51836,"parent_record_offset":51812,"ancestor_record_offset":45188,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":599,"record_offset":51688,"parent_record_offset":51664,"ancestor_record_offset":45188,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":599,"record_offset":52008,"parent_record_offset":51976,"ancestor_record_offset":45188,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":599,"record_offset":46220,"parent_record_offset":46192,"ancestor_record_offset":45188,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":599,"record_offset":45744,"parent_record_offset":45572,"ancestor_record_offset":45188,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":599,"record_offset":45912,"parent_record_offset":45880,"ancestor_record_offset":45188,"inlinee":7934,"source_line":3097,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":599,"record_offset":45512,"parent_record_offset":45476,"ancestor_record_offset":45188,"inlinee":4762,"source_line":3165,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":599,"record_offset":45676,"parent_record_offset":45648,"ancestor_record_offset":45188,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":599,"record_offset":45936,"parent_record_offset":45912,"ancestor_record_offset":45188,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":599,"record_offset":45700,"parent_record_offset":45676,"ancestor_record_offset":45188,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":599,"record_offset":45768,"parent_record_offset":45744,"ancestor_record_offset":45188,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":599,"record_offset":45536,"parent_record_offset":45512,"ancestor_record_offset":45188,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":599,"record_offset":52068,"parent_record_offset":51940,"ancestor_record_offset":45188,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":599,"record_offset":51512,"parent_record_offset":51484,"ancestor_record_offset":45188,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":599,"record_offset":51544,"parent_record_offset":51512,"ancestor_record_offset":45188,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":599,"record_offset":51484,"parent_record_offset":51444,"ancestor_record_offset":45188,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":599,"record_offset":47048,"parent_record_offset":46988,"ancestor_record_offset":45188,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":599,"record_offset":47104,"parent_record_offset":47048,"ancestor_record_offset":45188,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":599,"record_offset":46840,"parent_record_offset":46812,"ancestor_record_offset":45188,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":599,"record_offset":51812,"parent_record_offset":51208,"ancestor_record_offset":45188,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":599,"record_offset":51664,"parent_record_offset":51444,"ancestor_record_offset":45188,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":599,"record_offset":46988,"parent_record_offset":46896,"ancestor_record_offset":45188,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":599,"record_offset":51444,"parent_record_offset":51416,"ancestor_record_offset":45188,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":599,"record_offset":46868,"parent_record_offset":46840,"ancestor_record_offset":45188,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":599,"record_offset":52156,"parent_record_offset":52124,"ancestor_record_offset":45188,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":599,"record_offset":46960,"parent_record_offset":46896,"ancestor_record_offset":45188,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":599,"record_offset":46932,"parent_record_offset":46896,"ancestor_record_offset":45188,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":599,"record_offset":51208,"parent_record_offset":51148,"ancestor_record_offset":45188,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":599,"record_offset":51148,"parent_record_offset":51124,"ancestor_record_offset":45188,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":599,"record_offset":52124,"parent_record_offset":51148,"ancestor_record_offset":45188,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":599,"record_offset":51180,"parent_record_offset":51148,"ancestor_record_offset":45188,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":599,"record_offset":46896,"parent_record_offset":46868,"ancestor_record_offset":45188,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":599,"record_offset":52188,"parent_record_offset":52156,"ancestor_record_offset":45188,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":599,"record_offset":51124,"parent_record_offset":51100,"ancestor_record_offset":45188,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":599,"record_offset":51416,"parent_record_offset":51208,"ancestor_record_offset":45188,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":599,"record_offset":52656,"parent_record_offset":52632,"ancestor_record_offset":45188,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":599,"record_offset":52632,"parent_record_offset":52608,"ancestor_record_offset":45188,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":599,"record_offset":52680,"parent_record_offset":52656,"ancestor_record_offset":45188,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":599,"record_offset":46812,"parent_record_offset":46784,"ancestor_record_offset":45188,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":599,"record_offset":45648,"parent_record_offset":45624,"ancestor_record_offset":45188,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":599,"record_offset":45624,"parent_record_offset":45600,"ancestor_record_offset":45188,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":599,"record_offset":45572,"parent_record_offset":45440,"ancestor_record_offset":45188,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":599,"record_offset":46192,"parent_record_offset":46000,"ancestor_record_offset":45188,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":599,"record_offset":46000,"parent_record_offset":45972,"ancestor_record_offset":45188,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockWriteGuard::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\rwlock.rs
//     evidence={"module":599,"record_offset":45972,"parent_record_offset":45844,"ancestor_record_offset":45188,"inlinee":10575,"source_line":820,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$20::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\rwlock.rs
//     evidence={"module":599,"record_offset":45440,"parent_record_offset":45408,"ancestor_record_offset":45188,"inlinee":4764,"source_line":1129,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":599,"record_offset":45880,"parent_record_offset":45844,"ancestor_record_offset":45188,"inlinee":10573,"source_line":172,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write_unlock
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":599,"record_offset":45476,"parent_record_offset":45440,"ancestor_record_offset":45188,"inlinee":4772,"source_line":182,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":599,"record_offset":45600,"parent_record_offset":45572,"ancestor_record_offset":45188,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x143A2
// 首条位置: {"module":1682,"inlinee":82850}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1682,"record_offset":140168,"parent_record_offset":140076,"ancestor_record_offset":140076,"inlinee":27399,"source_line":1242,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::rt::DirtyCow::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/mod.rs
//     evidence={"module":1682,"record_offset":140112,"parent_record_offset":140076,"ancestor_record_offset":140076,"inlinee":28808,"source_line":105,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1682,"record_offset":140136,"parent_record_offset":140112,"ancestor_record_offset":140076,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1682,"record_offset":140252,"parent_record_offset":140228,"ancestor_record_offset":140076,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1682,"record_offset":140228,"parent_record_offset":140196,"ancestor_record_offset":140076,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExact::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1682,"record_offset":140196,"parent_record_offset":140168,"ancestor_record_offset":140076,"inlinee":27401,"source_line":1851,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D37
// 首条位置: {"module":1643,"inlinee":81207}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::transform_point3
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":1643,"record_offset":137456,"parent_record_offset":137420,"ancestor_record_offset":137420,"inlinee":27157,"source_line":392,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":1643,"record_offset":137572,"parent_record_offset":137544,"ancestor_record_offset":137420,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1643,"record_offset":137636,"parent_record_offset":137508,"ancestor_record_offset":137420,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1643,"record_offset":137608,"parent_record_offset":137508,"ancestor_record_offset":137420,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1643,"record_offset":137544,"parent_record_offset":137508,"ancestor_record_offset":137420,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1643,"record_offset":137508,"parent_record_offset":137456,"ancestor_record_offset":137420,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1643,"record_offset":137916,"parent_record_offset":137456,"ancestor_record_offset":137420,"inlinee":27159,"source_line":1577,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x89CE
// 首条位置: {"module":561,"inlinee":35278}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::iter::impl$171::all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":87076,"parent_record_offset":87048,"ancestor_record_offset":87048,"inlinee":11954,"source_line":309,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":87176,"parent_record_offset":87148,"ancestor_record_offset":87048,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":87148,"parent_record_offset":87124,"ancestor_record_offset":87048,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/bvh_builder.rs
//     evidence={"module":561,"record_offset":87100,"parent_record_offset":87076,"ancestor_record_offset":87048,"inlinee":35280,"source_line":89,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":561,"record_offset":87124,"parent_record_offset":87100,"ancestor_record_offset":87048,"inlinee":27905,"source_line":519,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x89D0
// 首条位置: {"module":561,"inlinee":35280}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":561,"record_offset":87124,"parent_record_offset":87100,"ancestor_record_offset":87100,"inlinee":27905,"source_line":519,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":87176,"parent_record_offset":87148,"ancestor_record_offset":87100,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":87148,"parent_record_offset":87124,"ancestor_record_offset":87100,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/00561.jsonl.gz"}
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
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > >", "constructor": "core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >", "tuple$<>", "enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >", "core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >", "constructor": "core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >", "tuple$<>", "enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >", "core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >", "constructor": "core::iter::adapters::map::impl$2::try_fold", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >", "std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >", "country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0", "tuple$<>", "core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >", "enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::rt::bvh_builder::RtBvhBuilderGlue
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::rwlock::RwLock<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>", "constructor": "std::sync::poison::rwlock::RwLock", "arguments": ["country_core::resources::rt::bvh_builder::RtBvhBuilderGlue"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::poison::rwlock::RwLock<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::poison::rwlock::RwLock<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::rwlock::RwLockReadGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>", "constructor": "std::sync::poison::rwlock::RwLockReadGuard", "arguments": ["country_core::resources::rt::bvh_builder::RtBvhBuilderGlue"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockReadGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >", "constructor": "std::sync::poison::PoisonError", "arguments": ["std::sync::poison::rwlock::RwLockReadGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockReadGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockReadGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<std::sync::poison::rwlock::RwLock<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["std::sync::poison::rwlock::RwLock<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>", "constructor": "std::sync::poison::rwlock::RwLockWriteGuard", "arguments": ["country_core::resources::rt::bvh_builder::RtBvhBuilderGlue"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::impl$1::fmt<std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >", "constructor": "std::sync::poison::impl$1::fmt", "arguments": ["std::sync::poison::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 27; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)
// 候选参数类型: ["ref$<tg_mods::overlay::Mod>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] country_core::systems::wall::structure_outline::WallOutline (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] relative_path::relative_path_buf::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh_builder::RtBvhBuilderGlue::is_idle
// 信息 ID: 3d6f950cedaa054c44138f79
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::rt::RtBvhBuilderGlueResource::is_idle; module=512, inline_record=65048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh_builder::bvh_builder_worker
// 信息 ID: 7ff142a4eb71b94f57954fac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$7,tuple$<> > @ 0x5e7544 -> 0xc44960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0
// 信息 ID: 9bc5833fad211366158821b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::rt::bvh_builder::bvh_builder_worker", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0 <- core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; module=1682, inline_record=140076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure$0
// 信息 ID: f0693f4b61b31924af968754
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::rt::bvh_builder::bvh_builder_worker", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0 <- core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; module=1643, inline_record=137420; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; module=1643, inline_record=139752; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; module=1643, inline_record=141780; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; module=1682, inline_record=140680; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$1
// 信息 ID: 674e57778180302c921ed110
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::rt::bvh_builder::bvh_builder_worker", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<array$<glam::f32::vec3::Vec3,3>,u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1 <- core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1 <- core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<array$<glam::f32::vec3::Vec3,3>,u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; module=561, inline_record=87048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$1::closure$0
// 信息 ID: 8f6ceee7e6c61200262d5771
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::rt::bvh_builder::bvh_builder_worker", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<array$<glam::f32::vec3::Vec3,3>,u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; module=561, inline_record=87100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
