// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/in_progress_curve.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: b387d959777d099a703f3e21905c2b8746a4446260535fb8f23fc32ee16ae3fb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: in_progress_curve.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 551, 588, 589
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
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":589,"record_offset":7864,"rva":12616032,"va":5381325152}
// 编译实例 RVAs: 0xc08160
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-5646626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381325152,"raw_hex":"e8ded6a9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381325309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-5646777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381325152,"raw_hex":"e847d6a9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381325476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=5
//     target_aliases=1699 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12603109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381325152,"raw_hex":"e81bb13fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381325296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=2
//     target_aliases=11 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12086767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381325152,"raw_hex":"e8119247ff","size":5,"soft_fail":false,"target_va":5369240528,"va":5381327290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a41d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7750364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381325152,"raw_hex":"e824bd89ff","size":5,"soft_fail":false,"target_va":5373575632,"va":5381325991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b44e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-800156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381325152,"raw_hex":"e864caf3ff","size":5,"soft_fail":false,"target_va":5380525680,"va":5381325831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b45450
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-798270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381325152,"raw_hex":"e8c2d1f3ff","size":5,"soft_fail":false,"target_va":5380527184,"va":5381325449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419700c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14055865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381325152,"raw_hex":"e8b979d600","size":5,"soft_fail":false,"target_va":5395382464,"va":5381326594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35731712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381325152,"raw_hex":"e800392102","size":5,"soft_fail":false,"target_va":5417057056,"va":5381325339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t35295377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381325152,"raw_hex":"e891901a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5381325412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35295225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381325152,"raw_hex":"e8f98f1a02","size":5,"soft_fail":false,"target_va":5416622320,"va":5381327090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":12944,"parent_record_offset":7864,"ancestor_record_offset":7864,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":12812,"parent_record_offset":7864,"ancestor_record_offset":7864,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":589,"record_offset":8052,"parent_record_offset":7864,"ancestor_record_offset":7864,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":589,"record_offset":11588,"parent_record_offset":7864,"ancestor_record_offset":7864,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":589,"record_offset":13548,"parent_record_offset":7864,"ancestor_record_offset":7864,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":589,"record_offset":13616,"parent_record_offset":7864,"ancestor_record_offset":7864,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$45::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":589,"record_offset":8168,"parent_record_offset":7864,"ancestor_record_offset":7864,"inlinee":12078,"source_line":61,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":8600,"parent_record_offset":8576,"ancestor_record_offset":7864,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":8624,"parent_record_offset":8600,"ancestor_record_offset":7864,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":9720,"parent_record_offset":9696,"ancestor_record_offset":7864,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":9744,"parent_record_offset":9720,"ancestor_record_offset":7864,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":8648,"parent_record_offset":8624,"ancestor_record_offset":7864,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":9768,"parent_record_offset":9744,"ancestor_record_offset":7864,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":8576,"parent_record_offset":8428,"ancestor_record_offset":7864,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":9696,"parent_record_offset":9640,"ancestor_record_offset":7864,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":8360,"parent_record_offset":8328,"ancestor_record_offset":7864,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":9668,"parent_record_offset":9640,"ancestor_record_offset":7864,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":9640,"parent_record_offset":9616,"ancestor_record_offset":7864,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":8428,"parent_record_offset":8392,"ancestor_record_offset":7864,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":8392,"parent_record_offset":8360,"ancestor_record_offset":7864,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":9616,"parent_record_offset":9592,"ancestor_record_offset":7864,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":8464,"parent_record_offset":8428,"ancestor_record_offset":7864,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":589,"record_offset":8256,"parent_record_offset":8228,"ancestor_record_offset":7864,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":589,"record_offset":8872,"parent_record_offset":8804,"ancestor_record_offset":7864,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":589,"record_offset":8288,"parent_record_offset":8256,"ancestor_record_offset":7864,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":8844,"parent_record_offset":8804,"ancestor_record_offset":7864,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":13508,"parent_record_offset":13028,"ancestor_record_offset":7864,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":8328,"parent_record_offset":8288,"ancestor_record_offset":7864,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":8228,"parent_record_offset":8168,"ancestor_record_offset":7864,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":8804,"parent_record_offset":8772,"ancestor_record_offset":7864,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":589,"record_offset":8488,"parent_record_offset":8464,"ancestor_record_offset":7864,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":589,"record_offset":9148,"parent_record_offset":9092,"ancestor_record_offset":7864,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":589,"record_offset":9372,"parent_record_offset":9316,"ancestor_record_offset":7864,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":589,"record_offset":8076,"parent_record_offset":8052,"ancestor_record_offset":7864,"inlinee":17819,"source_line":180,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":589,"record_offset":9316,"parent_record_offset":8872,"ancestor_record_offset":7864,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":589,"record_offset":9524,"parent_record_offset":8872,"ancestor_record_offset":7864,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":589,"record_offset":8512,"parent_record_offset":8488,"ancestor_record_offset":7864,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":589,"record_offset":8536,"parent_record_offset":8512,"ancestor_record_offset":7864,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":589,"record_offset":11424,"parent_record_offset":11396,"ancestor_record_offset":7864,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":589,"record_offset":8704,"parent_record_offset":8288,"ancestor_record_offset":7864,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":589,"record_offset":8728,"parent_record_offset":8704,"ancestor_record_offset":7864,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":589,"record_offset":9568,"parent_record_offset":8168,"ancestor_record_offset":7864,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":589,"record_offset":9484,"parent_record_offset":9416,"ancestor_record_offset":7864,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":589,"record_offset":8128,"parent_record_offset":8100,"ancestor_record_offset":7864,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":589,"record_offset":12904,"parent_record_offset":12868,"ancestor_record_offset":7864,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":589,"record_offset":12840,"parent_record_offset":12812,"ancestor_record_offset":7864,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":589,"record_offset":12868,"parent_record_offset":12840,"ancestor_record_offset":7864,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":589,"record_offset":9416,"parent_record_offset":9372,"ancestor_record_offset":7864,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":589,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":7864,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::impl$3::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x926A
// 首条位置: {"module":588,"inlinee":37482}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":588,"record_offset":20868,"parent_record_offset":18968,"ancestor_record_offset":18968,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":588,"record_offset":22960,"parent_record_offset":18968,"ancestor_record_offset":18968,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$45::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":588,"record_offset":19064,"parent_record_offset":18968,"ancestor_record_offset":18968,"inlinee":12078,"source_line":61,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_direct] utils::rdp::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":588,"record_offset":21452,"parent_record_offset":18968,"ancestor_record_offset":18968,"inlinee":37492,"source_line":6,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":588,"record_offset":19584,"parent_record_offset":19560,"ancestor_record_offset":18968,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":588,"record_offset":19608,"parent_record_offset":19584,"ancestor_record_offset":18968,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":588,"record_offset":20760,"parent_record_offset":20736,"ancestor_record_offset":18968,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":588,"record_offset":20784,"parent_record_offset":20760,"ancestor_record_offset":18968,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":588,"record_offset":19632,"parent_record_offset":19608,"ancestor_record_offset":18968,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":588,"record_offset":20808,"parent_record_offset":20784,"ancestor_record_offset":18968,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":588,"record_offset":19560,"parent_record_offset":19376,"ancestor_record_offset":18968,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":588,"record_offset":20736,"parent_record_offset":20648,"ancestor_record_offset":18968,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":588,"record_offset":19312,"parent_record_offset":19284,"ancestor_record_offset":18968,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":588,"record_offset":20676,"parent_record_offset":20648,"ancestor_record_offset":18968,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":588,"record_offset":20648,"parent_record_offset":20624,"ancestor_record_offset":18968,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":588,"record_offset":19376,"parent_record_offset":19340,"ancestor_record_offset":18968,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":588,"record_offset":19340,"parent_record_offset":19312,"ancestor_record_offset":18968,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":588,"record_offset":20624,"parent_record_offset":20600,"ancestor_record_offset":18968,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":588,"record_offset":19416,"parent_record_offset":19376,"ancestor_record_offset":18968,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":588,"record_offset":19864,"parent_record_offset":19792,"ancestor_record_offset":18968,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":588,"record_offset":19240,"parent_record_offset":19168,"ancestor_record_offset":18968,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":588,"record_offset":19212,"parent_record_offset":19168,"ancestor_record_offset":18968,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":588,"record_offset":19284,"parent_record_offset":19240,"ancestor_record_offset":18968,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":588,"record_offset":19132,"parent_record_offset":19064,"ancestor_record_offset":18968,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":588,"record_offset":19168,"parent_record_offset":19132,"ancestor_record_offset":18968,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":588,"record_offset":19440,"parent_record_offset":19416,"ancestor_record_offset":18968,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":588,"record_offset":19520,"parent_record_offset":19464,"ancestor_record_offset":18968,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":588,"record_offset":20380,"parent_record_offset":20324,"ancestor_record_offset":18968,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":588,"record_offset":20324,"parent_record_offset":19864,"ancestor_record_offset":18968,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":588,"record_offset":20532,"parent_record_offset":19864,"ancestor_record_offset":18968,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":588,"record_offset":19464,"parent_record_offset":19440,"ancestor_record_offset":18968,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":588,"record_offset":19492,"parent_record_offset":19464,"ancestor_record_offset":18968,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":588,"record_offset":20704,"parent_record_offset":20676,"ancestor_record_offset":18968,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":588,"record_offset":19688,"parent_record_offset":19240,"ancestor_record_offset":18968,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":588,"record_offset":19712,"parent_record_offset":19688,"ancestor_record_offset":18968,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":588,"record_offset":20576,"parent_record_offset":19064,"ancestor_record_offset":18968,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":588,"record_offset":20492,"parent_record_offset":20424,"ancestor_record_offset":18968,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":588,"record_offset":20464,"parent_record_offset":20424,"ancestor_record_offset":18968,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":588,"record_offset":20424,"parent_record_offset":20380,"ancestor_record_offset":18968,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":589,"record_offset":16612,"rva":12618864,"va":5381327984}
// 编译实例 RVAs: 0xc08c70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-5649663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327984,"raw_hex":"e801cba9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381328346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=16
//     evidence={"assembly":"\tcallq\t-5650790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327984,"raw_hex":"e89ac6a9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381329489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=5
//     target_aliases=1699 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12606146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327984,"raw_hex":"e83ea53fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381328333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=4
//     target_aliases=11 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12090621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327984,"raw_hex":"e8038347ff","size":5,"soft_fail":false,"target_va":5369240528,"va":5381331144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a41d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7754231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327984,"raw_hex":"e809ae89ff","size":5,"soft_fail":false,"target_va":5373575632,"va":5381329858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b45450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-802192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327984,"raw_hex":"e870c2f3ff","size":5,"soft_fail":false,"target_va":5380527184,"va":5381329371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196dd90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14045237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327984,"raw_hex":"e83550d600","size":5,"soft_fail":false,"target_va":5395373456,"va":5381328214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141971a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14060779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327984,"raw_hex":"e8eb8cd600","size":5,"soft_fail":false,"target_va":5395389072,"va":5381328288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35023391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327984,"raw_hex":"e81f6a1602","size":5,"soft_fail":false,"target_va":5416353040,"va":5381329644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35728491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327984,"raw_hex":"e86b2c2102","size":5,"soft_fail":false,"target_va":5417057056,"va":5381328560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35035291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327984,"raw_hex":"e89b981602","size":5,"soft_fail":false,"target_va":5416365968,"va":5381330672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t35292147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327984,"raw_hex":"e8f3831a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5381328642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35292089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327984,"raw_hex":"e8b9831a02","size":5,"soft_fail":false,"target_va":5416621600,"va":5381329506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35291235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327984,"raw_hex":"e863801a02","size":5,"soft_fail":false,"target_va":5416621762,"va":5381330522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35292672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327984,"raw_hex":"e800861a02","size":5,"soft_fail":false,"target_va":5416622240,"va":5381329563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35291185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327984,"raw_hex":"e831801a02","size":5,"soft_fail":false,"target_va":5416621632,"va":5381330442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35292717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327984,"raw_hex":"e82d861a02","size":5,"soft_fail":false,"target_va":5416622320,"va":5381329598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":24772,"parent_record_offset":16612,"ancestor_record_offset":16612,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":17380,"parent_record_offset":16612,"ancestor_record_offset":16612,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":16916,"parent_record_offset":16612,"ancestor_record_offset":16612,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":19852,"parent_record_offset":16612,"ancestor_record_offset":16612,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$22::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":21468,"parent_record_offset":16612,"ancestor_record_offset":16612,"inlinee":11091,"source_line":4166,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":589,"record_offset":17204,"parent_record_offset":16612,"ancestor_record_offset":16612,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":589,"record_offset":23416,"parent_record_offset":16612,"ancestor_record_offset":16612,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":589,"record_offset":16876,"parent_record_offset":16612,"ancestor_record_offset":16612,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":589,"record_offset":21176,"parent_record_offset":16612,"ancestor_record_offset":16612,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":589,"record_offset":16848,"parent_record_offset":16612,"ancestor_record_offset":16612,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":589,"record_offset":17024,"parent_record_offset":16612,"ancestor_record_offset":16612,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":589,"record_offset":21808,"parent_record_offset":16612,"ancestor_record_offset":16612,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":589,"record_offset":16820,"parent_record_offset":16612,"ancestor_record_offset":16612,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":589,"record_offset":18004,"parent_record_offset":16612,"ancestor_record_offset":16612,"inlinee":11669,"source_line":1078,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":589,"record_offset":17232,"parent_record_offset":16612,"ancestor_record_offset":16612,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$45::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":589,"record_offset":18032,"parent_record_offset":16612,"ancestor_record_offset":16612,"inlinee":12078,"source_line":61,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":17592,"parent_record_offset":17568,"ancestor_record_offset":16612,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":17616,"parent_record_offset":17592,"ancestor_record_offset":16612,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":19632,"parent_record_offset":19608,"ancestor_record_offset":16612,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":19656,"parent_record_offset":19632,"ancestor_record_offset":16612,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":17640,"parent_record_offset":17616,"ancestor_record_offset":16612,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":19680,"parent_record_offset":19656,"ancestor_record_offset":16612,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":17568,"parent_record_offset":17536,"ancestor_record_offset":16612,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":19608,"parent_record_offset":19552,"ancestor_record_offset":16612,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":20424,"parent_record_offset":20400,"ancestor_record_offset":16612,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":19988,"parent_record_offset":19960,"ancestor_record_offset":16612,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":17476,"parent_record_offset":17448,"ancestor_record_offset":16612,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":21680,"parent_record_offset":21656,"ancestor_record_offset":16612,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":19580,"parent_record_offset":19552,"ancestor_record_offset":16612,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":19552,"parent_record_offset":19528,"ancestor_record_offset":16612,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":20048,"parent_record_offset":20016,"ancestor_record_offset":16612,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":20472,"parent_record_offset":20448,"ancestor_record_offset":16612,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":20448,"parent_record_offset":20424,"ancestor_record_offset":16612,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":20016,"parent_record_offset":19988,"ancestor_record_offset":16612,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":17536,"parent_record_offset":17504,"ancestor_record_offset":16612,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":17504,"parent_record_offset":17476,"ancestor_record_offset":16612,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":19528,"parent_record_offset":19504,"ancestor_record_offset":16612,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":18376,"parent_record_offset":18332,"ancestor_record_offset":16612,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":589,"record_offset":18128,"parent_record_offset":18092,"ancestor_record_offset":16612,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":589,"record_offset":18784,"parent_record_offset":18716,"ancestor_record_offset":16612,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":589,"record_offset":18168,"parent_record_offset":18128,"ancestor_record_offset":16612,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":21524,"parent_record_offset":21496,"ancestor_record_offset":16612,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":20400,"parent_record_offset":19916,"ancestor_record_offset":16612,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":18756,"parent_record_offset":18716,"ancestor_record_offset":16612,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":19916,"parent_record_offset":19884,"ancestor_record_offset":16612,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":17696,"parent_record_offset":17412,"ancestor_record_offset":16612,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":21768,"parent_record_offset":21524,"ancestor_record_offset":16612,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":24800,"parent_record_offset":24772,"ancestor_record_offset":16612,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":19960,"parent_record_offset":19916,"ancestor_record_offset":16612,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":23568,"parent_record_offset":23524,"ancestor_record_offset":16612,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":17448,"parent_record_offset":17412,"ancestor_record_offset":16612,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":18092,"parent_record_offset":18032,"ancestor_record_offset":16612,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":23452,"parent_record_offset":23416,"ancestor_record_offset":16612,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":20292,"parent_record_offset":20264,"ancestor_record_offset":16612,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":18716,"parent_record_offset":18684,"ancestor_record_offset":16612,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":589,"record_offset":20356,"parent_record_offset":20292,"ancestor_record_offset":16612,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":589,"record_offset":20512,"parent_record_offset":19916,"ancestor_record_offset":16612,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":589,"record_offset":19884,"parent_record_offset":19852,"ancestor_record_offset":16612,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":589,"record_offset":21496,"parent_record_offset":21468,"ancestor_record_offset":16612,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":589,"record_offset":17412,"parent_record_offset":17380,"ancestor_record_offset":16612,"inlinee":8416,"source_line":22,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":589,"record_offset":23488,"parent_record_offset":23452,"ancestor_record_offset":16612,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":589,"record_offset":23524,"parent_record_offset":23488,"ancestor_record_offset":16612,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":589,"record_offset":18400,"parent_record_offset":18376,"ancestor_record_offset":16612,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":589,"record_offset":17312,"parent_record_offset":17288,"ancestor_record_offset":16612,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":589,"record_offset":17336,"parent_record_offset":17312,"ancestor_record_offset":16612,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":589,"record_offset":17808,"parent_record_offset":17772,"ancestor_record_offset":16612,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":589,"record_offset":17160,"parent_record_offset":17136,"ancestor_record_offset":16612,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":589,"record_offset":19060,"parent_record_offset":19004,"ancestor_record_offset":16612,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":589,"record_offset":19284,"parent_record_offset":19228,"ancestor_record_offset":16612,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":589,"record_offset":19228,"parent_record_offset":18784,"ancestor_record_offset":16612,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":589,"record_offset":17840,"parent_record_offset":17772,"ancestor_record_offset":16612,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":589,"record_offset":17772,"parent_record_offset":17740,"ancestor_record_offset":16612,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":589,"record_offset":17740,"parent_record_offset":17696,"ancestor_record_offset":16612,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::sources::repeat_n::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\sources\repeat_n.rs
//     evidence={"module":589,"record_offset":20184,"parent_record_offset":20144,"ancestor_record_offset":16612,"inlinee":37616,"source_line":110,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":589,"record_offset":20144,"parent_record_offset":20116,"ancestor_record_offset":16612,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":589,"record_offset":20116,"parent_record_offset":19916,"ancestor_record_offset":16612,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":589,"record_offset":20264,"parent_record_offset":20144,"ancestor_record_offset":16612,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":589,"record_offset":19436,"parent_record_offset":18784,"ancestor_record_offset":16612,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":589,"record_offset":18424,"parent_record_offset":18400,"ancestor_record_offset":16612,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":589,"record_offset":18448,"parent_record_offset":18424,"ancestor_record_offset":16612,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":589,"record_offset":17864,"parent_record_offset":17840,"ancestor_record_offset":16612,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":589,"record_offset":21308,"parent_record_offset":21276,"ancestor_record_offset":16612,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":589,"record_offset":20072,"parent_record_offset":20048,"ancestor_record_offset":16612,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":589,"record_offset":17076,"parent_record_offset":17048,"ancestor_record_offset":16612,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":589,"record_offset":17288,"parent_record_offset":17264,"ancestor_record_offset":16612,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":589,"record_offset":17264,"parent_record_offset":17232,"ancestor_record_offset":16612,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":589,"record_offset":20220,"parent_record_offset":20184,"ancestor_record_offset":16612,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":589,"record_offset":18616,"parent_record_offset":18168,"ancestor_record_offset":16612,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":589,"record_offset":18640,"parent_record_offset":18616,"ancestor_record_offset":16612,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":589,"record_offset":19480,"parent_record_offset":18032,"ancestor_record_offset":16612,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":589,"record_offset":17936,"parent_record_offset":17696,"ancestor_record_offset":16612,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":589,"record_offset":19396,"parent_record_offset":19328,"ancestor_record_offset":16612,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":589,"record_offset":19368,"parent_record_offset":19328,"ancestor_record_offset":16612,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":589,"record_offset":17904,"parent_record_offset":17696,"ancestor_record_offset":16612,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":589,"record_offset":21836,"parent_record_offset":21808,"ancestor_record_offset":16612,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":589,"record_offset":16948,"parent_record_offset":16916,"ancestor_record_offset":16612,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":589,"record_offset":16980,"parent_record_offset":16948,"ancestor_record_offset":16612,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":589,"record_offset":19804,"parent_record_offset":19772,"ancestor_record_offset":16612,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":589,"record_offset":19328,"parent_record_offset":19284,"ancestor_record_offset":16612,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":589,"record_offset":17104,"parent_record_offset":17048,"ancestor_record_offset":16612,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":589,"record_offset":17048,"parent_record_offset":17024,"ancestor_record_offset":16612,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":589,"record_offset":17136,"parent_record_offset":17104,"ancestor_record_offset":16612,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::last_raw_input_point
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x926B
// 首条位置: {"module":588,"inlinee":37483}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":588,"record_offset":24684,"parent_record_offset":24604,"ancestor_record_offset":24604,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::end_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":588,"record_offset":24628,"parent_record_offset":24604,"ancestor_record_offset":24604,"inlinee":35867,"source_line":1074,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":588,"record_offset":24652,"parent_record_offset":24628,"ancestor_record_offset":24604,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::first_raw_input_point
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x92ED
// 首条位置: {"module":589,"inlinee":37613}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":589,"record_offset":4468,"parent_record_offset":4384,"ancestor_record_offset":4384,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::start_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":589,"record_offset":4408,"parent_record_offset":4384,"ancestor_record_offset":4384,"inlinee":37614,"source_line":1070,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":589,"record_offset":4436,"parent_record_offset":4408,"ancestor_record_offset":4384,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::try_add_raw_input_points
// 记录形式: compiled_procedure; instances=1
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":589,"record_offset":14396,"rva":12618336,"va":5381327456}
// 编译实例 RVAs: 0xc08a60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5648865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327456,"raw_hex":"e81fcea9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381327548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5649123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327456,"raw_hex":"e81dcda9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381327822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12605348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327456,"raw_hex":"e85ca83fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381327535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196da90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14045185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327456,"raw_hex":"e80150d600","size":5,"soft_fail":false,"target_va":5395372688,"va":5381327498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35025160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327456,"raw_hex":"e808711602","size":5,"soft_fail":false,"target_va":5416353040,"va":5381327875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35729264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327456,"raw_hex":"e8702f2102","size":5,"soft_fail":false,"target_va":5417057056,"va":5381327787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35292894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381327456,"raw_hex":"e8de861a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5381327895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":14612,"parent_record_offset":14396,"ancestor_record_offset":14396,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":15252,"parent_record_offset":14396,"ancestor_record_offset":14396,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":589,"record_offset":14576,"parent_record_offset":14396,"ancestor_record_offset":14396,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":14848,"parent_record_offset":14824,"ancestor_record_offset":14396,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":14872,"parent_record_offset":14848,"ancestor_record_offset":14396,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":16112,"parent_record_offset":16088,"ancestor_record_offset":14396,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":16136,"parent_record_offset":16112,"ancestor_record_offset":14396,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":14896,"parent_record_offset":14872,"ancestor_record_offset":14396,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":16160,"parent_record_offset":16136,"ancestor_record_offset":14396,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":14824,"parent_record_offset":14784,"ancestor_record_offset":14396,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":589,"record_offset":16088,"parent_record_offset":16000,"ancestor_record_offset":14396,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":15636,"parent_record_offset":15608,"ancestor_record_offset":14396,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":14716,"parent_record_offset":14684,"ancestor_record_offset":14396,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":15732,"parent_record_offset":15704,"ancestor_record_offset":14396,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":16028,"parent_record_offset":16000,"ancestor_record_offset":14396,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":16000,"parent_record_offset":15976,"ancestor_record_offset":14396,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":15704,"parent_record_offset":15668,"ancestor_record_offset":14396,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":15668,"parent_record_offset":15636,"ancestor_record_offset":14396,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":14784,"parent_record_offset":14748,"ancestor_record_offset":14396,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":14748,"parent_record_offset":14716,"ancestor_record_offset":14396,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":589,"record_offset":15976,"parent_record_offset":15952,"ancestor_record_offset":14396,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":15560,"parent_record_offset":15344,"ancestor_record_offset":14396,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":14952,"parent_record_offset":14644,"ancestor_record_offset":14396,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":15824,"parent_record_offset":15560,"ancestor_record_offset":14396,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":15608,"parent_record_offset":15560,"ancestor_record_offset":14396,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":14684,"parent_record_offset":14644,"ancestor_record_offset":14396,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":589,"record_offset":15288,"parent_record_offset":15252,"ancestor_record_offset":14396,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":589,"record_offset":15392,"parent_record_offset":15344,"ancestor_record_offset":14396,"inlinee":5516,"source_line":88,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":589,"record_offset":15880,"parent_record_offset":15856,"ancestor_record_offset":14396,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":589,"record_offset":15928,"parent_record_offset":15904,"ancestor_record_offset":14396,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":589,"record_offset":15440,"parent_record_offset":15416,"ancestor_record_offset":14396,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":589,"record_offset":15344,"parent_record_offset":15252,"ancestor_record_offset":14396,"inlinee":6150,"source_line":32,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":589,"record_offset":14644,"parent_record_offset":14612,"ancestor_record_offset":14396,"inlinee":8416,"source_line":22,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":589,"record_offset":15060,"parent_record_offset":15024,"ancestor_record_offset":14396,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":589,"record_offset":15088,"parent_record_offset":15024,"ancestor_record_offset":14396,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":589,"record_offset":15024,"parent_record_offset":14992,"ancestor_record_offset":14396,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":589,"record_offset":14992,"parent_record_offset":14952,"ancestor_record_offset":14396,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":589,"record_offset":15416,"parent_record_offset":15392,"ancestor_record_offset":14396,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":589,"record_offset":15112,"parent_record_offset":15088,"ancestor_record_offset":14396,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":589,"record_offset":16056,"parent_record_offset":16028,"ancestor_record_offset":14396,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":589,"record_offset":15512,"parent_record_offset":15488,"ancestor_record_offset":14396,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":589,"record_offset":15856,"parent_record_offset":15344,"ancestor_record_offset":14396,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":589,"record_offset":15184,"parent_record_offset":14952,"ancestor_record_offset":14396,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":589,"record_offset":15488,"parent_record_offset":15464,"ancestor_record_offset":14396,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":589,"record_offset":15464,"parent_record_offset":15440,"ancestor_record_offset":14396,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":589,"record_offset":15152,"parent_record_offset":14952,"ancestor_record_offset":14396,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::raw_input_curve_length_ws
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8646
// 首条位置: {"module":551,"inlinee":34374}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":551,"record_offset":72064,"parent_record_offset":72040,"ancestor_record_offset":72040,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::add_weight_to_the_last_point
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x926C
// 首条位置: {"module":588,"inlinee":37484}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":588,"record_offset":24928,"parent_record_offset":24892,"ancestor_record_offset":24892,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$59::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":588,"record_offset":24956,"parent_record_offset":24892,"ancestor_record_offset":24892,"inlinee":37485,"source_line":2016,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::is_raw_input_curve_valid
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9271
// 首条位置: {"module":588,"inlinee":37489}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":588,"record_offset":30200,"parent_record_offset":30176,"ancestor_record_offset":30176,"inlinee":28987,"source_line":165,"archive":"_pdb/C/modules/00588.jsonl.gz"}
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
// 完整结构及原始类型记录: in_progress_curve.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::in_progress_curve::CurveSimplificationOptions
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::in_progress_curve::InProgressCurve
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 10; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::in_progress_curve::InProgressCurve>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::in_progress_curve::InProgressCurve"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: in_progress_curve.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: in_progress_curve.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bvh::bvh::bvh_node::BvhNodeBuildArgs<mesh_to_sdf::generate::generic::bvh::BvhNode<array$<f32,3> >,f32,3>,bvh::bvh::bvh_node::BvhNodeBuildArgs<mesh_to_sdf::generate::generic::bvh::BvhNode<array$<f32,3> >,f32,3>)
// 候选参数类型: ["bvh::bvh::bvh_node::BvhNodeBuildArgs<mesh_to_sdf::generate::generic::bvh::BvhNode<array$<f32,3> >,f32,3>", "bvh::bvh::bvh_node::BvhNodeBuildArgs<mesh_to_sdf::generate::generic::bvh::BvhNode<array$<f32,3> >,f32,3>"]

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::add_weight_to_the_last_point
// 信息 ID: cecf050a92dc0d2656f3e6a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=588, inline_record=24892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve
// 信息 ID: 9c5505f2f09c4b37795493ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0xbfe711 -> 0xc08160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0xbfe7ed -> 0xc08160 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::first_raw_input_point
// 信息 ID: 3199ba4c041c8b1acf803835
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_to_ends::snapping_to_ends; module=589, inline_record=4384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::is_raw_input_curve_valid
// 信息 ID: e007369ff0a70c0a349adc2d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=588, inline_record=30176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::last_raw_input_point
// 信息 ID: 11111711cf40ccebf16aaaa3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=588, inline_record=24604; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=588, inline_record=39272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::new
// 信息 ID: 348a5fcde6108c26910a3c07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::ui_continue_wall::ui_continue_wall @ 0xbd5e9d -> 0xc08c70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0xbff2e3 -> 0xc08c70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::raw_input_curve_length_ws
// 信息 ID: f72dd9a304b30b8592dff9d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> > >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0>; module=551, inline_record=72040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::try_add_raw_input_points
// 信息 ID: 9bf56d6a40eeef69f6a330f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0xbffbd3 -> 0xc08a60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0xbffdb0 -> 0xc08a60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0xbfded5 -> 0xc08a60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::impl$3::clone
// 信息 ID: ce76ceb203ede7f04e29bad8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=588, inline_record=18968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
