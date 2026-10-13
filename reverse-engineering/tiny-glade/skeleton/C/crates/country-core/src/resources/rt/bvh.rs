// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/rt/bvh.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e4f58eb2c189dcc41edff35d74b9dd4601c91360169dbc6c01594cacd69f706d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: bvh.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1612, 1643, 1682
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
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use obvhs; // namespace candidate
//   package identities: [{"name":"obvhs","source":"git+https://github.com/h3r2tic/obvhs.git?rev=095a626f11b78a0b2bda3f28f4e7e35dac4e4378#095a626f11b78a0b2bda3f28f4e7e35dac4e4378","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh::build_gpu_bvh
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1612,"record_offset":62468,"rva":22021888,"va":5390731008}
// 编译实例 RVAs: 0x1500700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6083387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390731008,"raw_hex":"e8c52ca3ff","size":5,"soft_fail":false,"target_va":5384648032,"va":5390731414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6122354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390731008,"raw_hex":"e88e94a2ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5390731757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15052434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390731008,"raw_hex":"e86e511aff","size":5,"soft_fail":false,"target_va":5375678688,"va":5390731117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-15053220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390731008,"raw_hex":"e85c4e1aff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390731919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: bvh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22008911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390731008,"raw_hex":"e8b12bb0fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390731098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098c380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bvh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12011491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390731008,"raw_hex":"e81db848ff","size":5,"soft_fail":false,"target_va":5378720640,"va":5390732126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f33320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bvh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6084120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390731008,"raw_hex":"e8e829a3ff","size":5,"soft_fail":false,"target_va":5384647456,"va":5390731571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415fd120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bvh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1033982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390731008,"raw_hex":"e8fec60f00","size":5,"soft_fail":false,"target_va":5391765792,"va":5390731805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141603840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bvh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1060349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390731008,"raw_hex":"e8fd2d1000","size":5,"soft_fail":false,"target_va":5391792192,"va":5390731838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25889721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390731008,"raw_hex":"e8b90b8b01","size":5,"soft_fail":false,"target_va":5416620794,"va":5390731068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] obvhs::cwbvh::builder::build_cwbvh_from_tris
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"obvhs","version":"0.1.0","source":"git+https://github.com/h3r2tic/obvhs.git?rev=095a626f11b78a0b2bda3f28f4e7e35dac4e4378#095a626f11b78a0b2bda3f28f4e7e35dac4e4378"}]
//     evidence={"assembly":"\tcallq\t16841625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390731008,"raw_hex":"e899fb0001","size":5,"soft_fail":false,"target_va":5407573152,"va":5390731522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1612,"record_offset":64352,"parent_record_offset":62468,"ancestor_record_offset":62468,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1612,"record_offset":62624,"parent_record_offset":62468,"ancestor_record_offset":62468,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1612,"record_offset":63956,"parent_record_offset":62468,"ancestor_record_offset":62468,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1612,"record_offset":64520,"parent_record_offset":62468,"ancestor_record_offset":62468,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::rt::bvh::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/bvh.rs
//     evidence={"module":1612,"record_offset":63928,"parent_record_offset":62468,"ancestor_record_offset":62468,"inlinee":80859,"source_line":132,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1612,"record_offset":64408,"parent_record_offset":62468,"ancestor_record_offset":62468,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1612,"record_offset":64268,"parent_record_offset":62468,"ancestor_record_offset":62468,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1612,"record_offset":63020,"parent_record_offset":62996,"ancestor_record_offset":62468,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1612,"record_offset":63044,"parent_record_offset":63020,"ancestor_record_offset":62468,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1612,"record_offset":64164,"parent_record_offset":64140,"ancestor_record_offset":62468,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1612,"record_offset":64188,"parent_record_offset":64164,"ancestor_record_offset":62468,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1612,"record_offset":63068,"parent_record_offset":63044,"ancestor_record_offset":62468,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1612,"record_offset":64212,"parent_record_offset":64188,"ancestor_record_offset":62468,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1612,"record_offset":62996,"parent_record_offset":62848,"ancestor_record_offset":62468,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1612,"record_offset":64140,"parent_record_offset":64040,"ancestor_record_offset":62468,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1612,"record_offset":62792,"parent_record_offset":62768,"ancestor_record_offset":62468,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1612,"record_offset":64072,"parent_record_offset":64040,"ancestor_record_offset":62468,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1612,"record_offset":64040,"parent_record_offset":64012,"ancestor_record_offset":62468,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1612,"record_offset":62848,"parent_record_offset":62816,"ancestor_record_offset":62468,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1612,"record_offset":62816,"parent_record_offset":62792,"ancestor_record_offset":62468,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1612,"record_offset":64012,"parent_record_offset":63984,"ancestor_record_offset":62468,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1612,"record_offset":62884,"parent_record_offset":62848,"ancestor_record_offset":62468,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1612,"record_offset":63156,"parent_record_offset":63128,"ancestor_record_offset":62468,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1612,"record_offset":62744,"parent_record_offset":62708,"ancestor_record_offset":62468,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1612,"record_offset":62768,"parent_record_offset":62744,"ancestor_record_offset":62468,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1612,"record_offset":62652,"parent_record_offset":62624,"ancestor_record_offset":62468,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1612,"record_offset":63732,"parent_record_offset":63700,"ancestor_record_offset":62468,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1612,"record_offset":63824,"parent_record_offset":63732,"ancestor_record_offset":62468,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1612,"record_offset":63128,"parent_record_offset":62708,"ancestor_record_offset":62468,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1612,"record_offset":62680,"parent_record_offset":62652,"ancestor_record_offset":62468,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1612,"record_offset":62708,"parent_record_offset":62680,"ancestor_record_offset":62468,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1612,"record_offset":62908,"parent_record_offset":62884,"ancestor_record_offset":62468,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1612,"record_offset":63372,"parent_record_offset":63308,"ancestor_record_offset":62468,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1612,"record_offset":62956,"parent_record_offset":62932,"ancestor_record_offset":62468,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1612,"record_offset":63212,"parent_record_offset":63184,"ancestor_record_offset":62468,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1612,"record_offset":63280,"parent_record_offset":63240,"ancestor_record_offset":62468,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1612,"record_offset":64652,"parent_record_offset":64628,"ancestor_record_offset":62468,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1612,"record_offset":63184,"parent_record_offset":63156,"ancestor_record_offset":62468,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1612,"record_offset":63700,"parent_record_offset":63280,"ancestor_record_offset":62468,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1612,"record_offset":62932,"parent_record_offset":62908,"ancestor_record_offset":62468,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1612,"record_offset":64108,"parent_record_offset":64072,"ancestor_record_offset":62468,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1612,"record_offset":64376,"parent_record_offset":64352,"ancestor_record_offset":62468,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1612,"record_offset":63984,"parent_record_offset":63956,"ancestor_record_offset":62468,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1612,"record_offset":64568,"parent_record_offset":64544,"ancestor_record_offset":62468,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1612,"record_offset":63864,"parent_record_offset":63240,"ancestor_record_offset":62468,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1612,"record_offset":63772,"parent_record_offset":63732,"ancestor_record_offset":62468,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1612,"record_offset":64544,"parent_record_offset":64520,"ancestor_record_offset":62468,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1612,"record_offset":63240,"parent_record_offset":63212,"ancestor_record_offset":62468,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1612,"record_offset":64292,"parent_record_offset":64268,"ancestor_record_offset":62468,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1612,"record_offset":64316,"parent_record_offset":64292,"ancestor_record_offset":62468,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::rt::bvh::build_gpu_bvh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/bvh.rs
//     evidence={"module":1612,"record_offset":63308,"parent_record_offset":63280,"ancestor_record_offset":62468,"inlinee":80861,"source_line":53,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1612,"record_offset":63436,"parent_record_offset":63404,"ancestor_record_offset":62468,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1612,"record_offset":63404,"parent_record_offset":63372,"ancestor_record_offset":62468,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1612,"record_offset":64456,"parent_record_offset":64432,"ancestor_record_offset":62468,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1612,"record_offset":64432,"parent_record_offset":64408,"ancestor_record_offset":62468,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1612,"record_offset":64480,"parent_record_offset":64456,"ancestor_record_offset":62468,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh::build_gpu_bvh::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13BDD
// 首条位置: {"module":1612,"inlinee":80861}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1612,"record_offset":63372,"parent_record_offset":63308,"ancestor_record_offset":63308,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1612,"record_offset":63436,"parent_record_offset":63404,"ancestor_record_offset":63308,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1612,"record_offset":63404,"parent_record_offset":63372,"ancestor_record_offset":63308,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh::build_gpu_bvh::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D8C
// 首条位置: {"module":1643,"inlinee":81292}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":346772,"parent_record_offset":346736,"ancestor_record_offset":346736,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1643,"record_offset":346804,"parent_record_offset":346772,"ancestor_record_offset":346736,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1643,"record_offset":346836,"parent_record_offset":346804,"ancestor_record_offset":346736,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh::build_gpu_bvh::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DA3
// 首条位置: {"module":1643,"inlinee":81315}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13BDB
// 首条位置: {"module":1612,"inlinee":80859}
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
// 完整结构及原始类型记录: bvh.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::rt::bvh::GpuBlBvh
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::rt::bvh::GpuBlBvh>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::rt::bvh::GpuBlBvh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: bvh.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: bvh.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 27; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] f32 (*)(f32,f32)
// 候选参数类型: ["f32", "f32"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] ref$<str$> (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > (*)(tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>)
// 候选参数类型: ["tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)
// 候选参数类型: ["rhapsody::raw::buffer::Buffer"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh::build_gpu_bvh
// 信息 ID: 1fa9ad3aeadd2deeaa976d58
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::rt::bvh_builder::bvh_builder_worker @ 0xc44b4d -> 0x1500700 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh::build_gpu_bvh::closure$0
// 信息 ID: b9eac17f05f7d0493b8fce76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::rt::bvh::build_gpu_bvh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::rt::bvh::build_gpu_bvh; module=1612, inline_record=63308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh::build_gpu_bvh::closure$1
// 信息 ID: 3b2d62a56ceb0246a54aa34f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::rt::bvh::build_gpu_bvh", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::rt::GpuRtTriangle,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::rt::GpuRtTriangle,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1> >; module=1643, inline_record=346736; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh::build_gpu_bvh::closure$2
// 信息 ID: 55c17e04b374d1a941fc9ce9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::resources::rt::bvh::build_gpu_bvh", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::rt::bvh::build_gpu_bvh::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$2> >; module=1643, inline_record=453208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh::impl$1::default
// 信息 ID: 8ef17b1b4f7c2bb0c771035b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::rt::bvh::build_gpu_bvh; module=1612, inline_record=63928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
