// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/color/src/style_picker_target.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 67f1b29552be06362147849f10968eadbbff6dd9f53168794851ede9b9de7c25
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: style_picker_target.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2689, 2695, 2697, 3991, 3992, 4010
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_color; // namespace candidate
//   package identities: [{"name":"system-color","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::impl$5::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2697,"record_offset":11508,"rva":34902704,"va":5403611824}
// 编译实例 RVAs: 0x21492b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_tuple_field1_finish
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-18794232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403611824,"raw_hex":"e80839e1fe","size":5,"soft_fail":false,"target_va":5384817664,"va":5403611891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::impl$7::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A6E3
// 首条位置: {"module":2697,"inlinee":108259}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2697,"record_offset":780,"parent_record_offset":744,"ancestor_record_offset":744,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02697.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2697,"record_offset":836,"parent_record_offset":780,"ancestor_record_offset":744,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/02697.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":2697,"record_offset":808,"parent_record_offset":780,"ancestor_record_offset":744,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/02697.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::impl$9::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20EBB
// 首条位置: {"module":4010,"inlinee":134843}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::StairPickingSnapshot::capture
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4010,"record_offset":3320,"rva":42551984,"va":5411261104}
// 编译实例 RVAs: 0x2894ab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-35582665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411261104,"raw_hex":"e8370de1fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5411261364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142115a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: style_picker_target.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7860577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411261104,"raw_hex":"e89f0e88ff","size":5,"soft_fail":false,"target_va":5403400736,"va":5411261308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsState::get_all_connected_nodes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31711704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411261104,"raw_hex":"e8281e1cfe","size":5,"soft_fail":false,"target_va":5379549488,"va":5411261187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4010,"record_offset":4064,"parent_record_offset":3320,"ancestor_record_offset":3320,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4010,"record_offset":3916,"parent_record_offset":3320,"ancestor_record_offset":3320,"inlinee":8664,"source_line":911,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4010,"record_offset":4148,"parent_record_offset":3320,"ancestor_record_offset":3320,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4010,"record_offset":3980,"parent_record_offset":3320,"ancestor_record_offset":3320,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::ColliderRegistry::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":4010,"record_offset":3488,"parent_record_offset":3320,"ancestor_record_offset":3320,"inlinee":134841,"source_line":280,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_direct] system_color::style_picker_target::impl$9::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":4010,"record_offset":4440,"parent_record_offset":3320,"ancestor_record_offset":3320,"inlinee":134843,"source_line":27,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4010,"record_offset":4336,"parent_record_offset":4312,"ancestor_record_offset":3320,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4010,"record_offset":4360,"parent_record_offset":4336,"ancestor_record_offset":3320,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4010,"record_offset":4384,"parent_record_offset":4360,"ancestor_record_offset":3320,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4010,"record_offset":4312,"parent_record_offset":4220,"ancestor_record_offset":3320,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4010,"record_offset":4248,"parent_record_offset":4220,"ancestor_record_offset":3320,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4010,"record_offset":4220,"parent_record_offset":4196,"ancestor_record_offset":3320,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4010,"record_offset":4196,"parent_record_offset":4172,"ancestor_record_offset":3320,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4010,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":3320,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4010,"record_offset":3776,"parent_record_offset":3748,"ancestor_record_offset":3320,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4010,"record_offset":3828,"parent_record_offset":3804,"ancestor_record_offset":3320,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::Filter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":4010,"record_offset":3944,"parent_record_offset":3916,"ancestor_record_offset":3320,"inlinee":8666,"source_line":27,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":4010,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":3320,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4010,"record_offset":4280,"parent_record_offset":4248,"ancestor_record_offset":3320,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4010,"record_offset":3720,"parent_record_offset":3688,"ancestor_record_offset":3320,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4010,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":3320,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4010,"record_offset":4028,"parent_record_offset":4004,"ancestor_record_offset":3320,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4010,"record_offset":4004,"parent_record_offset":3980,"ancestor_record_offset":3320,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4010,"record_offset":3748,"parent_record_offset":3688,"ancestor_record_offset":3320,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4010,"record_offset":3804,"parent_record_offset":3748,"ancestor_record_offset":3320,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4010,"record_offset":3544,"parent_record_offset":3516,"ancestor_record_offset":3320,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4010,"record_offset":3688,"parent_record_offset":3600,"ancestor_record_offset":3320,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4010,"record_offset":3572,"parent_record_offset":3544,"ancestor_record_offset":3320,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4010,"record_offset":3660,"parent_record_offset":3600,"ancestor_record_offset":3320,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4010,"record_offset":3632,"parent_record_offset":3600,"ancestor_record_offset":3320,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4010,"record_offset":3600,"parent_record_offset":3572,"ancestor_record_offset":3320,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4010,"record_offset":3516,"parent_record_offset":3488,"ancestor_record_offset":3320,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::impl$0::capture::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20E64
// 首条位置: {"module":3991,"inlinee":134756}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3991,"record_offset":20160,"parent_record_offset":20136,"ancestor_record_offset":20136,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3991,"record_offset":20260,"parent_record_offset":20236,"ancestor_record_offset":20136,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3991,"record_offset":20184,"parent_record_offset":20160,"ancestor_record_offset":20136,"inlinee":12688,"source_line":375,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3991,"record_offset":20292,"parent_record_offset":20208,"ancestor_record_offset":20136,"inlinee":12690,"source_line":376,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3991,"record_offset":20208,"parent_record_offset":20184,"ancestor_record_offset":20136,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3991,"record_offset":20236,"parent_record_offset":20208,"ancestor_record_offset":20136,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3991,"record_offset":20316,"parent_record_offset":20292,"ancestor_record_offset":20136,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::impl$0::capture::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20E65
// 首条位置: {"module":3991,"inlinee":134757}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::impl$0::capture::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20E62
// 首条位置: {"module":3991,"inlinee":134754}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::collision::world::ColliderRegistry::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":3991,"record_offset":15236,"parent_record_offset":15196,"ancestor_record_offset":15196,"inlinee":33883,"source_line":276,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3991,"record_offset":15960,"parent_record_offset":15936,"ancestor_record_offset":15196,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3991,"record_offset":15528,"parent_record_offset":15504,"ancestor_record_offset":15196,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3991,"record_offset":15612,"parent_record_offset":15588,"ancestor_record_offset":15196,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3991,"record_offset":16028,"parent_record_offset":15392,"ancestor_record_offset":15196,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":3991,"record_offset":15760,"parent_record_offset":15732,"ancestor_record_offset":15196,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":3991,"record_offset":15732,"parent_record_offset":15704,"ancestor_record_offset":15196,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3991,"record_offset":16200,"parent_record_offset":15236,"ancestor_record_offset":15196,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3991,"record_offset":15552,"parent_record_offset":15528,"ancestor_record_offset":15196,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3991,"record_offset":15876,"parent_record_offset":15852,"ancestor_record_offset":15196,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3991,"record_offset":15984,"parent_record_offset":15960,"ancestor_record_offset":15196,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$9::get::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":3991,"record_offset":16224,"parent_record_offset":16200,"ancestor_record_offset":15196,"inlinee":33885,"source_line":277,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3991,"record_offset":15672,"parent_record_offset":15644,"ancestor_record_offset":15196,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3991,"record_offset":15704,"parent_record_offset":15672,"ancestor_record_offset":15196,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3991,"record_offset":15644,"parent_record_offset":15392,"ancestor_record_offset":15196,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":3991,"record_offset":15504,"parent_record_offset":15392,"ancestor_record_offset":15196,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":3991,"record_offset":16056,"parent_record_offset":15392,"ancestor_record_offset":15196,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":3991,"record_offset":15588,"parent_record_offset":15392,"ancestor_record_offset":15196,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":3991,"record_offset":15448,"parent_record_offset":15392,"ancestor_record_offset":15196,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":3991,"record_offset":15936,"parent_record_offset":15912,"ancestor_record_offset":15196,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":3991,"record_offset":15288,"parent_record_offset":15264,"ancestor_record_offset":15196,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":3991,"record_offset":15912,"parent_record_offset":15804,"ancestor_record_offset":15196,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":3991,"record_offset":15316,"parent_record_offset":15288,"ancestor_record_offset":15196,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3991,"record_offset":15852,"parent_record_offset":15828,"ancestor_record_offset":15196,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3991,"record_offset":16148,"parent_record_offset":15392,"ancestor_record_offset":15196,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3991,"record_offset":15828,"parent_record_offset":15804,"ancestor_record_offset":15196,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3991,"record_offset":15368,"parent_record_offset":15344,"ancestor_record_offset":15196,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3991,"record_offset":15344,"parent_record_offset":15288,"ancestor_record_offset":15196,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3991,"record_offset":15392,"parent_record_offset":15368,"ancestor_record_offset":15196,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3991,"record_offset":15476,"parent_record_offset":15392,"ancestor_record_offset":15196,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3991,"record_offset":15804,"parent_record_offset":15392,"ancestor_record_offset":15196,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3991,"record_offset":15264,"parent_record_offset":15236,"ancestor_record_offset":15196,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::impl$0::capture::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20E63
// 首条位置: {"module":3991,"inlinee":134755}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3991,"record_offset":17824,"parent_record_offset":17764,"ancestor_record_offset":17764,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$47::branch
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3991,"record_offset":17792,"parent_record_offset":17764,"ancestor_record_offset":17764,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$134::clone
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":3991,"record_offset":17852,"parent_record_offset":17764,"ancestor_record_offset":17764,"inlinee":27795,"source_line":1581,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3991,"record_offset":17880,"parent_record_offset":17852,"ancestor_record_offset":17764,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3991,"record_offset":17908,"parent_record_offset":17880,"ancestor_record_offset":17764,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3991,"record_offset":17932,"parent_record_offset":17908,"ancestor_record_offset":17764,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::StairPickingSnapshot::raycast
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20EB0
// 首条位置: {"module":4010,"inlinee":134832}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4010,"record_offset":572,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4010,"record_offset":1232,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4010,"record_offset":1176,"parent_record_offset":1056,"ancestor_record_offset":536,"inlinee":5208,"source_line":518,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4010,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":536,"inlinee":5404,"source_line":1605,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":4010,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":536,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4010,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":536,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":4010,"record_offset":964,"parent_record_offset":892,"ancestor_record_offset":536,"inlinee":17049,"source_line":39,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":4010,"record_offset":868,"parent_record_offset":600,"ancestor_record_offset":536,"inlinee":17047,"source_line":142,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":4010,"record_offset":636,"parent_record_offset":600,"ancestor_record_offset":536,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4010,"record_offset":1032,"parent_record_offset":964,"ancestor_record_offset":536,"inlinee":5402,"source_line":3391,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4010,"record_offset":600,"parent_record_offset":572,"ancestor_record_offset":536,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4010,"record_offset":1264,"parent_record_offset":1232,"ancestor_record_offset":536,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4010,"record_offset":776,"parent_record_offset":660,"ancestor_record_offset":536,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4010,"record_offset":1088,"parent_record_offset":1056,"ancestor_record_offset":536,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4010,"record_offset":840,"parent_record_offset":600,"ancestor_record_offset":536,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4010,"record_offset":744,"parent_record_offset":688,"ancestor_record_offset":536,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4010,"record_offset":716,"parent_record_offset":688,"ancestor_record_offset":536,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4010,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":536,"inlinee":6772,"source_line":360,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4010,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":536,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4010,"record_offset":688,"parent_record_offset":660,"ancestor_record_offset":536,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_picker_target::impl$0::raycast::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":4010,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":536,"inlinee":134834,"source_line":69,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_picker_target::impl$0::raycast::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":4010,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":536,"inlinee":134835,"source_line":70,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::impl$0::raycast::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20EB2
// 首条位置: {"module":4010,"inlinee":134834}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::impl$0::raycast::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20EB3
// 首条位置: {"module":4010,"inlinee":134835}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4010,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":1112,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_changes::StyleScene::window_ids
// 记录形式: compiled_procedure; instances=1
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2695,"record_offset":35904,"rva":34891936,"va":5403601056}
// 编译实例 RVAs: 0x21468a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-27922486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403601056,"raw_hex":"e8caef55fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5403601169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-27922834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403601056,"raw_hex":"e86eee55fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5403601533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: style_picker_target.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34878967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403601056,"raw_hex":"e809caebfd","size":5,"soft_fail":false,"target_va":5368722192,"va":5403601154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a47e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: style_picker_target.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24111828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403601056,"raw_hex":"e82c1590fe","size":5,"soft_fail":false,"target_va":5379489392,"va":5403601215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421157e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: style_picker_target.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-201133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403601056,"raw_hex":"e853eefcff","size":5,"soft_fail":false,"target_va":5403400160,"va":5403601288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13019320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403601056,"raw_hex":"e8b8a8c600","size":5,"soft_fail":false,"target_va":5416620769,"va":5403601444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7643919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403601056,"raw_hex":"e80fa37400","size":5,"soft_fail":false,"target_va":5411245392,"va":5403601468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24098134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403601056,"raw_hex":"e8aa4a90fe","size":5,"soft_fail":false,"target_va":5379502992,"va":5403601121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2695,"record_offset":36956,"parent_record_offset":35904,"ancestor_record_offset":35904,"inlinee":11475,"source_line":131,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2695,"record_offset":36056,"parent_record_offset":35904,"ancestor_record_offset":35904,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2695,"record_offset":36140,"parent_record_offset":35904,"ancestor_record_offset":35904,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2695,"record_offset":36384,"parent_record_offset":35904,"ancestor_record_offset":35904,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2695,"record_offset":37540,"parent_record_offset":35904,"ancestor_record_offset":35904,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2695,"record_offset":36280,"parent_record_offset":36256,"ancestor_record_offset":35904,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2695,"record_offset":36304,"parent_record_offset":36280,"ancestor_record_offset":35904,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2695,"record_offset":37724,"parent_record_offset":37700,"ancestor_record_offset":35904,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2695,"record_offset":37748,"parent_record_offset":37724,"ancestor_record_offset":35904,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2695,"record_offset":36328,"parent_record_offset":36304,"ancestor_record_offset":35904,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2695,"record_offset":37772,"parent_record_offset":37748,"ancestor_record_offset":35904,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2695,"record_offset":36256,"parent_record_offset":36224,"ancestor_record_offset":35904,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2695,"record_offset":37700,"parent_record_offset":37612,"ancestor_record_offset":35904,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2695,"record_offset":36224,"parent_record_offset":36196,"ancestor_record_offset":35904,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2695,"record_offset":36196,"parent_record_offset":36168,"ancestor_record_offset":35904,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2695,"record_offset":37640,"parent_record_offset":37612,"ancestor_record_offset":35904,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2695,"record_offset":37612,"parent_record_offset":37588,"ancestor_record_offset":35904,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2695,"record_offset":37588,"parent_record_offset":37564,"ancestor_record_offset":35904,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2695,"record_offset":36984,"parent_record_offset":36956,"ancestor_record_offset":35904,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2695,"record_offset":36880,"parent_record_offset":36856,"ancestor_record_offset":35904,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2695,"record_offset":37320,"parent_record_offset":37296,"ancestor_record_offset":35904,"inlinee":6450,"source_line":1410,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2695,"record_offset":36720,"parent_record_offset":36692,"ancestor_record_offset":35904,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2695,"record_offset":36772,"parent_record_offset":36748,"ancestor_record_offset":35904,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2695,"record_offset":37048,"parent_record_offset":37012,"ancestor_record_offset":35904,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2695,"record_offset":36856,"parent_record_offset":36440,"ancestor_record_offset":35904,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2695,"record_offset":37668,"parent_record_offset":37640,"ancestor_record_offset":35904,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2695,"record_offset":37296,"parent_record_offset":37172,"ancestor_record_offset":35904,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2695,"record_offset":37344,"parent_record_offset":37320,"ancestor_record_offset":35904,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2695,"record_offset":36664,"parent_record_offset":36628,"ancestor_record_offset":35904,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2695,"record_offset":37260,"parent_record_offset":37236,"ancestor_record_offset":35904,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2695,"record_offset":37212,"parent_record_offset":37172,"ancestor_record_offset":35904,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2695,"record_offset":37116,"parent_record_offset":37076,"ancestor_record_offset":35904,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":2695,"record_offset":37236,"parent_record_offset":37212,"ancestor_record_offset":35904,"inlinee":4734,"source_line":302,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insert_tail
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":2695,"record_offset":37172,"parent_record_offset":37076,"ancestor_record_offset":35904,"inlinee":9485,"source_line":542,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":2695,"record_offset":37076,"parent_record_offset":37012,"ancestor_record_offset":35904,"inlinee":9500,"source_line":580,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":2695,"record_offset":37012,"parent_record_offset":36984,"ancestor_record_offset":35904,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2695,"record_offset":36692,"parent_record_offset":36628,"ancestor_record_offset":35904,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2695,"record_offset":36748,"parent_record_offset":36692,"ancestor_record_offset":35904,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2695,"record_offset":36492,"parent_record_offset":36468,"ancestor_record_offset":35904,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":36628,"parent_record_offset":36540,"ancestor_record_offset":35904,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":36516,"parent_record_offset":36492,"ancestor_record_offset":35904,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":36600,"parent_record_offset":36540,"ancestor_record_offset":35904,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":36572,"parent_record_offset":36540,"ancestor_record_offset":35904,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":36540,"parent_record_offset":36516,"ancestor_record_offset":35904,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2695,"record_offset":36468,"parent_record_offset":36440,"ancestor_record_offset":35904,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_picker_target::impl$1::window_ids::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":2695,"record_offset":36084,"parent_record_offset":36056,"ancestor_record_offset":35904,"inlinee":108241,"source_line":81,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_picker_target::impl$1::window_ids::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":2695,"record_offset":36168,"parent_record_offset":36140,"ancestor_record_offset":35904,"inlinee":108242,"source_line":82,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_picker_target::impl$1::window_ids::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":2695,"record_offset":36440,"parent_record_offset":36412,"ancestor_record_offset":35904,"inlinee":108243,"source_line":87,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_sample::is_window
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_sample.rs
//     evidence={"module":2695,"record_offset":36104,"parent_record_offset":36084,"ancestor_record_offset":35904,"inlinee":108197,"source_line":56,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::impl$1::window_ids::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A6D1
// 首条位置: {"module":2695,"inlinee":108241}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_color::style_sample::is_window
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_sample.rs
//     evidence={"module":2695,"record_offset":36104,"parent_record_offset":36084,"ancestor_record_offset":36084,"inlinee":108197,"source_line":56,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::impl$1::window_ids::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A6D2
// 首条位置: {"module":2695,"inlinee":108242}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2695,"record_offset":36196,"parent_record_offset":36168,"ancestor_record_offset":36168,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2695,"record_offset":36280,"parent_record_offset":36256,"ancestor_record_offset":36168,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2695,"record_offset":36304,"parent_record_offset":36280,"ancestor_record_offset":36168,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2695,"record_offset":36328,"parent_record_offset":36304,"ancestor_record_offset":36168,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2695,"record_offset":36256,"parent_record_offset":36224,"ancestor_record_offset":36168,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2695,"record_offset":36224,"parent_record_offset":36196,"ancestor_record_offset":36168,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::impl$1::window_ids::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A6D3
// 首条位置: {"module":2695,"inlinee":108243}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2695,"record_offset":36856,"parent_record_offset":36440,"ancestor_record_offset":36440,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2695,"record_offset":36468,"parent_record_offset":36440,"ancestor_record_offset":36440,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2695,"record_offset":36880,"parent_record_offset":36856,"ancestor_record_offset":36440,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2695,"record_offset":36720,"parent_record_offset":36692,"ancestor_record_offset":36440,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2695,"record_offset":36772,"parent_record_offset":36748,"ancestor_record_offset":36440,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2695,"record_offset":36664,"parent_record_offset":36628,"ancestor_record_offset":36440,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2695,"record_offset":36692,"parent_record_offset":36628,"ancestor_record_offset":36440,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2695,"record_offset":36748,"parent_record_offset":36692,"ancestor_record_offset":36440,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2695,"record_offset":36492,"parent_record_offset":36468,"ancestor_record_offset":36440,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":36628,"parent_record_offset":36540,"ancestor_record_offset":36440,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":36516,"parent_record_offset":36492,"ancestor_record_offset":36440,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":36600,"parent_record_offset":36540,"ancestor_record_offset":36440,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":36572,"parent_record_offset":36540,"ancestor_record_offset":36440,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":36540,"parent_record_offset":36516,"ancestor_record_offset":36440,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::impl$1::window_ids::closure$2::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A6A3
// 首条位置: {"module":2689,"inlinee":108195}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_color::style_sample::is_window
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_sample.rs
//     evidence={"module":2689,"record_offset":10684,"parent_record_offset":10660,"ancestor_record_offset":10660,"inlinee":108197,"source_line":56,"archive":"_pdb/C/modules/02689.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::pick_target
// 记录形式: compiled_procedure; instances=1
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4010,"record_offset":268,"rva":42550304,"va":5411259424}
// 编译实例 RVAs: 0x2894420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-35582367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411259424,"raw_hex":"e8610ee1fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5411261066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: style_picker_target.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33140326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411259424,"raw_hex":"e89a5106fe","size":5,"soft_fail":false,"target_va":5378120480,"va":5411260801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a03040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: style_picker_target.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32053079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411259424,"raw_hex":"e8a9e816fe","size":5,"soft_fail":false,"target_va":5379207232,"va":5411260306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b14470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: style_picker_target.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2619897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411259424,"raw_hex":"e8f9f92700","size":5,"soft_fail":false,"target_va":5413880944,"va":5411261042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b182d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: style_picker_target.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2636218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411259424,"raw_hex":"e8ba392800","size":5,"soft_fail":false,"target_va":5413896912,"va":5411260689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsState::get_all_connected_nodes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31711473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411259424,"raw_hex":"e80f1f1cfe","size":5,"soft_fail":false,"target_va":5379549488,"va":5411260956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31757847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411259424,"raw_hex":"e8e9691bfe","size":5,"soft_fail":false,"target_va":5379502992,"va":5411260834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29951598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411259424,"raw_hex":"e892f936fe","size":5,"soft_fail":false,"target_va":5381309184,"va":5411260777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMesh::raycast
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32103028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411259424,"raw_hex":"e88c2516fe","size":5,"soft_fail":false,"target_va":5379156592,"va":5411259615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::legacy_raycast_w_params
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32094670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411259424,"raw_hex":"e8324616fe","size":5,"soft_fail":false,"target_va":5379164880,"va":5411259545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4010,"record_offset":484,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4010,"record_offset":1336,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::filter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4010,"record_offset":1712,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4010,"record_offset":1364,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7338,"source_line":710,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4010,"record_offset":424,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4010,"record_offset":2260,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$122::bitor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4010,"record_offset":456,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":134828,"source_line":731,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4010,"record_offset":2944,"parent_record_offset":2920,"ancestor_record_offset":268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4010,"record_offset":2968,"parent_record_offset":2944,"ancestor_record_offset":268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4010,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4010,"record_offset":2920,"parent_record_offset":2832,"ancestor_record_offset":268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4010,"record_offset":2860,"parent_record_offset":2832,"ancestor_record_offset":268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4010,"record_offset":2832,"parent_record_offset":2808,"ancestor_record_offset":268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4010,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4010,"record_offset":2328,"parent_record_offset":2288,"ancestor_record_offset":268,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":4010,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":268,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":4010,"record_offset":2760,"parent_record_offset":2736,"ancestor_record_offset":268,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":4010,"record_offset":2540,"parent_record_offset":2504,"ancestor_record_offset":268,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4010,"record_offset":1176,"parent_record_offset":1056,"ancestor_record_offset":268,"inlinee":5208,"source_line":518,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4010,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":268,"inlinee":5404,"source_line":1605,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":4010,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":268,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4010,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":268,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4010,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":268,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":4010,"record_offset":964,"parent_record_offset":892,"ancestor_record_offset":268,"inlinee":17049,"source_line":39,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":4010,"record_offset":868,"parent_record_offset":600,"ancestor_record_offset":268,"inlinee":17047,"source_line":142,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":4010,"record_offset":636,"parent_record_offset":600,"ancestor_record_offset":268,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4010,"record_offset":2448,"parent_record_offset":2288,"ancestor_record_offset":268,"inlinee":51011,"source_line":3265,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4010,"record_offset":572,"parent_record_offset":536,"ancestor_record_offset":268,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4010,"record_offset":1032,"parent_record_offset":964,"ancestor_record_offset":268,"inlinee":5402,"source_line":3391,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4010,"record_offset":600,"parent_record_offset":572,"ancestor_record_offset":268,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":4010,"record_offset":2360,"parent_record_offset":2328,"ancestor_record_offset":268,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":4010,"record_offset":2384,"parent_record_offset":2360,"ancestor_record_offset":268,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4010,"record_offset":2888,"parent_record_offset":2860,"ancestor_record_offset":268,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4010,"record_offset":1492,"parent_record_offset":1452,"ancestor_record_offset":268,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4010,"record_offset":1264,"parent_record_offset":1232,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4010,"record_offset":776,"parent_record_offset":660,"ancestor_record_offset":268,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4010,"record_offset":1088,"parent_record_offset":1056,"ancestor_record_offset":268,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4010,"record_offset":1996,"parent_record_offset":1744,"ancestor_record_offset":268,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4010,"record_offset":2020,"parent_record_offset":1996,"ancestor_record_offset":268,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4010,"record_offset":1232,"parent_record_offset":536,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4010,"record_offset":840,"parent_record_offset":600,"ancestor_record_offset":268,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4010,"record_offset":2688,"parent_record_offset":2504,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4010,"record_offset":2416,"parent_record_offset":2328,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4010,"record_offset":744,"parent_record_offset":688,"ancestor_record_offset":268,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4010,"record_offset":2628,"parent_record_offset":2540,"ancestor_record_offset":268,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4010,"record_offset":716,"parent_record_offset":688,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4010,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":268,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4010,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":268,"inlinee":6772,"source_line":360,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4010,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":268,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4010,"record_offset":688,"parent_record_offset":660,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$26::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":4010,"record_offset":2224,"parent_record_offset":2164,"ancestor_record_offset":268,"inlinee":26794,"source_line":21,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4010,"record_offset":1772,"parent_record_offset":1744,"ancestor_record_offset":268,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4010,"record_offset":1424,"parent_record_offset":1392,"ancestor_record_offset":268,"inlinee":26483,"source_line":585,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4010,"record_offset":1520,"parent_record_offset":1452,"ancestor_record_offset":268,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4010,"record_offset":1452,"parent_record_offset":1424,"ancestor_record_offset":268,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4010,"record_offset":1900,"parent_record_offset":1864,"ancestor_record_offset":268,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_picker_target::StairPickingSnapshot::raycast
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":4010,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":268,"inlinee":134832,"source_line":66,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_picker_target::impl$0::raycast::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":4010,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":268,"inlinee":134834,"source_line":69,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_picker_target::impl$0::raycast::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":4010,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":268,"inlinee":134835,"source_line":70,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_picker_target::pick_target::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":4010,"record_offset":512,"parent_record_offset":484,"ancestor_record_offset":268,"inlinee":134830,"source_line":139,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_picker_target::pick_target::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":4010,"record_offset":1392,"parent_record_offset":1364,"ancestor_record_offset":268,"inlinee":134836,"source_line":140,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_picker_target::pick_target::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":4010,"record_offset":1744,"parent_record_offset":1712,"ancestor_record_offset":268,"inlinee":134837,"source_line":147,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_picker_target::pick_target::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":4010,"record_offset":2100,"parent_record_offset":2064,"ancestor_record_offset":268,"inlinee":134838,"source_line":162,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_picker_target::pick_target::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":4010,"record_offset":2164,"parent_record_offset":2132,"ancestor_record_offset":268,"inlinee":134839,"source_line":181,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_picker_target::pick_target::closure$6
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":4010,"record_offset":2288,"parent_record_offset":2260,"ancestor_record_offset":268,"inlinee":134840,"source_line":194,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_sample::is_window
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_sample.rs
//     evidence={"module":4010,"record_offset":2196,"parent_record_offset":2164,"ancestor_record_offset":268,"inlinee":108197,"source_line":56,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::pick_target::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20EAE
// 首条位置: {"module":4010,"inlinee":134830}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_color::style_picker_target::StairPickingSnapshot::raycast
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":4010,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":134832,"source_line":66,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4010,"record_offset":1176,"parent_record_offset":1056,"ancestor_record_offset":512,"inlinee":5208,"source_line":518,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4010,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":512,"inlinee":5404,"source_line":1605,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":4010,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":512,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4010,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":512,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":4010,"record_offset":964,"parent_record_offset":892,"ancestor_record_offset":512,"inlinee":17049,"source_line":39,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":4010,"record_offset":868,"parent_record_offset":600,"ancestor_record_offset":512,"inlinee":17047,"source_line":142,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":4010,"record_offset":636,"parent_record_offset":600,"ancestor_record_offset":512,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4010,"record_offset":572,"parent_record_offset":536,"ancestor_record_offset":512,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4010,"record_offset":1032,"parent_record_offset":964,"ancestor_record_offset":512,"inlinee":5402,"source_line":3391,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4010,"record_offset":600,"parent_record_offset":572,"ancestor_record_offset":512,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4010,"record_offset":1264,"parent_record_offset":1232,"ancestor_record_offset":512,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4010,"record_offset":776,"parent_record_offset":660,"ancestor_record_offset":512,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4010,"record_offset":1088,"parent_record_offset":1056,"ancestor_record_offset":512,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4010,"record_offset":1232,"parent_record_offset":536,"ancestor_record_offset":512,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4010,"record_offset":840,"parent_record_offset":600,"ancestor_record_offset":512,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4010,"record_offset":744,"parent_record_offset":688,"ancestor_record_offset":512,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4010,"record_offset":716,"parent_record_offset":688,"ancestor_record_offset":512,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4010,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":512,"inlinee":6772,"source_line":360,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4010,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":512,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4010,"record_offset":688,"parent_record_offset":660,"ancestor_record_offset":512,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_picker_target::impl$0::raycast::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":4010,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":512,"inlinee":134834,"source_line":69,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_picker_target::impl$0::raycast::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_picker_target.rs
//     evidence={"module":4010,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":512,"inlinee":134835,"source_line":70,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::pick_target::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20EB4
// 首条位置: {"module":4010,"inlinee":134836}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4010,"record_offset":1424,"parent_record_offset":1392,"ancestor_record_offset":1392,"inlinee":26483,"source_line":585,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4010,"record_offset":1492,"parent_record_offset":1452,"ancestor_record_offset":1392,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4010,"record_offset":1520,"parent_record_offset":1452,"ancestor_record_offset":1392,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4010,"record_offset":1452,"parent_record_offset":1424,"ancestor_record_offset":1392,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::pick_target::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20EB5
// 首条位置: {"module":4010,"inlinee":134837}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4010,"record_offset":1996,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4010,"record_offset":1772,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4010,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":1744,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4010,"record_offset":1836,"parent_record_offset":1796,"ancestor_record_offset":1744,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4010,"record_offset":2020,"parent_record_offset":1996,"ancestor_record_offset":1744,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4010,"record_offset":1864,"parent_record_offset":1796,"ancestor_record_offset":1744,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4010,"record_offset":1796,"parent_record_offset":1772,"ancestor_record_offset":1744,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4010,"record_offset":1900,"parent_record_offset":1864,"ancestor_record_offset":1744,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::pick_target::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20EB6
// 首条位置: {"module":4010,"inlinee":134838}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::pick_target::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20EB7
// 首条位置: {"module":4010,"inlinee":134839}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$26::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":4010,"record_offset":2224,"parent_record_offset":2164,"ancestor_record_offset":2164,"inlinee":26794,"source_line":21,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_direct] system_color::style_sample::is_window
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_sample.rs
//     evidence={"module":4010,"record_offset":2196,"parent_record_offset":2164,"ancestor_record_offset":2164,"inlinee":108197,"source_line":56,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_picker_target::pick_target::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 194
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20EB8
// 首条位置: {"module":4010,"inlinee":134840}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4010,"record_offset":2328,"parent_record_offset":2288,"ancestor_record_offset":2288,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4010,"record_offset":2448,"parent_record_offset":2288,"ancestor_record_offset":2288,"inlinee":51011,"source_line":3265,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4010,"record_offset":2944,"parent_record_offset":2920,"ancestor_record_offset":2288,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4010,"record_offset":2968,"parent_record_offset":2944,"ancestor_record_offset":2288,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4010,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":2288,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4010,"record_offset":2920,"parent_record_offset":2832,"ancestor_record_offset":2288,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4010,"record_offset":2860,"parent_record_offset":2832,"ancestor_record_offset":2288,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4010,"record_offset":2832,"parent_record_offset":2808,"ancestor_record_offset":2288,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4010,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":2288,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":4010,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":2288,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":4010,"record_offset":2760,"parent_record_offset":2736,"ancestor_record_offset":2288,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":4010,"record_offset":2540,"parent_record_offset":2504,"ancestor_record_offset":2288,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4010,"record_offset":2476,"parent_record_offset":2448,"ancestor_record_offset":2288,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4010,"record_offset":2504,"parent_record_offset":2476,"ancestor_record_offset":2288,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":4010,"record_offset":2360,"parent_record_offset":2328,"ancestor_record_offset":2288,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":4010,"record_offset":2384,"parent_record_offset":2360,"ancestor_record_offset":2288,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4010,"record_offset":2888,"parent_record_offset":2860,"ancestor_record_offset":2288,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4010,"record_offset":2688,"parent_record_offset":2504,"ancestor_record_offset":2288,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4010,"record_offset":2416,"parent_record_offset":2328,"ancestor_record_offset":2288,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4010,"record_offset":2600,"parent_record_offset":2540,"ancestor_record_offset":2288,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4010,"record_offset":2628,"parent_record_offset":2540,"ancestor_record_offset":2288,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4010,"record_offset":2572,"parent_record_offset":2540,"ancestor_record_offset":2288,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4010,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":2288,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/04010.jsonl.gz"}
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
// 完整结构及原始类型记录: style_picker_target.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_color::style_picker_target::StairPickingSnapshot
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: style_picker_target.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: style_picker_target.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]

// ------------------------------------------------------------------
// 函数: system_color::style_changes::StyleScene::window_ids
// 信息 ID: 7dd1b83280114ae1783dfa21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::style_changes::StyleScene::changes @ 0x214432c -> 0x21468a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::style_picker_feedback::outline_target @ 0x2892722 -> 0x21468a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::StairPickingSnapshot::capture
// 信息 ID: f5b11904ba2576a293002818
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::style_picker::style_picker @ 0x21484e8 -> 0x2894ab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::StairPickingSnapshot::raycast
// 信息 ID: c12cf6032adc28877cb92007
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::style_picker_target::pick_target; module=4010, inline_record=536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::impl$0::capture::closure$0
// 信息 ID: 009848b8974495cdd6c8f24e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::style_picker_target::impl$0::capture", "captures": "unknown"}
// [闭包环境类型引用] system_color::style_picker_target::impl$0::capture::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,system_color::style_picker_target::impl$0::capture::closure_env$2> >,system_color::style_picker_target::impl$0::capture::closure_env$3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::style_picker_target::impl$0::capture::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<country_core::systems::collision::world::SubworldColliderHandle,core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::style_picker_target::impl$0::capture::closure_env$0 <- core::iter::adapters::chain::impl$1::try_fold<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,system_color::style_picker_target::impl$0::capture::closure_env$2>,tuple$<>,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<country_core::systems::collision::world::SubworldColliderHandle,core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; module=3991, inline_record=20136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::impl$0::capture::closure$1
// 信息 ID: 28da4ff6d09608b4e2663d91
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_color::style_picker_target::impl$0::capture", "captures": "unknown"}
// [闭包环境类型引用] system_color::style_picker_target::impl$0::capture::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,system_color::style_picker_target::impl$0::capture::closure_env$2> >,system_color::style_picker_target::impl$0::capture::closure_env$3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::style_picker_target::impl$0::capture::closure_env$1 <- core::iter::adapters::map::impl$2::try_fold<country_core::systems::collision::world::SubworldColliderHandle,core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::style_picker_target::impl$0::capture::closure_env$1 <- core::iter::adapters::chain::impl$1::try_fold<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,system_color::style_picker_target::impl$0::capture::closure_env$2>,tuple$<>,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<country_core::systems::collision::world::SubworldColliderHandle,core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; module=3991, inline_record=20592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::impl$0::capture::closure$2
// 信息 ID: da441bfe3e83b226234be754
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_color::style_picker_target::impl$0::capture", "captures": "unknown"}
// [闭包环境类型引用] system_color::style_picker_target::impl$0::capture::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,system_color::style_picker_target::impl$0::capture::closure_env$2> >,system_color::style_picker_target::impl$0::capture::closure_env$3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::style_picker_target::impl$0::capture::closure_env$2 <- core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,system_color::style_picker_target::impl$0::capture::closure_env$2,tuple$<>,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::style_picker_target::impl$0::capture::closure_env$2 <- core::iter::adapters::chain::impl$1::try_fold<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,system_color::style_picker_target::impl$0::capture::closure_env$2>,tuple$<>,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,system_color::style_picker_target::impl$0::capture::closure_env$2,tuple$<>,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; module=3991, inline_record=15196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::impl$0::capture::closure$3
// 信息 ID: f45a5ec097bc6de36303bd74
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_color::style_picker_target::impl$0::capture", "captures": "unknown"}
// [闭包环境类型引用] system_color::style_picker_target::impl$0::capture::closure_env$3 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,system_color::style_picker_target::impl$0::capture::closure_env$2> >,system_color::style_picker_target::impl$0::capture::closure_env$3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::style_picker_target::impl$0::capture::closure_env$3 <- core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,system_color::style_picker_target::impl$0::capture::closure_env$2,tuple$<>,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::style_picker_target::impl$0::capture::closure_env$3 <- core::iter::adapters::map::impl$2::try_fold<country_core::systems::collision::world::SubworldColliderHandle,core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::style_picker_target::impl$0::capture::closure_env$3 <- core::iter::adapters::chain::impl$1::try_fold<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,system_color::style_picker_target::impl$0::capture::closure_env$2>,tuple$<>,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,system_color::style_picker_target::impl$0::capture::closure_env$2,tuple$<>,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; module=3991, inline_record=17764; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,system_color::style_picker_target::impl$0::capture::closure_env$2,tuple$<>,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; module=3991, inline_record=18060; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,system_color::style_picker_target::impl$0::capture::closure_env$2,tuple$<>,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; module=3991, inline_record=18368; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<country_core::systems::collision::world::SubworldColliderHandle,core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; module=3991, inline_record=20708; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::chain::impl$1::try_fold<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,system_color::style_picker_target::impl$0::capture::closure_env$2>,tuple$<>,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; module=3992, inline_record=2288; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::chain::impl$1::try_fold<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,system_color::style_picker_target::impl$0::capture::closure_env$2>,tuple$<>,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; module=3992, inline_record=3076; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::chain::impl$1::try_fold<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,system_color::style_picker_target::impl$0::capture::closure_env$2>,tuple$<>,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >; module=3992, inline_record=3856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::impl$0::raycast::closure$0
// 信息 ID: b9a4639de4e8d76009babb42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::style_picker_target::impl$0::raycast", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::style_picker_target::pick_target; module=4010, inline_record=800; 不是独立调用边。
// [PDB 内联宿主] system_color::style_picker_target::pick_target; module=4010, inline_record=1004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::impl$0::raycast::closure$1
// 信息 ID: a17fce864dc56df19f1bc16f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_color::style_picker_target::impl$0::raycast", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::style_picker_target::pick_target; module=4010, inline_record=1112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::impl$1::window_ids::closure$0
// 信息 ID: 2d045e1879dace8d56f89a94
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::style_picker_target::impl$1::window_ids", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::style_changes::StyleScene::window_ids; module=2695, inline_record=36084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::impl$1::window_ids::closure$1
// 信息 ID: c839942e047c0c2df4fb0d47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_color::style_picker_target::impl$1::window_ids", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::style_changes::StyleScene::window_ids; module=2695, inline_record=36168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::impl$1::window_ids::closure$2
// 信息 ID: ab1042f51f756ac2eb948e12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_color::style_picker_target::impl$1::window_ids", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::style_changes::StyleScene::window_ids; module=2695, inline_record=36440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::impl$1::window_ids::closure$2::closure$0
// 信息 ID: d6a8323881583be0a5aa4e68
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_color::style_picker_target::impl$1::window_ids", "captures": "unknown"}
// [闭包环境类型引用] system_color::style_picker_target::impl$1::window_ids::closure$2::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,system_color::style_picker_target::impl$1::window_ids::closure$2::closure_env$0>,system_color::style_picker_target::impl$1::window_ids::closure$2::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,system_color::style_picker_target::impl$1::window_ids::closure$2::closure_env$0>,system_color::style_picker_target::impl$1::window_ids::closure$2::closure_env$1> >; module=2689, inline_record=10660; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,system_color::style_picker_target::impl$1::window_ids::closure$2::closure_env$0>,system_color::style_picker_target::impl$1::window_ids::closure$2::closure_env$1> >; module=2689, inline_record=11868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::impl$5::fmt
// 信息 ID: 1b5baffd8155a36a9edd7d9d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::impl$7::eq
// 信息 ID: ab023aeaaa579dcf7e23aaa7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::style_picker::style_picker; module=2697, inline_record=744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::impl$9::default
// 信息 ID: 872f7125cfc271127c68c5b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::style_picker_target::StairPickingSnapshot::capture; module=4010, inline_record=4440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::pick_target
// 信息 ID: 41ddc456827223eecba2ed62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::style_picker::style_picker @ 0x2147d9d -> 0x2894420 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::pick_target::closure$0
// 信息 ID: 53305ce0cc51a3ce5a8a0364
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::style_picker_target::pick_target", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::style_picker_target::pick_target; module=4010, inline_record=512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::pick_target::closure$1
// 信息 ID: a3f1aaab3927b995c914d51c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_color::style_picker_target::pick_target", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::style_picker_target::pick_target; module=4010, inline_record=1392; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::pick_target::closure$2
// 信息 ID: 94ec3262e064214c91ab123f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_color::style_picker_target::pick_target", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::style_picker_target::pick_target; module=4010, inline_record=1744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::pick_target::closure$3
// 信息 ID: 4404f95ce24211c7a1625701
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_color::style_picker_target::pick_target", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::style_picker_target::pick_target; module=4010, inline_record=2100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::pick_target::closure$5
// 信息 ID: f40613145d126654d74ca3f4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_color::style_picker_target::pick_target", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::style_picker_target::pick_target; module=4010, inline_record=2164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_picker_target::pick_target::closure$6
// 信息 ID: d4d396a14f6dafdb081c994a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "system_color::style_picker_target::pick_target", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::style_picker_target::pick_target; module=4010, inline_record=2288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
