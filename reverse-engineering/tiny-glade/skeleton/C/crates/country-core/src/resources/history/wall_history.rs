// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/history/wall_history.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 59805d493c9dd0549ac3d898309e0cb53931bd74e4cbba30e07ca4a51e8b8202
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_history.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 568, 585
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_history::impl$0::ev_new_wall
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":568,"record_offset":548,"rva":11820688,"va":5380529808}
// 编译实例 RVAs: 0xb45e90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-4851687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380529808,"raw_hex":"e819f8b5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380530370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-4853555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380529808,"raw_hex":"e8cdf0b5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380532254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=7
//     target_aliases=1699 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11808164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380529808,"raw_hex":"e85cd24bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380530351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=2
//     target_aliases=11 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11292721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380529808,"raw_hex":"e8cfaf53ff","size":5,"soft_fail":false,"target_va":5369240528,"va":5380533244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14097ae00
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1882968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380529808,"raw_hex":"e8a844e3ff","size":5,"soft_fail":false,"target_va":5378649600,"va":5380532563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a62c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1705208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380529808,"raw_hex":"e808fbe5ff","size":5,"soft_fail":false,"target_va":5378826944,"va":5380532147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd7690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t594093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380529808,"raw_hex":"e8ad100900","size":5,"soft_fail":false,"target_va":5381125776,"va":5380531678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196dd90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14843447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380529808,"raw_hex":"e8377ee200","size":5,"soft_fail":false,"target_va":5395373456,"va":5380530004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36526589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380529808,"raw_hex":"e8fd592d02","size":5,"soft_fail":false,"target_va":5417057056,"va":5380530462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t36088847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380529808,"raw_hex":"e80fac2602","size":5,"soft_fail":false,"target_va":5416620769,"va":5380531917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36090528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380529808,"raw_hex":"e8a0b22602","size":5,"soft_fail":false,"target_va":5416620794,"va":5380530261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36090385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380529808,"raw_hex":"e811b22602","size":5,"soft_fail":false,"target_va":5416622208,"va":5380531818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36090353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380529808,"raw_hex":"e8f1b12602","size":5,"soft_fail":false,"target_va":5416622240,"va":5380531882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t24661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380529808,"raw_hex":"e855600000","size":5,"soft_fail":false,"target_va":5380557488,"va":5380532822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t20475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380529808,"raw_hex":"e8fb4f0000","size":5,"soft_fail":false,"target_va":5380552224,"va":5380531744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":568,"record_offset":3296,"parent_record_offset":548,"ancestor_record_offset":548,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":568,"record_offset":2656,"parent_record_offset":548,"ancestor_record_offset":548,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":568,"record_offset":3024,"parent_record_offset":548,"ancestor_record_offset":548,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":568,"record_offset":760,"parent_record_offset":548,"ancestor_record_offset":548,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":568,"record_offset":4004,"parent_record_offset":548,"ancestor_record_offset":548,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":568,"record_offset":3828,"parent_record_offset":548,"ancestor_record_offset":548,"inlinee":30601,"source_line":29,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$45::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":568,"record_offset":932,"parent_record_offset":548,"ancestor_record_offset":548,"inlinee":30580,"source_line":115,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":568,"record_offset":732,"parent_record_offset":548,"ancestor_record_offset":548,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":568,"record_offset":788,"parent_record_offset":548,"ancestor_record_offset":548,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":568,"record_offset":1412,"parent_record_offset":1388,"ancestor_record_offset":548,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":568,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":548,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":568,"record_offset":2544,"parent_record_offset":2520,"ancestor_record_offset":548,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":568,"record_offset":2568,"parent_record_offset":2544,"ancestor_record_offset":548,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":568,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":548,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":568,"record_offset":2592,"parent_record_offset":2568,"ancestor_record_offset":548,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":568,"record_offset":1388,"parent_record_offset":1204,"ancestor_record_offset":548,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":568,"record_offset":2520,"parent_record_offset":2432,"ancestor_record_offset":548,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":568,"record_offset":2684,"parent_record_offset":2656,"ancestor_record_offset":548,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":568,"record_offset":3064,"parent_record_offset":3024,"ancestor_record_offset":548,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":568,"record_offset":4028,"parent_record_offset":4004,"ancestor_record_offset":548,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":568,"record_offset":1148,"parent_record_offset":1124,"ancestor_record_offset":548,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":568,"record_offset":2460,"parent_record_offset":2432,"ancestor_record_offset":548,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":568,"record_offset":2432,"parent_record_offset":2408,"ancestor_record_offset":548,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":568,"record_offset":1204,"parent_record_offset":1172,"ancestor_record_offset":548,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":568,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":548,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":568,"record_offset":2408,"parent_record_offset":2384,"ancestor_record_offset":548,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":568,"record_offset":1244,"parent_record_offset":1204,"ancestor_record_offset":548,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":568,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":548,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":568,"record_offset":1668,"parent_record_offset":1608,"ancestor_record_offset":548,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":568,"record_offset":1092,"parent_record_offset":1032,"ancestor_record_offset":548,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":568,"record_offset":1064,"parent_record_offset":1032,"ancestor_record_offset":548,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":568,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":548,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":568,"record_offset":1124,"parent_record_offset":1092,"ancestor_record_offset":548,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":568,"record_offset":1008,"parent_record_offset":968,"ancestor_record_offset":548,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":568,"record_offset":3860,"parent_record_offset":3828,"ancestor_record_offset":548,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":568,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":548,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":568,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":548,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":568,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":548,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":568,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":548,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":568,"record_offset":1348,"parent_record_offset":1292,"ancestor_record_offset":548,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":568,"record_offset":2164,"parent_record_offset":2108,"ancestor_record_offset":548,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":568,"record_offset":3940,"parent_record_offset":3916,"ancestor_record_offset":548,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":568,"record_offset":2108,"parent_record_offset":1668,"ancestor_record_offset":548,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":568,"record_offset":3972,"parent_record_offset":3828,"ancestor_record_offset":548,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":568,"record_offset":3916,"parent_record_offset":3828,"ancestor_record_offset":548,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":568,"record_offset":2316,"parent_record_offset":1668,"ancestor_record_offset":548,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":568,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":548,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":568,"record_offset":1320,"parent_record_offset":1292,"ancestor_record_offset":548,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":568,"record_offset":2488,"parent_record_offset":2460,"ancestor_record_offset":548,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":568,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":548,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":568,"record_offset":820,"parent_record_offset":788,"ancestor_record_offset":548,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":568,"record_offset":1516,"parent_record_offset":1092,"ancestor_record_offset":548,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":568,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":548,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":568,"record_offset":2360,"parent_record_offset":968,"ancestor_record_offset":548,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":568,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":548,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":568,"record_offset":2276,"parent_record_offset":2208,"ancestor_record_offset":548,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":568,"record_offset":2248,"parent_record_offset":2208,"ancestor_record_offset":548,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":568,"record_offset":2208,"parent_record_offset":2164,"ancestor_record_offset":548,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$45::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":568,"record_offset":968,"parent_record_offset":932,"ancestor_record_offset":548,"inlinee":12078,"source_line":61,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_history::impl$0::ev_height_changed
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8C58
// 首条位置: {"module":568,"record_offset":4512,"rva":11824336,"va":5380533456}
// 编译实例 RVAs: 0xb46cd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140b49700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380533456,"raw_hex":"e81a2a0000","size":5,"soft_fail":false,"target_va":5380544256,"va":5380533473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t18728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380533456,"raw_hex":"e828490000","size":5,"soft_fail":false,"target_va":5380552224,"va":5380533491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::EditType::new_height_changed
// 记录形式: compiled_procedure; instances=1
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":568,"record_offset":42604,"rva":11835136,"va":5380544256}
// 编译实例 RVAs: 0xb49700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4865646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380544256,"raw_hex":"e892c1b5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380544329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4866007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380544256,"raw_hex":"e829c0b5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380544706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11822127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380544256,"raw_hex":"e8d19b4bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380544314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a62c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1717833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380544256,"raw_hex":"e8b7c9e5ff","size":5,"soft_fail":false,"target_va":5378826944,"va":5380544772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd7690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t581242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380544256,"raw_hex":"e87ade0800","size":5,"soft_fail":false,"target_va":5381125776,"va":5380544529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36512580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380544256,"raw_hex":"e844232d02","size":5,"soft_fail":false,"target_va":5417057056,"va":5380544471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36076174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380544256,"raw_hex":"e88e7a2602","size":5,"soft_fail":false,"target_va":5416620769,"va":5380544590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":568,"record_offset":42764,"parent_record_offset":42604,"ancestor_record_offset":42604,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":568,"record_offset":43124,"parent_record_offset":42604,"ancestor_record_offset":42604,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":568,"record_offset":43560,"parent_record_offset":42604,"ancestor_record_offset":42604,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":568,"record_offset":43384,"parent_record_offset":42604,"ancestor_record_offset":42604,"inlinee":30601,"source_line":29,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":568,"record_offset":42848,"parent_record_offset":42824,"ancestor_record_offset":42604,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":568,"record_offset":42872,"parent_record_offset":42848,"ancestor_record_offset":42604,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":568,"record_offset":43632,"parent_record_offset":43608,"ancestor_record_offset":42604,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":568,"record_offset":43656,"parent_record_offset":43632,"ancestor_record_offset":42604,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":568,"record_offset":42896,"parent_record_offset":42872,"ancestor_record_offset":42604,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":568,"record_offset":43680,"parent_record_offset":43656,"ancestor_record_offset":42604,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":568,"record_offset":42824,"parent_record_offset":42792,"ancestor_record_offset":42604,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":568,"record_offset":43608,"parent_record_offset":43584,"ancestor_record_offset":42604,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":568,"record_offset":42792,"parent_record_offset":42764,"ancestor_record_offset":42604,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":568,"record_offset":43156,"parent_record_offset":43124,"ancestor_record_offset":42604,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":568,"record_offset":43584,"parent_record_offset":43560,"ancestor_record_offset":42604,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":568,"record_offset":43416,"parent_record_offset":43384,"ancestor_record_offset":42604,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":568,"record_offset":43496,"parent_record_offset":43472,"ancestor_record_offset":42604,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":568,"record_offset":43528,"parent_record_offset":43384,"ancestor_record_offset":42604,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":568,"record_offset":43472,"parent_record_offset":43384,"ancestor_record_offset":42604,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":568,"record_offset":43348,"parent_record_offset":43156,"ancestor_record_offset":42604,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":568,"record_offset":43440,"parent_record_offset":43416,"ancestor_record_offset":42604,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00568.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_history.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_history.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_history.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::resources::history::EditType::new_height_changed
// 信息 ID: 2c274d5d5f3583fc62f8ea06
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::history::wall_history::impl$0::ev_height_changed @ 0xb46ce1 -> 0xb49700 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::deferred_wall_history::history_record_merge_walls @ 0xb48b0d -> 0xb49700 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::history_record_adjust_wall_height @ 0xbec033 -> 0xb49700 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_history::impl$0::ev_height_changed
// 信息 ID: bac729e4a0802548c3f05373
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::history_record_adjust_wall_height; module=585, inline_record=33116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_history::impl$0::ev_new_wall
// 信息 ID: d28ba4a5798d775dafda761f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::finish_wall::finish_wall @ 0xa3b4a4 -> 0xb45e90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::create_circle::create_circle @ 0xa3f291 -> 0xb45e90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::create_rectangle::create_rectangle @ 0xa40a47 -> 0xb45e90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
