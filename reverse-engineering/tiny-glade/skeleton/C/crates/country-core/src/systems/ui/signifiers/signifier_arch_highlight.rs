// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/ui/signifiers/signifier_arch_highlight.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d8040e9b2ac913ab25ee224f384137be2338e182f8843c34201e0082a28597ee
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: signifier_arch_highlight.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 562, 600, 1643, 1682
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::signifiers::signifier_arch_highlight::append_arch_highlights
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":600,"record_offset":56268,"rva":12891376,"va":5381600496}
// 编译实例 RVAs: 0xc4b4f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-5921976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381600496,"raw_hex":"e848a3a5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381600659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-5923084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381600496,"raw_hex":"e8f49ea5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381601783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: signifier_arch_highlight.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12878453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381600496,"raw_hex":"e88b7d3bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381600640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c922e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: signifier_arch_highlight.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t288679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381600496,"raw_hex":"e8a7670400","size":5,"soft_fail":false,"target_va":5381890784,"va":5381602100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418055f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: signifier_arch_highlight.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12295181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381600496,"raw_hex":"e80d9cbb00","size":5,"soft_fail":false,"target_va":5393896944,"va":5381601758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: signifier_arch_highlight.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34750699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381600496,"raw_hex":"e8eb401202","size":5,"soft_fail":false,"target_va":5416352064,"va":5381601360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: signifier_arch_highlight.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34751872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381600496,"raw_hex":"e880451202","size":5,"soft_fail":false,"target_va":5416353040,"va":5381601163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: signifier_arch_highlight.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35456336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381600496,"raw_hex":"e850051d02","size":5,"soft_fail":false,"target_va":5417057056,"va":5381600715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35018874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381600496,"raw_hex":"e87a581602","size":5,"soft_fail":false,"target_va":5416620769,"va":5381601890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3193358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381600496,"raw_hex":"e80eba3000","size":5,"soft_fail":false,"target_va":5384794864,"va":5381601501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35018939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381600496,"raw_hex":"e8bb581602","size":5,"soft_fail":false,"target_va":5416620794,"va":5381601850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35019844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381600496,"raw_hex":"e8445c1602","size":5,"soft_fail":false,"target_va":5416621762,"va":5381601913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":600,"record_offset":58928,"parent_record_offset":56268,"ancestor_record_offset":56268,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":600,"record_offset":59040,"parent_record_offset":56268,"ancestor_record_offset":56268,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":58716,"parent_record_offset":56268,"ancestor_record_offset":56268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":56668,"parent_record_offset":56268,"ancestor_record_offset":56268,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":58620,"parent_record_offset":56268,"ancestor_record_offset":56268,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":57288,"parent_record_offset":56268,"ancestor_record_offset":56268,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":600,"record_offset":59012,"parent_record_offset":56268,"ancestor_record_offset":56268,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":600,"record_offset":56572,"parent_record_offset":56268,"ancestor_record_offset":56268,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":600,"record_offset":58480,"parent_record_offset":56268,"ancestor_record_offset":56268,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":60456,"parent_record_offset":56268,"ancestor_record_offset":56268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":600,"record_offset":56488,"parent_record_offset":56268,"ancestor_record_offset":56268,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":600,"record_offset":59616,"parent_record_offset":56268,"ancestor_record_offset":56268,"inlinee":28605,"source_line":191,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":600,"record_offset":60428,"parent_record_offset":56268,"ancestor_record_offset":56268,"inlinee":28609,"source_line":61,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":57088,"parent_record_offset":57064,"ancestor_record_offset":56268,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":57112,"parent_record_offset":57088,"ancestor_record_offset":56268,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":59496,"parent_record_offset":59472,"ancestor_record_offset":56268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":59520,"parent_record_offset":59496,"ancestor_record_offset":56268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":57136,"parent_record_offset":57112,"ancestor_record_offset":56268,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":59544,"parent_record_offset":59520,"ancestor_record_offset":56268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":57064,"parent_record_offset":56888,"ancestor_record_offset":56268,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":59472,"parent_record_offset":59384,"ancestor_record_offset":56268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":600,"record_offset":59112,"parent_record_offset":59072,"ancestor_record_offset":56268,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":600,"record_offset":59072,"parent_record_offset":59040,"ancestor_record_offset":56268,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":600,"record_offset":58976,"parent_record_offset":58952,"ancestor_record_offset":56268,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":58352,"parent_record_offset":58328,"ancestor_record_offset":56268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":57424,"parent_record_offset":57392,"ancestor_record_offset":56268,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":56828,"parent_record_offset":56800,"ancestor_record_offset":56268,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":57520,"parent_record_offset":57492,"ancestor_record_offset":56268,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":59412,"parent_record_offset":59384,"ancestor_record_offset":56268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":59384,"parent_record_offset":59360,"ancestor_record_offset":56268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":57492,"parent_record_offset":57456,"ancestor_record_offset":56268,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":58400,"parent_record_offset":58376,"ancestor_record_offset":56268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":58376,"parent_record_offset":58352,"ancestor_record_offset":56268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":57456,"parent_record_offset":57424,"ancestor_record_offset":56268,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":56888,"parent_record_offset":56856,"ancestor_record_offset":56268,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":56856,"parent_record_offset":56828,"ancestor_record_offset":56268,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":59360,"parent_record_offset":59336,"ancestor_record_offset":56268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":56924,"parent_record_offset":56888,"ancestor_record_offset":56268,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":600,"record_offset":59736,"parent_record_offset":59708,"ancestor_record_offset":56268,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":600,"record_offset":59764,"parent_record_offset":59736,"ancestor_record_offset":56268,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":600,"record_offset":56760,"parent_record_offset":56696,"ancestor_record_offset":56268,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":600,"record_offset":59708,"parent_record_offset":59680,"ancestor_record_offset":56268,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":600,"record_offset":59680,"parent_record_offset":59652,"ancestor_record_offset":56268,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":600,"record_offset":59652,"parent_record_offset":59616,"ancestor_record_offset":56268,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":600,"record_offset":60296,"parent_record_offset":60272,"ancestor_record_offset":56268,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":58328,"parent_record_offset":57344,"ancestor_record_offset":56268,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":56732,"parent_record_offset":56696,"ancestor_record_offset":56268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":57344,"parent_record_offset":57316,"ancestor_record_offset":56268,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":58740,"parent_record_offset":58716,"ancestor_record_offset":56268,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":57392,"parent_record_offset":57344,"ancestor_record_offset":56268,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":57248,"parent_record_offset":56760,"ancestor_record_offset":56268,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":56800,"parent_record_offset":56760,"ancestor_record_offset":56268,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":58100,"parent_record_offset":58072,"ancestor_record_offset":56268,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":56696,"parent_record_offset":56668,"ancestor_record_offset":56268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":600,"record_offset":58440,"parent_record_offset":57344,"ancestor_record_offset":56268,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":600,"record_offset":58268,"parent_record_offset":58244,"ancestor_record_offset":56268,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":600,"record_offset":57316,"parent_record_offset":57288,"ancestor_record_offset":56268,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":600,"record_offset":56948,"parent_record_offset":56924,"ancestor_record_offset":56268,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":600,"record_offset":58576,"parent_record_offset":58552,"ancestor_record_offset":56268,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":600,"record_offset":58552,"parent_record_offset":58528,"ancestor_record_offset":56268,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":600,"record_offset":57024,"parent_record_offset":56972,"ancestor_record_offset":56268,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$3::rfold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":600,"record_offset":57640,"parent_record_offset":57608,"ancestor_record_offset":56268,"inlinee":26353,"source_line":161,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":600,"record_offset":57812,"parent_record_offset":57672,"ancestor_record_offset":56268,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":600,"record_offset":57608,"parent_record_offset":57576,"ancestor_record_offset":56268,"inlinee":10948,"source_line":79,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":600,"record_offset":58504,"parent_record_offset":58480,"ancestor_record_offset":56268,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":600,"record_offset":57724,"parent_record_offset":57672,"ancestor_record_offset":56268,"inlinee":17821,"source_line":124,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":600,"record_offset":58528,"parent_record_offset":58504,"ancestor_record_offset":56268,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":600,"record_offset":57764,"parent_record_offset":57724,"ancestor_record_offset":56268,"inlinee":17822,"source_line":355,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::double_ended::DoubleEndedIterator::rfold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\double_ended.rs
//     evidence={"module":600,"record_offset":57672,"parent_record_offset":57640,"ancestor_record_offset":56268,"inlinee":10950,"source_line":301,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":600,"record_offset":57576,"parent_record_offset":57344,"ancestor_record_offset":56268,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":600,"record_offset":58072,"parent_record_offset":57812,"ancestor_record_offset":56268,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":56972,"parent_record_offset":56948,"ancestor_record_offset":56268,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":56996,"parent_record_offset":56972,"ancestor_record_offset":56268,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":59440,"parent_record_offset":59412,"ancestor_record_offset":56268,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":600,"record_offset":57908,"parent_record_offset":57868,"ancestor_record_offset":56268,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":600,"record_offset":58032,"parent_record_offset":58004,"ancestor_record_offset":56268,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":600,"record_offset":58952,"parent_record_offset":58928,"ancestor_record_offset":56268,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":600,"record_offset":57192,"parent_record_offset":56760,"ancestor_record_offset":56268,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":57216,"parent_record_offset":57192,"ancestor_record_offset":56268,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":58172,"parent_record_offset":57672,"ancestor_record_offset":56268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":600,"record_offset":56536,"parent_record_offset":56512,"ancestor_record_offset":56268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":600,"record_offset":56628,"parent_record_offset":56600,"ancestor_record_offset":56268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":58128,"parent_record_offset":58100,"ancestor_record_offset":56268,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":600,"record_offset":58648,"parent_record_offset":58620,"ancestor_record_offset":56268,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":600,"record_offset":58676,"parent_record_offset":58648,"ancestor_record_offset":56268,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":600,"record_offset":56512,"parent_record_offset":56488,"ancestor_record_offset":56268,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":600,"record_offset":56600,"parent_record_offset":56572,"ancestor_record_offset":56268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":600,"record_offset":60332,"parent_record_offset":60296,"ancestor_record_offset":56268,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":600,"record_offset":60356,"parent_record_offset":60332,"ancestor_record_offset":56268,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":600,"record_offset":60388,"parent_record_offset":60296,"ancestor_record_offset":56268,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::signifiers::signifier_arch_highlight::append_arch_highlights::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_arch_highlight.rs
//     evidence={"module":600,"record_offset":57840,"parent_record_offset":57812,"ancestor_record_offset":56268,"inlinee":35372,"source_line":30,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":58004,"parent_record_offset":57840,"ancestor_record_offset":56268,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":57868,"parent_record_offset":57840,"ancestor_record_offset":56268,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::signifiers::signifier_arch_highlight::append_arch_highlights::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8A2C
// 首条位置: {"module":562,"inlinee":35372}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":58004,"parent_record_offset":57840,"ancestor_record_offset":57840,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":57868,"parent_record_offset":57840,"ancestor_record_offset":57840,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":600,"record_offset":57908,"parent_record_offset":57868,"ancestor_record_offset":57840,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":600,"record_offset":58032,"parent_record_offset":58004,"ancestor_record_offset":57840,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::signifiers::signifier_arch_highlight::append_arch_highlights::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":600,"record_offset":61320,"rva":12893232,"va":5381602352}
// 编译实例 RVAs: 0xc4bc30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5923731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381602352,"raw_hex":"e86d9ca5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381602414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5924160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381602352,"raw_hex":"e8c09aa5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381602859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: signifier_arch_highlight.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12880212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381602352,"raw_hex":"e8ac763bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381602399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: signifier_arch_highlight.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35454498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381602352,"raw_hex":"e822fe1c02","size":5,"soft_fail":false,"target_va":5417057056,"va":5381602553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35017961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381602352,"raw_hex":"e8e9541602","size":5,"soft_fail":false,"target_va":5416620769,"va":5381602803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35018317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381602352,"raw_hex":"e84d561602","size":5,"soft_fail":false,"target_va":5416620794,"va":5381602472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2663973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381602352,"raw_hex":"e8db59d7ff","size":5,"soft_fail":false,"target_va":5378938736,"va":5381602704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":600,"record_offset":62348,"parent_record_offset":61320,"ancestor_record_offset":61320,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":600,"record_offset":61524,"parent_record_offset":61320,"ancestor_record_offset":61320,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":61704,"parent_record_offset":61320,"ancestor_record_offset":61320,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":62432,"parent_record_offset":61320,"ancestor_record_offset":61320,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::line_topology
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":600,"record_offset":62320,"parent_record_offset":61320,"ancestor_record_offset":61320,"inlinee":28623,"source_line":284,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":600,"record_offset":62292,"parent_record_offset":61320,"ancestor_record_offset":61320,"inlinee":28622,"source_line":234,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":61608,"parent_record_offset":61584,"ancestor_record_offset":61320,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":61632,"parent_record_offset":61608,"ancestor_record_offset":61320,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":62504,"parent_record_offset":62480,"ancestor_record_offset":61320,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":62528,"parent_record_offset":62504,"ancestor_record_offset":61320,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":61656,"parent_record_offset":61632,"ancestor_record_offset":61320,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":62552,"parent_record_offset":62528,"ancestor_record_offset":61320,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":61584,"parent_record_offset":61552,"ancestor_record_offset":61320,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":62480,"parent_record_offset":62456,"ancestor_record_offset":61320,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":600,"record_offset":61552,"parent_record_offset":61524,"ancestor_record_offset":61320,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":600,"record_offset":62456,"parent_record_offset":62432,"ancestor_record_offset":61320,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":61860,"parent_record_offset":61836,"ancestor_record_offset":61320,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":61916,"parent_record_offset":61884,"ancestor_record_offset":61320,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":61884,"parent_record_offset":61860,"ancestor_record_offset":61320,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":61952,"parent_record_offset":61916,"ancestor_record_offset":61320,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":600,"record_offset":62372,"parent_record_offset":62348,"ancestor_record_offset":61320,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":600,"record_offset":61796,"parent_record_offset":61732,"ancestor_record_offset":61320,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":61768,"parent_record_offset":61732,"ancestor_record_offset":61320,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":62396,"parent_record_offset":62372,"ancestor_record_offset":61320,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":61836,"parent_record_offset":61796,"ancestor_record_offset":61320,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":61732,"parent_record_offset":61704,"ancestor_record_offset":61320,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":600,"record_offset":61976,"parent_record_offset":61952,"ancestor_record_offset":61320,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":600,"record_offset":62056,"parent_record_offset":62000,"ancestor_record_offset":61320,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":62000,"parent_record_offset":61976,"ancestor_record_offset":61320,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":62028,"parent_record_offset":62000,"ancestor_record_offset":61320,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":600,"record_offset":62224,"parent_record_offset":61796,"ancestor_record_offset":61320,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":62248,"parent_record_offset":62224,"ancestor_record_offset":61320,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00600.jsonl.gz"}
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
// 完整结构及原始类型记录: signifier_arch_highlight.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: signifier_arch_highlight.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: signifier_arch_highlight.rs.enriched.json.gz
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
// 函数: country_core::systems::ui::signifiers::signifier_arch_highlight::append_arch_highlights
// 信息 ID: f38481d92f71a5912c82c1ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::ui_arch_controls::do_ui_arch_controls @ 0x80fe2a -> 0xc4b4f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::signifiers::signifier_arch_highlight::append_arch_highlights::closure$0
// 信息 ID: 9d4d2009274d1198694139ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ui::signifiers::signifier_arch_highlight::append_arch_highlights", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::signifiers::signifier_arch_highlight::append_arch_highlights; module=600, inline_record=57840; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::signifiers::signifier_arch_highlight::append_arch_highlights::closure$1
// 信息 ID: dd6cea3ce1262ef43ad331da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::ui::signifiers::signifier_arch_highlight::append_arch_highlights", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ui::signifiers::signifier_arch_highlight::append_arch_highlights::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::systems::ui::signifiers::signifier_arch_highlight::append_arch_highlights::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui::signifiers::signifier_arch_highlight::append_arch_highlights::closure_env$1 <- core::ptr::drop_in_place<country_core::systems::ui::signifiers::signifier_arch_highlight::append_arch_highlights::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::ui::signifiers::signifier_arch_highlight::append_arch_highlights::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xc4d5c3 -> 0xc4bc30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
