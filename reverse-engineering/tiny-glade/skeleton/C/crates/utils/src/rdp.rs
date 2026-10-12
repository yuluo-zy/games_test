// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/rdp.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: b49a6c481759fef6347d0670af15381682435017dd3ad51e7830ee794b427f3a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: rdp.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 567, 588, 1455, 1482, 1511, 1643, 1682, 3167
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3167,"record_offset":280,"rva":38705984,"va":5407415104}
// 编译实例 RVAs: 0x24e9b40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-31736580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407415104,"raw_hex":"e8fcbc1bfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5407415263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-31738387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407415104,"raw_hex":"e8edb51bfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5407417086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38693063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407415104,"raw_hex":"e83997b1fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5407415250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413fbc10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17753779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407415104,"raw_hex":"e84d19f1fe","size":5,"soft_fail":false,"target_va":5389663248,"va":5407417022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8935479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407415104,"raw_hex":"e837588800","size":5,"soft_fail":false,"target_va":5416350944,"va":5407415460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9641698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407415104,"raw_hex":"e8e21e9300","size":5,"soft_fail":false,"target_va":5417057056,"va":5407415353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t8950542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407415104,"raw_hex":"e80e938800","size":5,"soft_fail":false,"target_va":5416365968,"va":5407415421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9203883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407415104,"raw_hex":"e8ab708c00","size":5,"soft_fail":false,"target_va":5416620794,"va":5407416906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9204893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407415104,"raw_hex":"e89d748c00","size":5,"soft_fail":false,"target_va":5416622208,"va":5407417310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9204364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407415104,"raw_hex":"e88c728c00","size":5,"soft_fail":false,"target_va":5416621762,"va":5407417393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9204981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407415104,"raw_hex":"e8f5748c00","size":5,"soft_fail":false,"target_va":5416622320,"va":5407417334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3167,"record_offset":1220,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":1740,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":2708,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5450,"source_line":2790,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":1796,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":6000,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":3167,"record_offset":3032,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":15216,"source_line":122,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3167,"record_offset":468,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":6764,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3167,"record_offset":6596,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3167,"record_offset":2232,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3167,"record_offset":2936,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":2260,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3167,"record_offset":2348,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":10939,"source_line":89,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3167,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":280,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3167,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":280,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3167,"record_offset":6952,"parent_record_offset":6928,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3167,"record_offset":6976,"parent_record_offset":6952,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3167,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":280,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3167,"record_offset":7000,"parent_record_offset":6976,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3167,"record_offset":736,"parent_record_offset":704,"ancestor_record_offset":280,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3167,"record_offset":6928,"parent_record_offset":6836,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3167,"record_offset":1876,"parent_record_offset":1852,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3167,"record_offset":644,"parent_record_offset":616,"ancestor_record_offset":280,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3167,"record_offset":6864,"parent_record_offset":6836,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3167,"record_offset":6836,"parent_record_offset":6812,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3167,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3167,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3167,"record_offset":704,"parent_record_offset":672,"ancestor_record_offset":280,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3167,"record_offset":672,"parent_record_offset":644,"ancestor_record_offset":280,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3167,"record_offset":6812,"parent_record_offset":6788,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3167,"record_offset":1432,"parent_record_offset":1400,"ancestor_record_offset":280,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3167,"record_offset":1248,"parent_record_offset":1220,"ancestor_record_offset":280,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3167,"record_offset":1276,"parent_record_offset":1248,"ancestor_record_offset":280,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":1852,"parent_record_offset":1820,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":2744,"parent_record_offset":2708,"ancestor_record_offset":280,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":6072,"parent_record_offset":6032,"ancestor_record_offset":280,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":280,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":1820,"parent_record_offset":1796,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":588,"parent_record_offset":552,"ancestor_record_offset":280,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":616,"parent_record_offset":588,"ancestor_record_offset":280,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":496,"parent_record_offset":468,"ancestor_record_offset":280,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":1104,"parent_record_offset":1076,"ancestor_record_offset":280,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":6032,"parent_record_offset":6000,"ancestor_record_offset":280,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3167,"record_offset":868,"parent_record_offset":552,"ancestor_record_offset":280,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":3167,"record_offset":524,"parent_record_offset":496,"ancestor_record_offset":280,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":3167,"record_offset":552,"parent_record_offset":524,"ancestor_record_offset":280,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":3167,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":280,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3167,"record_offset":4308,"parent_record_offset":3404,"ancestor_record_offset":280,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3167,"record_offset":2464,"parent_record_offset":2440,"ancestor_record_offset":280,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3167,"record_offset":6360,"parent_record_offset":6304,"ancestor_record_offset":280,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3167,"record_offset":964,"parent_record_offset":940,"ancestor_record_offset":280,"inlinee":10794,"source_line":129,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3167,"record_offset":1020,"parent_record_offset":988,"ancestor_record_offset":280,"inlinee":5430,"source_line":139,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3167,"record_offset":3072,"parent_record_offset":3032,"ancestor_record_offset":280,"inlinee":26654,"source_line":93,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3167,"record_offset":940,"parent_record_offset":916,"ancestor_record_offset":280,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3167,"record_offset":1048,"parent_record_offset":1020,"ancestor_record_offset":280,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":3167,"record_offset":6728,"parent_record_offset":6704,"ancestor_record_offset":280,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3167,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":280,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3167,"record_offset":1076,"parent_record_offset":1048,"ancestor_record_offset":280,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3167,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":280,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3167,"record_offset":2996,"parent_record_offset":2964,"ancestor_record_offset":280,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3167,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":280,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3167,"record_offset":6896,"parent_record_offset":6864,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3167,"record_offset":3964,"parent_record_offset":3932,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3167,"record_offset":3816,"parent_record_offset":3788,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3167,"record_offset":2288,"parent_record_offset":2260,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3167,"record_offset":3096,"parent_record_offset":3072,"ancestor_record_offset":280,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3167,"record_offset":1672,"parent_record_offset":1276,"ancestor_record_offset":280,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3167,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":280,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":1696,"parent_record_offset":1672,"ancestor_record_offset":280,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":6788,"parent_record_offset":6764,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3167,"record_offset":6644,"parent_record_offset":6620,"ancestor_record_offset":280,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3167,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":280,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3167,"record_offset":3272,"parent_record_offset":3164,"ancestor_record_offset":280,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3167,"record_offset":3244,"parent_record_offset":3164,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":1132,"parent_record_offset":1104,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3167,"record_offset":2964,"parent_record_offset":2936,"ancestor_record_offset":280,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3167,"record_offset":6620,"parent_record_offset":6596,"ancestor_record_offset":280,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3167,"record_offset":988,"parent_record_offset":964,"ancestor_record_offset":280,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":2404,"parent_record_offset":2376,"ancestor_record_offset":280,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":2376,"parent_record_offset":2348,"ancestor_record_offset":280,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":5172,"parent_record_offset":5148,"ancestor_record_offset":280,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":3760,"parent_record_offset":3732,"ancestor_record_offset":280,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":3788,"parent_record_offset":3760,"ancestor_record_offset":280,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":3932,"parent_record_offset":3760,"ancestor_record_offset":280,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":3584,"parent_record_offset":3552,"ancestor_record_offset":280,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3167,"record_offset":2440,"parent_record_offset":2404,"ancestor_record_offset":280,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] utils::distance_line_point
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3167,"record_offset":3476,"parent_record_offset":3404,"ancestor_record_offset":280,"inlinee":35270,"source_line":164,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3167,"record_offset":3672,"parent_record_offset":3552,"ancestor_record_offset":280,"inlinee":26202,"source_line":114,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3167,"record_offset":4144,"parent_record_offset":3476,"ancestor_record_offset":280,"inlinee":10939,"source_line":89,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::length_squared
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3167,"record_offset":5148,"parent_record_offset":3404,"ancestor_record_offset":280,"inlinee":117840,"source_line":94,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3167,"record_offset":3732,"parent_record_offset":3516,"ancestor_record_offset":280,"inlinee":26201,"source_line":109,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_line
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3167,"record_offset":3516,"parent_record_offset":3476,"ancestor_record_offset":280,"inlinee":26846,"source_line":179,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] utils::rdp::impl$0::new_with_opts::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":3167,"record_offset":3404,"parent_record_offset":3356,"ancestor_record_offset":280,"inlinee":35788,"source_line":127,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3167,"record_offset":3552,"parent_record_offset":3516,"ancestor_record_offset":280,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9274
// 首条位置: {"module":588,"inlinee":37492}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":588,"record_offset":21500,"parent_record_offset":21452,"ancestor_record_offset":21452,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":588,"record_offset":22668,"parent_record_offset":21452,"ancestor_record_offset":21452,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":588,"record_offset":21912,"parent_record_offset":21888,"ancestor_record_offset":21452,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":588,"record_offset":21936,"parent_record_offset":21912,"ancestor_record_offset":21452,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":588,"record_offset":22852,"parent_record_offset":22828,"ancestor_record_offset":21452,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":588,"record_offset":22876,"parent_record_offset":22852,"ancestor_record_offset":21452,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":588,"record_offset":21960,"parent_record_offset":21936,"ancestor_record_offset":21452,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":588,"record_offset":22900,"parent_record_offset":22876,"ancestor_record_offset":21452,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":588,"record_offset":21888,"parent_record_offset":21704,"ancestor_record_offset":21452,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":588,"record_offset":22828,"parent_record_offset":22740,"ancestor_record_offset":21452,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":588,"record_offset":21648,"parent_record_offset":21624,"ancestor_record_offset":21452,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":588,"record_offset":22768,"parent_record_offset":22740,"ancestor_record_offset":21452,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":588,"record_offset":22740,"parent_record_offset":22716,"ancestor_record_offset":21452,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":588,"record_offset":21704,"parent_record_offset":21672,"ancestor_record_offset":21452,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":588,"record_offset":21672,"parent_record_offset":21648,"ancestor_record_offset":21452,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":588,"record_offset":22716,"parent_record_offset":22692,"ancestor_record_offset":21452,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":588,"record_offset":21744,"parent_record_offset":21704,"ancestor_record_offset":21452,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":588,"record_offset":21592,"parent_record_offset":21528,"ancestor_record_offset":21452,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":588,"record_offset":21564,"parent_record_offset":21528,"ancestor_record_offset":21452,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":588,"record_offset":21624,"parent_record_offset":21592,"ancestor_record_offset":21452,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":588,"record_offset":21528,"parent_record_offset":21500,"ancestor_record_offset":21452,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":588,"record_offset":21768,"parent_record_offset":21744,"ancestor_record_offset":21452,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":588,"record_offset":21848,"parent_record_offset":21792,"ancestor_record_offset":21452,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":588,"record_offset":21792,"parent_record_offset":21768,"ancestor_record_offset":21452,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":588,"record_offset":21820,"parent_record_offset":21792,"ancestor_record_offset":21452,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":588,"record_offset":22796,"parent_record_offset":22768,"ancestor_record_offset":21452,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":588,"record_offset":22016,"parent_record_offset":21592,"ancestor_record_offset":21452,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":588,"record_offset":22040,"parent_record_offset":22016,"ancestor_record_offset":21452,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":588,"record_offset":22692,"parent_record_offset":22668,"ancestor_record_offset":21452,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::Rdp::stitch_with<glam::f32::vec2::Vec2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":567,"record_offset":81524,"rva":11816560,"va":5380525680}
// 编译实例 RVAs: 0xb44e70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::vec::Vec<_, _>>::remove::assert_failed
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t36094086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380525680,"raw_hex":"e886c02602","size":5,"soft_fail":false,"target_va":5416621000,"va":5380526909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4847156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380525680,"raw_hex":"e8cc09b6ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380525839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-4847891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380525680,"raw_hex":"e8ed06b6ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380526590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11803639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380525680,"raw_hex":"e809e44bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380525826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140997200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1761845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380525680,"raw_hex":"e8cb1de5ff","size":5,"soft_fail":false,"target_va":5378765312,"va":5380527152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35826235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380525680,"raw_hex":"e83baa2202","size":5,"soft_fail":false,"target_va":5416353040,"va":5380526800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36531155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380525680,"raw_hex":"e8d36b2d02","size":5,"soft_fail":false,"target_va":5417057056,"va":5380525896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36531121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380525680,"raw_hex":"e8b16b2d02","size":5,"soft_fail":false,"target_va":5417057072,"va":5380525946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36095002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380525680,"raw_hex":"e81ac42602","size":5,"soft_fail":false,"target_va":5416620794,"va":5380525787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36095548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380525680,"raw_hex":"e83cc62602","size":5,"soft_fail":false,"target_va":5416622320,"va":5380526767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t-4848031","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5380525680,"raw_hex":"e96106b6ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380526730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":81780,"parent_record_offset":81524,"ancestor_record_offset":81524,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":82376,"parent_record_offset":81524,"ancestor_record_offset":81524,"inlinee":6391,"source_line":2341,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":81680,"parent_record_offset":81524,"ancestor_record_offset":81524,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":84044,"parent_record_offset":81524,"ancestor_record_offset":81524,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$22::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":82952,"parent_record_offset":81524,"ancestor_record_offset":81524,"inlinee":11091,"source_line":4166,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":85120,"parent_record_offset":81524,"ancestor_record_offset":81524,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":82176,"parent_record_offset":82152,"ancestor_record_offset":81524,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":82200,"parent_record_offset":82176,"ancestor_record_offset":81524,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":85304,"parent_record_offset":85280,"ancestor_record_offset":81524,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":85328,"parent_record_offset":85304,"ancestor_record_offset":81524,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":82224,"parent_record_offset":82200,"ancestor_record_offset":81524,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":85352,"parent_record_offset":85328,"ancestor_record_offset":81524,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":82152,"parent_record_offset":81964,"ancestor_record_offset":81524,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":85280,"parent_record_offset":85192,"ancestor_record_offset":81524,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":82500,"parent_record_offset":82476,"ancestor_record_offset":81524,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":83084,"parent_record_offset":83056,"ancestor_record_offset":81524,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":81900,"parent_record_offset":81872,"ancestor_record_offset":81524,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":85220,"parent_record_offset":85192,"ancestor_record_offset":81524,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":85192,"parent_record_offset":85168,"ancestor_record_offset":81524,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":83144,"parent_record_offset":83112,"ancestor_record_offset":81524,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":82548,"parent_record_offset":82524,"ancestor_record_offset":81524,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":82524,"parent_record_offset":82500,"ancestor_record_offset":81524,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":83112,"parent_record_offset":83084,"ancestor_record_offset":81524,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":81964,"parent_record_offset":81928,"ancestor_record_offset":81524,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":81928,"parent_record_offset":81900,"ancestor_record_offset":81524,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":85168,"parent_record_offset":85144,"ancestor_record_offset":81524,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":82008,"parent_record_offset":81964,"ancestor_record_offset":81524,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":81808,"parent_record_offset":81780,"ancestor_record_offset":81524,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":81836,"parent_record_offset":81808,"ancestor_record_offset":81524,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":83008,"parent_record_offset":82980,"ancestor_record_offset":81524,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":82476,"parent_record_offset":82408,"ancestor_record_offset":81524,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":84108,"parent_record_offset":84076,"ancestor_record_offset":81524,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":82448,"parent_record_offset":82408,"ancestor_record_offset":81524,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":83056,"parent_record_offset":83008,"ancestor_record_offset":81524,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":82336,"parent_record_offset":81836,"ancestor_record_offset":81524,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::try_remove
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":82408,"parent_record_offset":82376,"ancestor_record_offset":81524,"inlinee":6392,"source_line":2377,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":81872,"parent_record_offset":81836,"ancestor_record_offset":81524,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":84692,"parent_record_offset":84648,"ancestor_record_offset":81524,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":567,"record_offset":85080,"parent_record_offset":84108,"ancestor_record_offset":81524,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":567,"record_offset":84908,"parent_record_offset":84880,"ancestor_record_offset":81524,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":567,"record_offset":84076,"parent_record_offset":84044,"ancestor_record_offset":81524,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":567,"record_offset":82980,"parent_record_offset":82952,"ancestor_record_offset":81524,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":567,"record_offset":82032,"parent_record_offset":82008,"ancestor_record_offset":81524,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":567,"record_offset":82112,"parent_record_offset":82056,"ancestor_record_offset":81524,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":567,"record_offset":84340,"parent_record_offset":84308,"ancestor_record_offset":81524,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":567,"record_offset":84472,"parent_record_offset":84372,"ancestor_record_offset":81524,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":84308,"parent_record_offset":84108,"ancestor_record_offset":81524,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":84648,"parent_record_offset":84472,"ancestor_record_offset":81524,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":82056,"parent_record_offset":82032,"ancestor_record_offset":81524,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":82084,"parent_record_offset":82056,"ancestor_record_offset":81524,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":85248,"parent_record_offset":85220,"ancestor_record_offset":81524,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":83168,"parent_record_offset":83144,"ancestor_record_offset":81524,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":84592,"parent_record_offset":84548,"ancestor_record_offset":81524,"inlinee":35784,"source_line":105,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$16::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":567,"record_offset":84548,"parent_record_offset":84504,"ancestor_record_offset":81524,"inlinee":35782,"source_line":27,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":82280,"parent_record_offset":81836,"ancestor_record_offset":81524,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":82304,"parent_record_offset":82280,"ancestor_record_offset":81524,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":84796,"parent_record_offset":84372,"ancestor_record_offset":81524,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":567,"record_offset":82588,"parent_record_offset":82408,"ancestor_record_offset":81524,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":84444,"parent_record_offset":84372,"ancestor_record_offset":81524,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":84736,"parent_record_offset":84692,"ancestor_record_offset":81524,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":81708,"parent_record_offset":81680,"ancestor_record_offset":81524,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":81736,"parent_record_offset":81708,"ancestor_record_offset":81524,"inlinee":11671,"source_line":696,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":84372,"parent_record_offset":84340,"ancestor_record_offset":81524,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::rdp::impl$0::stitch_with::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":567,"record_offset":84504,"parent_record_offset":84472,"ancestor_record_offset":81524,"inlinee":35780,"source_line":44,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::impl$0::stitch_with::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8BC4
// 首条位置: {"module":567,"inlinee":35780}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$16::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":567,"record_offset":84548,"parent_record_offset":84504,"ancestor_record_offset":84504,"inlinee":35782,"source_line":27,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":84592,"parent_record_offset":84548,"ancestor_record_offset":84504,"inlinee":35784,"source_line":105,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::Rdp::into_indices
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1384E
// 首条位置: {"module":1511,"inlinee":79950}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1511,"record_offset":984,"parent_record_offset":956,"ancestor_record_offset":956,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1511,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":956,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1511,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":956,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1511,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":956,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1511,"record_offset":1148,"parent_record_offset":1056,"ancestor_record_offset":956,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1511,"record_offset":1084,"parent_record_offset":1056,"ancestor_record_offset":956,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1511,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":956,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1511,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":956,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01511.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1511,"record_offset":1116,"parent_record_offset":1084,"ancestor_record_offset":956,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1511,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":956,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01511.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::Rdp::new<glam::f32::vec2::Vec2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":567,"record_offset":86888,"rva":11818064,"va":5380527184}
// 编译实例 RVAs: 0xb45450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4848609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380527184,"raw_hex":"e81f04b6ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380527292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-4849665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380527184,"raw_hex":"e8ffffb5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380528364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11805092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380527184,"raw_hex":"e85cde4bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380527279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416092a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11287023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380527184,"raw_hex":"e8ef39ac00","size":5,"soft_fail":false,"target_va":5391815328,"va":5380528300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35823463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380527184,"raw_hex":"e8679f2202","size":5,"soft_fail":false,"target_va":5416350944,"va":5380527476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36529682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380527184,"raw_hex":"e812662d02","size":5,"soft_fail":false,"target_va":5417057056,"va":5380527369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t35838526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380527184,"raw_hex":"e83eda2202","size":5,"soft_fail":false,"target_va":5416365968,"va":5380527437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36092606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380527184,"raw_hex":"e8beba2602","size":5,"soft_fail":false,"target_va":5416620794,"va":5380528183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36093662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380527184,"raw_hex":"e8debe2602","size":5,"soft_fail":false,"target_va":5416622208,"va":5380528541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36093133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380527184,"raw_hex":"e8cdbc2602","size":5,"soft_fail":false,"target_va":5416621762,"va":5380528624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36093750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380527184,"raw_hex":"e836bf2602","size":5,"soft_fail":false,"target_va":5416622320,"va":5380528565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::rdp::Rdp::new_with_opts
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":567,"record_offset":87016,"parent_record_offset":86888,"ancestor_record_offset":86888,"inlinee":35786,"source_line":81,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":87492,"parent_record_offset":87468,"ancestor_record_offset":86888,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":87516,"parent_record_offset":87492,"ancestor_record_offset":86888,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":92100,"parent_record_offset":92076,"ancestor_record_offset":86888,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":92124,"parent_record_offset":92100,"ancestor_record_offset":86888,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":87540,"parent_record_offset":87516,"ancestor_record_offset":86888,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":92148,"parent_record_offset":92124,"ancestor_record_offset":86888,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":87468,"parent_record_offset":87436,"ancestor_record_offset":86888,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":92076,"parent_record_offset":91984,"ancestor_record_offset":86888,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":88484,"parent_record_offset":88460,"ancestor_record_offset":86888,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":87376,"parent_record_offset":87348,"ancestor_record_offset":86888,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":92012,"parent_record_offset":91984,"ancestor_record_offset":86888,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":91984,"parent_record_offset":91960,"ancestor_record_offset":86888,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":88532,"parent_record_offset":88508,"ancestor_record_offset":86888,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":88508,"parent_record_offset":88484,"ancestor_record_offset":86888,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":87436,"parent_record_offset":87404,"ancestor_record_offset":86888,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":87404,"parent_record_offset":87376,"ancestor_record_offset":86888,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":91960,"parent_record_offset":91936,"ancestor_record_offset":86888,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":91404,"parent_record_offset":91364,"ancestor_record_offset":86888,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":87944,"parent_record_offset":87016,"ancestor_record_offset":86888,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":87972,"parent_record_offset":87944,"ancestor_record_offset":86888,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":88000,"parent_record_offset":87972,"ancestor_record_offset":86888,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":88460,"parent_record_offset":88428,"ancestor_record_offset":86888,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":89272,"parent_record_offset":89236,"ancestor_record_offset":86888,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":91220,"parent_record_offset":91180,"ancestor_record_offset":86888,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":87624,"parent_record_offset":87600,"ancestor_record_offset":86888,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":88348,"parent_record_offset":87016,"ancestor_record_offset":86888,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":89236,"parent_record_offset":87016,"ancestor_record_offset":86888,"inlinee":5450,"source_line":2790,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":88404,"parent_record_offset":87016,"ancestor_record_offset":86888,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":88428,"parent_record_offset":88404,"ancestor_record_offset":86888,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":87320,"parent_record_offset":87284,"ancestor_record_offset":86888,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":87348,"parent_record_offset":87320,"ancestor_record_offset":86888,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":91148,"parent_record_offset":87016,"ancestor_record_offset":86888,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":87228,"parent_record_offset":87200,"ancestor_record_offset":86888,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":87828,"parent_record_offset":87804,"ancestor_record_offset":86888,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":91180,"parent_record_offset":91148,"ancestor_record_offset":86888,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":567,"record_offset":87600,"parent_record_offset":87284,"ancestor_record_offset":86888,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":567,"record_offset":87256,"parent_record_offset":87228,"ancestor_record_offset":86888,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":567,"record_offset":87284,"parent_record_offset":87256,"ancestor_record_offset":86888,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":567,"record_offset":91428,"parent_record_offset":91404,"ancestor_record_offset":86888,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":567,"record_offset":90448,"parent_record_offset":89800,"ancestor_record_offset":86888,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":567,"record_offset":88992,"parent_record_offset":88968,"ancestor_record_offset":86888,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":567,"record_offset":91508,"parent_record_offset":91452,"ancestor_record_offset":86888,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":567,"record_offset":87696,"parent_record_offset":87672,"ancestor_record_offset":86888,"inlinee":10794,"source_line":129,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":567,"record_offset":87752,"parent_record_offset":87720,"ancestor_record_offset":86888,"inlinee":5430,"source_line":139,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":567,"record_offset":89600,"parent_record_offset":89564,"ancestor_record_offset":86888,"inlinee":26654,"source_line":93,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":567,"record_offset":87672,"parent_record_offset":87648,"ancestor_record_offset":86888,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":567,"record_offset":87780,"parent_record_offset":87752,"ancestor_record_offset":86888,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":567,"record_offset":89564,"parent_record_offset":87016,"ancestor_record_offset":86888,"inlinee":15216,"source_line":122,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":567,"record_offset":91876,"parent_record_offset":91852,"ancestor_record_offset":86888,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":87200,"parent_record_offset":87016,"ancestor_record_offset":86888,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":87648,"parent_record_offset":87624,"ancestor_record_offset":86888,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":87804,"parent_record_offset":87780,"ancestor_record_offset":86888,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":91452,"parent_record_offset":91428,"ancestor_record_offset":86888,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":89528,"parent_record_offset":89496,"ancestor_record_offset":86888,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":91480,"parent_record_offset":91452,"ancestor_record_offset":86888,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":92044,"parent_record_offset":92012,"ancestor_record_offset":86888,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":90196,"parent_record_offset":90172,"ancestor_record_offset":86888,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":90112,"parent_record_offset":90088,"ancestor_record_offset":86888,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":88856,"parent_record_offset":88832,"ancestor_record_offset":86888,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":567,"record_offset":89624,"parent_record_offset":89600,"ancestor_record_offset":86888,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":88280,"parent_record_offset":88000,"ancestor_record_offset":86888,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":88304,"parent_record_offset":88280,"ancestor_record_offset":86888,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":91912,"parent_record_offset":87016,"ancestor_record_offset":86888,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":567,"record_offset":90544,"parent_record_offset":90520,"ancestor_record_offset":86888,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":89736,"parent_record_offset":89688,"ancestor_record_offset":86888,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":87856,"parent_record_offset":87828,"ancestor_record_offset":86888,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":567,"record_offset":90496,"parent_record_offset":87016,"ancestor_record_offset":86888,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":89468,"parent_record_offset":87016,"ancestor_record_offset":86888,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":89496,"parent_record_offset":89468,"ancestor_record_offset":86888,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":567,"record_offset":90520,"parent_record_offset":90496,"ancestor_record_offset":86888,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":87720,"parent_record_offset":87696,"ancestor_record_offset":86888,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":88936,"parent_record_offset":88912,"ancestor_record_offset":86888,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":88912,"parent_record_offset":88888,"ancestor_record_offset":86888,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":90064,"parent_record_offset":90040,"ancestor_record_offset":86888,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":90088,"parent_record_offset":90064,"ancestor_record_offset":86888,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":90172,"parent_record_offset":90064,"ancestor_record_offset":86888,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":88832,"parent_record_offset":87016,"ancestor_record_offset":86888,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":567,"record_offset":88968,"parent_record_offset":88936,"ancestor_record_offset":86888,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::distance_line_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":89840,"parent_record_offset":89800,"ancestor_record_offset":86888,"inlinee":35270,"source_line":164,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":89980,"parent_record_offset":89896,"ancestor_record_offset":86888,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":88888,"parent_record_offset":87016,"ancestor_record_offset":86888,"inlinee":14720,"source_line":58,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":90040,"parent_record_offset":89868,"ancestor_record_offset":86888,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_line
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":89868,"parent_record_offset":89840,"ancestor_record_offset":86888,"inlinee":26846,"source_line":179,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::rdp::impl$0::new_with_opts::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":567,"record_offset":89800,"parent_record_offset":89764,"ancestor_record_offset":86888,"inlinee":35788,"source_line":127,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":89896,"parent_record_offset":89868,"ancestor_record_offset":86888,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::Rdp::new_with_opts
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8BCA
// 首条位置: {"module":567,"inlinee":35786}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":87944,"parent_record_offset":87016,"ancestor_record_offset":87016,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":88348,"parent_record_offset":87016,"ancestor_record_offset":87016,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":89236,"parent_record_offset":87016,"ancestor_record_offset":87016,"inlinee":5450,"source_line":2790,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":88404,"parent_record_offset":87016,"ancestor_record_offset":87016,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":91148,"parent_record_offset":87016,"ancestor_record_offset":87016,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":567,"record_offset":89564,"parent_record_offset":87016,"ancestor_record_offset":87016,"inlinee":15216,"source_line":122,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":87200,"parent_record_offset":87016,"ancestor_record_offset":87016,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":91912,"parent_record_offset":87016,"ancestor_record_offset":87016,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":567,"record_offset":90496,"parent_record_offset":87016,"ancestor_record_offset":87016,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":89468,"parent_record_offset":87016,"ancestor_record_offset":87016,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":88832,"parent_record_offset":87016,"ancestor_record_offset":87016,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":88888,"parent_record_offset":87016,"ancestor_record_offset":87016,"inlinee":14720,"source_line":58,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":87492,"parent_record_offset":87468,"ancestor_record_offset":87016,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":87516,"parent_record_offset":87492,"ancestor_record_offset":87016,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":92100,"parent_record_offset":92076,"ancestor_record_offset":87016,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":92124,"parent_record_offset":92100,"ancestor_record_offset":87016,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":87540,"parent_record_offset":87516,"ancestor_record_offset":87016,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":92148,"parent_record_offset":92124,"ancestor_record_offset":87016,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":87468,"parent_record_offset":87436,"ancestor_record_offset":87016,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":92076,"parent_record_offset":91984,"ancestor_record_offset":87016,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":88484,"parent_record_offset":88460,"ancestor_record_offset":87016,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":87376,"parent_record_offset":87348,"ancestor_record_offset":87016,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":92012,"parent_record_offset":91984,"ancestor_record_offset":87016,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":91984,"parent_record_offset":91960,"ancestor_record_offset":87016,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":88532,"parent_record_offset":88508,"ancestor_record_offset":87016,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":88508,"parent_record_offset":88484,"ancestor_record_offset":87016,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":87436,"parent_record_offset":87404,"ancestor_record_offset":87016,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":87404,"parent_record_offset":87376,"ancestor_record_offset":87016,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":91960,"parent_record_offset":91936,"ancestor_record_offset":87016,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":91404,"parent_record_offset":91364,"ancestor_record_offset":87016,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":87972,"parent_record_offset":87944,"ancestor_record_offset":87016,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":88000,"parent_record_offset":87972,"ancestor_record_offset":87016,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":88460,"parent_record_offset":88428,"ancestor_record_offset":87016,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":89272,"parent_record_offset":89236,"ancestor_record_offset":87016,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":91220,"parent_record_offset":91180,"ancestor_record_offset":87016,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":87624,"parent_record_offset":87600,"ancestor_record_offset":87016,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":88428,"parent_record_offset":88404,"ancestor_record_offset":87016,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":87320,"parent_record_offset":87284,"ancestor_record_offset":87016,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":87348,"parent_record_offset":87320,"ancestor_record_offset":87016,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":87228,"parent_record_offset":87200,"ancestor_record_offset":87016,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":87828,"parent_record_offset":87804,"ancestor_record_offset":87016,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":91180,"parent_record_offset":91148,"ancestor_record_offset":87016,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":567,"record_offset":87600,"parent_record_offset":87284,"ancestor_record_offset":87016,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":567,"record_offset":87256,"parent_record_offset":87228,"ancestor_record_offset":87016,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":567,"record_offset":87284,"parent_record_offset":87256,"ancestor_record_offset":87016,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":567,"record_offset":91428,"parent_record_offset":91404,"ancestor_record_offset":87016,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":567,"record_offset":90448,"parent_record_offset":89800,"ancestor_record_offset":87016,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":567,"record_offset":88992,"parent_record_offset":88968,"ancestor_record_offset":87016,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":567,"record_offset":91508,"parent_record_offset":91452,"ancestor_record_offset":87016,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":567,"record_offset":87696,"parent_record_offset":87672,"ancestor_record_offset":87016,"inlinee":10794,"source_line":129,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":567,"record_offset":87752,"parent_record_offset":87720,"ancestor_record_offset":87016,"inlinee":5430,"source_line":139,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":567,"record_offset":89600,"parent_record_offset":89564,"ancestor_record_offset":87016,"inlinee":26654,"source_line":93,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":567,"record_offset":87672,"parent_record_offset":87648,"ancestor_record_offset":87016,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":567,"record_offset":87780,"parent_record_offset":87752,"ancestor_record_offset":87016,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":567,"record_offset":91876,"parent_record_offset":91852,"ancestor_record_offset":87016,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":87648,"parent_record_offset":87624,"ancestor_record_offset":87016,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":87804,"parent_record_offset":87780,"ancestor_record_offset":87016,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":91452,"parent_record_offset":91428,"ancestor_record_offset":87016,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":89528,"parent_record_offset":89496,"ancestor_record_offset":87016,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":91480,"parent_record_offset":91452,"ancestor_record_offset":87016,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":92044,"parent_record_offset":92012,"ancestor_record_offset":87016,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":90196,"parent_record_offset":90172,"ancestor_record_offset":87016,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":90112,"parent_record_offset":90088,"ancestor_record_offset":87016,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":88856,"parent_record_offset":88832,"ancestor_record_offset":87016,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":567,"record_offset":89624,"parent_record_offset":89600,"ancestor_record_offset":87016,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":88280,"parent_record_offset":88000,"ancestor_record_offset":87016,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":88304,"parent_record_offset":88280,"ancestor_record_offset":87016,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":91936,"parent_record_offset":91912,"ancestor_record_offset":87016,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":567,"record_offset":90544,"parent_record_offset":90520,"ancestor_record_offset":87016,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":89736,"parent_record_offset":89688,"ancestor_record_offset":87016,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":87856,"parent_record_offset":87828,"ancestor_record_offset":87016,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":89496,"parent_record_offset":89468,"ancestor_record_offset":87016,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":567,"record_offset":90520,"parent_record_offset":90496,"ancestor_record_offset":87016,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":87720,"parent_record_offset":87696,"ancestor_record_offset":87016,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":88936,"parent_record_offset":88912,"ancestor_record_offset":87016,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":88912,"parent_record_offset":88888,"ancestor_record_offset":87016,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":90064,"parent_record_offset":90040,"ancestor_record_offset":87016,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":90088,"parent_record_offset":90064,"ancestor_record_offset":87016,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":90172,"parent_record_offset":90064,"ancestor_record_offset":87016,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":89924,"parent_record_offset":89896,"ancestor_record_offset":87016,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":567,"record_offset":88968,"parent_record_offset":88936,"ancestor_record_offset":87016,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::distance_line_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":89840,"parent_record_offset":89800,"ancestor_record_offset":87016,"inlinee":35270,"source_line":164,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":89980,"parent_record_offset":89896,"ancestor_record_offset":87016,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":90296,"parent_record_offset":89840,"ancestor_record_offset":87016,"inlinee":14720,"source_line":58,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":90040,"parent_record_offset":89868,"ancestor_record_offset":87016,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_line
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":89868,"parent_record_offset":89840,"ancestor_record_offset":87016,"inlinee":26846,"source_line":179,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::rdp::impl$0::new_with_opts::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":567,"record_offset":89800,"parent_record_offset":89764,"ancestor_record_offset":87016,"inlinee":35788,"source_line":127,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":89896,"parent_record_offset":89868,"ancestor_record_offset":87016,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::impl$0::new_with_opts::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8BCC
// 首条位置: {"module":567,"inlinee":35788}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":567,"record_offset":90448,"parent_record_offset":89800,"ancestor_record_offset":89800,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":5232,"parent_record_offset":3404,"ancestor_record_offset":3404,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] utils::distance_line_point
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":89840,"parent_record_offset":89800,"ancestor_record_offset":89800,"inlinee":35270,"source_line":164,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length_squared
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3167,"record_offset":5148,"parent_record_offset":3404,"ancestor_record_offset":3404,"inlinee":117840,"source_line":94,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":567,"record_offset":90400,"parent_record_offset":90376,"ancestor_record_offset":89800,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":90196,"parent_record_offset":90172,"ancestor_record_offset":89800,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":90112,"parent_record_offset":90088,"ancestor_record_offset":89800,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":89948,"parent_record_offset":89924,"ancestor_record_offset":89800,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":90004,"parent_record_offset":89980,"ancestor_record_offset":89800,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":90320,"parent_record_offset":90296,"ancestor_record_offset":89800,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":90064,"parent_record_offset":90040,"ancestor_record_offset":89800,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":90088,"parent_record_offset":90064,"ancestor_record_offset":89800,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":90172,"parent_record_offset":90064,"ancestor_record_offset":89800,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":89924,"parent_record_offset":89896,"ancestor_record_offset":89800,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":3696,"parent_record_offset":3672,"ancestor_record_offset":3404,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":4172,"parent_record_offset":4144,"ancestor_record_offset":3404,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":5172,"parent_record_offset":5148,"ancestor_record_offset":3404,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":3760,"parent_record_offset":3732,"ancestor_record_offset":3404,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":3788,"parent_record_offset":3760,"ancestor_record_offset":3404,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":3932,"parent_record_offset":3760,"ancestor_record_offset":3404,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":3584,"parent_record_offset":3552,"ancestor_record_offset":3404,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":567,"record_offset":90376,"parent_record_offset":90344,"ancestor_record_offset":89800,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":89980,"parent_record_offset":89896,"ancestor_record_offset":89800,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":90296,"parent_record_offset":89840,"ancestor_record_offset":89800,"inlinee":14720,"source_line":58,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":90040,"parent_record_offset":89868,"ancestor_record_offset":89800,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3167,"record_offset":3672,"parent_record_offset":3552,"ancestor_record_offset":3404,"inlinee":26202,"source_line":114,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3167,"record_offset":4144,"parent_record_offset":3476,"ancestor_record_offset":3404,"inlinee":10939,"source_line":89,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3167,"record_offset":3732,"parent_record_offset":3516,"ancestor_record_offset":3404,"inlinee":26201,"source_line":109,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_line
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":89868,"parent_record_offset":89840,"ancestor_record_offset":89800,"inlinee":26846,"source_line":179,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":89896,"parent_record_offset":89868,"ancestor_record_offset":89800,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::impl$0::new_with_opts::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1360C
// 首条位置: {"module":1455,"inlinee":79372}
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
// 完整结构及原始类型记录: rdp.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::rdp::Rdp
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 8; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "utils::rdp::Rdp<glam::f32::vec2::Vec2>", "constructor": "utils::rdp::Rdp", "arguments": ["glam::f32::vec2::Vec2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<utils::rdp::Rdp<glam::f32::vec2::Vec2> >", "constructor": "core::ptr::drop_in_place", "arguments": ["utils::rdp::Rdp<glam::f32::vec2::Vec2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::rdp::Rdp::stitch_with<glam::f32::vec2::Vec2>", "constructor": "utils::rdp::Rdp::stitch_with", "arguments": ["glam::f32::vec2::Vec2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::rdp::Rdp::new<glam::f32::vec2::Vec2>", "constructor": "utils::rdp::Rdp::new", "arguments": ["glam::f32::vec2::Vec2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>", "constructor": "utils::rdp::Rdp::new_with_opts", "arguments": ["glam::f32::vec3::Vec3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: rdp.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: rdp.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 31; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
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
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: utils::rdp::Rdp::into_indices
// 信息 ID: 1c6873dc30f6054a8d538848
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1511, inline_record=956; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::Rdp::new<glam::f32::vec2::Vec2>
// 信息 ID: f281e5bf5df7b062cb32df5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::rdp::Rdp::new -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve @ 0xc0889b -> 0xb45450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve @ 0xc083f1 -> 0xb45450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve @ 0xc08289 -> 0xb45450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::in_progress_curve::InProgressCurve::new @ 0xc091db -> 0xb45450 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::Rdp::new_with_opts
// 信息 ID: 56882c6fff40522981c887a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=567, inline_record=87016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>
// 信息 ID: 0c0dbe5f6b20cbe6a8dc12bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::rdp::Rdp::new_with_opts -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x1479d18 -> 0x24e9b40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::Rdp::stitch_with<glam::f32::vec2::Vec2>
// 信息 ID: d92f87edd0388e63d593cdbf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::rdp::Rdp::stitch_with -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve @ 0xc08407 -> 0xb44e70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::impl$0::new_with_opts::closure$1
// 信息 ID: 282074061b2ec02599e2559d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::rdp::impl$0::new_with_opts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=567, inline_record=89800; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=3167, inline_record=3404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::impl$0::new_with_opts::closure$2
// 信息 ID: 029757934491acc18692d64f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "utils::rdp::impl$0::new_with_opts", "captures": "unknown"}
// [闭包环境类型引用] utils::rdp::impl$0::new_with_opts::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<usize>,utils::rdp::impl$0::new_with_opts::closure_env$2<glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::rdp::impl$0::new_with_opts::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::slice::iter::Iter<usize>,utils::rdp::impl$0::new_with_opts::closure_env$2<glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<usize>,utils::rdp::impl$0::new_with_opts::closure_env$2<glam::f32::vec3::Vec3> > >; module=1455, inline_record=160884; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::slice::iter::Iter<usize>,utils::rdp::impl$0::new_with_opts::closure_env$2<glam::f32::vec2::Vec2> > >; module=1643, inline_record=533888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::impl$0::stitch_with::closure$0
// 信息 ID: 76a16777fa5e3e724bf8d5ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::rdp::impl$0::stitch_with", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::rdp::Rdp::stitch_with<glam::f32::vec2::Vec2>; module=567, inline_record=84504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::impl$1::clone
// 信息 ID: 6f037c7603c5ee7cc9a66064
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=588, inline_record=21452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
