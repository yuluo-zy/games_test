// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/usdl/src/video.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 8ec530b5f780c50fcc8e1d8433a93fb24ec8d957c4104e5053ef94850a0bc8cb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: video.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 16, 198, 281, 284, 285, 287
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use usdl; // namespace candidate
//   package identities: [{"name":"usdl","source":"local_or_path","version":"0.1.0"}]
// use usdl_sys; // namespace candidate
//   package identities: [{"name":"usdl-sys","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: usdl::video::VideoSubsystem::init
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A4D
// 首条位置: {"module":281,"inlinee":19021}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":281,"record_offset":1656,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1544,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":4151,"source_line":3134,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1600,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":19022,"source_line":2870,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1708,"parent_record_offset":1680,"ancestor_record_offset":1492,"inlinee":4762,"source_line":3165,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1568,"parent_record_offset":1544,"ancestor_record_offset":1492,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":1492,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":1492,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] usdl::sdl::impl$1::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/sdl.rs
//     evidence={"module":281,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":1492,"inlinee":19024,"source_line":69,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::VideoSubsystem::window
// 记录形式: compiled_procedure; instances=1
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":3940,"rva":7123040,"va":5375832160}
// 编译实例 RVAs: 0x6cb060
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-153549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832160,"raw_hex":"e833a8fdff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375832232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-153789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832160,"raw_hex":"e843a7fdff","size":5,"soft_fail":false,"target_va":5375678704,"va":5375832488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: video.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7110027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832160,"raw_hex":"e8758293ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375832214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: video.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41224797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832160,"raw_hex":"e85d0a7502","size":5,"soft_fail":false,"target_va":5417057056,"va":5375832254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40788588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832160,"raw_hex":"e86c626e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5375832201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40789844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832160,"raw_hex":"e854676e02","size":5,"soft_fail":false,"target_va":5416622240,"va":5375832391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl::video::WindowBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":4060,"parent_record_offset":3940,"ancestor_record_offset":3940,"inlinee":19090,"source_line":246,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":3940,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":4552,"parent_record_offset":4528,"ancestor_record_offset":3940,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":5076,"parent_record_offset":5052,"ancestor_record_offset":3940,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":5100,"parent_record_offset":5076,"ancestor_record_offset":3940,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":4576,"parent_record_offset":4552,"ancestor_record_offset":3940,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":5124,"parent_record_offset":5100,"ancestor_record_offset":3940,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":4504,"parent_record_offset":4348,"ancestor_record_offset":3940,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":5052,"parent_record_offset":4996,"ancestor_record_offset":3940,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":4284,"parent_record_offset":4256,"ancestor_record_offset":3940,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":5024,"parent_record_offset":4996,"ancestor_record_offset":3940,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":4996,"parent_record_offset":4972,"ancestor_record_offset":3940,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":4348,"parent_record_offset":4312,"ancestor_record_offset":3940,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":4312,"parent_record_offset":4284,"ancestor_record_offset":3940,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":4972,"parent_record_offset":4948,"ancestor_record_offset":3940,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":4392,"parent_record_offset":4348,"ancestor_record_offset":3940,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":285,"record_offset":4164,"parent_record_offset":4136,"ancestor_record_offset":3940,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":285,"record_offset":4192,"parent_record_offset":4164,"ancestor_record_offset":3940,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":285,"record_offset":4220,"parent_record_offset":4192,"ancestor_record_offset":3940,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":285,"record_offset":4136,"parent_record_offset":4108,"ancestor_record_offset":3940,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":285,"record_offset":4108,"parent_record_offset":4060,"ancestor_record_offset":3940,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":285,"record_offset":4256,"parent_record_offset":4220,"ancestor_record_offset":3940,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":285,"record_offset":4416,"parent_record_offset":4392,"ancestor_record_offset":3940,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":285,"record_offset":4464,"parent_record_offset":4440,"ancestor_record_offset":3940,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":285,"record_offset":4440,"parent_record_offset":4416,"ancestor_record_offset":3940,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":285,"record_offset":4632,"parent_record_offset":4220,"ancestor_record_offset":3940,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":285,"record_offset":4656,"parent_record_offset":4632,"ancestor_record_offset":3940,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":285,"record_offset":4900,"parent_record_offset":4060,"ancestor_record_offset":3940,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":285,"record_offset":4748,"parent_record_offset":4708,"ancestor_record_offset":3940,"inlinee":4151,"source_line":3134,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":285,"record_offset":4772,"parent_record_offset":4748,"ancestor_record_offset":3940,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] usdl::sdl::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/sdl.rs
//     evidence={"module":285,"record_offset":4804,"parent_record_offset":4708,"ancestor_record_offset":3940,"inlinee":19019,"source_line":78,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] usdl::video::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":4708,"parent_record_offset":4060,"ancestor_record_offset":3940,"inlinee":19082,"source_line":132,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::VideoSubsystem::current_video_driver
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":3444,"rva":7122848,"va":5375831968}
// 编译实例 RVAs: 0x6cafa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8979957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831968,"raw_hex":"e8f5058900","size":5,"soft_fail":false,"target_va":5384811968,"va":5375832006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bf30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: video.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41226102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831968,"raw_hex":"e8760f7502","size":5,"soft_fail":false,"target_va":5417058096,"va":5375831989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::video::uSDL_GetCurrentVideoDriver
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831968,"raw_hex":"e8c66c9300","size":5,"soft_fail":false,"target_va":5385493616,"va":5375831973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ffi\c_str.rs
//     evidence={"module":285,"record_offset":3652,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":8370,"source_line":253,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::const_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":285,"record_offset":3588,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":19030,"source_line":22,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":285,"record_offset":3744,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":8040,"source_line":1590,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ffi\c_str.rs
//     evidence={"module":285,"record_offset":3676,"parent_record_offset":3652,"ancestor_record_offset":3444,"inlinee":8372,"source_line":737,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":285,"record_offset":3704,"parent_record_offset":3676,"ancestor_record_offset":3444,"inlinee":8374,"source_line":2434,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::is_null::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":285,"record_offset":3616,"parent_record_offset":3588,"ancestor_record_offset":3444,"inlinee":19032,"source_line":2434,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::VideoSubsystem::num_video_displays
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":3156,"rva":7122688,"va":5375831808}
// 编译实例 RVAs: 0x6caf00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] usdl::sdl::get_error
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831808,"raw_hex":"e81ef5ffff","size":5,"soft_fail":false,"target_va":5375829072,"va":5375831853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::video::uSDL_GetNumVideoDisplays
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831808,"raw_hex":"e8f36b9300","size":5,"soft_fail":false,"target_va":5385493248,"va":5375831816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::VideoSubsystem::display_name
// 记录形式: compiled_procedure; instances=1
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":1944,"rva":7122304,"va":5375831424}
// 编译实例 RVAs: 0x6cad80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_string_lossy
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8961913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831424,"raw_hex":"e879bf8800","size":5,"soft_fail":false,"target_va":5384793392,"va":5375831474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-152894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831424,"raw_hex":"e8c2aafdff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375831577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: video.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7109371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831424,"raw_hex":"e8058593ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375831558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: video.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41225452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831424,"raw_hex":"e8ec0c7502","size":5,"soft_fail":false,"target_va":5417057056,"va":5375831599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bf30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: video.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41226634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831424,"raw_hex":"e88a117502","size":5,"soft_fail":false,"target_va":5417058096,"va":5375831457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40789282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831424,"raw_hex":"e822656e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5375831507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::sdl::get_error
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831424,"raw_hex":"e86ff6ffff","size":5,"soft_fail":false,"target_va":5375829072,"va":5375831516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::video::uSDL_GetDisplayName
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831424,"raw_hex":"e84a6b9300","size":5,"soft_fail":false,"target_va":5385492704,"va":5375831441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::borrow::Cow::into_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":285,"record_offset":2240,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":5422,"source_line":331,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ffi\c_str.rs
//     evidence={"module":285,"record_offset":2144,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":8370,"source_line":253,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::const_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":285,"record_offset":2080,"parent_record_offset":1944,"ancestor_record_offset":1944,"inlinee":19030,"source_line":22,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":1944,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":1944,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":1944,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":2664,"parent_record_offset":2516,"ancestor_record_offset":1944,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":2456,"parent_record_offset":2428,"ancestor_record_offset":1944,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":2516,"parent_record_offset":2484,"ancestor_record_offset":1944,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":2484,"parent_record_offset":2456,"ancestor_record_offset":1944,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":2552,"parent_record_offset":2516,"ancestor_record_offset":1944,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":285,"record_offset":2340,"parent_record_offset":2304,"ancestor_record_offset":1944,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":285,"record_offset":2368,"parent_record_offset":2340,"ancestor_record_offset":1944,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":285,"record_offset":2396,"parent_record_offset":2368,"ancestor_record_offset":1944,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":285,"record_offset":2304,"parent_record_offset":2276,"ancestor_record_offset":1944,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":285,"record_offset":2276,"parent_record_offset":2240,"ancestor_record_offset":1944,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":285,"record_offset":2428,"parent_record_offset":2396,"ancestor_record_offset":1944,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":285,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":1944,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ffi\c_str.rs
//     evidence={"module":285,"record_offset":2172,"parent_record_offset":2144,"ancestor_record_offset":1944,"inlinee":8372,"source_line":737,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":285,"record_offset":2200,"parent_record_offset":2172,"ancestor_record_offset":1944,"inlinee":8374,"source_line":2434,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":285,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":1944,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":285,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":1944,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":285,"record_offset":2792,"parent_record_offset":2396,"ancestor_record_offset":1944,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::is_null::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":285,"record_offset":2108,"parent_record_offset":2080,"ancestor_record_offset":1944,"inlinee":19032,"source_line":2434,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":285,"record_offset":2816,"parent_record_offset":2792,"ancestor_record_offset":1944,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::VideoSubsystem::display_bounds
// 记录形式: compiled_procedure; instances=1
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":2876,"rva":7122528,"va":5375831648}
// 编译实例 RVAs: 0x6cae60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] usdl::sdl::get_error
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831648,"raw_hex":"e8c7f5ffff","size":5,"soft_fail":false,"target_va":5375829072,"va":5375831684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::video::uSDL_GetDisplayBounds
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831648,"raw_hex":"e8d36b9300","size":5,"soft_fail":false,"target_va":5385493072,"va":5375831672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::rect::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/rect.rs
//     evidence={"module":285,"record_offset":3004,"parent_record_offset":2876,"ancestor_record_offset":2876,"inlinee":19088,"source_line":2,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::VideoSubsystem::num_display_modes
// 记录形式: compiled_procedure; instances=1
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":3032,"rva":7122624,"va":5375831744}
// 编译实例 RVAs: 0x6caec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] usdl::sdl::get_error
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831744,"raw_hex":"e85bf5ffff","size":5,"soft_fail":false,"target_va":5375829072,"va":5375831792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::video::uSDL_GetNumDisplayModes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831744,"raw_hex":"e8e06b9300","size":5,"soft_fail":false,"target_va":5385493168,"va":5375831755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::VideoSubsystem::display_mode
// 记录形式: compiled_procedure; instances=1
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":1788,"rva":7122208,"va":5375831328}
// 编译实例 RVAs: 0x6cad20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] usdl::sdl::get_error
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831328,"raw_hex":"e8f9f6ffff","size":5,"soft_fail":false,"target_va":5375829072,"va":5375831378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::video::uSDL_GetDisplayMode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831328,"raw_hex":"e8356b9300","size":5,"soft_fail":false,"target_va":5385492608,"va":5375831366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::video::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/video.rs
//     evidence={"module":285,"record_offset":1916,"parent_record_offset":1788,"ancestor_record_offset":1788,"inlinee":19086,"source_line":21,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::VideoSubsystem::current_display_mode
// 记录形式: compiled_procedure; instances=1
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":3280,"rva":7122752,"va":5375831872}
// 编译实例 RVAs: 0x6caf40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] usdl::sdl::get_error
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831872,"raw_hex":"e8def4ffff","size":5,"soft_fail":false,"target_va":5375829072,"va":5375831917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::video::uSDL_GetCurrentDisplayMode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375831872,"raw_hex":"e8aa6c9300","size":5,"soft_fail":false,"target_va":5385493520,"va":5375831905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::video::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/video.rs
//     evidence={"module":285,"record_offset":3416,"parent_record_offset":3280,"ancestor_record_offset":3280,"inlinee":19086,"source_line":21,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::VideoSubsystem::desktop_display_mode
// 记录形式: compiled_procedure; instances=1
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":3776,"rva":7122944,"va":5375832064}
// 编译实例 RVAs: 0x6cb000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] usdl::sdl::get_error
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832064,"raw_hex":"e81ef4ffff","size":5,"soft_fail":false,"target_va":5375829072,"va":5375832109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::video::uSDL_GetDesktopDisplayMode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832064,"raw_hex":"e89a6c9300","size":5,"soft_fail":false,"target_va":5385493696,"va":5375832097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::video::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/video.rs
//     evidence={"module":285,"record_offset":3912,"parent_record_offset":3776,"ancestor_record_offset":3776,"inlinee":19086,"source_line":21,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::impl$1::drop
// 记录形式: compiled_procedure; instances=1
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":287,"record_offset":844,"rva":7124912,"va":5375834032}
// 编译实例 RVAs: 0x6cb7b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] usdl_sys::uSDL_QuitSubSystem
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t9656685","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375834032,"raw_hex":"e96d599300","size":5,"soft_fail":false,"target_va":5385490736,"va":5375834046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":287,"record_offset":956,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":4762,"source_line":3165,"archive":"_pdb/C/modules/00287.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":287,"record_offset":976,"parent_record_offset":956,"ancestor_record_offset":844,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00287.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::impl$2::clone
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A8A
// 首条位置: {"module":285,"inlinee":19082}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":285,"record_offset":688,"parent_record_offset":644,"ancestor_record_offset":644,"inlinee":4151,"source_line":3134,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] usdl::sdl::impl$2::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/sdl.rs
//     evidence={"module":285,"record_offset":744,"parent_record_offset":644,"ancestor_record_offset":644,"inlinee":19019,"source_line":78,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":285,"record_offset":776,"parent_record_offset":744,"ancestor_record_offset":644,"inlinee":4151,"source_line":3134,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":285,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":644,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::impl$16::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 144
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":16,"record_offset":30260,"rva":1069632,"va":5369778752}
// 编译实例 RVAs: 0x105240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_tuple_field1_finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15038862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369778752,"raw_hex":"e88e79e500","size":5,"soft_fail":false,"target_va":5384817664,"va":5369778797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::PixelFormatEnum::byte_size_per_pixel
// 记录形式: compiled_procedure; instances=1
// 源码行: 150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":5192,"rva":7123392,"va":5375832512}
// 编译实例 RVAs: 0x6cb1c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] usdl::video::is_sdl_non_fourcc_format
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":5328,"parent_record_offset":5192,"ancestor_record_offset":5192,"inlinee":19092,"source_line":174,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::is_sdl_non_fourcc_format
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A94
// 首条位置: {"module":285,"inlinee":19092}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::impl$24::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1879
// 首条位置: {"module":16,"inlinee":6265}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::WindowBuilder::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 246
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A92
// 首条位置: {"module":285,"inlinee":19090}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":285,"record_offset":4108,"parent_record_offset":4060,"ancestor_record_offset":4060,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":285,"record_offset":4900,"parent_record_offset":4060,"ancestor_record_offset":4060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] usdl::video::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":4708,"parent_record_offset":4060,"ancestor_record_offset":4060,"inlinee":19082,"source_line":132,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":4060,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":4552,"parent_record_offset":4528,"ancestor_record_offset":4060,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":5076,"parent_record_offset":5052,"ancestor_record_offset":4060,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":5100,"parent_record_offset":5076,"ancestor_record_offset":4060,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":4576,"parent_record_offset":4552,"ancestor_record_offset":4060,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":5124,"parent_record_offset":5100,"ancestor_record_offset":4060,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":4504,"parent_record_offset":4348,"ancestor_record_offset":4060,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":5052,"parent_record_offset":4996,"ancestor_record_offset":4060,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":4284,"parent_record_offset":4256,"ancestor_record_offset":4060,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":5024,"parent_record_offset":4996,"ancestor_record_offset":4060,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":4996,"parent_record_offset":4972,"ancestor_record_offset":4060,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":4348,"parent_record_offset":4312,"ancestor_record_offset":4060,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":4312,"parent_record_offset":4284,"ancestor_record_offset":4060,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":4972,"parent_record_offset":4948,"ancestor_record_offset":4060,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":285,"record_offset":4392,"parent_record_offset":4348,"ancestor_record_offset":4060,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":285,"record_offset":4164,"parent_record_offset":4136,"ancestor_record_offset":4060,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":285,"record_offset":4192,"parent_record_offset":4164,"ancestor_record_offset":4060,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":285,"record_offset":4220,"parent_record_offset":4192,"ancestor_record_offset":4060,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":285,"record_offset":4136,"parent_record_offset":4108,"ancestor_record_offset":4060,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":285,"record_offset":4256,"parent_record_offset":4220,"ancestor_record_offset":4060,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":285,"record_offset":4416,"parent_record_offset":4392,"ancestor_record_offset":4060,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":285,"record_offset":4464,"parent_record_offset":4440,"ancestor_record_offset":4060,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":285,"record_offset":4440,"parent_record_offset":4416,"ancestor_record_offset":4060,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":285,"record_offset":4632,"parent_record_offset":4220,"ancestor_record_offset":4060,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":285,"record_offset":4656,"parent_record_offset":4632,"ancestor_record_offset":4060,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":285,"record_offset":4924,"parent_record_offset":4900,"ancestor_record_offset":4060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":285,"record_offset":4748,"parent_record_offset":4708,"ancestor_record_offset":4060,"inlinee":4151,"source_line":3134,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":285,"record_offset":4772,"parent_record_offset":4748,"ancestor_record_offset":4060,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] usdl::sdl::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/sdl.rs
//     evidence={"module":285,"record_offset":4804,"parent_record_offset":4708,"ancestor_record_offset":4060,"inlinee":19019,"source_line":78,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::WindowBuilder::hidden
// 记录形式: compiled_procedure; instances=1
// 源码行: 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":1336,"rva":7122128,"va":5375831248}
// 编译实例 RVAs: 0x6cacd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::WindowBuilder::resizable
// 记录形式: compiled_procedure; instances=1
// 源码行: 263
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":1672,"rva":7122192,"va":5375831312}
// 编译实例 RVAs: 0x6cad10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::WindowBuilder::vulkan
// 记录形式: compiled_procedure; instances=1
// 源码行: 268
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":1448,"rva":7122144,"va":5375831264}
// 编译实例 RVAs: 0x6cace0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::WindowBuilder::position
// 记录形式: compiled_procedure; instances=1
// 源码行: 273
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":1560,"rva":7122160,"va":5375831280}
// 编译实例 RVAs: 0x6cacf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::WindowBuilder::allow_highdpi
// 记录形式: compiled_procedure; instances=1
// 源码行: 279
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":260,"rva":7121696,"va":5375830816}
// 编译实例 RVAs: 0x6cab20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::WindowBuilder::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 284
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":380,"rva":7121712,"va":5375830832}
// 编译实例 RVAs: 0x6cab30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-152412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e8a4acfdff","size":5,"soft_fail":false,"target_va":5375678704,"va":5375831111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f574e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: video.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8964485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e885c98800","size":5,"soft_fail":false,"target_va":5384795360,"va":5375830870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40791089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e8316c6e02","size":5,"soft_fail":false,"target_va":5416622240,"va":5375831146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::sdl::get_error
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e83df8ffff","size":5,"soft_fail":false,"target_va":5375829072,"va":5375831054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::video::uSDL_CreateWindow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e85e6b9300","size":5,"soft_fail":false,"target_va":5385492240,"va":5375830957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\ffi\c_str.rs
//     evidence={"module":285,"record_offset":520,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":6244,"source_line":257,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":285,"record_offset":580,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":6793,"source_line":1363,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":285,"record_offset":896,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":285,"record_offset":548,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":285,"record_offset":840,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] usdl::video::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":644,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":19082,"source_line":132,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":380,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":380,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":380,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":285,"record_offset":980,"parent_record_offset":952,"ancestor_record_offset":380,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":285,"record_offset":952,"parent_record_offset":920,"ancestor_record_offset":380,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\ffi\c_str.rs
//     evidence={"module":285,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":380,"inlinee":4646,"source_line":700,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":285,"record_offset":1156,"parent_record_offset":1104,"ancestor_record_offset":380,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":285,"record_offset":688,"parent_record_offset":644,"ancestor_record_offset":380,"inlinee":4151,"source_line":3134,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":285,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":380,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] usdl::sdl::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/sdl.rs
//     evidence={"module":285,"record_offset":744,"parent_record_offset":644,"ancestor_record_offset":380,"inlinee":19019,"source_line":78,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] usdl::video::impl$5::build::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":612,"parent_record_offset":580,"ancestor_record_offset":380,"inlinee":19084,"source_line":296,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::impl$5::build::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A8C
// 首条位置: {"module":285,"inlinee":19084}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::impl$39::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 304
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":16,"record_offset":30364,"rva":1069696,"va":5369778816}
// 编译实例 RVAs: 0x105280, 0x55e2c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_tuple_field1_finish
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t15038787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369778816,"raw_hex":"e84379e500","size":5,"soft_fail":false,"target_va":5384817664,"va":5369778872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::impl$6::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 311
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":7416,"rva":7124352,"va":5375833472}
// 编译实例 RVAs: 0x6cb580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8992619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375833472,"raw_hex":"e86b378900","size":5,"soft_fail":false,"target_va":5384826176,"va":5375833552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: video.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t8990574","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375833472,"raw_hex":"e96e2f8900","size":5,"soft_fail":false,"target_va":5384824080,"va":5375833501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":285,"record_offset":7524,"parent_record_offset":7416,"ancestor_record_offset":7416,"inlinee":4604,"source_line":2121,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::Window::show
// 记录形式: compiled_procedure; instances=1
// 源码行: 329
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":6816,"rva":7124224,"va":5375833344}
// 编译实例 RVAs: 0x6cb500
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] usdl_sys::video::uSDL_ShowWindow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t9658696","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375833344,"raw_hex":"e948619300","size":5,"soft_fail":false,"target_va":5385492048,"va":5375833347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::Window::maximize
// 记录形式: compiled_procedure; instances=1
// 源码行: 333
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":7092,"rva":7124304,"va":5375833424}
// 编译实例 RVAs: 0x6cb550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] usdl_sys::video::uSDL_MaximizeWindow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t9659448","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375833424,"raw_hex":"e938649300","size":5,"soft_fail":false,"target_va":5385492880,"va":5375833427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::Window::minimize
// 记录形式: compiled_procedure; instances=1
// 源码行: 337
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":7200,"rva":7124320,"va":5375833440}
// 编译实例 RVAs: 0x6cb560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] usdl_sys::video::uSDL_MinimizeWindow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t9659528","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375833440,"raw_hex":"e988649300","size":5,"soft_fail":false,"target_va":5385492976,"va":5375833443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::Window::set_grab
// 记录形式: compiled_procedure; instances=1
// 源码行: 341
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":7308,"rva":7124336,"va":5375833456}
// 编译实例 RVAs: 0x6cb570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] usdl_sys::video::uSDL_SetWindowGrab
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t9659045","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375833456,"raw_hex":"e9a5629300","size":5,"soft_fail":false,"target_va":5385492512,"va":5375833462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::Window::display_index
// 记录形式: compiled_procedure; instances=1
// 源码行: 344
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":5356,"rva":7123520,"va":5375832640}
// 编译实例 RVAs: 0x6cb240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] usdl::sdl::get_error
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832640,"raw_hex":"e8dbf1ffff","size":5,"soft_fail":false,"target_va":5375829072,"va":5375832688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::video::uSDL_GetWindowDisplayIndex
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832640,"raw_hex":"e8d06a9300","size":5,"soft_fail":false,"target_va":5385493792,"va":5375832651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::Window::id
// 记录形式: compiled_procedure; instances=1
// 源码行: 354
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":6716,"rva":7124208,"va":5375833328}
// 编译实例 RVAs: 0x6cb4f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] usdl_sys::video::uSDL_GetWindowID
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t9658808","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375833328,"raw_hex":"e9b8619300","size":5,"soft_fail":false,"target_va":5385492144,"va":5375833331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::Window::size
// 记录形式: compiled_procedure; instances=1
// 源码行: 357
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":6920,"rva":7124240,"va":5375833360}
// 编译实例 RVAs: 0x6cb510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] usdl_sys::video::uSDL_GetWindowSize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9659018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375833360,"raw_hex":"e88a629300","size":5,"soft_fail":false,"target_va":5385492416,"va":5375833393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":285,"record_offset":7032,"parent_record_offset":6920,"ancestor_record_offset":6920,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::Window::vulkan_drawable_size
// 记录形式: compiled_procedure; instances=1
// 源码行: 364
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":6528,"rva":7124144,"va":5375833264}
// 编译实例 RVAs: 0x6cb4b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] usdl_sys::video::uSDL_Vulkan_GetDrawableSize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9660586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375833264,"raw_hex":"e8aa689300","size":5,"soft_fail":false,"target_va":5385493888,"va":5375833297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":285,"record_offset":6656,"parent_record_offset":6528,"ancestor_record_offset":6528,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::Window::window_handle
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 372
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A9A
// 首条位置: {"module":285,"inlinee":19098}
// 编译实例 RVAs: 0x6cb280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141001de0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: video.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9661257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832704,"raw_hex":"e8496b9300","size":5,"soft_fail":false,"target_va":5385493984,"va":5375832722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::syswm::uSDL_GetVersion
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832704,"raw_hex":"e8486b9300","size":5,"soft_fail":false,"target_va":5385494016,"va":5375832755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::syswm::uSDL_GetWindowWMInfo
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832704,"raw_hex":"e88d6b9300","size":5,"soft_fail":false,"target_va":5385494096,"va":5375832766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":285,"record_offset":5668,"parent_record_offset":5468,"ancestor_record_offset":5468,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":285,"record_offset":5696,"parent_record_offset":5468,"ancestor_record_offset":5468,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] usdl::video::Window::wm_info
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":5600,"parent_record_offset":5468,"ancestor_record_offset":5468,"inlinee":19093,"source_line":427,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] usdl::video::window_handle_from_wm_info
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":5724,"parent_record_offset":5468,"ancestor_record_offset":5468,"inlinee":19096,"source_line":454,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":285,"record_offset":5760,"parent_record_offset":5724,"ancestor_record_offset":5468,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::syswm::impl$21::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/syswm.rs
//     evidence={"module":285,"record_offset":5632,"parent_record_offset":5600,"ancestor_record_offset":5468,"inlinee":19095,"source_line":171,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::impl$9::window_handle
// 记录形式: compiled_procedure; instances=1
// 源码行: 372
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":7556,"rva":7123584,"va":5375832704}
// 编译实例 RVAs: 0x6cb280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141001de0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: video.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9661257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832704,"raw_hex":"e8496b9300","size":5,"soft_fail":false,"target_va":5385493984,"va":5375832722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::syswm::uSDL_GetVersion
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832704,"raw_hex":"e8486b9300","size":5,"soft_fail":false,"target_va":5385494016,"va":5375832755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::syswm::uSDL_GetWindowWMInfo
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832704,"raw_hex":"e88d6b9300","size":5,"soft_fail":false,"target_va":5385494096,"va":5375832766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl::video::Window::window_handle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":7676,"parent_record_offset":7556,"ancestor_record_offset":7556,"inlinee":19098,"source_line":372,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":285,"record_offset":7860,"parent_record_offset":7824,"ancestor_record_offset":7556,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":285,"record_offset":7768,"parent_record_offset":7676,"ancestor_record_offset":7556,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":285,"record_offset":7796,"parent_record_offset":7676,"ancestor_record_offset":7556,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] usdl::video::Window::wm_info
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":7700,"parent_record_offset":7676,"ancestor_record_offset":7556,"inlinee":19093,"source_line":427,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] usdl::video::window_handle_from_wm_info
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":7824,"parent_record_offset":7676,"ancestor_record_offset":7556,"inlinee":19096,"source_line":454,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::syswm::impl$21::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/syswm.rs
//     evidence={"module":285,"record_offset":7732,"parent_record_offset":7700,"ancestor_record_offset":7556,"inlinee":19095,"source_line":171,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::Window::display_handle
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 378
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A9B
// 首条位置: {"module":285,"inlinee":19099}
// 编译实例 RVAs: 0x6cb330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141001de0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: video.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9661081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832880,"raw_hex":"e8996a9300","size":5,"soft_fail":false,"target_va":5385493984,"va":5375832898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::syswm::uSDL_GetVersion
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832880,"raw_hex":"e8986a9300","size":5,"soft_fail":false,"target_va":5385494016,"va":5375832931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::syswm::uSDL_GetWindowWMInfo
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832880,"raw_hex":"e8dd6a9300","size":5,"soft_fail":false,"target_va":5385494096,"va":5375832942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":285,"record_offset":6024,"parent_record_offset":5824,"ancestor_record_offset":5824,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":285,"record_offset":6052,"parent_record_offset":5824,"ancestor_record_offset":5824,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] usdl::video::Window::wm_info
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":5956,"parent_record_offset":5824,"ancestor_record_offset":5824,"inlinee":19093,"source_line":427,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] usdl::video::display_handle_from_wm_info
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":6080,"parent_record_offset":5824,"ancestor_record_offset":5824,"inlinee":19097,"source_line":546,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::syswm::impl$21::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/syswm.rs
//     evidence={"module":285,"record_offset":5988,"parent_record_offset":5956,"ancestor_record_offset":5824,"inlinee":19095,"source_line":171,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::impl$10::display_handle
// 记录形式: compiled_procedure; instances=1
// 源码行: 378
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":7928,"rva":7123760,"va":5375832880}
// 编译实例 RVAs: 0x6cb330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141001de0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: video.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9661081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832880,"raw_hex":"e8996a9300","size":5,"soft_fail":false,"target_va":5385493984,"va":5375832898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::syswm::uSDL_GetVersion
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832880,"raw_hex":"e8986a9300","size":5,"soft_fail":false,"target_va":5385494016,"va":5375832931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::syswm::uSDL_GetWindowWMInfo
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832880,"raw_hex":"e8dd6a9300","size":5,"soft_fail":false,"target_va":5385494096,"va":5375832942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl::video::Window::display_handle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":8048,"parent_record_offset":7928,"ancestor_record_offset":7928,"inlinee":19099,"source_line":378,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":285,"record_offset":8140,"parent_record_offset":8048,"ancestor_record_offset":7928,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":285,"record_offset":8168,"parent_record_offset":8048,"ancestor_record_offset":7928,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] usdl::video::Window::wm_info
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":8072,"parent_record_offset":8048,"ancestor_record_offset":7928,"inlinee":19093,"source_line":427,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] usdl::video::display_handle_from_wm_info
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":285,"record_offset":8196,"parent_record_offset":8048,"ancestor_record_offset":7928,"inlinee":19097,"source_line":546,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::syswm::impl$21::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/syswm.rs
//     evidence={"module":285,"record_offset":8104,"parent_record_offset":8072,"ancestor_record_offset":7928,"inlinee":19095,"source_line":171,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::Window::set_display_mode
// 记录形式: compiled_procedure; instances=1
// 源码行: 383
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":6340,"rva":7124016,"va":5375833136}
// 编译实例 RVAs: 0x6cb430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] usdl::sdl::get_error
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375833136,"raw_hex":"e8ceefffff","size":5,"soft_fail":false,"target_va":5375829072,"va":5375833213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::video::uSDL_SetWindowDisplayMode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9660218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375833136,"raw_hex":"e83a679300","size":5,"soft_fail":false,"target_va":5385493424,"va":5375833201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":285,"record_offset":6496,"parent_record_offset":6340,"ancestor_record_offset":6340,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":285,"record_offset":6468,"parent_record_offset":6340,"ancestor_record_offset":6340,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::Window::set_fullscreen
// 记录形式: compiled_procedure; instances=1
// 源码行: 396
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":6112,"rva":7123904,"va":5375833024}
// 编译实例 RVAs: 0x6cb3c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] usdl::sdl::get_error
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375833024,"raw_hex":"e871f0ffff","size":5,"soft_fail":false,"target_va":5375829072,"va":5375833050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::video::uSDL_SetWindowFullscreen
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9660285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375833024,"raw_hex":"e87d679300","size":5,"soft_fail":false,"target_va":5385493328,"va":5375833038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::Window::fullscreen_state
// 记录形式: compiled_procedure; instances=1
// 源码行: 405
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":285,"record_offset":6224,"rva":7123968,"va":5375833088}
// 编译实例 RVAs: 0x6cb400
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] usdl_sys::video::uSDL_GetWindowFlags
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9659684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375833088,"raw_hex":"e824659300","size":5,"soft_fail":false,"target_va":5385492784,"va":5375833095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::Window::raw
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 422
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A7D
// 首条位置: {"module":284,"inlinee":19069}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::Window::wm_info
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 427
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A95
// 首条位置: {"module":285,"inlinee":19093}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] usdl_sys::syswm::impl$21::default
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/syswm.rs
//     evidence={"module":285,"record_offset":5632,"parent_record_offset":5600,"ancestor_record_offset":5600,"inlinee":19095,"source_line":171,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::window_handle_from_wm_info
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 454
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A98
// 首条位置: {"module":285,"inlinee":19096}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::ok_or
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":285,"record_offset":5760,"parent_record_offset":5724,"ancestor_record_offset":5724,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00285.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::display_handle_from_wm_info
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 546
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A99
// 首条位置: {"module":285,"inlinee":19097}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::video::impl$11::drop
// 记录形式: compiled_procedure; instances=1
// 源码行: 649
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":287,"record_offset":572,"rva":7124880,"va":5375834000}
// 编译实例 RVAs: 0x6cb790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] usdl_sys::video::uSDL_DestroyWindow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t9658328","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375834000,"raw_hex":"e9d85f9300","size":5,"soft_fail":false,"target_va":5385492336,"va":5375834003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 完整结构及原始类型记录: video.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] usdl::video::DisplayMode
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 13
// [PDB 泛型关系，非字段关系] {"type": "tuple$<i64,usdl::video::DisplayMode>", "constructor": "tuple$", "arguments": ["i64", "usdl::video::DisplayMode"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<usdl::video::DisplayMode,i64,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$2>", "constructor": "core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0", "arguments": ["usdl::video::DisplayMode", "i64", "tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tuple$<i64,usdl::video::DisplayMode> >", "constructor": "ref$", "arguments": ["tuple$<i64,usdl::video::DisplayMode>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<tuple$<i64,usdl::video::DisplayMode>,core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)>", "constructor": "core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0", "arguments": ["tuple$<i64,usdl::video::DisplayMode>", "core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<usdl::video::DisplayMode,tuple$<i64,usdl::video::DisplayMode>,tuple$<i64,usdl::video::DisplayMode>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<usdl::video::DisplayMode,i64,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$2>,core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<tuple$<i64,usdl::video::DisplayMode>,core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)> >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["usdl::video::DisplayMode", "tuple$<i64,usdl::video::DisplayMode>", "tuple$<i64,usdl::video::DisplayMode>", "core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<usdl::video::DisplayMode,i64,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$2>", "core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<tuple$<i64,usdl::video::DisplayMode>,core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::fold<usdl::video::DisplayMode,alloc::alloc::Global,tuple$<i64,usdl::video::DisplayMode>,core::iter::adapters::map::map_fold::closure_env$0<usdl::video::DisplayMode,tuple$<i64,usdl::video::DisplayMode>,tuple$<i64,usdl::video::DisplayMode>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<usdl::video::DisplayMode,i64,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$2>,core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<tuple$<i64,usdl::video::DisplayMode>,core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)> > >", "constructor": "alloc::vec::into_iter::impl$6::fold", "arguments": ["usdl::video::DisplayMode", "alloc::alloc::Global", "tuple$<i64,usdl::video::DisplayMode>", "core::iter::adapters::map::map_fold::closure_env$0<usdl::video::DisplayMode,tuple$<i64,usdl::video::DisplayMode>,tuple$<i64,usdl::video::DisplayMode>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<usdl::video::DisplayMode,i64,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$2>,core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<tuple$<i64,usdl::video::DisplayMode>,core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<usdl::video::DisplayMode>", "constructor": "core::fmt::impl$80::fmt", "arguments": ["usdl::video::DisplayMode"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1>", "constructor": "alloc::slice::impl$0::sort_by_key::closure_env$0", "arguments": ["usdl::video::DisplayMode", "tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >", "tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insert_tail<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::insert_tail", "arguments": ["usdl::video::DisplayMode", "alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::sort4_stable", "arguments": ["usdl::video::DisplayMode", "alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::bidirectional_merge<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::bidirectional_merge", "arguments": ["usdl::video::DisplayMode", "alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["usdl::video::DisplayMode", "alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<usdl::video::DisplayMode>", "constructor": "core::slice::iter::Iter", "arguments": ["usdl::video::DisplayMode"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<usdl::video::DisplayMode>,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$3>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<usdl::video::DisplayMode>", "tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::map::Map<core::slice::iter::Iter<usdl::video::DisplayMode>,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$3> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["alloc::string::String", "core::iter::adapters::map::Map<core::slice::iter::Iter<usdl::video::DisplayMode>,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["usdl::video::DisplayMode", "alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::merge::merge<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >", "constructor": "core::slice::sort::stable::merge::merge", "arguments": ["usdl::video::DisplayMode", "alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<usdl::video::DisplayMode,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["usdl::video::DisplayMode", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["usdl::video::DisplayMode", "alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["usdl::video::DisplayMode", "alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<usdl::video::DisplayMode,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["usdl::video::DisplayMode", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1>,alloc::vec::Vec<usdl::video::DisplayMode,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["usdl::video::DisplayMode", "alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1>", "alloc::vec::Vec<usdl::video::DisplayMode,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] usdl::video::PixelFormatEnum
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] usdl::video::VideoSubsystem
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 11; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<usdl::video::VideoSubsystem>", "constructor": "core::ptr::drop_in_place", "arguments": ["usdl::video::VideoSubsystem"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] usdl::video::Window
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 17; 过程引用数: 14
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<usdl::video::Window>", "constructor": "core::ptr::drop_in_place", "arguments": ["usdl::video::Window"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] usdl::video::WindowBuildError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 20
// [PDB 泛型关系，非字段关系] {"type": "enum2$<usdl::video::WindowBuildError>", "constructor": "enum2$", "arguments": ["usdl::video::WindowBuildError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> >", "constructor": "anyhow::error::ContextError", "arguments": ["ref$<str$>", "enum2$<usdl::video::WindowBuildError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> > >", "constructor": "anyhow::error::ErrorImpl", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<usdl::video::WindowBuildError> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<usdl::video::WindowBuildError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<enum2$<usdl::video::WindowBuildError> >", "constructor": "core::error::Error::type_id", "arguments": ["enum2$<usdl::video::WindowBuildError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> > >", "constructor": "core::error::Error::type_id", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> > > >", "constructor": "core::error::Error::type_id", "arguments": ["anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_ref<anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> > >", "constructor": "anyhow::error::object_ref", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_boxed<anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> > >", "constructor": "anyhow::error::object_boxed", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::context_downcast<ref$<str$>,enum2$<usdl::video::WindowBuildError> >", "constructor": "anyhow::error::context_downcast", "arguments": ["ref$<str$>", "enum2$<usdl::video::WindowBuildError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::Error::construct<anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> > >", "constructor": "anyhow::Error::construct", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$4::source<ref$<str$>,enum2$<usdl::video::WindowBuildError> >", "constructor": "anyhow::context::impl$4::source", "arguments": ["ref$<str$>", "enum2$<usdl::video::WindowBuildError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::cause<anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> > >", "constructor": "core::error::Error::cause", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_drop<anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> > >", "constructor": "anyhow::error::object_drop", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::context_drop_rest<ref$<str$>,enum2$<usdl::video::WindowBuildError> >", "constructor": "anyhow::error::context_drop_rest", "arguments": ["ref$<str$>", "enum2$<usdl::video::WindowBuildError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_reallocate_boxed<anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> > >", "constructor": "anyhow::error::object_reallocate_boxed", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl::video::WindowBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$2::fmt<ref$<str$>,enum2$<usdl::video::WindowBuildError> >", "constructor": "anyhow::context::impl$2::fmt", "arguments": ["ref$<str$>", "enum2$<usdl::video::WindowBuildError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$3::fmt<ref$<str$>,enum2$<usdl::video::WindowBuildError> >", "constructor": "anyhow::context::impl$3::fmt", "arguments": ["ref$<str$>", "enum2$<usdl::video::WindowBuildError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] usdl::video::WindowBuilder
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 8; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<usdl::video::WindowBuilder>", "constructor": "core::ptr::drop_in_place", "arguments": ["usdl::video::WindowBuilder"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: video.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: video.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)
// 候选参数类型: ["ref$<tuple$<i64,usdl::video::DisplayMode> >", "ref$<tuple$<i64,usdl::video::DisplayMode> >"]

// ------------------------------------------------------------------
// 函数: usdl::video::PixelFormatEnum::byte_size_per_pixel
// 信息 ID: 3e3c6997e41d35bf2532c4f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a4cc73cfe63fb403c18f57c3: core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)
// 引用方: 原程序直接转移 69 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::into_iter::impl$6::fold<usdl::video::DisplayMode,alloc::alloc::Global,tuple$<i64,usdl::video::DisplayMode>,core::iter::adapters::map::map_fold::closure_env$0<usdl::video::DisplayMode,tuple$<i64,usdl::video::DisplayMode>,tuple$<i64,usdl::video::DisplayMode>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<usdl::video::DisplayMode,i64,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$2>,core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<tuple$<i64,usdl::video::DisplayMode>,core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)> > > @ 0xc9f35 -> 0x6cb1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::slice::sort::shared::smallsort::insert_tail<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> > @ 0x1d351d -> 0x6cb1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::slice::sort::shared::smallsort::insert_tail<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> > @ 0x1d3536 -> 0x6cb1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::slice::sort::shared::smallsort::insert_tail<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> > @ 0x1d35d2 -> 0x6cb1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::slice::sort::shared::smallsort::insert_tail<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> > @ 0x1d35e9 -> 0x6cb1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> > @ 0x1d4d72 -> 0x6cb1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> > @ 0x1d4d87 -> 0x6cb1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> > @ 0x1d4dc2 -> 0x6cb1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> > @ 0x1d4dd8 -> 0x6cb1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> > @ 0x1d4e44 -> 0x6cb1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> > @ 0x1d4e65 -> 0x6cb1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> > @ 0x1d4ec7 -> 0x6cb1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> > @ 0x1d4edf -> 0x6cb1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> > @ 0x1d4f44 -> 0x6cb1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::slice::sort::shared::smallsort::sort4_stable<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> > @ 0x1d4f67 -> 0x6cb1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::slice::sort::shared::smallsort::bidirectional_merge<usdl::video::DisplayMode,alloc::slice::impl$0::sort_by_key::closure_env$0<usdl::video::DisplayMode,tuple$<core::cmp::Reverse<i32>,core::cmp::Reverse<i32>,core::cmp::Reverse<u32>,core::cmp::Reverse<i32> >,tiny_glade::sdl_setup::find_best_fullscreen_video_mode::closure_env$1> > @ 0x1d5d02 -> 0x6cb1c0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::VideoSubsystem::current_display_mode
// 信息 ID: 9b4a2e6a2cab085701e9f750
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::find_best_fullscreen_video_mode @ 0x3f9fec -> 0x6caf40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::VideoSubsystem::current_video_driver
// 信息 ID: 9a6e6a83ab70a76901e5ccc9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8af9 -> 0x6cafa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::VideoSubsystem::desktop_display_mode
// 信息 ID: ba9a4cb02bb33c8d7276dcc6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::find_best_fullscreen_video_mode @ 0x3fa03a -> 0x6cb000 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_event_loop::extract_available_monitors_info @ 0x517461 -> 0x6cb000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::VideoSubsystem::display_bounds
// 信息 ID: 8abacd12e0fe6d289cebfdb8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8b5c -> 0x6cae60 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8b9b -> 0x6cae60 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_setup::monitor_match_from_sdl @ 0x3f9776 -> 0x6cae60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::VideoSubsystem::display_mode
// 信息 ID: 5337fcb699918740c0ee9f38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::find_best_fullscreen_video_mode @ 0x3f9a91 -> 0x6cad20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_event_loop::extract_available_monitors_info @ 0x517373 -> 0x6cad20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::VideoSubsystem::display_name
// 信息 ID: 4e6b1c2c7e0fff9f301bfe5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::monitor_match_from_sdl @ 0x3f97dc -> 0x6cad80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_setup::find_best_fullscreen_video_mode @ 0x3f996f -> 0x6cad80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::VideoSubsystem::init
// 信息 ID: 4c9287606d6e4bfb839cc0ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl::sdl::Sdl::video; module=281, inline_record=1492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::VideoSubsystem::num_display_modes
// 信息 ID: 9372a63dc08026e0c9e74fba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::find_best_fullscreen_video_mode @ 0x3f99fb -> 0x6caec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_event_loop::extract_available_monitors_info @ 0x5172ea -> 0x6caec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::VideoSubsystem::num_video_displays
// 信息 ID: 273ff96e2726ace92e3cebc3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::enumerate_displays @ 0x3f95b6 -> 0x6caf00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::VideoSubsystem::window
// 信息 ID: 0309807378bd9dc0081d4c9d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8af0 -> 0x6cb060 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::Window::display_handle
// 信息 ID: 5fbeb90a7ab6ee655d1efea3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::setup::create_swapchain_with_fallback<ref_mut$<enum2$<tiny_glade::run_game::WindowBackendWindow> > > @ 0x412e7c -> 0x6cb330 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::impl$5::display_handle @ 0x41ad70 -> 0x6cb330 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls> @ 0x52f787 -> 0x6cb330 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] usdl::video::impl$10::display_handle; module=285, inline_record=8048; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::Window::display_index
// 信息 ID: 1dacb4997eb26b4010d72771
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_event_loop::run_event_loop @ 0x5135ef -> 0x6cb240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::Window::fullscreen_state
// 信息 ID: dacdf2b8317a47f276902441
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::impl$4::fullscreen_mode @ 0x41a77c -> 0x6cb400 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::WindowBackendWindow::cancel_fullscreen @ 0x42b563 -> 0x6cb400 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::Window::id
// 信息 ID: 5a6bad42caecf94a3761fc80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::Window::maximize
// 信息 ID: 9ca8480b021fae20eb33e366
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::WindowBackendWindow::cancel_fullscreen @ 0x42b59b -> 0x6cb550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::Window::minimize
// 信息 ID: 3bf46df00f8a3bd3d07c1854
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::Window::raw
// 信息 ID: 307732520a8f803d1b4333bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl::mouse::MouseUtil::warp_mouse_in_window; module=284, inline_record=840; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::Window::set_display_mode
// 信息 ID: ff93aac9f03f8f6b2604368f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8d2d -> 0x6cb430 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::Window::set_fullscreen
// 信息 ID: ce441387f8e2a762ca6a8206
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8d50 -> 0x6cb3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f9008 -> 0x6cb3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::impl$4::downgrade_exclusive_fullscreen_to_borderless @ 0x41a97b -> 0x6cb3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::WindowBackendWindow::cancel_fullscreen @ 0x42b576 -> 0x6cb3c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::Window::set_grab
// 信息 ID: b48a0157386de3a44be59ccb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::Window::show
// 信息 ID: 6f6c0a505fa19e240067b7d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_event_loop::run_event_loop @ 0x513963 -> 0x6cb500 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls> @ 0x52f8ea -> 0x6cb500 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::Window::size
// 信息 ID: 2d897b79da28163813793728
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_event_loop::run_event_loop @ 0x51375f -> 0x6cb510 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls> @ 0x52f6fa -> 0x6cb510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::Window::vulkan_drawable_size
// 信息 ID: 5b88e37945cd51d4099be19d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::WindowBackendWindow::inner_size @ 0x42b3f4 -> 0x6cb4b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_event_loop::run_event_loop @ 0x513771 -> 0x6cb4b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::Window::window_handle
// 信息 ID: e8226829fb11d8e9aacea9d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::setup::create_swapchain_with_fallback<ref_mut$<enum2$<tiny_glade::run_game::WindowBackendWindow> > > @ 0x412ffb -> 0x6cb280 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls> @ 0x52f750 -> 0x6cb280 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] usdl::video::impl$9::window_handle; module=285, inline_record=7676; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::Window::wm_info
// 信息 ID: e170086523da7284b30da788
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl::video::Window::window_handle; module=285, inline_record=5600; 不是独立调用边。
// [PDB 内联宿主] usdl::video::Window::display_handle; module=285, inline_record=5956; 不是独立调用边。
// [PDB 内联宿主] usdl::video::impl$9::window_handle; module=285, inline_record=7700; 不是独立调用边。
// [PDB 内联宿主] usdl::video::impl$10::display_handle; module=285, inline_record=8072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::WindowBuilder::allow_highdpi
// 信息 ID: da75441ba6b814a8a23a90d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8b38 -> 0x6cab20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::WindowBuilder::build
// 信息 ID: 7921b0ef0ee911cb2740e598
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8cbc -> 0x6cab30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::WindowBuilder::hidden
// 信息 ID: a20d847d2696f11db003b4fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8b1c -> 0x6cacd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::WindowBuilder::new
// 信息 ID: 8ba1e7b8f0380e5503c6001c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl::video::VideoSubsystem::window; module=285, inline_record=4060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::WindowBuilder::position
// 信息 ID: 3893529be914a19ea2314427
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8c97 -> 0x6cacf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8c25 -> 0x6cacf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::WindowBuilder::resizable
// 信息 ID: b95d1aee0aa23aba9bbac53d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8b26 -> 0x6cad10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::WindowBuilder::vulkan
// 信息 ID: 894549c59d90b755ce2d4a9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8b2f -> 0x6cace0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::display_handle_from_wm_info
// 信息 ID: 41653ca4e397e46cd581dd3d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl::video::Window::display_handle; module=285, inline_record=6080; 不是独立调用边。
// [PDB 内联宿主] usdl::video::impl$10::display_handle; module=285, inline_record=8196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::impl$10::display_handle
// 信息 ID: ec5107225688958a57841ec6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::setup::create_swapchain_with_fallback<ref_mut$<enum2$<tiny_glade::run_game::WindowBackendWindow> > > @ 0x412e7c -> 0x6cb330 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::impl$5::display_handle @ 0x41ad70 -> 0x6cb330 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls> @ 0x52f787 -> 0x6cb330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::impl$11::drop
// 信息 ID: e5368d6e2d23685556aae325
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<usdl::video::Window> @ 0x99766 -> 0x6cb790 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x430a52 -> 0x6cb790 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x4308cd -> 0x6cb790 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42f6ac -> 0x6cb790 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42e371 -> 0x6cb790 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<enum2$<tiny_glade::run_game::WindowBackend>,enum2$<tiny_glade::run_game::WindowBackendWindow> >,anyhow::Error> > > @ 0x435bbe -> 0x6cb790 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<enum2$<tiny_glade::run_game::WindowBackendWindow> > @ 0x436115 -> 0x6cb790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::impl$16::fmt
// 信息 ID: 49f6712877810aea07c9a507
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::impl$1::drop
// 信息 ID: 473085c64fe0e4c6d68fcc27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<usdl::video::Window> @ 0x99778 -> 0x6cb7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<usdl::video::WindowBuilder> @ 0x9bce8 -> 0x6cb7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<usdl::video::VideoSubsystem> @ 0x9da06 -> 0x6cb7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<tiny_glade::sdl_setup::SdlWindowBackend> @ 0xac0f6 -> 0x6cb7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f9039 -> 0x6cb7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8e75 -> 0x6cb7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x430a5f -> 0x6cb7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x4308da -> 0x6cb7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42f6b9 -> 0x6cb7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42e37e -> 0x6cb7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<enum2$<tiny_glade::run_game::WindowBackend>,enum2$<tiny_glade::run_game::WindowBackendWindow> >,anyhow::Error> > > @ 0x435bd0 -> 0x6cb7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<enum2$<tiny_glade::run_game::WindowBackendWindow> > @ 0x436127 -> 0x6cb7b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::impl$24::fmt
// 信息 ID: 9659e44eeac1f20bb7aeface
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<usdl::video::DisplayMode>; module=16, inline_record=7860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::impl$2::clone
// 信息 ID: ce03b3d75b1ef0a6eb44b08d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl::video::WindowBuilder::build; module=285, inline_record=644; 不是独立调用边。
// [PDB 内联宿主] usdl::video::VideoSubsystem::window; module=285, inline_record=4708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::impl$39::fmt
// 信息 ID: 4c82154659cf9efd14711f30
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::impl$5::build::closure$0
// 信息 ID: 7af7da953fdaa30adbcb78f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "usdl::video::impl$5::build", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl::video::WindowBuilder::build; module=285, inline_record=612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::impl$6::fmt
// 信息 ID: 85600907443b269d25b74975
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::impl$9::window_handle
// 信息 ID: 54916782bfed81c1bf7fe1e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::setup::create_swapchain_with_fallback<ref_mut$<enum2$<tiny_glade::run_game::WindowBackendWindow> > > @ 0x412ffb -> 0x6cb280 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls> @ 0x52f750 -> 0x6cb280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::is_sdl_non_fourcc_format
// 信息 ID: f759eea18b7262799c0acfd0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl::video::PixelFormatEnum::byte_size_per_pixel; module=285, inline_record=5328; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::video::window_handle_from_wm_info
// 信息 ID: c74c978b219af9f1ba4c6273
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl::video::Window::window_handle; module=285, inline_record=5724; 不是独立调用边。
// [PDB 内联宿主] usdl::video::impl$9::window_handle; module=285, inline_record=7824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
