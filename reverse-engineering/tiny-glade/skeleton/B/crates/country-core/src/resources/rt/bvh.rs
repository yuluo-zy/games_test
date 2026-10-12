// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/rt/bvh.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e4f58eb2c189dcc41edff35d74b9dd4601c91360169dbc6c01594cacd69f706d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: bvh.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 577, 1585, 1610
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
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use obvhs; // namespace candidate
//   package identities: [{"name":"obvhs","source":"git+https://github.com/h3r2tic/obvhs.git?rev=a3f3b3bfad18f3dce0b0505a927200b3575477d0#a3f3b3bfad18f3dce0b0505a927200b3575477d0","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh::build_gpu_bvh
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":577,"record_offset":11116,"rva":11590224,"va":5380299344}
// 编译实例 RVAs: 0xb0da50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2344366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380299344,"raw_hex":"e8aec52300","size":5,"soft_fail":false,"target_va":5382644528,"va":5380300157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: bvh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5217976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380299344,"raw_hex":"e84861b0ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380299459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: bvh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5218883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380299344,"raw_hex":"e8bd5db0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380300382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: bvh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4783966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380299344,"raw_hex":"e8a200b7ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5380299993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140886ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bvh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2650277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380299344,"raw_hex":"e85b8fd7ff","size":5,"soft_fail":false,"target_va":5377650416,"va":5380300688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d63680
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bvh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2447825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380299344,"raw_hex":"e8d1592500","size":5,"soft_fail":false,"target_va":5382747776,"va":5380299946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141309ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bvh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8371496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380299344,"raw_hex":"e828bd7f00","size":5,"soft_fail":false,"target_va":5388671712,"va":5380300211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31240433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380299344,"raw_hex":"e8f1b0dc01","size":5,"soft_fail":false,"target_va":5411540963,"va":5380300525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31241786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380299344,"raw_hex":"e83ab6dc01","size":5,"soft_fail":false,"target_va":5411542296,"va":5380300505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] obvhs::cwbvh::builder::build_cwbvh_from_tris
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"obvhs","version":"0.1.0","source":"git+https://github.com/h3r2tic/obvhs.git?rev=a3f3b3bfad18f3dce0b0505a927200b3575477d0#a3f3b3bfad18f3dce0b0505a927200b3575477d0"}]
//     evidence={"assembly":"\tcallq\t13350736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380299344,"raw_hex":"e850b7cb00","size":5,"soft_fail":false,"target_va":5393650640,"va":5380299899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2447813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380299344,"raw_hex":"e8c5592500","size":5,"soft_fail":false,"target_va":5382747616,"va":5380299798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":577,"record_offset":13004,"parent_record_offset":11116,"ancestor_record_offset":11116,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":11272,"parent_record_offset":11116,"ancestor_record_offset":11116,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":12688,"parent_record_offset":11116,"ancestor_record_offset":11116,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":577,"record_offset":13212,"parent_record_offset":11116,"ancestor_record_offset":11116,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::rt::bvh::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/bvh.rs
//     evidence={"module":577,"record_offset":12660,"parent_record_offset":11116,"ancestor_record_offset":11116,"inlinee":33008,"source_line":132,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":577,"record_offset":13036,"parent_record_offset":11116,"ancestor_record_offset":11116,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":12920,"parent_record_offset":11116,"ancestor_record_offset":11116,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":11728,"parent_record_offset":11704,"ancestor_record_offset":11116,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":11752,"parent_record_offset":11728,"ancestor_record_offset":11116,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":12872,"parent_record_offset":12848,"ancestor_record_offset":11116,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":11704,"parent_record_offset":11528,"ancestor_record_offset":11116,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":12848,"parent_record_offset":12760,"ancestor_record_offset":11116,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":11468,"parent_record_offset":11440,"ancestor_record_offset":11116,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":12788,"parent_record_offset":12760,"ancestor_record_offset":11116,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":12760,"parent_record_offset":12736,"ancestor_record_offset":11116,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":11528,"parent_record_offset":11496,"ancestor_record_offset":11116,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":11496,"parent_record_offset":11468,"ancestor_record_offset":11116,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":12736,"parent_record_offset":12712,"ancestor_record_offset":11116,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":11564,"parent_record_offset":11528,"ancestor_record_offset":11116,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":11836,"parent_record_offset":11808,"ancestor_record_offset":11116,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":11412,"parent_record_offset":11368,"ancestor_record_offset":11116,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":11440,"parent_record_offset":11412,"ancestor_record_offset":11116,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":11304,"parent_record_offset":11272,"ancestor_record_offset":11116,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":12468,"parent_record_offset":12436,"ancestor_record_offset":11116,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":577,"record_offset":12556,"parent_record_offset":12468,"ancestor_record_offset":11116,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":577,"record_offset":11808,"parent_record_offset":11368,"ancestor_record_offset":11116,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":577,"record_offset":11336,"parent_record_offset":11304,"ancestor_record_offset":11116,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":577,"record_offset":11368,"parent_record_offset":11336,"ancestor_record_offset":11116,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":577,"record_offset":11588,"parent_record_offset":11564,"ancestor_record_offset":11116,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":577,"record_offset":11612,"parent_record_offset":11588,"ancestor_record_offset":11116,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":577,"record_offset":12108,"parent_record_offset":12044,"ancestor_record_offset":11116,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":577,"record_offset":11892,"parent_record_offset":11864,"ancestor_record_offset":11116,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":577,"record_offset":12016,"parent_record_offset":11920,"ancestor_record_offset":11116,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":577,"record_offset":13344,"parent_record_offset":13320,"ancestor_record_offset":11116,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":11864,"parent_record_offset":11836,"ancestor_record_offset":11116,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":577,"record_offset":12436,"parent_record_offset":12016,"ancestor_record_offset":11116,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":11636,"parent_record_offset":11612,"ancestor_record_offset":11116,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":11660,"parent_record_offset":11636,"ancestor_record_offset":11116,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":577,"record_offset":12816,"parent_record_offset":12788,"ancestor_record_offset":11116,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":577,"record_offset":13092,"parent_record_offset":13060,"ancestor_record_offset":11116,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":577,"record_offset":13120,"parent_record_offset":13060,"ancestor_record_offset":11116,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":12712,"parent_record_offset":12688,"ancestor_record_offset":11116,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":577,"record_offset":13260,"parent_record_offset":13236,"ancestor_record_offset":11116,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":12596,"parent_record_offset":11920,"ancestor_record_offset":11116,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":11984,"parent_record_offset":11920,"ancestor_record_offset":11116,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":12508,"parent_record_offset":12468,"ancestor_record_offset":11116,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":577,"record_offset":13236,"parent_record_offset":13212,"ancestor_record_offset":11116,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":11920,"parent_record_offset":11892,"ancestor_record_offset":11116,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":577,"record_offset":12944,"parent_record_offset":12920,"ancestor_record_offset":11116,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":577,"record_offset":12968,"parent_record_offset":12944,"ancestor_record_offset":11116,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::rt::bvh::build_gpu_bvh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/bvh.rs
//     evidence={"module":577,"record_offset":12044,"parent_record_offset":12016,"ancestor_record_offset":11116,"inlinee":33010,"source_line":53,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::rt::bvh::build_gpu_bvh::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/bvh.rs
//     evidence={"module":577,"record_offset":13976,"parent_record_offset":13948,"ancestor_record_offset":11116,"inlinee":33011,"source_line":117,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":577,"record_offset":12172,"parent_record_offset":12140,"ancestor_record_offset":11116,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":577,"record_offset":12140,"parent_record_offset":12108,"ancestor_record_offset":11116,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":13148,"parent_record_offset":13060,"ancestor_record_offset":11116,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":577,"record_offset":13060,"parent_record_offset":13036,"ancestor_record_offset":11116,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":13172,"parent_record_offset":13148,"ancestor_record_offset":11116,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh::build_gpu_bvh::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x80F2
// 首条位置: {"module":577,"inlinee":33010}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":577,"record_offset":12108,"parent_record_offset":12044,"ancestor_record_offset":12044,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":577,"record_offset":12172,"parent_record_offset":12140,"ancestor_record_offset":12044,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":577,"record_offset":12140,"parent_record_offset":12108,"ancestor_record_offset":12044,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh::build_gpu_bvh::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1207B
// 首条位置: {"module":1610,"inlinee":73851}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":52780,"parent_record_offset":52744,"ancestor_record_offset":52744,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::rt::GpuRtTriangle::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/mod.rs
//     evidence={"module":1610,"record_offset":52888,"parent_record_offset":52744,"ancestor_record_offset":52744,"inlinee":27408,"source_line":310,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1610,"record_offset":52964,"parent_record_offset":52940,"ancestor_record_offset":52744,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1610,"record_offset":52812,"parent_record_offset":52780,"ancestor_record_offset":52744,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1610,"record_offset":52844,"parent_record_offset":52812,"ancestor_record_offset":52744,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1610,"record_offset":52940,"parent_record_offset":52888,"ancestor_record_offset":52744,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh::build_gpu_bvh::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x80F3
// 首条位置: {"module":577,"inlinee":33011}
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
// IPI IDs: 0x80F0
// 首条位置: {"module":577,"inlinee":33008}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

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
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh::build_gpu_bvh
// 信息 ID: 820aaaf8d4907fb23a228d8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::rt::bvh_builder::bvh_builder_worker @ 0xb24f3b -> 0xb0da50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh::build_gpu_bvh::closure$0
// 信息 ID: 6d177aa25198ae1dd1305de6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::rt::bvh::build_gpu_bvh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::rt::bvh::build_gpu_bvh; module=577, inline_record=12044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh::build_gpu_bvh::closure$1
// 信息 ID: 8999fb64fe1cb0bb3429a7cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::rt::bvh::build_gpu_bvh", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::rt::GpuRtTriangle,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1 <- core::iter::adapters::map::impl$2::fold<country_core::resources::rt::GpuRtTriangle,core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1 <- core::iter::adapters::map::impl$2::fold<country_core::resources::rt::GpuRtTriangle,core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<country_core::resources::rt::GpuRtTriangle,core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1> > > >; module=1610, inline_record=52744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh::build_gpu_bvh::closure$2
// 信息 ID: 9decd782ec2a362e29ea1375
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::resources::rt::bvh::build_gpu_bvh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::rt::bvh::build_gpu_bvh; module=577, inline_record=13976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh::impl$1::default
// 信息 ID: ea66fa746f15cfef040ae1d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::rt::bvh::build_gpu_bvh; module=577, inline_record=12660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
