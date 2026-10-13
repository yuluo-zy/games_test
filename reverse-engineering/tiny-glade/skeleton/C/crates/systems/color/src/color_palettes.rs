// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/color/src/color_palettes.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: be6f83c2ef43e67dc2bb3139571ef0baacfa052ecf92a215f23dbe1f6f7652fa
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: color_palettes.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 4791
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use system_color; // namespace candidate
//   package identities: [{"name":"system-color","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::color_palettes::roof_colors
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4791,"record_offset":268,"rva":45177632,"va":5413886752}
// 编译实例 RVAs: 0x2b15b20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-38208122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413886752,"raw_hex":"e886fdb8fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5413886805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=13
//     evidence={"assembly":"\tcallq\t-38210144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413886752,"raw_hex":"e8a0f5b8fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5413888843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=14
//     target_aliases=1699 candidates; complete list: color_palettes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-45164603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413886752,"raw_hex":"e8c5d74efd","size":5,"soft_fail":false,"target_va":5368722192,"va":5413886790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2732502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413886752,"raw_hex":"e8d6b12900","size":5,"soft_fail":false,"target_va":5416620769,"va":5413888262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t2732507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413886752,"raw_hex":"e8dbb12900","size":5,"soft_fail":false,"target_va":5416620794,"va":5413888282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4791,"record_offset":7980,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4791,"record_offset":408,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4791,"record_offset":8372,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_direct] system_color::radial_menu::IconInfo::new_color
//     owner=private_source_ownership_candidate; count=14
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/radial_menu/mod.rs
//     evidence={"module":4791,"record_offset":588,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":134802,"source_line":124,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":492,"parent_record_offset":468,"ancestor_record_offset":268,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":516,"parent_record_offset":492,"ancestor_record_offset":268,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":8444,"parent_record_offset":8420,"ancestor_record_offset":268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":8468,"parent_record_offset":8444,"ancestor_record_offset":268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":268,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":8492,"parent_record_offset":8468,"ancestor_record_offset":268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":468,"parent_record_offset":436,"ancestor_record_offset":268,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":8420,"parent_record_offset":8396,"ancestor_record_offset":268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4791,"record_offset":436,"parent_record_offset":408,"ancestor_record_offset":268,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4791,"record_offset":8396,"parent_record_offset":8372,"ancestor_record_offset":268,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4791,"record_offset":848,"parent_record_offset":820,"ancestor_record_offset":268,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4791,"record_offset":8660,"parent_record_offset":8636,"ancestor_record_offset":268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4791,"record_offset":908,"parent_record_offset":876,"ancestor_record_offset":268,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4791,"record_offset":876,"parent_record_offset":848,"ancestor_record_offset":268,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4791,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4791,"record_offset":8004,"parent_record_offset":7980,"ancestor_record_offset":268,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4791,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":268,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4791,"record_offset":756,"parent_record_offset":728,"ancestor_record_offset":268,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4791,"record_offset":784,"parent_record_offset":756,"ancestor_record_offset":268,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4791,"record_offset":700,"parent_record_offset":672,"ancestor_record_offset":268,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":4791,"record_offset":672,"parent_record_offset":644,"ancestor_record_offset":268,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":4791,"record_offset":644,"parent_record_offset":616,"ancestor_record_offset":268,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4791,"record_offset":8028,"parent_record_offset":8004,"ancestor_record_offset":268,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4791,"record_offset":820,"parent_record_offset":784,"ancestor_record_offset":268,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":4791,"record_offset":616,"parent_record_offset":588,"ancestor_record_offset":268,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4791,"record_offset":1064,"parent_record_offset":784,"ancestor_record_offset":268,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4791,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":268,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4791,"record_offset":8564,"parent_record_offset":8540,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_palettes::wood_colors
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4791,"record_offset":23176,"rva":45183712,"va":5413892832}
// 编译实例 RVAs: 0x2b172e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t-38214202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413892832,"raw_hex":"e8c6e5b8fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5413892885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=11
//     evidence={"assembly":"\tcallq\t-38215904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413892832,"raw_hex":"e820dfb8fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5413894603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=12
//     target_aliases=1699 candidates; complete list: color_palettes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-45170683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413892832,"raw_hex":"e805c04efd","size":5,"soft_fail":false,"target_va":5368722192,"va":5413892870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2726644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413892832,"raw_hex":"e8f49a2900","size":5,"soft_fail":false,"target_va":5416620769,"va":5413894120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t2726649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413892832,"raw_hex":"e8f99a2900","size":5,"soft_fail":false,"target_va":5416620794,"va":5413894140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4791,"record_offset":29700,"parent_record_offset":23176,"ancestor_record_offset":23176,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4791,"record_offset":23316,"parent_record_offset":23176,"ancestor_record_offset":23176,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4791,"record_offset":30092,"parent_record_offset":23176,"ancestor_record_offset":23176,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_direct] system_color::radial_menu::IconInfo::new_color
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/radial_menu/mod.rs
//     evidence={"module":4791,"record_offset":23496,"parent_record_offset":23176,"ancestor_record_offset":23176,"inlinee":134802,"source_line":124,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":23400,"parent_record_offset":23376,"ancestor_record_offset":23176,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":23424,"parent_record_offset":23400,"ancestor_record_offset":23176,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":30164,"parent_record_offset":30140,"ancestor_record_offset":23176,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":30188,"parent_record_offset":30164,"ancestor_record_offset":23176,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":23448,"parent_record_offset":23424,"ancestor_record_offset":23176,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":30212,"parent_record_offset":30188,"ancestor_record_offset":23176,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":23376,"parent_record_offset":23344,"ancestor_record_offset":23176,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":30140,"parent_record_offset":30116,"ancestor_record_offset":23176,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4791,"record_offset":23344,"parent_record_offset":23316,"ancestor_record_offset":23176,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4791,"record_offset":30116,"parent_record_offset":30092,"ancestor_record_offset":23176,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4791,"record_offset":23756,"parent_record_offset":23728,"ancestor_record_offset":23176,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4791,"record_offset":30380,"parent_record_offset":30356,"ancestor_record_offset":23176,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4791,"record_offset":23816,"parent_record_offset":23784,"ancestor_record_offset":23176,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4791,"record_offset":23784,"parent_record_offset":23756,"ancestor_record_offset":23176,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4791,"record_offset":30356,"parent_record_offset":30332,"ancestor_record_offset":23176,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4791,"record_offset":29724,"parent_record_offset":29700,"ancestor_record_offset":23176,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4791,"record_offset":23636,"parent_record_offset":23608,"ancestor_record_offset":23176,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4791,"record_offset":23664,"parent_record_offset":23636,"ancestor_record_offset":23176,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4791,"record_offset":23692,"parent_record_offset":23664,"ancestor_record_offset":23176,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4791,"record_offset":23608,"parent_record_offset":23580,"ancestor_record_offset":23176,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":4791,"record_offset":23580,"parent_record_offset":23552,"ancestor_record_offset":23176,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":4791,"record_offset":23552,"parent_record_offset":23524,"ancestor_record_offset":23176,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4791,"record_offset":29748,"parent_record_offset":29724,"ancestor_record_offset":23176,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4791,"record_offset":23728,"parent_record_offset":23692,"ancestor_record_offset":23176,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":4791,"record_offset":23524,"parent_record_offset":23496,"ancestor_record_offset":23176,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4791,"record_offset":23972,"parent_record_offset":23692,"ancestor_record_offset":23176,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4791,"record_offset":23996,"parent_record_offset":23972,"ancestor_record_offset":23176,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4791,"record_offset":30284,"parent_record_offset":30260,"ancestor_record_offset":23176,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_palettes::wall_colors
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4791,"record_offset":11904,"rva":45180720,"va":5413889840}
// 编译实例 RVAs: 0x2b16730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-38211210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413889840,"raw_hex":"e876f1b8fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5413889893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=13
//     evidence={"assembly":"\tcallq\t-38213136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413889840,"raw_hex":"e8f0e9b8fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5413891835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=14
//     target_aliases=1699 candidates; complete list: color_palettes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-45167691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413889840,"raw_hex":"e8b5cb4efd","size":5,"soft_fail":false,"target_va":5368722192,"va":5413889878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2729435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413889840,"raw_hex":"e8dba52900","size":5,"soft_fail":false,"target_va":5416620769,"va":5413891329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t2729440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413889840,"raw_hex":"e8e0a52900","size":5,"soft_fail":false,"target_va":5416620794,"va":5413891349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4791,"record_offset":19560,"parent_record_offset":11904,"ancestor_record_offset":11904,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4791,"record_offset":12048,"parent_record_offset":11904,"ancestor_record_offset":11904,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4791,"record_offset":19644,"parent_record_offset":11904,"ancestor_record_offset":11904,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_direct] system_color::radial_menu::IconInfo::new_color
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/radial_menu/mod.rs
//     evidence={"module":4791,"record_offset":12228,"parent_record_offset":11904,"ancestor_record_offset":11904,"inlinee":134802,"source_line":124,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_direct] system_color::radial_menu::IconInfo::new_color_array
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/radial_menu/mod.rs
//     evidence={"module":4791,"record_offset":15048,"parent_record_offset":11904,"ancestor_record_offset":11904,"inlinee":134874,"source_line":132,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":12132,"parent_record_offset":12108,"ancestor_record_offset":11904,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":12156,"parent_record_offset":12132,"ancestor_record_offset":11904,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":19716,"parent_record_offset":19692,"ancestor_record_offset":11904,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":19740,"parent_record_offset":19716,"ancestor_record_offset":11904,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":12180,"parent_record_offset":12156,"ancestor_record_offset":11904,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":19764,"parent_record_offset":19740,"ancestor_record_offset":11904,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":12108,"parent_record_offset":12076,"ancestor_record_offset":11904,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4791,"record_offset":19692,"parent_record_offset":19668,"ancestor_record_offset":11904,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4791,"record_offset":12076,"parent_record_offset":12048,"ancestor_record_offset":11904,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4791,"record_offset":19668,"parent_record_offset":19644,"ancestor_record_offset":11904,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4791,"record_offset":12488,"parent_record_offset":12460,"ancestor_record_offset":11904,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4791,"record_offset":19932,"parent_record_offset":19908,"ancestor_record_offset":11904,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4791,"record_offset":12548,"parent_record_offset":12516,"ancestor_record_offset":11904,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4791,"record_offset":12516,"parent_record_offset":12488,"ancestor_record_offset":11904,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4791,"record_offset":19908,"parent_record_offset":19884,"ancestor_record_offset":11904,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4791,"record_offset":19584,"parent_record_offset":19560,"ancestor_record_offset":11904,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4791,"record_offset":12368,"parent_record_offset":12340,"ancestor_record_offset":11904,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4791,"record_offset":12396,"parent_record_offset":12368,"ancestor_record_offset":11904,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4791,"record_offset":12424,"parent_record_offset":12396,"ancestor_record_offset":11904,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4791,"record_offset":12340,"parent_record_offset":12312,"ancestor_record_offset":11904,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":4791,"record_offset":12312,"parent_record_offset":12284,"ancestor_record_offset":11904,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":4791,"record_offset":12284,"parent_record_offset":12256,"ancestor_record_offset":11904,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4791,"record_offset":19608,"parent_record_offset":19584,"ancestor_record_offset":11904,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4791,"record_offset":12460,"parent_record_offset":12424,"ancestor_record_offset":11904,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":4791,"record_offset":12256,"parent_record_offset":12228,"ancestor_record_offset":11904,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4791,"record_offset":12704,"parent_record_offset":12424,"ancestor_record_offset":11904,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4791,"record_offset":12728,"parent_record_offset":12704,"ancestor_record_offset":11904,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04791.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4791,"record_offset":19836,"parent_record_offset":19812,"ancestor_record_offset":11904,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04791.jsonl.gz"}
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
// 完整结构及原始类型记录: color_palettes.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: color_palettes.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: color_palettes.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_color::color_palettes::roof_colors
// 信息 ID: 229fa254abda1806c35a0a26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::style_picker_feedback::picker_cursor @ 0x28913ef -> 0x2b15b20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_palettes::wall_colors
// 信息 ID: e1e82ecb5ab0df325749eac4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::style_picker_feedback::picker_cursor @ 0x2891186 -> 0x2b16730 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_palettes::wood_colors
// 信息 ID: 344640de1e27fc69111e4344
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::style_picker_feedback::picker_cursor @ 0x2890fff -> 0x2b172e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::style_picker_feedback::picker_cursor @ 0x289112b -> 0x2b172e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::show_color_menu::show_color_options::closure$0 @ 0x28aa3ec -> 0x2b172e0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
